//! Physical ephemerides — apparent diameter, phase angle, illumination, elongation.
//!
//! Computes observable physical properties of solar system bodies: angular size
//! as seen from Earth, the Sun-body-Earth phase angle, the fraction of the disk
//! that is illuminated, and the angular separation from the Sun.
//!
//! All functions accept a Julian Date in TT/TDB and return geometric
//! (uncorrected) values suitable for planning observations.

use crate::coords::deg_to_rad;
use crate::error::{JyotishError, Result};
use crate::moon::{lunar_distance_km, lunar_longitude};
use crate::planet::Planet;
use crate::planetary::compute_position;
use crate::sun::{solar_distance_au, solar_longitude};

// ---------------------------------------------------------------------------
// Equatorial diameters (km)
// ---------------------------------------------------------------------------

/// Equatorial diameter in km for each body.
fn equatorial_diameter_km(planet: Planet) -> f64 {
    match planet {
        Planet::Sun => 1_392_700.0,
        Planet::Moon => 3_474.8,
        Planet::Mercury => 4_879.4,
        Planet::Venus => 12_104.0,
        Planet::Mars => 6_779.0,
        Planet::Jupiter => 139_822.0,
        Planet::Saturn => 116_464.0,
        Planet::Uranus => 50_724.0,
        Planet::Neptune => 49_244.0,
        Planet::Pluto => 2_376.6,
    }
}

/// One astronomical unit in kilometres.
const AU_KM: f64 = 149_597_870.7;

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Apparent angular diameter of a body as seen from Earth, in arcseconds.
///
/// Uses the small-angle formula: `diameter_km / distance_km * 206_265`.
///
/// For the Sun the distance comes from [`solar_distance_au`], for the Moon
/// from [`lunar_distance_km`], and for planets from [`compute_position`].
///
/// # Errors
///
/// Returns [`JyotishError::MathError`] if a computed distance is zero or
/// negative (should not occur for valid dates).
///
/// # Examples
///
/// ```
/// # use jyotish::physical::apparent_diameter;
/// # use jyotish::planet::Planet;
/// // Sun at J2000.0 — about 32.5 arcminutes (≈ 1950")
/// let d = apparent_diameter(Planet::Sun, 2_451_545.0).unwrap();
/// assert!(d > 1890.0 && d < 1970.0, "got {d}");
///
/// // Jupiter — typically 30–50"
/// let d = apparent_diameter(Planet::Jupiter, 2_451_545.0).unwrap();
/// assert!(d > 25.0 && d < 55.0, "got {d}");
/// ```
pub fn apparent_diameter(planet: Planet, jd: f64) -> Result<f64> {
    let diam_km = equatorial_diameter_km(planet);

    match planet {
        Planet::Moon => {
            let dist_km = lunar_distance_km(jd);
            if dist_km <= 0.0 {
                return Err(JyotishError::MathError(
                    "lunar distance is non-positive".into(),
                ));
            }
            Ok(diam_km / dist_km * 206_265.0)
        }
        Planet::Sun => {
            let dist_au = solar_distance_au(jd);
            if dist_au <= 0.0 {
                return Err(JyotishError::MathError(
                    "solar distance is non-positive".into(),
                ));
            }
            Ok(diam_km / (dist_au * AU_KM) * 206_265.0)
        }
        _ => {
            let pos = compute_position(planet, jd)?;
            if pos.distance_au <= 0.0 {
                return Err(JyotishError::MathError(format!(
                    "{planet} distance is non-positive"
                )));
            }
            Ok(diam_km / (pos.distance_au * AU_KM) * 206_265.0)
        }
    }
}

/// Phase angle (Sun-body-Earth angle) in degrees.
///
/// The phase angle *i* is the angle at the body between the Sun and the
/// Earth. It determines how much of the body's disk is illuminated.
///
/// - **Sun**: always 0.0 (by definition — we *are* the observer).
/// - **Moon**: approximated as `180° − elongation` (at full Moon the
///   phase angle is near 0°, at new Moon near 180°).
/// - **Planets**: computed from the triangle Sun-planet-Earth using the
///   cosine rule: `cos(i) = (r² + d² − R²) / (2·r·d)` where
///   *r* = heliocentric distance, *d* = geocentric distance,
///   *R* = Sun-Earth distance.
///
/// # Errors
///
/// Propagates errors from [`compute_position`] for planets.
///
/// # Examples
///
/// ```
/// # use jyotish::physical::phase_angle;
/// # use jyotish::planet::Planet;
/// // Sun phase angle is always zero
/// let i = phase_angle(Planet::Sun, 2_451_545.0).unwrap();
/// assert!(i.abs() < 1e-10);
/// ```
pub fn phase_angle(planet: Planet, jd: f64) -> Result<f64> {
    match planet {
        Planet::Sun => Ok(0.0),
        Planet::Moon => {
            // Phase angle = 180° - elongation.
            // At full Moon (elongation ≈ 180°) the phase angle ≈ 0° (fully lit).
            // At new Moon (elongation ≈ 0°) the phase angle ≈ 180° (dark side).
            let elong = elongation(planet, jd)?;
            Ok(180.0 - elong)
        }
        _ => {
            let big_r = solar_distance_au(jd); // Sun–Earth
            let pos = compute_position(planet, jd)?; // geocentric
            let d = pos.distance_au; // planet–Earth

            // Heliocentric distance of the planet
            let (_, _, r) = crate::vsop87::planet_heliocentric(planet, jd)?;

            let cos_i = (r * r + d * d - big_r * big_r) / (2.0 * r * d);
            // Clamp to [-1, 1] for numerical safety
            let cos_i = cos_i.clamp(-1.0, 1.0);
            Ok(cos_i.acos().to_degrees())
        }
    }
}

/// Fraction of the body's disk that is illuminated, in \[0.0, 1.0\].
///
/// Uses the standard formula `k = (1 + cos(i)) / 2` where *i* is the
/// phase angle.
///
/// # Errors
///
/// Propagates errors from [`phase_angle`].
///
/// # Examples
///
/// ```
/// # use jyotish::physical::illuminated_fraction;
/// # use jyotish::planet::Planet;
/// // The Sun is always fully illuminated (phase angle = 0)
/// let k = illuminated_fraction(Planet::Sun, 2_451_545.0).unwrap();
/// assert!((k - 1.0).abs() < 1e-10);
/// ```
pub fn illuminated_fraction(planet: Planet, jd: f64) -> Result<f64> {
    let i_deg = phase_angle(planet, jd)?;
    let i_rad = deg_to_rad(i_deg);
    Ok((1.0 + i_rad.cos()) / 2.0)
}

/// Angular elongation of a body from the Sun, in degrees \[0, 180\].
///
/// Elongation is the geocentric angular separation between the body and the
/// Sun measured along the ecliptic. For the Sun itself, the result is 0.
///
/// # Errors
///
/// Propagates errors from [`compute_position`] for planets.
///
/// # Examples
///
/// ```
/// # use jyotish::physical::elongation;
/// # use jyotish::planet::Planet;
/// // Sun elongation from itself is zero
/// let e = elongation(Planet::Sun, 2_451_545.0).unwrap();
/// assert!(e.abs() < 1e-10);
///
/// // Venus is always within ~47° of the Sun
/// let e = elongation(Planet::Venus, 2_451_545.0).unwrap();
/// assert!(e < 48.0, "got {e}");
/// ```
pub fn elongation(planet: Planet, jd: f64) -> Result<f64> {
    if planet == Planet::Sun {
        return Ok(0.0);
    }

    let sun_lon = solar_longitude(jd);

    let body_lon = match planet {
        Planet::Moon => lunar_longitude(jd),
        _ => compute_position(planet, jd)?.longitude_deg,
    };

    let mut diff = (body_lon - sun_lon).abs();
    if diff > 180.0 {
        diff = 360.0 - diff;
    }
    Ok(diff)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const JD_J2000: f64 = 2_451_545.0;

    // --- apparent_diameter ---

    #[test]
    fn sun_apparent_diameter_j2000() {
        // Sun ≈ 32 arcminutes ≈ 1920–1960" (varies with Earth-Sun distance)
        let d = apparent_diameter(Planet::Sun, JD_J2000).unwrap();
        assert!(
            d > 1890.0 && d < 1970.0,
            "Sun apparent diameter at J2000 = {d}\", expected 1890–1970\""
        );
    }

    #[test]
    fn moon_apparent_diameter_j2000() {
        // Moon ≈ ~1900" (varies 1760–2000" with distance)
        let d = apparent_diameter(Planet::Moon, JD_J2000).unwrap();
        assert!(
            d > 1760.0 && d < 2010.0,
            "Moon apparent diameter at J2000 = {d}\""
        );
    }

    #[test]
    fn jupiter_apparent_diameter() {
        // Jupiter ≈ 30–50"
        let d = apparent_diameter(Planet::Jupiter, JD_J2000).unwrap();
        assert!(d > 25.0 && d < 55.0, "Jupiter apparent diameter = {d}\"");
    }

    #[test]
    fn mercury_apparent_diameter() {
        // Mercury ≈ 5–13"
        let d = apparent_diameter(Planet::Mercury, JD_J2000).unwrap();
        assert!(d > 3.0 && d < 15.0, "Mercury apparent diameter = {d}\"");
    }

    #[test]
    fn pluto_apparent_diameter() {
        // Pluto — very small, < 0.2"
        let d = apparent_diameter(Planet::Pluto, JD_J2000).unwrap();
        assert!(d > 0.0 && d < 0.2, "Pluto apparent diameter = {d}\"");
    }

    #[test]
    fn apparent_diameter_all_positive() {
        for planet in [
            Planet::Sun,
            Planet::Moon,
            Planet::Mercury,
            Planet::Venus,
            Planet::Mars,
            Planet::Jupiter,
            Planet::Saturn,
            Planet::Uranus,
            Planet::Neptune,
            Planet::Pluto,
        ] {
            let d = apparent_diameter(planet, JD_J2000).unwrap();
            assert!(
                d > 0.0,
                "{planet} apparent diameter should be positive, got {d}"
            );
        }
    }

    // --- phase_angle ---

    #[test]
    fn sun_phase_angle_always_zero() {
        for day in 0..30 {
            let jd = JD_J2000 + day as f64 * 10.0;
            let i = phase_angle(Planet::Sun, jd).unwrap();
            assert!(i.abs() < 1e-10, "Sun phase angle = {i} at JD {jd}");
        }
    }

    #[test]
    fn planet_phase_angle_range() {
        // Phase angle should be in [0, 180] for all planets
        for planet in [
            Planet::Mercury,
            Planet::Venus,
            Planet::Mars,
            Planet::Jupiter,
            Planet::Saturn,
        ] {
            let i = phase_angle(planet, JD_J2000).unwrap();
            assert!(
                (0.0..=180.0).contains(&i),
                "{planet} phase angle = {i} out of range"
            );
        }
    }

    #[test]
    fn outer_planet_phase_angle_small() {
        // Outer planets (Jupiter, Saturn) have small phase angles (< ~12°)
        for planet in [Planet::Jupiter, Planet::Saturn] {
            let i = phase_angle(planet, JD_J2000).unwrap();
            assert!(i < 15.0, "{planet} phase angle = {i}, expected < 15");
        }
    }

    // --- illuminated_fraction ---

    #[test]
    fn sun_illuminated_fraction_one() {
        let k = illuminated_fraction(Planet::Sun, JD_J2000).unwrap();
        assert!((k - 1.0).abs() < 1e-10, "Sun illuminated fraction = {k}");
    }

    #[test]
    fn illuminated_fraction_range() {
        for planet in [
            Planet::Sun,
            Planet::Moon,
            Planet::Mercury,
            Planet::Venus,
            Planet::Mars,
            Planet::Jupiter,
            Planet::Saturn,
        ] {
            let k = illuminated_fraction(planet, JD_J2000).unwrap();
            assert!(
                (0.0..=1.0).contains(&k),
                "{planet} illuminated fraction = {k} out of [0, 1]"
            );
        }
    }

    #[test]
    fn outer_planet_nearly_full() {
        // Jupiter and Saturn are nearly fully illuminated (> 0.99)
        for planet in [Planet::Jupiter, Planet::Saturn] {
            let k = illuminated_fraction(planet, JD_J2000).unwrap();
            assert!(
                k > 0.98,
                "{planet} illuminated fraction = {k}, expected > 0.98"
            );
        }
    }

    #[test]
    fn full_moon_illuminated_fraction() {
        // Search for the date with maximum elongation over a lunar month
        // to find a full Moon near J2000.
        let mut best_jd = JD_J2000;
        let mut best_elong = 0.0_f64;
        for hour in 0..(30 * 24) {
            let jd = JD_J2000 + hour as f64 / 24.0;
            let e = elongation(Planet::Moon, jd).unwrap();
            if e > best_elong {
                best_elong = e;
                best_jd = jd;
            }
        }
        // Near full Moon, elongation should be close to 180°
        assert!(
            best_elong > 170.0,
            "max elongation = {best_elong}, expected ~180"
        );
        let k = illuminated_fraction(Planet::Moon, best_jd).unwrap();
        assert!(
            k > 0.95,
            "illuminated fraction at full Moon = {k}, expected ~1.0"
        );
    }

    // --- elongation ---

    #[test]
    fn sun_elongation_zero() {
        let e = elongation(Planet::Sun, JD_J2000).unwrap();
        assert!(e.abs() < 1e-10, "Sun elongation = {e}");
    }

    #[test]
    fn venus_elongation_bounded() {
        // Venus elongation should never exceed ~47°
        // Check over a full synodic period (~584 days)
        let mut max_elong = 0.0_f64;
        for day in 0..600 {
            let jd = JD_J2000 + day as f64;
            let e = elongation(Planet::Venus, jd).unwrap();
            max_elong = max_elong.max(e);
        }
        assert!(
            max_elong < 48.0,
            "Venus max elongation = {max_elong}, expected < 48"
        );
        // Should also get reasonably large at greatest elongation
        assert!(
            max_elong > 40.0,
            "Venus max elongation = {max_elong}, expected > 40"
        );
    }

    #[test]
    fn mars_elongation_can_reach_opposition() {
        // Mars elongation ranges 0–180°. Check over ~2 years.
        let mut max_elong = 0.0_f64;
        for day in 0..800 {
            let jd = JD_J2000 + day as f64;
            let e = elongation(Planet::Mars, jd).unwrap();
            max_elong = max_elong.max(e);
        }
        assert!(
            max_elong > 150.0,
            "Mars max elongation = {max_elong}, expected > 150 (near opposition)"
        );
    }

    #[test]
    fn elongation_range_valid() {
        // All elongations should be in [0, 180]
        for planet in [
            Planet::Moon,
            Planet::Mercury,
            Planet::Venus,
            Planet::Mars,
            Planet::Jupiter,
            Planet::Saturn,
        ] {
            for day in (0..365).step_by(30) {
                let jd = JD_J2000 + day as f64;
                let e = elongation(planet, jd).unwrap();
                assert!(
                    (0.0..=180.0).contains(&e),
                    "{planet} elongation = {e} out of [0, 180] at day {day}"
                );
            }
        }
    }

    #[test]
    fn moon_elongation_full_range() {
        // Moon elongation should span 0–180° over a month
        let mut min_elong = 180.0_f64;
        let mut max_elong = 0.0_f64;
        for day in 0..30 {
            let jd = JD_J2000 + day as f64;
            let e = elongation(Planet::Moon, jd).unwrap();
            min_elong = min_elong.min(e);
            max_elong = max_elong.max(e);
        }
        assert!(
            min_elong < 15.0,
            "Moon min elongation = {min_elong}, expected near 0"
        );
        assert!(
            max_elong > 165.0,
            "Moon max elongation = {max_elong}, expected near 180"
        );
    }
}
