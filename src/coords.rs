//! Coordinate systems and transformations.
//!
//! Provides conversions between ecliptic, equatorial, and horizontal
//! coordinate systems used in positional astronomy.

use std::f64::consts::PI;

/// Degrees to radians.
pub fn deg_to_rad(deg: f64) -> f64 {
    deg * PI / 180.0
}

/// Radians to degrees.
pub fn rad_to_deg(rad: f64) -> f64 {
    rad * 180.0 / PI
}

/// Normalize an angle in degrees to [0, 360).
pub fn normalize_degrees(deg: f64) -> f64 {
    let r = deg % 360.0;
    if r < 0.0 { r + 360.0 } else { r }
}

/// Normalize an angle in radians to [0, 2π).
pub fn normalize_radians(rad: f64) -> f64 {
    let r = rad % (2.0 * PI);
    if r < 0.0 { r + 2.0 * PI } else { r }
}

/// Mean obliquity of the ecliptic (Meeus eq. 22.2).
///
/// `t` is Julian centuries from J2000.0.
/// Returns the obliquity in degrees.
pub fn mean_obliquity(t: f64) -> f64 {
    // Horner's method: ((c3*t + c2)*t + c1)*t + c0
    ((5.036_11e-7 * t - 1.638_89e-7) * t - 0.013_004_167) * t + 23.439_291_11
}

/// Convert ecliptic coordinates to equatorial coordinates.
///
/// Takes ecliptic longitude and latitude in degrees, plus the obliquity in degrees.
/// Returns (right ascension, declination) in degrees.
pub fn ecliptic_to_equatorial(lon_deg: f64, lat_deg: f64, obliquity_deg: f64) -> (f64, f64) {
    let lon = deg_to_rad(lon_deg);
    let lat = deg_to_rad(lat_deg);
    let eps = deg_to_rad(obliquity_deg);

    let sin_lon = lon.sin();
    let cos_lon = lon.cos();
    let sin_lat = lat.sin();
    let cos_lat = lat.cos();
    let sin_eps = eps.sin();
    let cos_eps = eps.cos();

    let ra = (sin_lon * cos_eps - sin_lat / cos_lat * sin_eps).atan2(cos_lon);
    let dec = (sin_lat * cos_eps + cos_lat * sin_eps * sin_lon).asin();

    (normalize_degrees(rad_to_deg(ra)), rad_to_deg(dec))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deg_rad_roundtrip() {
        let deg = 123.456;
        assert!((rad_to_deg(deg_to_rad(deg)) - deg).abs() < 1e-12);
    }

    #[test]
    fn normalize_degrees_cases() {
        assert!((normalize_degrees(0.0)).abs() < 1e-10);
        assert!((normalize_degrees(360.0)).abs() < 1e-10);
        assert!((normalize_degrees(-90.0) - 270.0).abs() < 1e-10);
        assert!((normalize_degrees(720.5) - 0.5).abs() < 1e-10);
    }

    #[test]
    fn mean_obliquity_j2000() {
        let eps = mean_obliquity(0.0);
        assert!((eps - 23.4393).abs() < 0.001, "got {eps}");
    }

    #[test]
    fn ecliptic_to_equatorial_vernal_equinox() {
        // At the vernal equinox (lon=0, lat=0), RA and Dec should both be ~0
        let (ra, dec) = ecliptic_to_equatorial(0.0, 0.0, 23.4393);
        assert!(ra.abs() < 1e-10 || (ra - 360.0).abs() < 1e-10);
        assert!(dec.abs() < 1e-10);
    }
}
