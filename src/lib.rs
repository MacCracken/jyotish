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

/// Numerical utilities (compensated summation, etc.).
pub mod num;

/// Delta T (TT − UT1) computation and time scale conversions.
pub mod delta_t;

/// Planetary bodies and position data.
pub mod planet;

/// Calendar systems — Julian, Gregorian, sidereal time, Julian Day Number conversions.
pub mod calendar;

/// Coordinate systems and transformations.
pub mod coords;

/// Solar position computation.
pub mod sun;

/// Lunar position computation.
pub mod moon;

/// Planetary position computation (Mercury through Pluto).
pub mod planetary;

/// Nutation and precession corrections.
pub mod nutation;

/// Annual aberration and light-time correction.
pub mod aberration;

/// Apparent position pipeline (geometric → aberration → nutation → apparent).
pub mod apparent;

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

/// Rise, set, and meridian transit times.
pub mod riseset;

/// Parallax correction for topocentric positions.
pub mod parallax;

#[cfg(feature = "orbital")]
/// High-fidelity orbital mechanics via falak.
pub mod orbital;

#[cfg(feature = "logging")]
/// Logging initialization for the jyotish crate.
pub mod logging;

// --- Core re-exports ---
pub use calendar::{
    DAYS_PER_JULIAN_CENTURY, J2000_0, JD_UNIX_EPOCH, day_of_week, gmst_degrees, gmst_degrees_tt,
    gmst_hours, gregorian_to_jd, gregorian_to_jdn, is_gregorian_leap_year, is_julian_leap_year,
    jd_to_gregorian, jd_to_unix, jdn_to_gregorian, jdn_to_julian, julian_centuries, julian_to_jdn,
    local_sidereal_time, local_sidereal_time_tt, unix_to_jd,
};
pub use error::{JyotishError, Result};
pub use planet::{Planet, PlanetaryPosition};
