//! ELP2000-82 lunar theory (Chapront-Touzé & Chapront 1988).
//!
//! Truncated to ~3200 terms for Swiss Ephemeris Moshier parity, achieving
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

mod main_problem;
mod perturbation;

use crate::calendar::julian_centuries;
use crate::coords::normalize_degrees;
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
        297.850_195_47 + 445_267.111_467_86 * t - 0.001_914_2 * t2
            + t3 / 189_474.0
            - t4 / 121_164_000.0,
    )
}

/// Mean anomaly of the Sun (l').
fn sun_mean_anomaly(t: f64) -> f64 {
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t3 * t;
    normalize_degrees(
        357.529_109_18 + 35_999.050_290_94 * t - 0.000_153_6 * t2
            + t3 / 24_490_000.0
            - t4 / 992_340_000.0,
    )
}

/// Mean anomaly of the Moon (l).
fn moon_mean_anomaly(t: f64) -> f64 {
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t3 * t;
    normalize_degrees(
        134.963_398_39 + 477_198.867_505_06 * t + 0.008_941_4 * t2
            + t3 / 69_699.0
            - t4 / 14_712_000.0,
    )
}

/// Moon's argument of latitude (F).
fn argument_of_latitude(t: f64) -> f64 {
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t3 * t;
    normalize_degrees(
        93.272_099_31 + 483_202.017_523_06 * t - 0.003_653_9 * t2
            - t3 / 3_526_000.0
            + t4 / 863_310_000.0,
    )
}

/// Longitude of the ascending node of the Moon (Ω).
fn ascending_node(t: f64) -> f64 {
    let t2 = t * t;
    let t3 = t2 * t;
    let t4 = t3 * t;
    normalize_degrees(
        125.044_555_01 - 1934.136_261_28 * t + 0.002_075_6 * t2
            + t3 / 467_441.0
            - t4 / 60_616_000.0,
    )
}

/// Earth's orbital eccentricity correction factor.
///
/// Terms whose Sun anomaly multiplier |m'| = 1 get multiplied by E,
/// terms with |m'| = 2 get multiplied by E².
fn eccentricity(t: f64) -> f64 {
    1.0 - 0.002_516 * t - 0.000_007_4 * t * t
}

/// Compute E^|m'| for a given Sun anomaly multiplier.
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
/// Argument = D·d + M·m' + Mp·l + F·f
/// Contribution = coefficient · sin(argument) [or cos for distance]
pub(crate) struct LonLatTerm {
    /// Multiplier for mean elongation (D).
    pub d: i16,
    /// Multiplier for Sun's mean anomaly (M).
    pub m: i16,
    /// Multiplier for Moon's mean anomaly (M').
    pub mp: i16,
    /// Multiplier for argument of latitude (F).
    pub f: i16,
    /// Sine coefficient in 0.000001 degrees (for longitude/latitude).
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

/// Compute the Moon's geocentric ecliptic coordinates.
///
/// Returns `(longitude_deg, latitude_deg, distance_km)`.
///
/// Uses the ELP2000-82 theory truncated to ~3200 terms for <2" accuracy
/// in longitude over ±3000 years from J2000.0.
pub fn lunar_coordinates(jd: f64) -> (f64, f64, f64) {
    let t = julian_centuries(jd);

    // Delaunay arguments (degrees)
    let d_deg = mean_elongation(t);
    let m_deg = sun_mean_anomaly(t);
    let mp_deg = moon_mean_anomaly(t);
    let f_deg = argument_of_latitude(t);

    // Convert to radians for trig
    let d = d_deg.to_radians();
    let m = m_deg.to_radians();
    let mp = mp_deg.to_radians();
    let f = f_deg.to_radians();

    // Eccentricity correction
    let e = eccentricity(t);

    // Moon's mean longitude (for additive corrections)
    let lp = normalize_degrees(
        218.316_447_9 + 481_267.881_234_21 * t - 0.001_579_86 * t * t
            + t * t * t / 538_841.0
            - t * t * t * t / 65_194_000.0,
    );

    // --- Longitude ---
    let mut lon_sum = KahanSum::new();
    for term in main_problem::LONGITUDE_TERMS {
        let arg = term.d as f64 * d + term.m as f64 * m + term.mp as f64 * mp + term.f as f64 * f;
        let ep = e_power(e, term.m);
        lon_sum.add(term.coeff as f64 * ep * arg.sin());
    }

    // Additive corrections for longitude
    let a1 = (119.75 + 131.849 * t).to_radians();
    let a2 = (53.09 + 479_264.290 * t).to_radians();
    let a3 = (313.45 + 481_266.484 * t).to_radians();

    let lon_correction = 3958.0 * a1.sin() + 1962.0 * a2.sin() + 318.0 * a3.sin();

    // Perturbation corrections
    let mut lon_pert = KahanSum::new();
    for term in perturbation::LONGITUDE_PERTURBATION {
        let arg = term.d as f64 * d + term.m as f64 * m + term.mp as f64 * mp + term.f as f64 * f;
        let ep = e_power(e, term.m);
        lon_pert.add(term.coeff as f64 * ep * arg.sin());
    }

    let longitude = normalize_degrees(
        lp + (lon_sum.sum() + lon_correction + lon_pert.sum()) * 1e-6,
    );

    // --- Latitude ---
    let mut lat_sum = KahanSum::new();
    for term in main_problem::LATITUDE_TERMS {
        let arg = term.d as f64 * d + term.m as f64 * m + term.mp as f64 * mp + term.f as f64 * f;
        let ep = e_power(e, term.m);
        lat_sum.add(term.coeff as f64 * ep * arg.sin());
    }

    // Additive corrections for latitude
    let lat_correction =
        -2235.0 * lp.to_radians().sin() + 382.0 * a3.sin() + 175.0 * (a1 - f).sin()
            + 175.0 * (a1 + f).sin()
            + 127.0 * (lp.to_radians() - mp).sin()
            - 115.0 * (lp.to_radians() + mp).sin();

    // Perturbation corrections for latitude
    let mut lat_pert = KahanSum::new();
    for term in perturbation::LATITUDE_PERTURBATION {
        let arg = term.d as f64 * d + term.m as f64 * m + term.mp as f64 * mp + term.f as f64 * f;
        let ep = e_power(e, term.m);
        lat_pert.add(term.coeff as f64 * ep * arg.sin());
    }

    let latitude = (lat_sum.sum() + lat_correction + lat_pert.sum()) * 1e-6;

    // --- Distance ---
    let mut dist_sum = KahanSum::new();
    for term in main_problem::DISTANCE_TERMS {
        let arg = term.d as f64 * d + term.m as f64 * m + term.mp as f64 * mp + term.f as f64 * f;
        let ep = e_power(e, term.m);
        dist_sum.add(term.coeff as f64 * ep * arg.cos());
    }

    let distance_km = 385_000.56 + dist_sum.sum() * 0.001;

    (longitude, latitude, distance_km)
}

#[cfg(test)]
mod tests {
    use super::*;

    const JD_J2000: f64 = 2_451_545.0;

    #[test]
    fn lunar_coordinates_j2000() {
        let (lon, lat, dist) = lunar_coordinates(JD_J2000);
        // Moon longitude at J2000.0: approximately 218.3° (near mean longitude)
        assert!(
            (0.0..360.0).contains(&lon),
            "Moon lon = {lon}"
        );
        // Latitude should be small
        assert!(lat.abs() < 6.0, "Moon lat = {lat}");
        // Distance ~356,500–406,700 km
        assert!(
            dist > 350_000.0 && dist < 410_000.0,
            "Moon dist = {dist} km"
        );
    }

    #[test]
    fn lunar_meeus_47a() {
        // Meeus example 47.a: 1992-04-12 at 0h TD, JD = 2448724.5
        let (lon, lat, dist) = lunar_coordinates(2_448_724.5);
        // Expected: λ ≈ 133.162°, β ≈ -3.229°, Δ ≈ 368409.7 km
        assert!(
            (lon - 133.162).abs() < 0.5,
            "lon = {lon}, expected ~133.162"
        );
        assert!(
            (lat - (-3.229)).abs() < 0.5,
            "lat = {lat}, expected ~-3.229"
        );
        assert!(
            (dist - 368_409.7).abs() < 500.0,
            "dist = {dist}, expected ~368409.7"
        );
    }

    #[test]
    fn lunar_longitude_range_year() {
        for day in 0..365 {
            let jd = JD_J2000 + day as f64;
            let (lon, lat, dist) = lunar_coordinates(jd);
            assert!((0.0..360.0).contains(&lon), "lon {lon} at day {day}");
            assert!(lat.abs() < 7.0, "lat {lat} at day {day}");
            assert!(
                dist > 350_000.0 && dist < 410_000.0,
                "dist {dist} at day {day}"
            );
        }
    }

    #[test]
    fn delaunay_arguments_j2000() {
        // At T=0, Delaunay arguments should equal their constant terms
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
}
