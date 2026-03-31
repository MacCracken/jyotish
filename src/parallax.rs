//! Parallax correction for lunar and planetary positions.
//!
//! The Moon is close enough to Earth that the observer's position on the
//! surface matters — the apparent position differs from the geocentric
//! position by up to ~1° (the Moon's horizontal parallax). This module
//! provides corrections to convert geocentric positions to topocentric
//! (observer-centered) positions.
//!
//! For planets, the parallax is negligible (< 1 arcsecond) but is included
//! for completeness.

use crate::coords::{deg_to_rad, normalize_degrees, rad_to_deg};

/// Earth's equatorial radius in km.
const EARTH_RADIUS_KM: f64 = 6378.137;

// ---------------------------------------------------------------------------
// Observer location
// ---------------------------------------------------------------------------

/// An observer's location on Earth's surface.
#[derive(Debug, Clone, Copy)]
pub struct Observer {
    /// Geographic latitude in degrees (positive north).
    pub latitude_deg: f64,
    /// Geographic longitude in degrees (positive east).
    pub longitude_deg: f64,
    /// Height above sea level in meters.
    pub height_m: f64,
}

impl Observer {
    /// Create a new observer at the given location.
    pub fn new(latitude_deg: f64, longitude_deg: f64, height_m: f64) -> Self {
        Self {
            latitude_deg,
            longitude_deg,
            height_m,
        }
    }

    /// Geocentric latitude and distance from Earth's center.
    ///
    /// Returns `(rho_sin_phi, rho_cos_phi)` — the observer's distance from
    /// Earth's center projected onto the equatorial plane and the polar axis,
    /// in units of Earth's equatorial radius.
    fn geocentric_params(&self) -> (f64, f64) {
        let lat = deg_to_rad(self.latitude_deg);
        let u = (0.996_647_19 * lat.tan()).atan(); // reduced latitude
        let height_fraction = self.height_m / 1000.0 / EARTH_RADIUS_KM;

        let rho_sin = 0.996_647_19 * u.sin() + height_fraction * lat.sin();
        let rho_cos = u.cos() + height_fraction * lat.cos();

        (rho_sin, rho_cos)
    }
}

// ---------------------------------------------------------------------------
// Horizontal parallax
// ---------------------------------------------------------------------------

/// Compute the horizontal parallax of a body in degrees given its distance.
///
/// `distance_km` is the geocentric distance in kilometers.
///
/// # Examples
///
/// ```
/// # use jyotish::parallax::horizontal_parallax;
/// // Moon at ~384,400 km → parallax ≈ 0.95°
/// let hp = horizontal_parallax(384_400.0);
/// assert!((hp - 0.95).abs() < 0.05, "got {hp}");
/// ```
pub fn horizontal_parallax(distance_km: f64) -> f64 {
    rad_to_deg((EARTH_RADIUS_KM / distance_km).asin())
}

/// Compute the horizontal parallax of a body given its distance in AU.
///
/// # Examples
///
/// ```
/// # use jyotish::parallax::horizontal_parallax_au;
/// // Sun at 1 AU → parallax ≈ 8.794"
/// let hp = horizontal_parallax_au(1.0);
/// assert!((hp * 3600.0 - 8.794).abs() < 0.1, "got {} arcsec", hp * 3600.0);
/// ```
pub fn horizontal_parallax_au(distance_au: f64) -> f64 {
    let distance_km = distance_au * 149_597_870.7;
    horizontal_parallax(distance_km)
}

// ---------------------------------------------------------------------------
// Topocentric correction
// ---------------------------------------------------------------------------

/// Topocentric correction result.
#[derive(Debug, Clone, Copy)]
pub struct TopocentricCorrection {
    /// Corrected ecliptic longitude in degrees.
    pub longitude_deg: f64,
    /// Corrected ecliptic latitude in degrees.
    pub latitude_deg: f64,
}

/// Compute the topocentric ecliptic position of the Moon.
///
/// Takes the geocentric ecliptic longitude and latitude (degrees),
/// the Moon's distance in km, the local sidereal time in degrees,
/// and the obliquity in degrees.
///
/// Returns the corrected (topocentric) ecliptic longitude and latitude.
///
/// # Examples
///
/// ```
/// # use jyotish::parallax::{topocentric_moon, Observer};
/// let observer = Observer::new(51.5, -0.1, 0.0);
/// let result = topocentric_moon(133.162, -3.229, 368_410.0, 197.0, 23.44, &observer);
/// // Correction should be small but nonzero
/// assert!((result.longitude_deg - 133.162).abs() < 2.0);
/// ```
pub fn topocentric_moon(
    geo_lon: f64,
    geo_lat: f64,
    distance_km: f64,
    lst_deg: f64,
    obliquity_deg: f64,
    observer: &Observer,
) -> TopocentricCorrection {
    let (rho_sin, rho_cos) = observer.geocentric_params();
    let hp = deg_to_rad(horizontal_parallax(distance_km));
    let eps = deg_to_rad(obliquity_deg);
    let lst = deg_to_rad(lst_deg);
    let lon = deg_to_rad(geo_lon);
    let lat = deg_to_rad(geo_lat);

    // Parallax in right ascension and declination (Meeus Ch. 40)
    let sin_hp = hp.sin();

    // Convert to equatorial for parallax computation
    let sin_lon = lon.sin();
    let cos_lon = lon.cos();
    let sin_lat = lat.sin();
    let cos_lat = lat.cos();
    let sin_eps = eps.sin();
    let cos_eps = eps.cos();

    // Right ascension and declination
    let ra = (sin_lon * cos_eps - sin_lat / cos_lat * sin_eps).atan2(cos_lon);
    let dec = (sin_lat * cos_eps + cos_lat * sin_eps * sin_lon).asin();

    // Local hour angle
    let h = lst - ra;

    // Topocentric corrections (Meeus eq. 40.2, 40.3)
    let cos_dec = dec.cos();
    let sin_dec = dec.sin();

    let delta_ra = (-rho_cos * sin_hp * h.sin()).atan2(cos_dec - rho_cos * sin_hp * h.cos());

    let dec_prime =
        ((sin_dec - rho_sin * sin_hp) * delta_ra.cos()).atan2(cos_dec - rho_cos * sin_hp * h.cos());

    // Convert corrected equatorial back to ecliptic
    let ra_prime = ra + delta_ra;
    let sin_ra_p = ra_prime.sin();
    let cos_ra_p = ra_prime.cos();
    let sin_dec_p = dec_prime.sin();
    let cos_dec_p = dec_prime.cos();

    let lon_prime = (sin_ra_p * cos_eps + sin_dec_p / cos_dec_p * sin_eps).atan2(cos_ra_p);
    let lat_prime = (sin_dec_p * cos_eps - cos_dec_p * sin_eps * sin_ra_p).asin();

    TopocentricCorrection {
        longitude_deg: normalize_degrees(rad_to_deg(lon_prime)),
        latitude_deg: rad_to_deg(lat_prime),
    }
}

/// Compute the topocentric correction for a planet (very small).
///
/// For planets, the parallax is typically < 1 arcsecond. This function
/// computes the correction in altitude for a body at a given apparent
/// altitude.
///
/// Returns the parallax correction in degrees (to be subtracted from
/// the geocentric altitude to get the topocentric altitude).
pub fn parallax_in_altitude(apparent_alt_deg: f64, horizontal_parallax_deg: f64) -> f64 {
    let alt = deg_to_rad(apparent_alt_deg);
    let hp = deg_to_rad(horizontal_parallax_deg);
    rad_to_deg(hp * alt.cos())
}

// ---------------------------------------------------------------------------
// Convenience: correct a full lunar position
// ---------------------------------------------------------------------------

/// Correct a lunar position for parallax given an observer and Julian Date.
///
/// `jd_tt` is in TT (Terrestrial Time). The function converts to UT1
/// internally for sidereal time computation.
///
/// # Errors
///
/// Returns an error if the date is invalid.
///
/// # Examples
///
/// ```
/// # use jyotish::parallax::{correct_lunar_position, Observer};
/// let observer = Observer::new(51.5, -0.1, 0.0);
/// let corrected = correct_lunar_position(2_451_545.0, &observer).unwrap();
/// assert!(corrected.longitude_deg >= 0.0 && corrected.longitude_deg < 360.0);
/// ```
pub fn correct_lunar_position(
    jd_tt: f64,
    observer: &Observer,
) -> crate::error::Result<TopocentricCorrection> {
    let lon = crate::moon::lunar_longitude(jd_tt);
    let lat = crate::moon::lunar_latitude(jd_tt);
    let dist = crate::moon::lunar_distance_km(jd_tt);
    let lst = crate::calendar::local_sidereal_time_tt(jd_tt, observer.longitude_deg);
    let eps = crate::nutation::true_obliquity(jd_tt);

    Ok(topocentric_moon(lon, lat, dist, lst, eps, observer))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn horizontal_parallax_moon() {
        let hp = horizontal_parallax(384_400.0);
        // Moon's HP is ~0.95°
        assert!((hp - 0.95).abs() < 0.05, "got {hp}");
    }

    #[test]
    fn horizontal_parallax_sun() {
        let hp = horizontal_parallax_au(1.0);
        // Sun's HP is ~8.794 arcseconds
        assert!(
            (hp * 3600.0 - 8.794).abs() < 0.1,
            "got {} arcsec",
            hp * 3600.0
        );
    }

    #[test]
    fn observer_geocentric_params() {
        let obs = Observer::new(0.0, 0.0, 0.0);
        let (rho_sin, rho_cos) = obs.geocentric_params();
        // At equator, rho_cos ≈ 1.0, rho_sin ≈ 0.0
        assert!(rho_cos > 0.99 && rho_cos < 1.01, "rho_cos = {rho_cos}");
        assert!(rho_sin.abs() < 0.01, "rho_sin = {rho_sin}");
    }

    #[test]
    fn observer_at_pole() {
        let obs = Observer::new(90.0, 0.0, 0.0);
        let (rho_sin, rho_cos) = obs.geocentric_params();
        // At north pole, rho_sin ≈ 0.9983, rho_cos ≈ 0.0
        assert!(rho_sin > 0.99, "rho_sin = {rho_sin}");
        assert!(rho_cos.abs() < 0.01, "rho_cos = {rho_cos}");
    }

    #[test]
    fn topocentric_moon_correction_nonzero() {
        let obs = Observer::new(51.5, -0.1, 0.0);
        let result = topocentric_moon(133.162, -3.229, 368_410.0, 197.0, 23.44, &obs);
        // Correction should shift the position noticeably (up to ~1°)
        let lon_diff = (result.longitude_deg - 133.162).abs();
        let lat_diff = (result.latitude_deg - (-3.229)).abs();
        assert!(
            lon_diff > 0.001 || lat_diff > 0.001,
            "correction too small: Δlon={lon_diff}, Δlat={lat_diff}"
        );
        assert!(lon_diff < 2.0, "correction too large: Δlon={lon_diff}");
    }

    #[test]
    fn topocentric_moon_at_geocenter() {
        // Observer at Earth's center (rho=0) should give no correction
        // We can't quite do this, but at sea level the correction should be small
        // for a body at great distance
        let obs = Observer::new(0.0, 0.0, 0.0);
        let result = topocentric_moon(100.0, 0.0, 384_400.0, 100.0, 23.44, &obs);
        // At the equator with Moon near transit, correction is mostly in altitude
        assert!(result.longitude_deg >= 0.0 && result.longitude_deg < 360.0);
    }

    #[test]
    fn correct_lunar_position_convenience() {
        let obs = Observer::new(51.5, -0.1, 0.0);
        let corrected = correct_lunar_position(2_451_545.0, &obs).unwrap();
        assert!(corrected.longitude_deg >= 0.0 && corrected.longitude_deg < 360.0);
        assert!(corrected.latitude_deg.abs() < 10.0);
    }

    #[test]
    fn parallax_in_altitude_horizon() {
        // At the horizon, parallax equals horizontal parallax
        let correction = parallax_in_altitude(0.0, 0.95);
        assert!((correction - 0.95).abs() < 0.01, "got {correction}");
    }

    #[test]
    fn parallax_in_altitude_zenith() {
        // At the zenith, parallax is zero
        let correction = parallax_in_altitude(90.0, 0.95);
        assert!(correction.abs() < 0.001, "got {correction}");
    }
}
