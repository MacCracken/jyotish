//! Solar position computation.
//!
//! Implements the low-precision solar longitude algorithm from Meeus
//! (*Astronomical Algorithms*, Chapter 25). Accuracy is approximately
//! 0.01° (36 arcseconds) for dates within a few centuries of J2000.0.

use crate::calendar::julian_centuries;
use crate::coords::{deg_to_rad, normalize_degrees};
use crate::planet::{Planet, PlanetaryPosition};

/// Geometric mean longitude of the Sun (Meeus eq. 25.2).
///
/// `t` is Julian centuries from J2000.0. Returns degrees.
fn mean_longitude(t: f64) -> f64 {
    normalize_degrees((0.000_303_2 * t + 36_000.769_83) * t + 280.466_46)
}

/// Mean anomaly of the Sun (Meeus eq. 25.3).
///
/// `t` is Julian centuries from J2000.0. Returns degrees.
fn mean_anomaly(t: f64) -> f64 {
    normalize_degrees((-0.000_153_7 * t + 35_999.050_29) * t + 357.529_11)
}

/// Equation of center for the Sun.
///
/// `t` is Julian centuries, `m_deg` is mean anomaly in degrees. Returns degrees.
fn equation_of_center(t: f64, m_deg: f64) -> f64 {
    let m = deg_to_rad(m_deg);

    ((-0.000_014 * t - 0.004_817) * t + 1.914_602) * m.sin()
        + (-0.000_101 * t + 0.019_993) * (2.0 * m).sin()
        + 0.000_289 * (3.0 * m).sin()
}

/// Apparent longitude correction for nutation and aberration (simplified).
///
/// `omega_deg` is the longitude of the ascending node of the Moon's orbit.
/// Returns the correction in degrees.
fn apparent_correction(t: f64) -> f64 {
    let omega = 125.04 - 1934.136 * t;
    -0.005_69 - 0.004_78 * deg_to_rad(omega).sin()
}

/// Compute the Sun's ecliptic longitude in degrees for a given Julian Date.
///
/// Returns the apparent (corrected for nutation + aberration) ecliptic longitude.
///
/// # Examples
///
/// ```
/// # use jyotish::sun::solar_longitude;
/// // Meeus example 25.a: 1992-10-13 at 0h TD
/// // JD = 2448908.5
/// let lon = solar_longitude(2_448_908.5);
/// assert!((lon - 199.907).abs() < 0.01, "got {lon}");
/// ```
pub fn solar_longitude(jd: f64) -> f64 {
    let t = julian_centuries(jd);
    let l0 = mean_longitude(t);
    let m = mean_anomaly(t);
    let c = equation_of_center(t, m);
    let sun_lon = l0 + c;
    let correction = apparent_correction(t);
    normalize_degrees(sun_lon + correction)
}

/// Compute the Sun's distance from Earth in AU for a given Julian Date.
///
/// Uses the equation of center to compute the Sun-Earth distance.
///
/// # Examples
///
/// ```
/// # use jyotish::sun::solar_distance_au;
/// let dist = solar_distance_au(2_448_908.5);
/// assert!((dist - 0.9976).abs() < 0.001, "got {dist}");
/// ```
pub fn solar_distance_au(jd: f64) -> f64 {
    let t = julian_centuries(jd);
    let m = mean_anomaly(t);
    let m_rad = deg_to_rad(m);
    let e = (-0.000_000_126_7 * t - 0.000_042_037) * t + 0.016_708_634;
    let v_rad = m_rad + deg_to_rad(equation_of_center(t, m));

    1.000_001_018 * (1.0 - e * e) / (1.0 + e * v_rad.cos())
}

/// Compute the Sun's ecliptic latitude in degrees.
///
/// Returns 0.0 — the Sun's geocentric ecliptic latitude is always within
/// ~1.2 arcseconds of zero, below this model's precision threshold.
pub fn solar_latitude(_jd: f64) -> f64 {
    0.0
}

/// Compute the Sun's position as a [`PlanetaryPosition`].
///
/// # Examples
///
/// ```
/// # use jyotish::sun::solar_position;
/// # use jyotish::planet::Planet;
/// let pos = solar_position(2_448_908.5);
/// assert_eq!(pos.planet, Planet::Sun);
/// assert!((pos.longitude_deg - 199.907).abs() < 0.01);
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
    let l0 = deg_to_rad(mean_longitude(t));
    let m = deg_to_rad(mean_anomaly(t));
    let e = 0.016_708_634 - 0.000_042_037 * t;
    let eps = deg_to_rad(crate::coords::mean_obliquity(t));

    let y = (eps / 2.0).tan().powi(2);

    let eot = y * (2.0 * l0).sin() - 2.0 * e * m.sin() + 4.0 * e * y * m.sin() * (2.0 * l0).cos()
        - 0.5 * y * y * (4.0 * l0).sin()
        - 1.25 * e * e * (2.0 * m).sin();

    // Convert radians to minutes of time (1 radian = 180/π degrees, 4 min/degree)
    crate::coords::rad_to_deg(eot) * 4.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solar_longitude_meeus_25a() {
        // Meeus example 25.a: 1992-10-13 at 0h TD → λ ≈ 199.907°
        let lon = solar_longitude(2_448_908.5);
        assert!((lon - 199.907).abs() < 0.02, "got {lon}");
    }

    #[test]
    fn solar_distance_meeus_25a() {
        // Meeus example 25.a: R ≈ 0.99766 AU
        let dist = solar_distance_au(2_448_908.5);
        assert!((dist - 0.9977).abs() < 0.001, "got {dist}");
    }

    #[test]
    fn solar_position_struct() {
        let pos = solar_position(2_451_545.0);
        assert_eq!(pos.planet, Planet::Sun);
        assert!(pos.longitude_deg >= 0.0 && pos.longitude_deg < 360.0);
        assert!(pos.distance_au > 0.98 && pos.distance_au < 1.02);
    }

    #[test]
    fn solar_longitude_j2000() {
        // At J2000.0 (2000-01-01 12:00 TT), Sun longitude ≈ 280.5°
        let lon = solar_longitude(2_451_545.0);
        assert!((lon - 280.5).abs() < 0.5, "got {lon}");
    }

    #[test]
    fn solar_longitude_range() {
        // Check that longitude is always in [0, 360)
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
        // EoT should be roughly in [-17, +17] minutes throughout the year
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
