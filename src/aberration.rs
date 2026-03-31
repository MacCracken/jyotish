//! Annual aberration and light-time correction.
//!
//! Aberration is the apparent displacement of a celestial body caused by
//! the finite speed of light and the observer's motion. Annual aberration
//! (due to Earth's orbital velocity) can displace apparent positions by up
//! to ~20.5 arcseconds.
//!
//! Light-time correction accounts for the time light takes to travel from
//! a body to Earth — during which the body moves.

use crate::coords::{deg_to_rad, normalize_degrees, rad_to_deg};

/// Speed of light in AU/day.
const C_AU_DAY: f64 = 173.144_632_72;

/// Constant of aberration in arcseconds (κ).
const ABERRATION_CONSTANT: f64 = 20.495_52;

// ---------------------------------------------------------------------------
// Light-time correction
// ---------------------------------------------------------------------------

/// Compute the light travel time from a body to Earth in days.
///
/// `distance_au` is the geocentric distance in AU.
///
/// # Examples
///
/// ```
/// # use jyotish::aberration::light_time;
/// // Sun at 1 AU: ~499 seconds = ~0.00578 days
/// let lt = light_time(1.0);
/// assert!((lt - 0.005_776).abs() < 0.001, "got {lt}");
/// ```
pub fn light_time(distance_au: f64) -> f64 {
    distance_au / C_AU_DAY
}

/// Apply iterative light-time correction to a planetary position.
///
/// Given a function that computes heliocentric longitude at a JD, and the
/// Earth's heliocentric longitude, iteratively adjusts the planet's position
/// to account for light travel time.
///
/// Returns `(corrected_longitude, corrected_latitude, distance, light_time_days)`.
///
/// `planet_at_jd` takes a JD and returns `(helio_lon, helio_lat, helio_r)`.
/// `earth_at_jd` takes a JD and returns `(helio_lon, helio_lat, helio_r)`.
pub fn light_time_correction<F, G>(jd: f64, planet_at_jd: F, earth_at_jd: G) -> (f64, f64, f64, f64)
where
    F: Fn(f64) -> (f64, f64, f64),
    G: Fn(f64) -> (f64, f64, f64),
{
    let (e_lon, _e_lat, e_r) = earth_at_jd(jd);

    // Initial position (no correction)
    let (mut p_lon, mut p_lat, mut p_r) = planet_at_jd(jd);
    let mut dist = geocentric_distance(p_lon, p_lat, p_r, e_lon, e_r);
    let mut tau = light_time(dist);

    // Iterate: recompute planet position at jd - tau
    for _ in 0..3 {
        let (lon, lat, r) = planet_at_jd(jd - tau);
        p_lon = lon;
        p_lat = lat;
        p_r = r;
        dist = geocentric_distance(p_lon, p_lat, p_r, e_lon, e_r);
        tau = light_time(dist);
    }

    // Geocentric ecliptic coordinates
    let p_lon_r = deg_to_rad(p_lon);
    let p_lat_r = deg_to_rad(p_lat);
    let e_lon_r = deg_to_rad(e_lon);

    let x = p_r * p_lat_r.cos() * p_lon_r.cos() - e_r * e_lon_r.cos();
    let y = p_r * p_lat_r.cos() * p_lon_r.sin() - e_r * e_lon_r.sin();
    let z = p_r * p_lat_r.sin();

    let geo_lon = normalize_degrees(rad_to_deg(y.atan2(x)));
    let geo_lat = rad_to_deg(z.atan2((x * x + y * y).sqrt()));

    (geo_lon, geo_lat, dist, tau)
}

/// Geocentric distance from heliocentric coordinates.
fn geocentric_distance(p_lon: f64, p_lat: f64, p_r: f64, e_lon: f64, e_r: f64) -> f64 {
    let p_lon_r = deg_to_rad(p_lon);
    let p_lat_r = deg_to_rad(p_lat);
    let e_lon_r = deg_to_rad(e_lon);

    let x = p_r * p_lat_r.cos() * p_lon_r.cos() - e_r * e_lon_r.cos();
    let y = p_r * p_lat_r.cos() * p_lon_r.sin() - e_r * e_lon_r.sin();
    let z = p_r * p_lat_r.sin();

    (x * x + y * y + z * z).sqrt()
}

// ---------------------------------------------------------------------------
// Annual aberration
// ---------------------------------------------------------------------------

/// Compute the annual aberration correction in ecliptic longitude and latitude.
///
/// Uses the Ron-Vondrák method (simplified, Meeus Ch. 23).
/// `sun_lon` is the Sun's geometric ecliptic longitude in degrees.
/// `obj_lon` and `obj_lat` are the object's geometric ecliptic longitude
/// and latitude in degrees.
/// `obliquity` is the obliquity of the ecliptic in degrees.
///
/// Returns `(delta_lon, delta_lat)` in degrees (add to geometric position
/// to get apparent position).
///
/// # Examples
///
/// ```
/// # use jyotish::aberration::annual_aberration;
/// let (dlon, dlat) = annual_aberration(280.0, 100.0, 5.0, 23.44);
/// // Correction should be small (< 0.01°)
/// assert!(dlon.abs() < 0.01, "dlon = {dlon}");
/// assert!(dlat.abs() < 0.01, "dlat = {dlat}");
/// ```
pub fn annual_aberration(sun_lon: f64, obj_lon: f64, obj_lat: f64, obliquity: f64) -> (f64, f64) {
    let sun_r = deg_to_rad(sun_lon);
    let lon_r = deg_to_rad(obj_lon);
    let lat_r = deg_to_rad(obj_lat);
    let _eps_r = deg_to_rad(obliquity); // reserved for full Ron-Vondrák
    let kappa = ABERRATION_CONSTANT / 3600.0; // convert to degrees

    // Aberration in longitude (Meeus eq. 23.2)
    let delta_lon = (-kappa * (sun_r - lon_r).cos() / lat_r.cos())
        + (kappa * 0.016_708_634 * (deg_to_rad(102.937_35) - lon_r).cos() / lat_r.cos());

    // Aberration in latitude (Meeus eq. 23.3)
    let delta_lat = -kappa
        * lat_r.sin()
        * ((sun_r - lon_r).sin() - 0.016_708_634 * (deg_to_rad(102.937_35) - lon_r).sin());

    (delta_lon, delta_lat)
}

/// Apply annual aberration to an ecliptic position.
///
/// `geo_lon`, `geo_lat` are the geometric ecliptic longitude and latitude.
/// `sun_lon` is the Sun's geometric longitude.
/// `obliquity` is the obliquity of the ecliptic.
///
/// Returns `(apparent_lon, apparent_lat)` in degrees.
pub fn apply_aberration(geo_lon: f64, geo_lat: f64, sun_lon: f64, obliquity: f64) -> (f64, f64) {
    let (dlon, dlat) = annual_aberration(sun_lon, geo_lon, geo_lat, obliquity);
    (normalize_degrees(geo_lon + dlon), geo_lat + dlat)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn light_time_sun() {
        let lt = light_time(1.0);
        // 1 AU / c ≈ 499 seconds ≈ 0.005776 days
        assert!((lt - 0.005_776).abs() < 0.001, "got {lt}");
    }

    #[test]
    fn light_time_jupiter() {
        // Jupiter at ~5 AU: ~2500 seconds
        let lt = light_time(5.0);
        assert!(
            (lt * 86400.0 - 2497.0).abs() < 10.0,
            "got {} seconds",
            lt * 86400.0
        );
    }

    #[test]
    fn aberration_magnitude() {
        // Check aberration at various elongations — maximum ~20.5" = 0.00569°
        // The actual correction depends on both sun-object angle and eccentricity term
        let mut max_abs = 0.0_f64;
        for obj_lon in (0..360).step_by(10) {
            let (dlon, _) = annual_aberration(280.0, obj_lon as f64, 0.0, 23.44);
            max_abs = max_abs.max(dlon.abs());
        }
        // Maximum should be near the aberration constant (~0.00569°)
        assert!(max_abs > 0.004 && max_abs < 0.007, "max_abs = {max_abs}°");
    }

    #[test]
    fn aberration_zero_latitude() {
        // For an object on the ecliptic, latitude correction should be ~0
        let (_dlon, dlat) = annual_aberration(280.0, 100.0, 0.0, 23.44);
        assert!(dlat.abs() < 0.001, "dlat = {dlat}°");
    }

    #[test]
    fn apply_aberration_roundtrip() {
        let lon = 100.0;
        let lat = 5.0;
        let (app_lon, app_lat) = apply_aberration(lon, lat, 280.0, 23.44);
        // Corrections should be small
        assert!((app_lon - lon).abs() < 0.01);
        assert!((app_lat - lat).abs() < 0.01);
    }

    #[test]
    fn light_time_correction_basic() {
        // Simple test: planet and Earth at known positions
        let planet_fn = |_jd: f64| (100.0_f64, 2.0_f64, 1.5_f64);
        let earth_fn = |_jd: f64| (280.0_f64, 0.0_f64, 1.0_f64);

        let (lon, lat, dist, tau) = light_time_correction(2_451_545.0, planet_fn, earth_fn);
        assert!((0.0..360.0).contains(&lon));
        assert!(lat.abs() < 90.0);
        assert!(dist > 0.0);
        assert!(tau > 0.0);
    }
}
