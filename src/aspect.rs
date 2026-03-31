//! Planetary aspects — conjunction, opposition, trine, square, sextile, orbs.
//!
//! An aspect is the angular separation between two celestial bodies along
//! the ecliptic. This module provides aspect type definitions, detection
//! with configurable orbs, and aspect strength computation.

use crate::coords::normalize_degrees;
use serde::{Deserialize, Serialize};
use std::fmt;

// ---------------------------------------------------------------------------
// Aspect types
// ---------------------------------------------------------------------------

/// A type of angular aspect between two bodies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AspectKind {
    /// Conjunction (0°)
    Conjunction,
    /// Sextile (60°)
    Sextile,
    /// Square (90°)
    Square,
    /// Trine (120°)
    Trine,
    /// Opposition (180°)
    Opposition,
}

impl AspectKind {
    /// The exact angular separation for this aspect in degrees.
    pub fn angle(self) -> f64 {
        match self {
            Self::Conjunction => 0.0,
            Self::Sextile => 60.0,
            Self::Square => 90.0,
            Self::Trine => 120.0,
            Self::Opposition => 180.0,
        }
    }

    /// Default orb for this aspect in degrees.
    ///
    /// These are standard orbs used in classical astrology; tighter aspects
    /// get wider orbs.
    pub fn default_orb(self) -> f64 {
        match self {
            Self::Conjunction => 10.0,
            Self::Opposition => 10.0,
            Self::Trine => 8.0,
            Self::Square => 8.0,
            Self::Sextile => 6.0,
        }
    }

    /// All major aspect kinds.
    pub const ALL: [AspectKind; 5] = [
        Self::Conjunction,
        Self::Sextile,
        Self::Square,
        Self::Trine,
        Self::Opposition,
    ];
}

impl fmt::Display for AspectKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Conjunction => write!(f, "Conjunction"),
            Self::Sextile => write!(f, "Sextile"),
            Self::Square => write!(f, "Square"),
            Self::Trine => write!(f, "Trine"),
            Self::Opposition => write!(f, "Opposition"),
        }
    }
}

// ---------------------------------------------------------------------------
// Aspect detection
// ---------------------------------------------------------------------------

/// A detected aspect between two bodies.
#[derive(Debug, Clone, Copy)]
pub struct Aspect {
    /// The type of aspect.
    pub kind: AspectKind,
    /// The actual angular separation in degrees.
    pub separation: f64,
    /// The orb (deviation from exact aspect) in degrees.
    pub orb: f64,
    /// Aspect strength from 0.0 (at orb limit) to 1.0 (exact).
    pub strength: f64,
}

/// Compute the shortest angular separation between two ecliptic longitudes.
///
/// Returns a value in \[0, 180\].
///
/// # Examples
///
/// ```
/// # use jyotish::aspect::angular_separation;
/// assert!((angular_separation(10.0, 350.0) - 20.0).abs() < 1e-10);
/// assert!((angular_separation(0.0, 180.0) - 180.0).abs() < 1e-10);
/// ```
pub fn angular_separation(lon1: f64, lon2: f64) -> f64 {
    let diff = normalize_degrees(lon1 - lon2);
    if diff > 180.0 { 360.0 - diff } else { diff }
}

/// Find an aspect between two ecliptic longitudes, if any, using default orbs.
///
/// Returns the strongest (closest to exact) aspect found, or `None`.
///
/// # Examples
///
/// ```
/// # use jyotish::aspect::{find_aspect, AspectKind};
/// let asp = find_aspect(0.0, 120.5).unwrap();
/// assert_eq!(asp.kind, AspectKind::Trine);
/// assert!(asp.orb < 1.0);
/// ```
pub fn find_aspect(lon1: f64, lon2: f64) -> Option<Aspect> {
    find_aspect_with_orbs(lon1, lon2, &AspectKind::ALL.map(|k| (k, k.default_orb())))
}

/// Find an aspect between two ecliptic longitudes with custom orbs.
///
/// `orbs` is a slice of `(AspectKind, max_orb_degrees)` pairs to check.
/// Returns the strongest aspect found, or `None`.
pub fn find_aspect_with_orbs(lon1: f64, lon2: f64, orbs: &[(AspectKind, f64)]) -> Option<Aspect> {
    let sep = angular_separation(lon1, lon2);
    let mut best: Option<Aspect> = None;

    for &(kind, max_orb) in orbs {
        let orb = (sep - kind.angle()).abs();
        if orb <= max_orb {
            let strength = 1.0 - orb / max_orb;
            let is_better = best.as_ref().is_none_or(|b| strength > b.strength);
            if is_better {
                best = Some(Aspect {
                    kind,
                    separation: sep,
                    orb,
                    strength,
                });
            }
        }
    }

    best
}

/// Find all aspects between two ecliptic longitudes using default orbs.
///
/// Returns all matching aspects, sorted by strength (strongest first).
pub fn find_all_aspects(lon1: f64, lon2: f64) -> Vec<Aspect> {
    find_all_aspects_with_orbs(lon1, lon2, &AspectKind::ALL.map(|k| (k, k.default_orb())))
}

/// Find all aspects between two ecliptic longitudes with custom orbs.
pub fn find_all_aspects_with_orbs(lon1: f64, lon2: f64, orbs: &[(AspectKind, f64)]) -> Vec<Aspect> {
    let sep = angular_separation(lon1, lon2);
    let mut aspects: Vec<Aspect> = orbs
        .iter()
        .filter_map(|&(kind, max_orb)| {
            let orb = (sep - kind.angle()).abs();
            if orb <= max_orb {
                Some(Aspect {
                    kind,
                    separation: sep,
                    orb,
                    strength: 1.0 - orb / max_orb,
                })
            } else {
                None
            }
        })
        .collect();

    aspects.sort_by(|a, b| {
        b.strength
            .partial_cmp(&a.strength)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    aspects
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn angular_separation_basic() {
        assert!((angular_separation(0.0, 0.0)).abs() < 1e-10);
        assert!((angular_separation(0.0, 180.0) - 180.0).abs() < 1e-10);
        assert!((angular_separation(10.0, 350.0) - 20.0).abs() < 1e-10);
        assert!((angular_separation(350.0, 10.0) - 20.0).abs() < 1e-10);
    }

    #[test]
    fn find_conjunction() {
        let asp = find_aspect(100.0, 103.0).unwrap();
        assert_eq!(asp.kind, AspectKind::Conjunction);
        assert!((asp.orb - 3.0).abs() < 1e-10);
    }

    #[test]
    fn find_trine() {
        let asp = find_aspect(0.0, 120.5).unwrap();
        assert_eq!(asp.kind, AspectKind::Trine);
        assert!((asp.orb - 0.5).abs() < 1e-10);
    }

    #[test]
    fn find_opposition() {
        let asp = find_aspect(10.0, 191.0).unwrap();
        assert_eq!(asp.kind, AspectKind::Opposition);
        assert!((asp.orb - 1.0).abs() < 1e-10);
    }

    #[test]
    fn find_square() {
        let asp = find_aspect(0.0, 88.0).unwrap();
        assert_eq!(asp.kind, AspectKind::Square);
        assert!((asp.orb - 2.0).abs() < 1e-10);
    }

    #[test]
    fn find_sextile() {
        let asp = find_aspect(0.0, 62.0).unwrap();
        assert_eq!(asp.kind, AspectKind::Sextile);
        assert!((asp.orb - 2.0).abs() < 1e-10);
    }

    #[test]
    fn no_aspect_found() {
        // 45° is not a major aspect
        let asp = find_aspect(0.0, 45.0);
        assert!(asp.is_none());
    }

    #[test]
    fn exact_aspect_strength_is_one() {
        let asp = find_aspect(0.0, 120.0).unwrap();
        assert_eq!(asp.kind, AspectKind::Trine);
        assert!((asp.strength - 1.0).abs() < 1e-10);
    }

    #[test]
    fn aspect_strength_at_orb_limit() {
        // Default trine orb is 8°
        let asp = find_aspect(0.0, 128.0).unwrap();
        assert_eq!(asp.kind, AspectKind::Trine);
        assert!(asp.strength.abs() < 1e-10); // strength → 0 at orb limit
    }

    #[test]
    fn find_all_aspects_multiple() {
        // 0° and 0° — conjunction
        let aspects = find_all_aspects(0.0, 0.5);
        assert!(!aspects.is_empty());
        assert_eq!(aspects[0].kind, AspectKind::Conjunction);
    }

    #[test]
    fn aspect_kind_display() {
        assert_eq!(AspectKind::Conjunction.to_string(), "Conjunction");
        assert_eq!(AspectKind::Opposition.to_string(), "Opposition");
    }

    #[test]
    fn aspect_kind_serde() {
        let kind = AspectKind::Trine;
        let json = serde_json::to_string(&kind).unwrap();
        let restored: AspectKind = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, kind);
    }

    #[test]
    fn custom_orbs() {
        let tight_orbs = [(AspectKind::Conjunction, 1.0)];
        assert!(find_aspect_with_orbs(0.0, 5.0, &tight_orbs).is_none());
        assert!(find_aspect_with_orbs(0.0, 0.5, &tight_orbs).is_some());
    }
}
