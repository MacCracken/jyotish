//! Planetary bodies and position data.
//!
//! Defines the major solar system bodies used in astronomical computation and
//! provides the [`PlanetaryPosition`] struct for representing a body's
//! position at a given instant.

use serde::{Deserialize, Serialize};
use std::fmt;

/// A solar system body.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Planet {
    /// The Sun (Sol).
    Sun,
    /// The Moon (Luna).
    Moon,
    /// Mercury.
    Mercury,
    /// Venus.
    Venus,
    /// Mars.
    Mars,
    /// Jupiter.
    Jupiter,
    /// Saturn.
    Saturn,
    /// Uranus.
    Uranus,
    /// Neptune.
    Neptune,
    /// Pluto (dwarf planet).
    Pluto,
}

impl fmt::Display for Planet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sun => write!(f, "Sun"),
            Self::Moon => write!(f, "Moon"),
            Self::Mercury => write!(f, "Mercury"),
            Self::Venus => write!(f, "Venus"),
            Self::Mars => write!(f, "Mars"),
            Self::Jupiter => write!(f, "Jupiter"),
            Self::Saturn => write!(f, "Saturn"),
            Self::Uranus => write!(f, "Uranus"),
            Self::Neptune => write!(f, "Neptune"),
            Self::Pluto => write!(f, "Pluto"),
        }
    }
}

/// The position of a planetary body at a given instant.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct PlanetaryPosition {
    /// Which body this position refers to.
    pub planet: Planet,
    /// Ecliptic longitude in degrees (0..360).
    pub longitude_deg: f64,
    /// Ecliptic latitude in degrees (-90..90).
    pub latitude_deg: f64,
    /// Distance from the observer in astronomical units.
    pub distance_au: f64,
    /// Unix timestamp (seconds since epoch) of the observation.
    pub timestamp: i64,
}

impl PlanetaryPosition {
    /// Create a new planetary position.
    pub fn new(
        planet: Planet,
        longitude_deg: f64,
        latitude_deg: f64,
        distance_au: f64,
        timestamp: i64,
    ) -> Self {
        Self {
            planet,
            longitude_deg,
            latitude_deg,
            distance_au,
            timestamp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn planet_display() {
        assert_eq!(Planet::Sun.to_string(), "Sun");
        assert_eq!(Planet::Moon.to_string(), "Moon");
        assert_eq!(Planet::Mercury.to_string(), "Mercury");
        assert_eq!(Planet::Venus.to_string(), "Venus");
        assert_eq!(Planet::Mars.to_string(), "Mars");
        assert_eq!(Planet::Jupiter.to_string(), "Jupiter");
        assert_eq!(Planet::Saturn.to_string(), "Saturn");
        assert_eq!(Planet::Uranus.to_string(), "Uranus");
        assert_eq!(Planet::Neptune.to_string(), "Neptune");
        assert_eq!(Planet::Pluto.to_string(), "Pluto");
    }

    #[test]
    fn planet_serde_roundtrip() {
        let planet = Planet::Jupiter;
        let json = serde_json::to_string(&planet).unwrap();
        let restored: Planet = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, planet);
    }

    #[test]
    fn position_serde_roundtrip() {
        let pos = PlanetaryPosition::new(Planet::Mars, 145.3, 1.2, 1.524, 1711324800);
        let json = serde_json::to_string(&pos).unwrap();
        let restored: PlanetaryPosition = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.planet, Planet::Mars);
        assert!((restored.longitude_deg - 145.3).abs() < f64::EPSILON);
    }
}
