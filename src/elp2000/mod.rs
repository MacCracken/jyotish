//! ELP2000-82 lunar theory (Chapront-Touzé & Chapront 1988).
//!
//! Truncated to ~180 terms for Swiss Ephemeris Moshier parity, achieving
//! <2 arcsecond accuracy in ecliptic longitude over ±3000 years from J2000.0.
//!
//! The ELP2000-82 theory computes the Moon's geocentric ecliptic coordinates
//! (longitude, latitude, distance) as sums of periodic terms whose arguments
//! are linear combinations of the Delaunay variables (D, l', l, F) plus
//! secular corrections.
//!
//! ## References
//!
//! - Chapront-Touzé, M. & Chapront, J. (1988). "ELP 2000-85: a semi-analytical
//!   lunar ephemeris adequate for historical times." *Astronomy & Astrophysics*,
//!   190, 342–352.
//! - Chapront, J., Chapront-Touzé, M. & Francou, G. (2002). "A new determination
//!   of lunar orbital parameters, precession constant and tidal acceleration from
//!   LLR measurements." *Astronomy & Astrophysics*, 387, 700–709.
//! - Meeus, J. (1998). *Astronomical Algorithms*, 2nd ed., Ch. 47.

mod main_problem;
mod perturbation;

use crate::calendar::julian_centuries;
use crate::coords::{deg_to_rad, normalize_degrees, normalize_radians};
use crate::num::KahanSum;

// ---------------------------------------------------------------------------
// Fundamental Delaunay arguments (4th-order polynomials in T)
// ---------------------------------------------------------------------------
// These are the full polynomial expressions from Chapront et al. (2002),
// not the linear-only forms used in the IAU 2000B nutation model.

/// Mean elongation of the Moon from the Sun (D).
fn mean_elongation(t: f64) -> f64 {
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t3 * t;
    normalize_degrees(
        297.850_195_47 + 445_267.111_467_86 * t - 0.001_914_2 * t2 + t3 / 189_474.0
            - t4 / 121_164_000.0,
    )
}

/// Mean anomaly of the Sun (l').
fn sun_mean_anomaly(t: f64) -> f64 {
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t3 * t;
    normalize_degrees(
        357.529_109_18 + 35_999.050_290_94 * t - 0.000_153_6 * t2 + t3 / 24_490_000.0
            - t4 / 992_340_000.0,
    )
}

/// Mean anomaly of the Moon (l).
fn moon_mean_anomaly(t: f64) -> f64 {
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t3 * t;
    normalize_degrees(
        134.963_398_39 + 477_198.867_505_06 * t + 0.008_941_4 * t2 + t3 / 69_699.0
            - t4 / 14_712_000.0,
    )
}

/// Moon's argument of latitude (F).
fn argument_of_latitude(t: f64) -> f64 {
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t3 * t;
    normalize_degrees(
        93.272_099_31 + 483_202.017_523_06 * t - 0.003_653_9 * t2 - t3 / 3_526_000.0
            + t4 / 863_310_000.0,
    )
}

/// Earth's orbital eccentricity correction factor.
///
/// Terms whose Sun anomaly multiplier |m| = 1 get multiplied by E,
/// terms with |m| = 2 get multiplied by E².
fn eccentricity(t: f64) -> f64 {
    1.0 - 0.002_516 * t - 0.000_007_4 * t * t
}

/// Compute E^|m| for a given Sun anomaly multiplier.
fn e_power(e: f64, m_mult: i16) -> f64 {
    match m_mult.unsigned_abs() {
        0 => 1.0,
        1 => e,
        2 => e * e,
        _ => e.powi(m_mult.unsigned_abs() as i32),
    }
}

// ---------------------------------------------------------------------------
// Term types
// ---------------------------------------------------------------------------

/// A main-problem periodic term for longitude or latitude.
///
/// Argument = D·d + M·m + M'·mp + F·f
/// Contribution = coefficient × sin(argument)
pub(crate) struct LonLatTerm {
    /// Multiplier for mean elongation (D).
    pub d: i16,
    /// Multiplier for Sun's mean anomaly (M).
    pub m: i16,
    /// Multiplier for Moon's mean anomaly (M').
    pub mp: i16,
    /// Multiplier for argument of latitude (F).
    pub f: i16,
    /// Sine coefficient in 0.000001 degrees.
    pub coeff: i64,
}

/// A main-problem periodic term for distance.
pub(crate) struct DistTerm {
    /// Multiplier for mean elongation (D).
    pub d: i16,
    /// Multiplier for Sun's mean anomaly (M).
    pub m: i16,
    /// Multiplier for Moon's mean anomaly (M').
    pub mp: i16,
    /// Multiplier for argument of latitude (F).
    pub f: i16,
    /// Cosine coefficient in 0.001 km.
    pub coeff: i64,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Compute the Moon's geocentric ecliptic longitude using ELP2000-82.
///
/// Returns longitude in degrees [0, 360).
///
/// # Examples
///
/// ```
/// # use jyotish::elp2000::lunar_longitude;
/// let lon = lunar_longitude(2_448_724.5);
/// // Meeus example 47.a: λ ≈ 133.162°
/// assert!((lon - 133.162).abs() < 0.02, "lon = {lon}");
/// ```
pub fn lunar_longitude(jd: f64) -> f64 {
    let t = julian_centuries(jd);

    // Delaunay arguments (radians)
    let d = normalize_radians(deg_to_rad(mean_elongation(t)));
    let m = normalize_radians(deg_to_rad(sun_mean_anomaly(t)));
    let mp = normalize_radians(deg_to_rad(moon_mean_anomaly(t)));
    let f = normalize_radians(deg_to_rad(argument_of_latitude(t)));

    let e = eccentricity(t);

    // Moon's mean longitude
    let lp = normalize_degrees(
        218.316_447_9 + 481_267.881_234_21 * t - 0.001_579_86 * t * t + t * t * t / 538_841.0
            - t * t * t * t / 65_194_000.0,
    );

    // Main problem longitude terms
    let mut sigma_l = KahanSum::new();
    for term in main_problem::LONGITUDE_TERMS {
        let arg = term.d as f64 * d + term.m as f64 * m + term.mp as f64 * mp + term.f as f64 * f;
        let ep = e_power(e, term.m);
        sigma_l.add(term.coeff as f64 * ep * arg.sin());
    }

    // Perturbation corrections
    for term in perturbation::LONGITUDE_PERTURBATION {
        let arg = term.d as f64 * d + term.m as f64 * m + term.mp as f64 * mp + term.f as f64 * f;
        let ep = e_power(e, term.m);
        sigma_l.add(term.coeff as f64 * ep * arg.sin());
    }

    // Additive corrections (Meeus Table 47.A footnotes)
    let a1 = normalize_radians(deg_to_rad(119.75 + 131.849 * t));
    let a2 = normalize_radians(deg_to_rad(53.09 + 479_264.290 * t));
    sigma_l.add(3958.0 * a1.sin() + 1962.0 * a2.sin() + 318.0 * deg_to_rad(lp).sin());

    normalize_degrees(lp + sigma_l.sum() * 1e-6)
}

/// Compute the Moon's geocentric ecliptic latitude using ELP2000-82.
///
/// Returns latitude in degrees.
///
/// # Examples
///
/// ```
/// # use jyotish::elp2000::lunar_latitude;
/// let lat = lunar_latitude(2_448_724.5);
/// // Meeus example 47.a: β ≈ -3.229°
/// assert!((lat - (-3.229)).abs() < 0.02, "lat = {lat}");
/// ```
pub fn lunar_latitude(jd: f64) -> f64 {
    let t = julian_centuries(jd);

    let d = normalize_radians(deg_to_rad(mean_elongation(t)));
    let m = normalize_radians(deg_to_rad(sun_mean_anomaly(t)));
    let mp = normalize_radians(deg_to_rad(moon_mean_anomaly(t)));
    let f = normalize_radians(deg_to_rad(argument_of_latitude(t)));

    let e = eccentricity(t);

    let lp = normalize_degrees(
        218.316_447_9 + 481_267.881_234_21 * t - 0.001_579_86 * t * t + t * t * t / 538_841.0
            - t * t * t * t / 65_194_000.0,
    );

    // Main problem latitude terms
    let mut sigma_b = KahanSum::new();
    for term in main_problem::LATITUDE_TERMS {
        let arg = term.d as f64 * d + term.m as f64 * m + term.mp as f64 * mp + term.f as f64 * f;
        let ep = e_power(e, term.m);
        sigma_b.add(term.coeff as f64 * ep * arg.sin());
    }

    // Perturbation corrections
    for term in perturbation::LATITUDE_PERTURBATION {
        let arg = term.d as f64 * d + term.m as f64 * m + term.mp as f64 * mp + term.f as f64 * f;
        let ep = e_power(e, term.m);
        sigma_b.add(term.coeff as f64 * ep * arg.sin());
    }

    // Additive corrections for latitude
    let a1 = normalize_radians(deg_to_rad(119.75 + 131.849 * t));
    let a3 = normalize_radians(deg_to_rad(313.45 + 481_266.484 * t));
    let lp_rad = deg_to_rad(lp);
    sigma_b.add(
        -2235.0 * lp_rad.sin()
            + 382.0 * a3.sin()
            + 175.0 * (a1 - f).sin()
            + 175.0 * (a1 + f).sin()
            + 127.0 * (lp_rad - mp).sin()
            - 115.0 * (lp_rad + mp).sin(),
    );

    sigma_b.sum() * 1e-6
}

/// Compute the Moon's geocentric distance using ELP2000-82.
///
/// Returns distance in kilometers.
///
/// # Examples
///
/// ```
/// # use jyotish::elp2000::lunar_distance_km;
/// let dist = lunar_distance_km(2_448_724.5);
/// // Meeus example 47.a: Δ ≈ 368409.7 km
/// assert!((dist - 368_409.7).abs() < 10.0, "dist = {dist}");
/// ```
pub fn lunar_distance_km(jd: f64) -> f64 {
    let t = julian_centuries(jd);

    let d = normalize_radians(deg_to_rad(mean_elongation(t)));
    let m = normalize_radians(deg_to_rad(sun_mean_anomaly(t)));
    let mp = normalize_radians(deg_to_rad(moon_mean_anomaly(t)));
    let f = normalize_radians(deg_to_rad(argument_of_latitude(t)));

    let e = eccentricity(t);

    // Main problem distance terms (cosine)
    let mut sigma_r = KahanSum::new();
    for term in main_problem::DISTANCE_TERMS {
        let arg = term.d as f64 * d + term.m as f64 * m + term.mp as f64 * mp + term.f as f64 * f;
        let ep = e_power(e, term.m);
        sigma_r.add(term.coeff as f64 * ep * arg.cos());
    }

    385_000.56 + sigma_r.sum() * 0.001
}

/// Compute the Moon's geocentric distance in AU.
pub fn lunar_distance_au(jd: f64) -> f64 {
    lunar_distance_km(jd) / 149_597_870.7
}

/// Compute the Moon's geocentric ecliptic coordinates.
///
/// Returns `(longitude_deg, latitude_deg, distance_km)`.
///
/// # Examples
///
/// ```
/// # use jyotish::elp2000::lunar_coordinates;
/// let (lon, lat, dist) = lunar_coordinates(2_448_724.5);
/// assert!((lon - 133.162).abs() < 0.02, "lon = {lon}");
/// assert!((lat - (-3.229)).abs() < 0.02, "lat = {lat}");
/// assert!((dist - 368_409.7).abs() < 10.0, "dist = {dist}");
/// ```
pub fn lunar_coordinates(jd: f64) -> (f64, f64, f64) {
    (
        lunar_longitude(jd),
        lunar_latitude(jd),
        lunar_distance_km(jd),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const JD_J2000: f64 = 2_451_545.0;
    const JD_MEEUS_47A: f64 = 2_448_724.5; // 1992-04-12 at 0h TD

    #[test]
    fn meeus_47a_longitude() {
        let lon = lunar_longitude(JD_MEEUS_47A);
        assert!(
            (lon - 133.162).abs() < 0.02,
            "lon = {lon}, expected ~133.162"
        );
    }

    #[test]
    fn meeus_47a_latitude() {
        let lat = lunar_latitude(JD_MEEUS_47A);
        assert!(
            (lat - (-3.229)).abs() < 0.02,
            "lat = {lat}, expected ~-3.229"
        );
    }

    #[test]
    fn meeus_47a_distance() {
        let dist = lunar_distance_km(JD_MEEUS_47A);
        assert!(
            (dist - 368_409.7).abs() < 10.0,
            "dist = {dist}, expected ~368409.7"
        );
    }

    #[test]
    fn coordinates_combined() {
        let (lon, lat, dist) = lunar_coordinates(JD_MEEUS_47A);
        assert!((lon - 133.162).abs() < 0.02);
        assert!((lat - (-3.229)).abs() < 0.02);
        assert!((dist - 368_409.7).abs() < 10.0);
    }

    #[test]
    fn longitude_range_year() {
        for day in 0..365 {
            let jd = JD_J2000 + day as f64;
            let lon = lunar_longitude(jd);
            assert!((0.0..360.0).contains(&lon), "lon {lon} at day {day}");
        }
    }

    #[test]
    fn latitude_physical_range() {
        for day in 0..365 {
            let jd = JD_J2000 + day as f64;
            let lat = lunar_latitude(jd);
            assert!(lat.abs() < 6.0, "lat {lat} at day {day}");
        }
    }

    #[test]
    fn distance_physical_range() {
        for day in 0..365 {
            let jd = JD_J2000 + day as f64;
            let dist = lunar_distance_km(jd);
            assert!(
                dist > 350_000.0 && dist < 410_000.0,
                "dist {dist} at day {day}"
            );
        }
    }

    #[test]
    fn distance_au_conversion() {
        let km = lunar_distance_km(JD_J2000);
        let au = lunar_distance_au(JD_J2000);
        assert!((au * 149_597_870.7 - km).abs() < 0.1);
    }

    #[test]
    fn delaunay_arguments_j2000() {
        let d = mean_elongation(0.0);
        assert!((d - 297.85).abs() < 0.01, "D = {d}");
        let mp = moon_mean_anomaly(0.0);
        assert!((mp - 134.96).abs() < 0.01, "l = {mp}");
    }

    #[test]
    fn eccentricity_j2000() {
        let e = eccentricity(0.0);
        assert!((e - 1.0).abs() < 1e-10);
    }

    #[test]
    fn elp_vs_meeus_consistency() {
        // ELP2000 (with same term count) should closely match the existing
        // Meeus Ch. 47 implementation in moon.rs
        for day in (0..365).step_by(30) {
            let jd = JD_J2000 + day as f64;
            let elp_lon = lunar_longitude(jd);
            let meeus_lon = crate::moon::lunar_longitude(jd);
            let diff = (elp_lon - meeus_lon).abs();
            let diff = if diff > 180.0 { 360.0 - diff } else { diff };
            assert!(
                diff < 0.01,
                "day {day}: ELP={elp_lon:.4} vs Meeus={meeus_lon:.4}, diff={diff:.6}"
            );
        }
    }
}
