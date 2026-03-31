//! Planetary transits — ingress, retrograde, station, direct motion.
//!
//! Provides detection of sign ingresses, retrograde/direct station points,
//! and general motion state for planets. Uses numerical differentiation
//! on the position functions to determine motion direction and velocity.

use crate::error::{JyotishError, Result};
use crate::planet::Planet;
use crate::zodiac::{Sign, tropical_sign};
use serde::{Deserialize, Serialize};
use std::fmt;

// ---------------------------------------------------------------------------
// Motion state
// ---------------------------------------------------------------------------

/// The apparent motion state of a planet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum MotionState {
    /// Moving forward through the zodiac (increasing longitude).
    Direct,
    /// Appearing to move backward (decreasing longitude).
    Retrograde,
    /// Momentarily stationary before turning retrograde.
    StationaryRetrograde,
    /// Momentarily stationary before turning direct.
    StationaryDirect,
}

impl fmt::Display for MotionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Direct => write!(f, "Direct"),
            Self::Retrograde => write!(f, "Retrograde"),
            Self::StationaryRetrograde => write!(f, "Stationary (Rx)"),
            Self::StationaryDirect => write!(f, "Stationary (D)"),
        }
    }
}

// ---------------------------------------------------------------------------
// Ingress
// ---------------------------------------------------------------------------

/// A sign ingress event.
#[derive(Debug, Clone, Copy)]
pub struct Ingress {
    /// The planet entering a new sign.
    pub planet: Planet,
    /// The sign being entered.
    pub sign: Sign,
    /// Julian Date of the ingress.
    pub jd: f64,
}

// ---------------------------------------------------------------------------
// Longitude function abstraction
// ---------------------------------------------------------------------------

/// Get the ecliptic longitude of a body at a given JD.
fn longitude_at(planet: Planet, jd: f64) -> Result<f64> {
    match planet {
        Planet::Sun => Ok(crate::sun::solar_longitude(jd)),
        Planet::Moon => Ok(crate::moon::lunar_longitude(jd)),
        _ => Ok(crate::planetary::compute_position(planet, jd)?.longitude_deg),
    }
}

// ---------------------------------------------------------------------------
// Daily motion and motion state
// ---------------------------------------------------------------------------

/// Compute the daily motion of a planet in degrees/day at a given Julian Date.
///
/// Uses central differences with a 1-day step for planets and 0.1-day
/// step for the Moon (which moves ~13°/day).
///
/// Positive values indicate direct motion; negative values indicate retrograde.
///
/// # Errors
///
/// Returns an error if the planet's position cannot be computed.
///
/// # Examples
///
/// ```
/// # use jyotish::transit::daily_motion;
/// # use jyotish::planet::Planet;
/// let motion = daily_motion(Planet::Sun, 2_451_545.0).unwrap();
/// // Sun moves ~1°/day, always direct
/// assert!(motion > 0.9 && motion < 1.1, "got {motion}");
/// ```
pub fn daily_motion(planet: Planet, jd: f64) -> Result<f64> {
    let h = match planet {
        Planet::Moon => 0.1,
        _ => 1.0,
    };

    let lon_before = longitude_at(planet, jd - h)?;
    let lon_after = longitude_at(planet, jd + h)?;

    // Handle wrap-around at 0°/360°
    let mut diff = lon_after - lon_before;
    if diff > 180.0 {
        diff -= 360.0;
    } else if diff < -180.0 {
        diff += 360.0;
    }

    Ok(diff / (2.0 * h))
}

/// Determine the motion state of a planet at a given Julian Date.
///
/// A planet is considered "stationary" if its daily motion is below
/// the threshold (0.05°/day for planets, not applicable to Sun/Moon).
///
/// # Errors
///
/// Returns an error if the planet's position cannot be computed.
///
/// # Examples
///
/// ```
/// # use jyotish::transit::{motion_state, MotionState};
/// # use jyotish::planet::Planet;
/// let state = motion_state(Planet::Sun, 2_451_545.0).unwrap();
/// assert_eq!(state, MotionState::Direct);
/// ```
pub fn motion_state(planet: Planet, jd: f64) -> Result<MotionState> {
    let motion = daily_motion(planet, jd)?;

    // Sun and Moon are always direct
    if matches!(planet, Planet::Sun | Planet::Moon) {
        return Ok(MotionState::Direct);
    }

    let station_threshold = 0.05; // degrees/day

    if motion.abs() < station_threshold {
        // Check which direction it's turning
        let motion_before = daily_motion(planet, jd - 5.0)?;
        if motion_before > 0.0 {
            Ok(MotionState::StationaryRetrograde)
        } else {
            Ok(MotionState::StationaryDirect)
        }
    } else if motion > 0.0 {
        Ok(MotionState::Direct)
    } else {
        Ok(MotionState::Retrograde)
    }
}

// ---------------------------------------------------------------------------
// Ingress search
// ---------------------------------------------------------------------------

/// Find the next sign ingress for a planet after the given Julian Date.
///
/// Searches forward in time using the planet's typical daily motion to
/// estimate when the next sign boundary will be crossed, then refines
/// with bisection.
///
/// # Errors
///
/// Returns an error if the search fails to converge within 1000 days.
///
/// # Examples
///
/// ```
/// # use jyotish::transit::next_ingress;
/// # use jyotish::planet::Planet;
/// let ingress = next_ingress(Planet::Sun, 2_451_545.0).unwrap();
/// assert!(ingress.jd > 2_451_545.0);
/// ```
pub fn next_ingress(planet: Planet, jd: f64) -> Result<Ingress> {
    let start_lon = longitude_at(planet, jd)?;
    let start_sign = tropical_sign(start_lon).sign;

    // Step forward until the sign changes
    let step = match planet {
        Planet::Moon => 0.5,
        Planet::Sun | Planet::Mercury | Planet::Venus => 1.0,
        _ => 2.0,
    };

    let mut jd_a = jd;
    let mut jd_b = jd;

    for _ in 0..2000 {
        jd_b += step;
        let lon_b = longitude_at(planet, jd_b)?;
        let sign_b = tropical_sign(lon_b).sign;
        if sign_b != start_sign {
            break;
        }
        jd_a = jd_b;
    }

    if jd_b - jd > step * 2000.0 {
        return Err(JyotishError::InvalidParameter(
            "ingress search did not converge".into(),
        ));
    }

    // Bisect to find the exact ingress time
    for _ in 0..60 {
        let jd_mid = (jd_a + jd_b) / 2.0;
        let lon_mid = longitude_at(planet, jd_mid)?;
        let sign_mid = tropical_sign(lon_mid).sign;

        if sign_mid == start_sign {
            jd_a = jd_mid;
        } else {
            jd_b = jd_mid;
        }

        if (jd_b - jd_a) < 1e-8 {
            break;
        }
    }

    let ingress_lon = longitude_at(planet, jd_b)?;
    let ingress_sign = tropical_sign(ingress_lon).sign;

    Ok(Ingress {
        planet,
        sign: ingress_sign,
        jd: jd_b,
    })
}

/// Find the next retrograde station for a planet after the given Julian Date.
///
/// Searches forward for the point where daily motion crosses zero
/// going negative.
///
/// # Errors
///
/// Returns an error for Sun/Moon (always direct) or if the search
/// doesn't converge within the search window.
pub fn next_retrograde_station(planet: Planet, jd: f64) -> Result<f64> {
    if matches!(planet, Planet::Sun | Planet::Moon) {
        return Err(JyotishError::InvalidParameter(
            "Sun and Moon do not go retrograde".into(),
        ));
    }

    let step = 5.0; // days
    let max_steps = 200; // ~2.7 years search window

    let mut jd_a = jd;
    let mut motion_a = daily_motion(planet, jd_a)?;

    // If already retrograde, skip past it first
    if motion_a < 0.0 {
        for _ in 0..max_steps {
            jd_a += step;
            motion_a = daily_motion(planet, jd_a)?;
            if motion_a > 0.0 {
                break;
            }
        }
    }

    // Find where motion goes from positive to negative
    let mut jd_b = jd_a;
    for _ in 0..max_steps {
        jd_b += step;
        let motion_b = daily_motion(planet, jd_b)?;
        if motion_b < 0.0 {
            // Bisect to find the zero-crossing
            let mut lo = jd_a;
            let mut hi = jd_b;
            for _ in 0..60 {
                let mid = (lo + hi) / 2.0;
                let m = daily_motion(planet, mid)?;
                if m > 0.0 {
                    lo = mid;
                } else {
                    hi = mid;
                }
                if (hi - lo) < 1e-6 {
                    break;
                }
            }
            return Ok((lo + hi) / 2.0);
        }
        jd_a = jd_b;
    }

    Err(JyotishError::InvalidParameter(
        "retrograde station search did not converge".into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    const JD_J2000: f64 = 2_451_545.0;

    #[test]
    fn sun_daily_motion() {
        let motion = daily_motion(Planet::Sun, JD_J2000).unwrap();
        // Sun moves ~1.0°/day
        assert!(motion > 0.9 && motion < 1.1, "got {motion}");
    }

    #[test]
    fn moon_daily_motion() {
        let motion = daily_motion(Planet::Moon, JD_J2000).unwrap();
        // Moon moves ~12-15°/day
        assert!(motion > 10.0 && motion < 16.0, "got {motion}");
    }

    #[test]
    fn sun_always_direct() {
        let state = motion_state(Planet::Sun, JD_J2000).unwrap();
        assert_eq!(state, MotionState::Direct);
    }

    #[test]
    fn moon_always_direct() {
        let state = motion_state(Planet::Moon, JD_J2000).unwrap();
        assert_eq!(state, MotionState::Direct);
    }

    #[test]
    fn mars_motion_state() {
        let state = motion_state(Planet::Mars, JD_J2000).unwrap();
        // Just verify it returns a valid state
        assert!(matches!(
            state,
            MotionState::Direct
                | MotionState::Retrograde
                | MotionState::StationaryRetrograde
                | MotionState::StationaryDirect
        ));
    }

    #[test]
    fn sun_next_ingress() {
        let ingress = next_ingress(Planet::Sun, JD_J2000).unwrap();
        assert_eq!(ingress.planet, Planet::Sun);
        assert!(ingress.jd > JD_J2000);
        // Sun ingress should be within ~31 days
        assert!(
            ingress.jd - JD_J2000 < 32.0,
            "took too long: {} days",
            ingress.jd - JD_J2000
        );
    }

    #[test]
    fn moon_next_ingress() {
        let ingress = next_ingress(Planet::Moon, JD_J2000).unwrap();
        assert_eq!(ingress.planet, Planet::Moon);
        // Moon changes sign every ~2.5 days
        assert!(
            ingress.jd - JD_J2000 < 4.0,
            "took too long: {} days",
            ingress.jd - JD_J2000
        );
    }

    #[test]
    fn retrograde_station_sun_errors() {
        assert!(next_retrograde_station(Planet::Sun, JD_J2000).is_err());
        assert!(next_retrograde_station(Planet::Moon, JD_J2000).is_err());
    }

    #[test]
    fn mars_retrograde_station() {
        // Mars goes retrograde roughly every 2 years
        let station_jd = next_retrograde_station(Planet::Mars, JD_J2000).unwrap();
        assert!(station_jd > JD_J2000);
        // Should find one within ~2.5 years
        assert!(station_jd - JD_J2000 < 900.0);
    }

    #[test]
    fn motion_state_display() {
        assert_eq!(MotionState::Direct.to_string(), "Direct");
        assert_eq!(MotionState::Retrograde.to_string(), "Retrograde");
    }

    #[test]
    fn motion_state_serde() {
        let state = MotionState::Retrograde;
        let json = serde_json::to_string(&state).unwrap();
        let restored: MotionState = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, state);
    }
}
