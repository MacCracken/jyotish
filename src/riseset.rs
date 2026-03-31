//! Rise, set, and meridian transit times for celestial bodies.
//!
//! Computes the times at which a body rises above, sets below, or crosses
//! the observer's meridian. Uses the algorithm from Meeus (*Astronomical
//! Algorithms*, Chapter 15) with iterative refinement.
//!
//! All times are returned as Julian Dates in UT.
//!
//! Positions used internally are geometric (not apparent). The atmospheric
//! refraction correction at the horizon (~34') dominates over aberration
//! (~20") and nutation (~9"), so this is standard practice for rise/set
//! computation.

use crate::calendar::{gmst_degrees, gregorian_to_jd};
use crate::coords::{deg_to_rad, mean_obliquity, normalize_degrees, rad_to_deg};
use crate::error::Result;
use crate::planet::Planet;

// ---------------------------------------------------------------------------
// Standard altitude corrections
// ---------------------------------------------------------------------------

/// Standard altitude for rise/set of a point source (stars, planets).
/// Accounts for atmospheric refraction at the horizon (~34 arcminutes).
const REFRACTION_CORRECTION: f64 = -0.5667;

/// Standard altitude for the Sun's upper limb at rise/set.
/// Refraction (34') + solar semi-diameter (16') = 50 arcminutes.
const SUN_RISE_SET_ALTITUDE: f64 = -0.8333;

/// Standard altitude for the Moon's upper limb at rise/set.
/// Approximate; a more precise value depends on the Moon's horizontal parallax.
const MOON_RISE_SET_ALTITUDE: f64 = 0.125;

/// Get the standard horizon altitude for a body in degrees.
fn standard_altitude(planet: Planet) -> f64 {
    match planet {
        Planet::Sun => SUN_RISE_SET_ALTITUDE,
        Planet::Moon => MOON_RISE_SET_ALTITUDE,
        _ => REFRACTION_CORRECTION,
    }
}

// ---------------------------------------------------------------------------
// Position at a given JD
// ---------------------------------------------------------------------------

/// Get the equatorial coordinates (RA in degrees, Dec in degrees) of a body.
fn equatorial_position(planet: Planet, jd: f64) -> Result<(f64, f64)> {
    let (lon, lat) = match planet {
        Planet::Sun => (crate::sun::solar_longitude(jd), 0.0),
        Planet::Moon => (
            crate::moon::lunar_longitude(jd),
            crate::moon::lunar_latitude(jd),
        ),
        _ => {
            let pos = crate::planetary::compute_position(planet, jd)?;
            (pos.longitude_deg, pos.latitude_deg)
        }
    };

    let t = crate::calendar::julian_centuries(jd);
    let eps = mean_obliquity(t);
    let (ra, dec) = crate::coords::ecliptic_to_equatorial(lon, lat, eps);
    Ok((ra, dec))
}

// ---------------------------------------------------------------------------
// Rise/set/transit result
// ---------------------------------------------------------------------------

/// Rise, set, and transit times for a body on a given date.
#[derive(Debug, Clone, Copy)]
pub struct RiseSetTransit {
    /// Julian Date of the body's rise (upper limb above horizon), or `None`
    /// if the body is circumpolar or never rises.
    pub rise: Option<f64>,
    /// Julian Date of the body's meridian transit (culmination).
    pub transit: Option<f64>,
    /// Julian Date of the body's set (upper limb below horizon), or `None`
    /// if the body is circumpolar or never sets.
    pub set: Option<f64>,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Compute rise, set, and meridian transit times for a body on a given date.
///
/// `year`, `month`, `day` specify the date (UT).
/// `lat_deg` and `lon_deg` are the observer's geographic coordinates
/// (latitude positive north, longitude positive east).
///
/// # Errors
///
/// Returns an error if position computation fails.
///
/// # Examples
///
/// ```
/// # use jyotish::riseset::rise_set_transit;
/// # use jyotish::planet::Planet;
/// let rst = rise_set_transit(Planet::Sun, 2000, 1, 1, 51.5, -0.1).unwrap();
/// assert!(rst.rise.is_some());
/// assert!(rst.transit.is_some());
/// assert!(rst.set.is_some());
/// ```
pub fn rise_set_transit(
    planet: Planet,
    year: i32,
    month: u32,
    day: u32,
    lat_deg: f64,
    lon_deg: f64,
) -> Result<RiseSetTransit> {
    let h0 = standard_altitude(planet);

    // JD at 0h UT on the given date
    let jd0 = gregorian_to_jd(year, month, day, 0, 0, 0.0)?;

    // Compute positions at JD-1, JD, JD+1 for interpolation
    let (ra1, dec1) = equatorial_position(planet, jd0 - 1.0)?;
    let (ra2, dec2) = equatorial_position(planet, jd0)?;
    let (ra3, dec3) = equatorial_position(planet, jd0 + 1.0)?;

    // Apparent sidereal time at Greenwich at 0h UT
    let theta0 = gmst_degrees(jd0);

    let lat_rad = deg_to_rad(lat_deg);
    let h0_rad = deg_to_rad(h0);

    // Compute the hour angle at rise/set
    let dec2_rad = deg_to_rad(dec2);
    let cos_h = (h0_rad.sin() - lat_rad.sin() * dec2_rad.sin()) / (lat_rad.cos() * dec2_rad.cos());

    // Check for circumpolar / never-rise conditions
    let (rise_jd, set_jd) = if cos_h < -1.0 {
        // Body is always above the horizon (circumpolar)
        (None, None)
    } else if cos_h > 1.0 {
        // Body never rises
        (None, None)
    } else {
        let h_deg = rad_to_deg(cos_h.acos());

        // Approximate times as fractions of a day
        let m0 = (ra2 + lon_deg - theta0) / 360.0;
        let m1 = m0 - h_deg / 360.0;
        let m2 = m0 + h_deg / 360.0;

        // Normalize to [0, 1)
        let m1 = normalize_m(m1);
        let m2 = normalize_m(m2);

        // Refine with interpolation (one iteration)
        let interp = InterpData {
            ra: [ra1, ra2, ra3],
            dec: [dec1, dec2, dec3],
        };
        let rise = refine_rise_set(m1, theta0, lon_deg, lat_deg, h0, &interp);
        let set = refine_rise_set(m2, theta0, lon_deg, lat_deg, h0, &interp);

        (Some(jd0 + rise), Some(jd0 + set))
    };

    // Transit time
    let m0 = normalize_m((ra2 + lon_deg - theta0) / 360.0);
    let transit_jd = Some(jd0 + m0);

    Ok(RiseSetTransit {
        rise: rise_jd,
        transit: transit_jd,
        set: set_jd,
    })
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Normalize a fractional day to [0, 1).
fn normalize_m(m: f64) -> f64 {
    let r = m % 1.0;
    if r < 0.0 { r + 1.0 } else { r }
}

/// Interpolation data for three consecutive days.
struct InterpData {
    ra: [f64; 3],
    dec: [f64; 3],
}

/// Refine a rise/set time using Meeus's interpolation method.
fn refine_rise_set(
    m: f64,
    theta0: f64,
    lon_deg: f64,
    lat_deg: f64,
    h0: f64,
    interp: &InterpData,
) -> f64 {
    let mut m_refined = m;

    for _ in 0..4 {
        // Interpolated RA and Dec at time m
        let ra = interpolate_angle(interp.ra[0], interp.ra[1], interp.ra[2], m_refined);
        let dec = interpolate(interp.dec[0], interp.dec[1], interp.dec[2], m_refined);

        // Local hour angle
        let theta = theta0 + 360.985_647 * m_refined;
        let h = normalize_degrees(theta - lon_deg - ra);
        let h = if h > 180.0 { h - 360.0 } else { h };

        // Altitude
        let lat_r = deg_to_rad(lat_deg);
        let dec_r = deg_to_rad(dec);
        let h_r = deg_to_rad(h);
        let alt =
            rad_to_deg((lat_r.sin() * dec_r.sin() + lat_r.cos() * dec_r.cos() * h_r.cos()).asin());

        // Correction
        let dm = (alt - h0) / (360.0 * dec_r.cos() * lat_r.cos() * h_r.sin());
        m_refined += dm;

        if dm.abs() < 1e-8 {
            break;
        }
    }

    m_refined
}

/// Quadratic interpolation for a value.
fn interpolate(y1: f64, y2: f64, y3: f64, n: f64) -> f64 {
    let a = y2 - y1;
    let b = y3 - y2;
    let c = b - a;
    y2 + n * (a + b + n * c) / 2.0
}

/// Quadratic interpolation for an angle in degrees, handling wrap-around.
fn interpolate_angle(y1: f64, y2: f64, y3: f64, n: f64) -> f64 {
    // Normalize differences to avoid 360° jumps
    let mut a = y2 - y1;
    let mut b = y3 - y2;
    if a > 180.0 {
        a -= 360.0;
    } else if a < -180.0 {
        a += 360.0;
    }
    if b > 180.0 {
        b -= 360.0;
    } else if b < -180.0 {
        b += 360.0;
    }
    let c = b - a;
    normalize_degrees(y2 + n * (a + b + n * c) / 2.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::jd_to_gregorian;

    #[test]
    fn sun_rise_set_london_j2000() {
        let rst = rise_set_transit(Planet::Sun, 2000, 1, 1, 51.5, -0.1).unwrap();
        assert!(rst.rise.is_some());
        assert!(rst.transit.is_some());
        assert!(rst.set.is_some());

        let rise = rst.rise.unwrap();
        let transit = rst.transit.unwrap();
        let set = rst.set.unwrap();

        // Rise should be before transit, transit before set
        assert!(
            rise < transit,
            "rise {rise} should be before transit {transit}"
        );
        assert!(
            transit < set,
            "transit {transit} should be before set {set}"
        );

        // Sun rises around 8:06 UT in London on Jan 1, 2000
        let (_, _, _, h, m, _) = jd_to_gregorian(rise);
        assert!(h == 8 || h == 7, "rise hour = {h}:{m}");
    }

    #[test]
    fn sun_rise_set_equator() {
        let rst = rise_set_transit(Planet::Sun, 2000, 3, 20, 0.0, 0.0).unwrap();
        assert!(rst.rise.is_some());
        assert!(rst.set.is_some());

        let rise = rst.rise.unwrap();
        let set = rst.set.unwrap();
        // Near equinox at equator, day length ≈ 12h
        let day_length = (set - rise) * 24.0;
        assert!(
            (day_length - 12.0).abs() < 1.0,
            "day length = {day_length}h"
        );
    }

    #[test]
    fn moon_rise_set() {
        let rst = rise_set_transit(Planet::Moon, 2000, 1, 1, 51.5, -0.1).unwrap();
        // Moon should have a transit at minimum
        assert!(rst.transit.is_some());
    }

    #[test]
    fn mars_rise_set() {
        let rst = rise_set_transit(Planet::Mars, 2000, 1, 1, 51.5, -0.1).unwrap();
        assert!(rst.transit.is_some());
    }

    #[test]
    fn normalize_m_range() {
        assert!((normalize_m(0.5) - 0.5).abs() < 1e-10);
        assert!((normalize_m(-0.3) - 0.7).abs() < 1e-10);
        assert!((normalize_m(1.3) - 0.3).abs() < 1e-10);
    }
}
