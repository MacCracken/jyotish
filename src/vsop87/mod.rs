// Phase values like 3.142 are actual VSOP87 coefficients, not approximations of π.
#![allow(clippy::approx_constant)]

//! VSOP87D heliocentric ecliptic coordinates (J2000.0 ecliptic and equinox).
//!
//! Implements the VSOP87 planetary theory (Bretagnon & Francou 1988) in its
//! version D form, providing heliocentric ecliptic longitude (L), latitude (B),
//! and radius vector (R) referred to the J2000.0 ecliptic and equinox.
//!
//! Accuracy: <1" for the Sun and inner planets over ±4000 years from J2000.0.
//! This matches or exceeds Swiss Ephemeris Moshier mode accuracy.
//!
//! Each planet's coordinates are expressed as power series in τ (Julian millennia
//! from J2000.0), where each coefficient is itself a sum of periodic terms:
//!
//! ```text
//! L = L₀ + L₁τ + L₂τ² + L₃τ³ + L₄τ⁴ + L₅τ⁵
//! ```
//!
//! where `Lₙ = Σ Aᵢ cos(Bᵢ + Cᵢτ)`.

mod earth;
mod jupiter;
mod mars;
mod mercury;
mod neptune;
mod saturn;
mod uranus;
mod venus;

use crate::calendar::julian_centuries;
use crate::coords::{normalize_degrees, rad_to_deg};
use crate::error::{JyotishError, Result};
use crate::num::KahanSum;
use crate::planet::Planet;

/// A single VSOP87 term: `A * cos(B + C * τ)`.
#[derive(Clone, Copy)]
pub(crate) struct VsopTerm {
    /// Amplitude (dimensionless).
    pub a: f64,
    /// Phase at epoch (radians).
    pub b: f64,
    /// Frequency (radians per Julian millennium).
    pub c: f64,
}

/// Evaluate a VSOP87 series: sum over powers of τ.
///
/// `sub_series` is an array of term slices `[S₀, S₁, S₂, ...]` where each
/// `Sₙ = Σ Aᵢ cos(Bᵢ + Cᵢτ)`.
///
/// Returns `S₀ + S₁τ + S₂τ² + ...`
fn evaluate_series(sub_series: &[&[VsopTerm]], tau: f64) -> f64 {
    let mut result = KahanSum::new();
    let mut tau_power = 1.0;
    for &terms in sub_series {
        let mut sub_sum = KahanSum::new();
        for term in terms {
            sub_sum.add(term.a * (term.b + term.c * tau).cos());
        }
        result.add(sub_sum.sum() * tau_power);
        tau_power *= tau;
    }
    result.sum()
}

/// Heliocentric ecliptic coordinates for Earth (J2000.0 ecliptic).
///
/// Returns `(longitude_deg, latitude_deg, radius_au)`.
///
/// This is the fundamental function for computing the Sun's geocentric
/// position (Sun = Earth + 180°) and for converting other planets from
/// heliocentric to geocentric coordinates.
pub fn earth_heliocentric(jd: f64) -> (f64, f64, f64) {
    let t = julian_centuries(jd);
    let tau = t / 10.0; // Julian millennia

    let l = evaluate_series(&earth::L_SERIES, tau);
    let b = evaluate_series(&earth::B_SERIES, tau);
    let r = evaluate_series(&earth::R_SERIES, tau);

    (normalize_degrees(rad_to_deg(l)), rad_to_deg(b), r)
}

/// Heliocentric ecliptic coordinates for a planet (J2000.0 ecliptic).
///
/// Returns `(longitude_deg, latitude_deg, radius_au)`.
///
/// # Errors
///
/// Returns an error for Sun, Moon, or Pluto (not covered by VSOP87).
pub fn planet_heliocentric(planet: Planet, jd: f64) -> Result<(f64, f64, f64)> {
    let t = julian_centuries(jd);
    let tau = t / 10.0;

    let (l_series, b_series, r_series) = match planet {
        Planet::Mercury => (
            mercury::L_SERIES.as_slice(),
            mercury::B_SERIES.as_slice(),
            mercury::R_SERIES.as_slice(),
        ),
        Planet::Venus => (
            venus::L_SERIES.as_slice(),
            venus::B_SERIES.as_slice(),
            venus::R_SERIES.as_slice(),
        ),
        Planet::Mars => (
            mars::L_SERIES.as_slice(),
            mars::B_SERIES.as_slice(),
            mars::R_SERIES.as_slice(),
        ),
        Planet::Jupiter => (
            jupiter::L_SERIES.as_slice(),
            jupiter::B_SERIES.as_slice(),
            jupiter::R_SERIES.as_slice(),
        ),
        Planet::Saturn => (
            saturn::L_SERIES.as_slice(),
            saturn::B_SERIES.as_slice(),
            saturn::R_SERIES.as_slice(),
        ),
        Planet::Uranus => (
            uranus::L_SERIES.as_slice(),
            uranus::B_SERIES.as_slice(),
            uranus::R_SERIES.as_slice(),
        ),
        Planet::Neptune => (
            neptune::L_SERIES.as_slice(),
            neptune::B_SERIES.as_slice(),
            neptune::R_SERIES.as_slice(),
        ),
        Planet::Sun | Planet::Moon | Planet::Pluto => {
            return Err(JyotishError::InvalidParameter(format!(
                "{planet} is not supported by VSOP87"
            )));
        }
    };

    let l = evaluate_series(l_series, tau);
    let b = evaluate_series(b_series, tau);
    let r = evaluate_series(r_series, tau);

    Ok((normalize_degrees(rad_to_deg(l)), rad_to_deg(b), r))
}

#[cfg(test)]
mod tests {
    use super::*;

    const JD_J2000: f64 = 2_451_545.0;

    #[test]
    fn earth_heliocentric_j2000() {
        let (lon, lat, r) = earth_heliocentric(JD_J2000);
        // Earth's heliocentric longitude at J2000.0 ≈ 100.47°
        assert!(
            (lon - 100.47).abs() < 0.5,
            "Earth lon = {lon}, expected ~100.47"
        );
        // Earth's latitude should be near 0
        assert!(lat.abs() < 0.01, "Earth lat = {lat}");
        // Earth's radius ≈ 0.9833 AU at J2000.0 (early January)
        assert!(
            (r - 0.983).abs() < 0.02,
            "Earth r = {r}, expected ~0.983"
        );
    }

    #[test]
    fn sun_geocentric_from_earth() {
        let (e_lon, e_lat, e_r) = earth_heliocentric(JD_J2000);
        let sun_lon = normalize_degrees(e_lon + 180.0);
        // Sun geocentric longitude at J2000.0 ≈ 280.47°
        assert!(
            (sun_lon - 280.47).abs() < 0.5,
            "Sun lon = {sun_lon}, expected ~280.47"
        );
        // Sun geocentric latitude = -Earth heliocentric latitude
        let sun_lat = -e_lat;
        assert!(sun_lat.abs() < 0.01, "Sun lat = {sun_lat}");
        assert!(e_r > 0.98 && e_r < 1.02);
    }

    #[test]
    fn mars_heliocentric_j2000() {
        let (lon, lat, r) = planet_heliocentric(Planet::Mars, JD_J2000).unwrap();
        assert!((0.0..360.0).contains(&lon), "Mars lon = {lon}");
        assert!(lat.abs() < 10.0, "Mars lat = {lat}");
        assert!(r > 1.3 && r < 1.7, "Mars r = {r}");
    }

    #[test]
    fn all_vsop87_planets() {
        for planet in [
            Planet::Mercury,
            Planet::Venus,
            Planet::Mars,
            Planet::Jupiter,
            Planet::Saturn,
            Planet::Uranus,
            Planet::Neptune,
        ] {
            let (lon, lat, r) = planet_heliocentric(planet, JD_J2000).unwrap();
            assert!(
                (0.0..360.0).contains(&lon),
                "{planet} lon = {lon}"
            );
            assert!(lat.abs() < 15.0, "{planet} lat = {lat}");
            assert!(r > 0.3 && r < 35.0, "{planet} r = {r}");
        }
    }

    #[test]
    fn pluto_rejected() {
        assert!(planet_heliocentric(Planet::Pluto, JD_J2000).is_err());
    }

    #[test]
    fn sun_moon_rejected() {
        assert!(planet_heliocentric(Planet::Sun, JD_J2000).is_err());
        assert!(planet_heliocentric(Planet::Moon, JD_J2000).is_err());
    }
}
