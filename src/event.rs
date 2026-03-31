//! Celestial events — eclipses, conjunctions, oppositions, equinoxes, solstices.
//!
//! Provides search functions for major astronomical events. Uses bisection
//! on the position functions to locate event times to high precision.

use crate::coords::normalize_degrees;
use crate::error::{JyotishError, Result};
use crate::planet::Planet;
use crate::sun::solar_longitude;
use serde::{Deserialize, Serialize};
use std::fmt;

// ---------------------------------------------------------------------------
// Event types
// ---------------------------------------------------------------------------

/// A seasonal event (equinox or solstice).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Season {
    /// March equinox (Sun at 0° longitude).
    VernalEquinox,
    /// June solstice (Sun at 90° longitude).
    SummerSolstice,
    /// September equinox (Sun at 180° longitude).
    AutumnalEquinox,
    /// December solstice (Sun at 270° longitude).
    WinterSolstice,
}

impl Season {
    /// The target solar longitude for this seasonal event.
    pub fn target_longitude(self) -> f64 {
        match self {
            Self::VernalEquinox => 0.0,
            Self::SummerSolstice => 90.0,
            Self::AutumnalEquinox => 180.0,
            Self::WinterSolstice => 270.0,
        }
    }
}

impl fmt::Display for Season {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::VernalEquinox => write!(f, "Vernal Equinox"),
            Self::SummerSolstice => write!(f, "Summer Solstice"),
            Self::AutumnalEquinox => write!(f, "Autumnal Equinox"),
            Self::WinterSolstice => write!(f, "Winter Solstice"),
        }
    }
}

/// A conjunction or opposition between two bodies.
#[derive(Debug, Clone, Copy)]
pub struct SynodicEvent {
    /// The two bodies involved.
    pub body1: Planet,
    /// The second body.
    pub body2: Planet,
    /// Julian Date of the event.
    pub jd: f64,
    /// Angular separation at the event (0° for conjunction, ~180° for opposition).
    pub separation: f64,
    /// Whether this is an opposition (true) or conjunction (false).
    pub is_opposition: bool,
}

// ---------------------------------------------------------------------------
// Equinox and solstice search
// ---------------------------------------------------------------------------

/// Find the next occurrence of a seasonal event after the given Julian Date.
///
/// Uses the Sun's ecliptic longitude to search for the target crossing.
///
/// # Examples
///
/// ```
/// # use jyotish::event::{next_season, Season};
/// let jd = next_season(Season::VernalEquinox, 2_451_545.0).unwrap();
/// assert!(jd > 2_451_545.0);
/// // Vernal equinox 2000 was around March 20 → JD ~2451623
/// assert!((jd - 2_451_623.0).abs() < 2.0, "got {jd}");
/// ```
pub fn next_season(season: Season, jd: f64) -> Result<f64> {
    let target = season.target_longitude();

    // Estimate: find roughly when the Sun reaches the target longitude
    let current_lon = solar_longitude(jd);
    let mut angle_ahead = target - current_lon;
    if angle_ahead <= 0.0 {
        angle_ahead += 360.0;
    }
    // Sun moves ~1°/day
    let estimate = jd + angle_ahead;

    // Search in a window around the estimate
    let mut lo = estimate - 20.0;
    let mut hi = estimate + 20.0;

    // Bisect: find where solar_longitude crosses the target
    for _ in 0..80 {
        let mid = (lo + hi) / 2.0;
        let lon = solar_longitude(mid);
        let diff = signed_angle_diff(lon, target);

        if diff < 0.0 {
            lo = mid;
        } else {
            hi = mid;
        }

        if (hi - lo) < 1e-8 {
            break;
        }
    }

    let result = (lo + hi) / 2.0;

    // Verify the result is after the input JD
    if result < jd {
        // Rare edge case: search again one year ahead
        return next_season(season, jd + 300.0);
    }

    Ok(result)
}

/// Find all four seasonal events (equinoxes and solstices) for the year
/// containing the given Julian Date.
///
/// Returns events in chronological order.
pub fn seasons_of_year(jd: f64) -> Result<Vec<(Season, f64)>> {
    let seasons = [
        Season::VernalEquinox,
        Season::SummerSolstice,
        Season::AutumnalEquinox,
        Season::WinterSolstice,
    ];

    // Start from the beginning of the approximate year
    let start = jd - 180.0;
    let mut results = Vec::with_capacity(4);

    for season in seasons {
        let event_jd = next_season(season, start)?;
        if event_jd < jd + 366.0 {
            results.push((season, event_jd));
        }
    }

    results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    Ok(results)
}

// ---------------------------------------------------------------------------
// Conjunction / opposition search
// ---------------------------------------------------------------------------

/// Find the next conjunction between two planets after the given Julian Date.
///
/// A conjunction occurs when two bodies have the same ecliptic longitude
/// (angular separation ≈ 0°).
///
/// # Errors
///
/// Returns an error if the search doesn't converge within 2 years.
pub fn next_conjunction(body1: Planet, body2: Planet, jd: f64) -> Result<SynodicEvent> {
    find_synodic_event(body1, body2, jd, false)
}

/// Find the next opposition between two planets after the given Julian Date.
///
/// An opposition occurs when two bodies are ~180° apart in ecliptic longitude.
/// Only meaningful for superior planets (Mars+) relative to the Sun.
///
/// # Errors
///
/// Returns an error if the search doesn't converge within 2 years.
pub fn next_opposition(body1: Planet, body2: Planet, jd: f64) -> Result<SynodicEvent> {
    find_synodic_event(body1, body2, jd, true)
}

fn longitude_of(planet: Planet, jd: f64) -> Result<f64> {
    match planet {
        Planet::Sun => Ok(solar_longitude(jd)),
        Planet::Moon => Ok(crate::moon::lunar_longitude(jd)),
        _ => Ok(crate::planetary::compute_position(planet, jd)?.longitude_deg),
    }
}

fn find_synodic_event(
    body1: Planet,
    body2: Planet,
    jd: f64,
    opposition: bool,
) -> Result<SynodicEvent> {
    let target_sep = if opposition { 180.0 } else { 0.0 };
    let step = 5.0;
    let max_steps = 200;

    let mut jd_prev = jd;
    let mut diff_prev = synodic_diff(body1, body2, jd_prev, target_sep)?;

    for _ in 0..max_steps {
        let jd_curr = jd_prev + step;
        let diff_curr = synodic_diff(body1, body2, jd_curr, target_sep)?;

        // Look for sign change (crossing through zero)
        if diff_prev * diff_curr < 0.0 {
            // Bisect
            let mut lo = jd_prev;
            let mut hi = jd_curr;
            for _ in 0..60 {
                let mid = (lo + hi) / 2.0;
                let diff_mid = synodic_diff(body1, body2, mid, target_sep)?;
                if diff_mid * diff_prev > 0.0 {
                    lo = mid;
                } else {
                    hi = mid;
                }
                if (hi - lo) < 1e-8 {
                    break;
                }
            }

            let event_jd = (lo + hi) / 2.0;
            let lon1 = longitude_of(body1, event_jd)?;
            let lon2 = longitude_of(body2, event_jd)?;
            let sep = crate::aspect::angular_separation(lon1, lon2);

            // Verify this is actually the correct type of event
            // (wrapping at 0°/360° can cause false positives)
            if (sep - target_sep).abs() < 10.0 {
                return Ok(SynodicEvent {
                    body1,
                    body2,
                    jd: event_jd,
                    separation: sep,
                    is_opposition: opposition,
                });
            }
        }

        diff_prev = diff_curr;
        jd_prev = jd_curr;
    }

    Err(JyotishError::InvalidParameter(
        "synodic event search did not converge".into(),
    ))
}

/// Compute the signed angular difference for synodic event detection.
///
/// For conjunction (target=0): tracks when lon1-lon2 crosses 0°.
/// For opposition (target=180): tracks when lon1-lon2 crosses 180°.
fn synodic_diff(body1: Planet, body2: Planet, jd: f64, target_sep: f64) -> Result<f64> {
    let lon1 = longitude_of(body1, jd)?;
    let lon2 = longitude_of(body2, jd)?;
    // Elongation of body1 from body2 in [0, 360)
    let elongation = normalize_degrees(lon1 - lon2);
    // Signed difference from target, in [-180, 180)
    let mut diff = elongation - target_sep;
    if diff > 180.0 {
        diff -= 360.0;
    } else if diff < -180.0 {
        diff += 360.0;
    }
    Ok(diff)
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Signed angular difference: positive if `lon` is ahead of `target` (CCW).
fn signed_angle_diff(lon: f64, target: f64) -> f64 {
    let mut diff = lon - target;
    if diff > 180.0 {
        diff -= 360.0;
    } else if diff < -180.0 {
        diff += 360.0;
    }
    diff
}

#[cfg(test)]
mod tests {
    use super::*;

    const JD_J2000: f64 = 2_451_545.0;

    #[test]
    fn vernal_equinox_2000() {
        let jd = next_season(Season::VernalEquinox, JD_J2000).unwrap();
        // Vernal equinox 2000: March 20, ~JD 2451623.8
        assert!((jd - 2_451_623.8).abs() < 1.5, "got {jd}");
    }

    #[test]
    fn summer_solstice_2000() {
        let jd = next_season(Season::SummerSolstice, JD_J2000).unwrap();
        // Summer solstice 2000: June 21, ~JD 2451716
        assert!((jd - 2_451_716.0).abs() < 2.0, "got {jd}");
    }

    #[test]
    fn all_four_seasons() {
        let seasons = seasons_of_year(JD_J2000).unwrap();
        assert_eq!(seasons.len(), 4);
        // Should be in chronological order
        for i in 0..3 {
            assert!(seasons[i].1 < seasons[i + 1].1);
        }
    }

    #[test]
    fn season_display() {
        assert_eq!(Season::VernalEquinox.to_string(), "Vernal Equinox");
        assert_eq!(Season::WinterSolstice.to_string(), "Winter Solstice");
    }

    #[test]
    fn season_serde() {
        let season = Season::AutumnalEquinox;
        let json = serde_json::to_string(&season).unwrap();
        let restored: Season = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, season);
    }

    #[test]
    fn conjunction_venus_jupiter() {
        // Find the next Venus-Jupiter conjunction after J2000.0
        let event = next_conjunction(Planet::Venus, Planet::Jupiter, JD_J2000).unwrap();
        assert!(event.jd > JD_J2000);
        assert!(!event.is_opposition);
        // At conjunction, separation should be small
        assert!(event.separation < 5.0, "separation = {}", event.separation);
    }

    #[test]
    fn opposition_jupiter_sun() {
        // Find the next Jupiter-Sun opposition after J2000.0
        let event = next_opposition(Planet::Jupiter, Planet::Sun, JD_J2000).unwrap();
        assert!(event.jd > JD_J2000);
        assert!(event.is_opposition);
        // At opposition, separation should be near 180°
        assert!(
            (event.separation - 180.0).abs() < 10.0,
            "separation = {}",
            event.separation
        );
    }
}
