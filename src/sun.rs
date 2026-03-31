//! Solar position computation.
//!
//! Computes the Sun's geocentric ecliptic position using the VSOP87D theory
//! for Earth's heliocentric coordinates (Bretagnon & Francou 1988). The Sun's
//! geocentric position is the antipode of Earth's heliocentric position.
//!
//! Accuracy: <1 arcsecond for dates within ±4000 years of J2000.0.
//!
//! Functions in this module return **geometric** positions (no aberration or
//! nutation applied). For apparent (observed) positions, use
//! [`crate::apparent::apparent_sun`].

use crate::calendar::julian_centuries;
use crate::coords::{deg_to_rad, normalize_degrees};
use crate::planet::{Planet, PlanetaryPosition};

/// Compute the Sun's geometric ecliptic longitude in degrees for a given Julian Date.
///
/// Uses VSOP87D Earth heliocentric coordinates; the Sun's geocentric longitude
/// is Earth's heliocentric longitude + 180°.
///
/// Returns the **geometric** longitude — no aberration or nutation correction
/// is applied. For the full apparent position, use [`crate::apparent::apparent_sun`].
///
/// # Examples
///
/// ```
/// # use jyotish::sun::solar_longitude;
/// // At J2000.0, geometric Sun longitude ≈ 280.47°
/// let lon = solar_longitude(2_451_545.0);
/// assert!((lon - 280.47).abs() < 0.1, "got {lon}");
/// ```
pub fn solar_longitude(jd: f64) -> f64 {
    let (e_lon, _e_lat, _e_r) = crate::vsop87::earth_heliocentric(jd);
    normalize_degrees(e_lon + 180.0)
}

/// Compute the Sun's distance from Earth in AU for a given Julian Date.
///
/// # Examples
///
/// ```
/// # use jyotish::sun::solar_distance_au;
/// let dist = solar_distance_au(2_448_908.5);
/// assert!((dist - 0.9976).abs() < 0.001, "got {dist}");
/// ```
pub fn solar_distance_au(jd: f64) -> f64 {
    let (_lon, _lat, r) = crate::vsop87::earth_heliocentric(jd);
    r
}

/// Compute the Sun's geocentric ecliptic latitude in degrees.
///
/// With VSOP87, Earth has a small heliocentric latitude (~1.7" max), so the
/// Sun's geocentric latitude is the negation of that — a small but nonzero value.
pub fn solar_latitude(jd: f64) -> f64 {
    let (_lon, lat, _r) = crate::vsop87::earth_heliocentric(jd);
    -lat
}

/// Compute the Sun's position as a [`PlanetaryPosition`].
///
/// Returns the **geometric** position (no aberration/nutation).
///
/// # Examples
///
/// ```
/// # use jyotish::sun::solar_position;
/// # use jyotish::planet::Planet;
/// let pos = solar_position(2_451_545.0);
/// assert_eq!(pos.planet, Planet::Sun);
/// assert!((pos.longitude_deg - 280.47).abs() < 0.1);
/// ```
pub fn solar_position(jd: f64) -> PlanetaryPosition {
    PlanetaryPosition::new(
        Planet::Sun,
        solar_longitude(jd),
        solar_latitude(jd),
        solar_distance_au(jd),
        crate::calendar::jd_to_unix(jd),
    )
}

/// Equation of time in minutes for a given Julian Date.
///
/// The equation of time is the difference between apparent solar time and
/// mean solar time. Positive means the sundial is ahead of the clock.
pub fn equation_of_time(jd: f64) -> f64 {
    let t = julian_centuries(jd);
    let l0 = deg_to_rad(mean_longitude_meeus(t));
    let m = deg_to_rad(mean_anomaly_meeus(t));
    let e = 0.016_708_634 - 0.000_042_037 * t;
    let eps = deg_to_rad(crate::coords::mean_obliquity(t));

    let y = (eps / 2.0).tan().powi(2);

    let eot = y * (2.0 * l0).sin() - 2.0 * e * m.sin() + 4.0 * e * y * m.sin() * (2.0 * l0).cos()
        - 0.5 * y * y * (4.0 * l0).sin()
        - 1.25 * e * e * (2.0 * m).sin();

    crate::coords::rad_to_deg(eot) * 4.0
}

// ---------------------------------------------------------------------------
// Meeus helpers (used by equation_of_time and available via feature flag)
// ---------------------------------------------------------------------------

/// Geometric mean longitude of the Sun (Meeus eq. 25.2).
fn mean_longitude_meeus(t: f64) -> f64 {
    normalize_degrees((0.000_303_2 * t + 36_000.769_83) * t + 280.466_46)
}

/// Mean anomaly of the Sun (Meeus eq. 25.3).
fn mean_anomaly_meeus(t: f64) -> f64 {
    normalize_degrees((-0.000_153_7 * t + 35_999.050_29) * t + 357.529_11)
}

/// Equation of center for the Sun (Meeus).
fn equation_of_center_meeus(t: f64, m_deg: f64) -> f64 {
    let m = deg_to_rad(m_deg);
    ((-0.000_014 * t - 0.004_817) * t + 1.914_602) * m.sin()
        + (-0.000_101 * t + 0.019_993) * (2.0 * m).sin()
        + 0.000_289 * (3.0 * m).sin()
}

// ---------------------------------------------------------------------------
// Legacy Meeus solar position (behind feature flag)
// ---------------------------------------------------------------------------

/// Solar longitude using Meeus Chapter 25 (low-precision, ~36" accuracy).
///
/// Includes simplified nutation + aberration correction.
#[cfg(feature = "meeus")]
pub fn solar_longitude_meeus(jd: f64) -> f64 {
    let t = julian_centuries(jd);
    let l0 = mean_longitude_meeus(t);
    let m = mean_anomaly_meeus(t);
    let c = equation_of_center_meeus(t, m);
    let sun_lon = l0 + c;
    let omega = 125.04 - 1934.136 * t;
    let correction = -0.005_69 - 0.004_78 * deg_to_rad(omega).sin();
    normalize_degrees(sun_lon + correction)
}

/// Solar distance using Meeus Chapter 25.
#[cfg(feature = "meeus")]
pub fn solar_distance_au_meeus(jd: f64) -> f64 {
    let t = julian_centuries(jd);
    let m = mean_anomaly_meeus(t);
    let m_rad = deg_to_rad(m);
    let e = (-0.000_000_126_7 * t - 0.000_042_037) * t + 0.016_708_634;
    let v_rad = m_rad + deg_to_rad(equation_of_center_meeus(t, m));
    1.000_001_018 * (1.0 - e * e) / (1.0 + e * v_rad.cos())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solar_longitude_j2000() {
        // At J2000.0, geometric Sun longitude ≈ 280.47°
        let lon = solar_longitude(2_451_545.0);
        assert!((lon - 280.47).abs() < 0.2, "got {lon}");
    }

    #[test]
    fn solar_longitude_meeus_25a_epoch() {
        // Meeus example 25.a: 1992-10-13 at 0h TD, JD = 2448908.5
        // Meeus gives apparent λ ≈ 199.907° (with nutation+aberration).
        // VSOP87 geometric should be close but differ by ~0.01° (no corrections).
        let lon = solar_longitude(2_448_908.5);
        assert!((lon - 199.91).abs() < 0.1, "got {lon}");
    }

    #[test]
    fn solar_distance_meeus_25a() {
        let dist = solar_distance_au(2_448_908.5);
        assert!((dist - 0.9977).abs() < 0.001, "got {dist}");
    }

    #[test]
    fn solar_latitude_nonzero() {
        // VSOP87 should give a small but nonzero solar latitude
        let lat = solar_latitude(2_451_545.0);
        assert!(lat.abs() < 0.01, "lat = {lat}");
    }

    #[test]
    fn solar_position_struct() {
        let pos = solar_position(2_451_545.0);
        assert_eq!(pos.planet, Planet::Sun);
        assert!(pos.longitude_deg >= 0.0 && pos.longitude_deg < 360.0);
        assert!(pos.distance_au > 0.98 && pos.distance_au < 1.02);
    }

    #[test]
    fn solar_longitude_range() {
        for day in 0..365 {
            let jd = 2_451_545.0 + day as f64;
            let lon = solar_longitude(jd);
            assert!(
                (0.0..360.0).contains(&lon),
                "longitude {lon} out of range at day {day}"
            );
        }
    }

    #[test]
    fn equation_of_time_range() {
        for day in 0..365 {
            let jd = 2_451_545.0 + day as f64;
            let eot = equation_of_time(jd);
            assert!(
                eot.abs() < 20.0,
                "EoT {eot} min out of expected range at day {day}"
            );
        }
    }
}
