//! # Jyotish
//!
//! **Jyotish** (ज्योतिष — Sanskrit for "science of light", the traditional term
//! for Vedic astrology and astronomical computation) — astronomical computation
//! engine for the AGNOS ecosystem.
//!
//! Provides planetary position computation, calendar system conversions, celestial
//! event prediction, zodiac/constellation mapping, house systems, aspect
//! computation, and transit tracking.
//!
//! ## Optional features
//!
//! - **`orbital`** — high-fidelity orbital mechanics integration via
//!   [falak](https://crates.io/crates/falak).
//! - **`logging`** — tracing-subscriber initialization via `JYOTISH_LOG` env var.

#![warn(missing_docs)]
#![forbid(unsafe_code)]

/// Error types for jyotish computations.
pub mod error;

/// Planetary bodies and position data.
pub mod planet;

/// Calendar systems — Julian, Gregorian, sidereal time, Julian Day Number conversions.
pub mod calendar;

/// Celestial events — eclipses, conjunctions, oppositions, transits, equinoxes, solstices.
pub mod event;

/// Zodiac signs and constellations — tropical, sidereal, bounds, cusps.
pub mod zodiac;

/// House systems — Placidus, Koch, Equal, Whole Sign, Porphyry.
pub mod house;

/// Planetary aspects — conjunction, opposition, trine, square, sextile, orbs.
pub mod aspect;

/// Planetary transits — ingress, retrograde, station, direct motion.
pub mod transit;

#[cfg(feature = "logging")]
/// Logging initialization for the jyotish crate.
pub mod logging;

// --- Core re-exports ---
pub use error::{JyotishError, Result};
pub use planet::{Planet, PlanetaryPosition};
