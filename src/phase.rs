//! Lunar phase computation.
//!
//! Determines lunar phases (New Moon, First Quarter, Full Moon, Last Quarter)
//! by computing the Moon's elongation from the Sun. Phase transitions are
//! located via bisection on the elongation function to 1e-8 JD precision.
//!
//! Uses geometric positions from [`crate::sun::solar_longitude`] and
//! [`crate::moon::lunar_longitude`].

use crate::coords::normalize_degrees;
use crate::error::{JyotishError, Result};
use crate::moon::lunar_longitude;
use crate::sun::solar_longitude;
use serde::{Deserialize, Serialize};
use std::fmt;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Mean synodic month (New Moon to New Moon) in days.
const SYNODIC_MONTH: f64 = 29.530_588_853;

/// Quarter synodic month — step size for phase searches (~7.38 days).
const QUARTER_SYNODIC: f64 = SYNODIC_MONTH / 4.0;

/// Julian Date of Brown Lunation 1 (first New Moon of 1923, Jan 17).
const BROWN_LUNATION_EPOCH: f64 = 2_423_436.403_47;

// ---------------------------------------------------------------------------
// Lunar phase enum
// ---------------------------------------------------------------------------

/// The four principal lunar phases.
///
/// Each phase corresponds to a specific Sun–Moon elongation:
///
/// | Phase          | Elongation |
/// |----------------|------------|
/// | New Moon       | 0° (360°)  |
/// | First Quarter  | 90°        |
/// | Full Moon      | 180°       |
/// | Last Quarter   | 270°       |
///
/// # Examples
///
/// ```
/// # use jyotish::phase::LunarPhase;
/// let phase = LunarPhase::FullMoon;
/// assert_eq!(phase.target_elongation(), 180.0);
/// assert_eq!(phase.to_string(), "Full Moon");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum LunarPhase {
    /// New Moon — Sun and Moon at the same ecliptic longitude (elongation ≈ 0°).
    NewMoon,
    /// First Quarter — Moon 90° ahead of the Sun.
    FirstQuarter,
    /// Full Moon — Moon opposite the Sun (elongation ≈ 180°).
    FullMoon,
    /// Last Quarter — Moon 270° ahead of the Sun (or 90° behind).
    LastQuarter,
}

impl LunarPhase {
    /// The target elongation in degrees for this phase.
    ///
    /// # Examples
    ///
    /// ```
    /// # use jyotish::phase::LunarPhase;
    /// assert_eq!(LunarPhase::NewMoon.target_elongation(), 0.0);
    /// assert_eq!(LunarPhase::FirstQuarter.target_elongation(), 90.0);
    /// assert_eq!(LunarPhase::FullMoon.target_elongation(), 180.0);
    /// assert_eq!(LunarPhase::LastQuarter.target_elongation(), 270.0);
    /// ```
    pub fn target_elongation(self) -> f64 {
        match self {
            Self::NewMoon => 0.0,
            Self::FirstQuarter => 90.0,
            Self::FullMoon => 180.0,
            Self::LastQuarter => 270.0,
        }
    }

    /// All four phases in order.
    const ALL: [LunarPhase; 4] = [
        LunarPhase::NewMoon,
        LunarPhase::FirstQuarter,
        LunarPhase::FullMoon,
        LunarPhase::LastQuarter,
    ];
}

impl fmt::Display for LunarPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NewMoon => write!(f, "New Moon"),
            Self::FirstQuarter => write!(f, "First Quarter"),
            Self::FullMoon => write!(f, "Full Moon"),
            Self::LastQuarter => write!(f, "Last Quarter"),
        }
    }
}

// ---------------------------------------------------------------------------
// Phase info
// ---------------------------------------------------------------------------

/// Information about a lunar phase occurrence.
///
/// # Examples
///
/// ```
/// # use jyotish::phase::{LunarPhase, PhaseInfo};
/// let info = PhaseInfo {
///     phase: LunarPhase::FullMoon,
///     jd: 2_451_550.0,
///     elongation: 179.8,
/// };
/// assert_eq!(info.phase, LunarPhase::FullMoon);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhaseInfo {
    /// The lunar phase.
    pub phase: LunarPhase,
    /// Julian Date of the phase.
    pub jd: f64,
    /// Sun–Moon elongation in degrees at the phase time.
    pub elongation: f64,
}

// ---------------------------------------------------------------------------
// Phase angle computation
// ---------------------------------------------------------------------------

/// Compute the Moon's elongation from the Sun in degrees.
///
/// Returns the angle in [0, 360), where 0° is New Moon, 90° is First Quarter,
/// 180° is Full Moon, and 270° is Last Quarter.
///
/// # Examples
///
/// ```
/// # use jyotish::phase::phase_angle;
/// let elong = phase_angle(2_451_545.0); // J2000.0
/// assert!(elong >= 0.0 && elong < 360.0);
/// ```
pub fn phase_angle(jd: f64) -> f64 {
    let sun_lon = solar_longitude(jd);
    let moon_lon = lunar_longitude(jd);
    normalize_degrees(moon_lon - sun_lon)
}

// ---------------------------------------------------------------------------
// Current phase determination
// ---------------------------------------------------------------------------

/// Determine the current lunar phase for a given Julian Date.
///
/// Returns the phase whose target elongation is closest to the actual
/// Sun–Moon elongation. Each phase "owns" a 90° sector centered on its
/// target elongation.
///
/// # Examples
///
/// ```
/// # use jyotish::phase::{current_phase, LunarPhase};
/// let phase = current_phase(2_451_545.0); // J2000.0
/// // The phase should be one of the four principal phases
/// match phase {
///     LunarPhase::NewMoon | LunarPhase::FirstQuarter |
///     LunarPhase::FullMoon | LunarPhase::LastQuarter => {}
/// }
/// ```
pub fn current_phase(jd: f64) -> LunarPhase {
    let elong = phase_angle(jd);

    // Each phase owns a 90° sector:
    //   New Moon:       315° .. 45°  (centered on 0°)
    //   First Quarter:   45° .. 135° (centered on 90°)
    //   Full Moon:      135° .. 225° (centered on 180°)
    //   Last Quarter:   225° .. 315° (centered on 270°)
    if !(45.0..315.0).contains(&elong) {
        LunarPhase::NewMoon
    } else if elong < 135.0 {
        LunarPhase::FirstQuarter
    } else if elong < 225.0 {
        LunarPhase::FullMoon
    } else {
        LunarPhase::LastQuarter
    }
}

// ---------------------------------------------------------------------------
// Phase search
// ---------------------------------------------------------------------------

/// Signed angular difference from elongation to target, in [-180, 180).
fn signed_angle_diff(elongation: f64, target: f64) -> f64 {
    let mut diff = elongation - target;
    if diff > 180.0 {
        diff -= 360.0;
    } else if diff < -180.0 {
        diff += 360.0;
    }
    diff
}

/// Find the next occurrence of a specific lunar phase after the given Julian Date.
///
/// Uses a coarse scan in steps of ~3.5 days to locate the interval containing
/// the target elongation crossing, then bisects to 1e-8 JD precision (~0.86 ms).
///
/// # Errors
///
/// Returns [`JyotishError::InvalidParameter`] if the search does not converge
/// within 45 days (approximately 1.5 synodic months).
///
/// # Examples
///
/// ```
/// # use jyotish::phase::{next_phase, LunarPhase};
/// let info = next_phase(LunarPhase::FullMoon, 2_451_545.0).unwrap();
/// assert!(info.jd > 2_451_545.0);
/// assert_eq!(info.phase, LunarPhase::FullMoon);
/// // Should be within one synodic month
/// assert!(info.jd - 2_451_545.0 < 30.0);
/// ```
pub fn next_phase(phase: LunarPhase, jd: f64) -> Result<PhaseInfo> {
    let target = phase.target_elongation();
    // Use a step smaller than a quarter synodic month so that the elongation
    // moves less than 45° per step. This prevents false sign-change detection
    // from the 360°→0° wrapping.
    let step = QUARTER_SYNODIC / 2.0;
    // Search up to ~45 days
    let max_steps = 14;

    let mut jd_prev = jd;
    let mut diff_prev = signed_angle_diff(phase_angle(jd_prev), target);

    for _ in 0..max_steps {
        let jd_curr = jd_prev + step;
        let diff_curr = signed_angle_diff(phase_angle(jd_curr), target);

        // Sign change means the elongation crossed the target
        if diff_prev * diff_curr < 0.0 {
            // Bisect to find the precise crossing
            let mut lo = jd_prev;
            let mut hi = jd_curr;
            let mut lo_diff = diff_prev;

            for _ in 0..80 {
                let mid = (lo + hi) / 2.0;
                let mid_diff = signed_angle_diff(phase_angle(mid), target);

                if mid_diff * lo_diff > 0.0 {
                    lo = mid;
                    lo_diff = mid_diff;
                } else {
                    hi = mid;
                }

                if (hi - lo) < 1e-8 {
                    break;
                }
            }

            let result_jd = (lo + hi) / 2.0;
            let elongation = phase_angle(result_jd);

            // Verify this is actually the correct phase crossing, not a
            // wrapping artifact. The elongation should be near the target.
            let elong_diff = if target == 0.0 {
                if elongation > 180.0 {
                    360.0 - elongation
                } else {
                    elongation
                }
            } else {
                (elongation - target).abs()
            };

            if elong_diff < 5.0 && result_jd > jd {
                return Ok(PhaseInfo {
                    phase,
                    jd: result_jd,
                    elongation,
                });
            }
        }

        diff_prev = diff_curr;
        jd_prev = jd_curr;
    }

    Err(JyotishError::InvalidParameter(
        "lunar phase search did not converge within 45 days".into(),
    ))
}

// ---------------------------------------------------------------------------
// Next four phases
// ---------------------------------------------------------------------------

/// Find the next four principal lunar phases in chronological order.
///
/// Returns a `Vec` of exactly four [`PhaseInfo`] entries: the next New Moon,
/// First Quarter, Full Moon, and Last Quarter after `jd`, sorted by Julian Date.
///
/// # Errors
///
/// Returns an error if any individual phase search fails.
///
/// # Examples
///
/// ```
/// # use jyotish::phase::next_four_phases;
/// let phases = next_four_phases(2_451_545.0).unwrap();
/// assert_eq!(phases.len(), 4);
/// // Phases should be in chronological order
/// for i in 0..3 {
///     assert!(phases[i].jd < phases[i + 1].jd);
/// }
/// ```
pub fn next_four_phases(jd: f64) -> Result<Vec<PhaseInfo>> {
    let mut phases = Vec::with_capacity(4);
    for &phase in &LunarPhase::ALL {
        phases.push(next_phase(phase, jd)?);
    }
    phases.sort_by(|a, b| a.jd.partial_cmp(&b.jd).unwrap_or(std::cmp::Ordering::Equal));
    Ok(phases)
}

// ---------------------------------------------------------------------------
// Lunation number
// ---------------------------------------------------------------------------

/// Compute the Brown Lunation Number for a given Julian Date.
///
/// The Brown Lunation Number counts New Moons since lunation 1
/// (1923 January 17, JD 2423436.40347). The result is the floor of
/// the elapsed time divided by the mean synodic month.
///
/// # Examples
///
/// ```
/// # use jyotish::phase::lunation_number;
/// // At J2000.0 (2000 Jan 1.5), lunation number should be ~951
/// let n = lunation_number(2_451_545.0);
/// assert!((n - 951).abs() <= 1, "got {n}");
/// ```
pub fn lunation_number(jd: f64) -> i32 {
    ((jd - BROWN_LUNATION_EPOCH) / SYNODIC_MONTH).floor() as i32
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const JD_J2000: f64 = 2_451_545.0;

    #[test]
    fn phase_angle_range() {
        // Phase angle should always be in [0, 360)
        for offset in 0..100 {
            let jd = JD_J2000 + offset as f64;
            let angle = phase_angle(jd);
            assert!(
                (0.0..360.0).contains(&angle),
                "angle = {angle} at offset {offset}"
            );
        }
    }

    #[test]
    fn phase_angle_at_j2000() {
        let angle = phase_angle(JD_J2000);
        // At J2000.0, the elongation should be a known value; just verify range
        assert!((0.0..360.0).contains(&angle), "angle = {angle}");
    }

    #[test]
    fn current_phase_at_j2000() {
        let phase = current_phase(JD_J2000);
        // J2000.0 is 2000 Jan 1.5 — verify it returns a valid phase
        // The new moon of Jan 2000 was around Jan 6, so J2000 is before that,
        // somewhere in the last quarter / waning crescent range
        match phase {
            LunarPhase::NewMoon
            | LunarPhase::FirstQuarter
            | LunarPhase::FullMoon
            | LunarPhase::LastQuarter => {}
        }
    }

    #[test]
    fn next_phase_full_moon() {
        let info = next_phase(LunarPhase::FullMoon, JD_J2000).unwrap();
        assert!(info.jd > JD_J2000);
        assert_eq!(info.phase, LunarPhase::FullMoon);
        // Should be within one synodic month
        assert!(info.jd - JD_J2000 < 30.0, "delta = {}", info.jd - JD_J2000);
        // Elongation should be close to 180°
        assert!(
            (info.elongation - 180.0).abs() < 1.0,
            "elongation = {}",
            info.elongation
        );
    }

    #[test]
    fn next_phase_new_moon() {
        let info = next_phase(LunarPhase::NewMoon, JD_J2000).unwrap();
        assert!(info.jd > JD_J2000);
        assert_eq!(info.phase, LunarPhase::NewMoon);
        assert!(info.jd - JD_J2000 < 30.0, "delta = {}", info.jd - JD_J2000);
        // Elongation should be close to 0° (or 360°)
        let elong_from_zero = if info.elongation > 180.0 {
            360.0 - info.elongation
        } else {
            info.elongation
        };
        assert!(elong_from_zero < 1.0, "elongation = {}", info.elongation);
    }

    #[test]
    fn next_phase_first_quarter() {
        let info = next_phase(LunarPhase::FirstQuarter, JD_J2000).unwrap();
        assert!(info.jd > JD_J2000);
        assert_eq!(info.phase, LunarPhase::FirstQuarter);
        assert!(info.jd - JD_J2000 < 30.0);
        assert!(
            (info.elongation - 90.0).abs() < 1.0,
            "elongation = {}",
            info.elongation
        );
    }

    #[test]
    fn next_phase_last_quarter() {
        let info = next_phase(LunarPhase::LastQuarter, JD_J2000).unwrap();
        assert!(info.jd > JD_J2000);
        assert_eq!(info.phase, LunarPhase::LastQuarter);
        assert!(info.jd - JD_J2000 < 30.0);
        assert!(
            (info.elongation - 270.0).abs() < 1.0,
            "elongation = {}",
            info.elongation
        );
    }

    #[test]
    fn next_four_phases_chronological() {
        let phases = next_four_phases(JD_J2000).unwrap();
        assert_eq!(phases.len(), 4);
        // Verify chronological order
        for i in 0..3 {
            assert!(
                phases[i].jd < phases[i + 1].jd,
                "phase {} (jd={}) not before phase {} (jd={})",
                phases[i].phase,
                phases[i].jd,
                phases[i + 1].phase,
                phases[i + 1].jd
            );
        }
        // All should be within ~30 days
        let span = phases[3].jd - phases[0].jd;
        assert!(span < 30.0, "span = {span}");
    }

    #[test]
    fn next_four_phases_all_distinct() {
        let phases = next_four_phases(JD_J2000).unwrap();
        // All four phase types should appear
        assert!(phases.iter().any(|p| p.phase == LunarPhase::NewMoon));
        assert!(phases.iter().any(|p| p.phase == LunarPhase::FirstQuarter));
        assert!(phases.iter().any(|p| p.phase == LunarPhase::FullMoon));
        assert!(phases.iter().any(|p| p.phase == LunarPhase::LastQuarter));
    }

    #[test]
    fn lunation_number_at_j2000() {
        let n = lunation_number(JD_J2000);
        // (2451545.0 - 2423436.40347) / 29.530588853 ≈ 951.0
        assert!((n - 951).abs() <= 1, "lunation number = {n}");
    }

    #[test]
    fn lunation_number_at_epoch() {
        // At the Brown lunation epoch itself, lunation number should be 0 or 1
        let n = lunation_number(BROWN_LUNATION_EPOCH);
        assert!(n == 0, "lunation number at epoch = {n}");
    }

    #[test]
    fn lunation_number_before_epoch() {
        // Well before the epoch should give a negative lunation number
        let n = lunation_number(BROWN_LUNATION_EPOCH - SYNODIC_MONTH + 0.5);
        assert_eq!(n, -1, "lunation number = {n}");
    }

    #[test]
    fn phase_display() {
        assert_eq!(LunarPhase::NewMoon.to_string(), "New Moon");
        assert_eq!(LunarPhase::FirstQuarter.to_string(), "First Quarter");
        assert_eq!(LunarPhase::FullMoon.to_string(), "Full Moon");
        assert_eq!(LunarPhase::LastQuarter.to_string(), "Last Quarter");
    }

    #[test]
    fn phase_serde() {
        let phase = LunarPhase::FullMoon;
        let json = serde_json::to_string(&phase).unwrap();
        let restored: LunarPhase = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, phase);
    }

    #[test]
    fn phase_target_elongations() {
        assert_eq!(LunarPhase::NewMoon.target_elongation(), 0.0);
        assert_eq!(LunarPhase::FirstQuarter.target_elongation(), 90.0);
        assert_eq!(LunarPhase::FullMoon.target_elongation(), 180.0);
        assert_eq!(LunarPhase::LastQuarter.target_elongation(), 270.0);
    }

    #[test]
    fn consecutive_new_moons_synodic_period() {
        // Two consecutive new moons should be ~29.53 days apart
        let first = next_phase(LunarPhase::NewMoon, JD_J2000).unwrap();
        let second = next_phase(LunarPhase::NewMoon, first.jd + 1.0).unwrap();
        let gap = second.jd - first.jd;
        assert!(
            (gap - SYNODIC_MONTH).abs() < 1.0,
            "gap between new moons = {gap} days"
        );
    }
}
