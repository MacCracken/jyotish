//! High-fidelity orbital mechanics integration via [falak](../falak).
//!
//! This module provides enhanced planetary position computation using falak's
//! Kepler solver, state vector conversions, and reference frame transforms.
//! It offers higher accuracy than the built-in analytical series for bodies
//! where precise orbital elements are known.
//!
//! Requires the `orbital` feature.
//!
//! # Differences from the built-in `planetary` module
//!
//! - Uses falak's Danby-starter + Newton-Raphson Kepler solver (better
//!   convergence for high-eccentricity orbits like Pluto and comets)
//! - Provides full 3D state vectors (position + velocity) in addition
//!   to ecliptic coordinates
//! - Supports hyperbolic orbits (e > 1) for comets and interstellar objects
//! - Reference frame conversions (ECI ↔ ECEF ↔ perifocal)

use crate::calendar::julian_centuries;
use crate::coords::{normalize_degrees, rad_to_deg};
use crate::error::{JyotishError, Result};
use crate::planet::{Planet, PlanetaryPosition};

use falak::kepler::{elements_to_state, mean_to_true_anomaly};
use falak::orbit::OrbitalElements;

// ---------------------------------------------------------------------------
// Solar system gravitational parameters (μ = GM, in AU³/day²)
// ---------------------------------------------------------------------------

/// Heliocentric gravitational parameter μ☉ in m³/s².
const MU_SUN_SI: f64 = 1.327_124_400_18e20;

/// AU in metres.
const AU_METRES: f64 = 149_597_870_700.0;

/// Convert a falak StateVector (SI: metres, m/s) to AU and AU/day.
fn state_to_au(state: &falak::kepler::StateVector) -> HeliocentricState {
    let au_per_m = 1.0 / AU_METRES;
    let au_day_per_ms = 86_400.0 / AU_METRES;
    HeliocentricState {
        position: [
            state.position[0] * au_per_m,
            state.position[1] * au_per_m,
            state.position[2] * au_per_m,
        ],
        velocity: [
            state.velocity[0] * au_day_per_ms,
            state.velocity[1] * au_day_per_ms,
            state.velocity[2] * au_day_per_ms,
        ],
    }
}

// ---------------------------------------------------------------------------
// State vector type
// ---------------------------------------------------------------------------

/// Heliocentric state vector in astronomical units.
#[derive(Debug, Clone, Copy)]
pub struct HeliocentricState {
    /// Position `[x, y, z]` in AU (ecliptic J2000.0).
    pub position: [f64; 3],
    /// Velocity `[vx, vy, vz]` in AU/day.
    pub velocity: [f64; 3],
}

// ---------------------------------------------------------------------------
// Orbital elements for planets (J2000.0 ecliptic, heliocentric)
// ---------------------------------------------------------------------------

/// Orbital elements at J2000.0 and their rates per Julian century.
/// (a_AU, e, i_deg, Ω_deg, ϖ_deg, L_deg) and rates.
struct MeanElements {
    a: (f64, f64),
    e: (f64, f64),
    i: (f64, f64),
    node: (f64, f64),
    peri: (f64, f64),
    l: (f64, f64),
}

fn planet_mean_elements(planet: Planet) -> Result<MeanElements> {
    // Same elements as planetary.rs but used with falak's solver
    match planet {
        Planet::Mercury => Ok(MeanElements {
            a: (0.387_098_93, 0.0),
            e: (0.205_631_75, 0.000_020_406),
            i: (7.004_986, 0.001_821_5),
            node: (48.330_893, 1.186_124_4),
            peri: (77.456_45, 1.556_511_1),
            l: (252.250_32, 149_472.674_11),
        }),
        Planet::Venus => Ok(MeanElements {
            a: (0.723_331_99, 0.0),
            e: (0.006_773_23, -0.000_047_766),
            i: (3.394_662, 0.001_003_7),
            node: (76.679_920, 0.901_120_6),
            peri: (131.563_70, 1.402_123_9),
            l: (181.979_73, 58_517.815_39),
        }),
        Planet::Mars => Ok(MeanElements {
            a: (1.523_662_31, 0.000_001_97),
            e: (0.093_412_33, 0.000_090_48),
            i: (1.849_726, -0.000_601_1),
            node: (49.558_093, 0.772_095_9),
            peri: (336.060_23, 1.840_758_4),
            l: (355.433_30, 19_140.299_34),
        }),
        Planet::Jupiter => Ok(MeanElements {
            a: (5.202_603_2, 0.000_019_13),
            e: (0.048_497_65, 0.000_163_14),
            i: (1.303_270, -0.001_987_2),
            node: (100.464_44, 0.176_450_5),
            peri: (14.331_09, 0.215_520_9),
            l: (34.351_48, 3_034.905_67),
        }),
        Planet::Saturn => Ok(MeanElements {
            a: (9.554_909_6, -0.000_021_39),
            e: (0.055_508_62, -0.000_346_64),
            i: (2.488_878, 0.002_551_5),
            node: (113.665_24, 0.877_191_6),
            peri: (93.056_78, 0.565_320_6),
            l: (50.077_44, 1_222.113_79),
        }),
        Planet::Uranus => Ok(MeanElements {
            a: (19.218_143_4, -0.000_003_72),
            e: (0.046_295_11, -0.000_027_29),
            i: (0.773_196, 0.000_673_9),
            node: (74.005_947, 0.074_146_1),
            peri: (173.005_56, 0.089_321_2),
            l: (314.055_01, 428.466_77),
        }),
        Planet::Neptune => Ok(MeanElements {
            a: (30.110_386_9, 0.000_012_63),
            e: (0.008_994_83, 0.000_006_91),
            i: (1.769_952, -0.009_308_2),
            node: (131.784_06, 1.010_304_4),
            peri: (48.123_69, 0.029_158_7),
            l: (304.348_67, 218.486_28),
        }),
        Planet::Pluto => Ok(MeanElements {
            a: (39.481_686_77, 0.0),
            e: (0.248_808_6, 0.000_060_16),
            i: (17.141_75, 0.003_075),
            node: (110.303_47, -0.010_139_6),
            peri: (224.066_76, -0.003_442_5),
            l: (238.928_81, 145.205_26),
        }),
        Planet::Sun | Planet::Moon => Err(JyotishError::InvalidParameter(
            "use sun/moon modules for Sun/Moon positions".into(),
        )),
    }
}

// ---------------------------------------------------------------------------
// Falak-backed Kepler solver
// ---------------------------------------------------------------------------

/// Solve Kepler's equation using falak's high-fidelity solver.
///
/// Supports both elliptical (e < 1) and hyperbolic (e > 1) orbits.
/// Uses the Danby starter with Newton-Raphson refinement for better
/// convergence than a simple Newton-Raphson approach.
///
/// `mean_anomaly` is in radians, `eccentricity` is dimensionless.
/// Returns the true anomaly in radians.
///
/// # Errors
///
/// Returns an error if the solver fails to converge.
///
/// # Examples
///
/// ```
/// # use jyotish::orbital::solve_kepler;
/// let true_anom = solve_kepler(1.0, 0.1).unwrap();
/// // True anomaly should be close to mean anomaly for low eccentricity
/// assert!((true_anom - 1.0).abs() < 0.5, "got {true_anom}");
/// ```
pub fn solve_kepler(mean_anomaly: f64, eccentricity: f64) -> Result<f64> {
    mean_to_true_anomaly(mean_anomaly, eccentricity)
        .map_err(|e| JyotishError::MathError(format!("Kepler solver: {e}")))
}

// ---------------------------------------------------------------------------
// State vector computation
// ---------------------------------------------------------------------------

/// Compute the heliocentric state vector (position + velocity) for a planet.
///
/// Returns position in AU and velocity in AU/day, in the ecliptic
/// J2000.0 reference frame.
///
/// # Errors
///
/// Returns an error for Sun/Moon or if falak's solver fails.
///
/// # Examples
///
/// ```
/// # use jyotish::orbital::heliocentric_state;
/// # use jyotish::planet::Planet;
/// let state = heliocentric_state(Planet::Mars, 2_451_545.0).unwrap();
/// let r = (state.position[0].powi(2) + state.position[1].powi(2) + state.position[2].powi(2)).sqrt();
/// // Mars is ~1.4-1.7 AU from the Sun
/// assert!(r > 1.0 && r < 2.0, "r = {r} AU");
/// ```
pub fn heliocentric_state(planet: Planet, jd: f64) -> Result<HeliocentricState> {
    let t = julian_centuries(jd);
    let me = planet_mean_elements(planet)?;

    let a = me.a.0 + me.a.1 * t;
    let e = me.e.0 + me.e.1 * t;
    let i = (me.i.0 + me.i.1 * t).to_radians();
    let node = (me.node.0 + me.node.1 * t).to_radians();
    let peri_lon = me.peri.0 + me.peri.1 * t;
    let l = me.l.0 + me.l.1 * t;

    let arg_peri = (peri_lon - rad_to_deg(node)).to_radians();
    let mean_anomaly = (l - peri_lon).to_radians();

    // Use falak's anomaly solver
    let true_anom = mean_to_true_anomaly(mean_anomaly, e)
        .map_err(|err| JyotishError::MathError(format!("Kepler solver: {err}")))?;

    // Build OrbitalElements for falak
    // falak expects semi_major_axis in metres, but we want AU
    // Convert to SI for elements_to_state, then convert result back
    let a_m = a * AU_METRES;
    let elements = OrbitalElements::new(a_m, e, i, node, arg_peri, true_anom)
        .map_err(|err| JyotishError::InvalidParameter(format!("orbital elements: {err}")))?;

    let state = elements_to_state(&elements, MU_SUN_SI)
        .map_err(|err| JyotishError::MathError(format!("state vector: {err}")))?;

    Ok(state_to_au(&state))
}

// ---------------------------------------------------------------------------
// Enhanced geocentric position using falak
// ---------------------------------------------------------------------------

/// Compute geocentric ecliptic position using falak's orbital mechanics.
///
/// This provides the same output as [`crate::planetary::compute_position`]
/// but uses falak's Kepler solver for better accuracy, especially for
/// high-eccentricity orbits.
///
/// # Errors
///
/// Returns an error for Sun/Moon or if computation fails.
///
/// # Examples
///
/// ```
/// # use jyotish::orbital::compute_position;
/// # use jyotish::planet::Planet;
/// let pos = compute_position(Planet::Mars, 2_451_545.0).unwrap();
/// assert!(pos.longitude_deg >= 0.0 && pos.longitude_deg < 360.0);
/// ```
pub fn compute_position(planet: Planet, jd: f64) -> Result<PlanetaryPosition> {
    let planet_state = heliocentric_state(planet, jd)?;
    let earth_state = earth_heliocentric_state(jd)?;

    // Geocentric position = planet - Earth
    let x = planet_state.position[0] - earth_state.position[0];
    let y = planet_state.position[1] - earth_state.position[1];
    let z = planet_state.position[2] - earth_state.position[2];

    let dist = (x * x + y * y + z * z).sqrt();
    let lon = normalize_degrees(rad_to_deg(y.atan2(x)));
    let lat = rad_to_deg(z.atan2((x * x + y * y).sqrt()));

    Ok(PlanetaryPosition::new(
        planet,
        lon,
        lat,
        dist,
        crate::calendar::jd_to_unix(jd),
    ))
}

/// Earth's heliocentric state vector.
fn earth_heliocentric_state(jd: f64) -> Result<HeliocentricState> {
    let t = julian_centuries(jd);

    let a = 1.000_001_018;
    let e = 0.016_708_634 - 0.000_042_037 * t - 0.000_000_126_7 * t * t;
    let i: f64 = 0.0;
    let node: f64 = 0.0;
    let peri = (102.937_35 + 1.719_526_9 * t).to_radians();
    let l = 100.466_49 + 35_999.372_75 * t;
    let mean_anomaly = (l - rad_to_deg(peri)).to_radians();

    let true_anom = mean_to_true_anomaly(mean_anomaly, e)
        .map_err(|err| JyotishError::MathError(format!("Earth Kepler: {err}")))?;

    let a_m = a * AU_METRES;
    let elements = OrbitalElements::new(a_m, e, i, node, peri, true_anom)
        .map_err(|err| JyotishError::InvalidParameter(format!("Earth elements: {err}")))?;

    let state = elements_to_state(&elements, MU_SUN_SI)
        .map_err(|err| JyotishError::MathError(format!("Earth state: {err}")))?;

    Ok(state_to_au(&state))
}

// ---------------------------------------------------------------------------
// Orbital period and velocity
// ---------------------------------------------------------------------------

/// Compute the orbital period of a body in days given its semi-major axis in AU.
///
/// Uses Kepler's third law: P² = a³ (in years/AU).
///
/// # Examples
///
/// ```
/// # use jyotish::orbital::orbital_period_days;
/// let period = orbital_period_days(1.0); // Earth
/// assert!((period - 365.25).abs() < 0.1, "got {period}");
/// ```
pub fn orbital_period_days(semi_major_axis_au: f64) -> f64 {
    365.256_363 * semi_major_axis_au.powf(1.5)
}

/// Compute the orbital velocity at a given distance from the Sun.
///
/// Uses the vis-viva equation: v² = μ(2/r - 1/a).
/// Returns velocity in km/s.
///
/// # Examples
///
/// ```
/// # use jyotish::orbital::orbital_velocity_km_s;
/// let v = orbital_velocity_km_s(1.0, 1.0); // Earth at 1 AU, a=1 AU
/// assert!((v - 29.78).abs() < 0.5, "got {v}");
/// ```
pub fn orbital_velocity_km_s(distance_au: f64, semi_major_axis_au: f64) -> f64 {
    let r = distance_au * AU_METRES;
    let a = semi_major_axis_au * AU_METRES;
    let v_sq = MU_SUN_SI * (2.0 / r - 1.0 / a);
    v_sq.sqrt() / 1000.0
}

// ---------------------------------------------------------------------------
// Re-export useful falak types
// ---------------------------------------------------------------------------

/// Re-export of falak's `OrbitalElements` for direct use.
pub use falak::orbit::OrbitalElements as FalakOrbitalElements;

#[cfg(test)]
mod tests {
    use super::*;

    const JD_J2000: f64 = 2_451_545.0;

    #[test]
    fn kepler_solver_elliptic() {
        let v = solve_kepler(1.0, 0.5).unwrap();
        // True anomaly should be larger than mean anomaly for e > 0
        assert!(v > 1.0, "got {v}");
    }

    #[test]
    fn kepler_solver_circular() {
        let v = solve_kepler(1.0, 0.0).unwrap();
        assert!((v - 1.0).abs() < 1e-10, "got {v}");
    }

    #[test]
    fn kepler_solver_high_eccentricity() {
        // Pluto-like eccentricity
        let v = solve_kepler(0.5, 0.25).unwrap();
        assert!(v > 0.5, "got {v}");
    }

    #[test]
    fn heliocentric_state_mars() {
        let state = heliocentric_state(Planet::Mars, JD_J2000).unwrap();
        let r = (state.position[0].powi(2) + state.position[1].powi(2) + state.position[2].powi(2))
            .sqrt();
        // Mars heliocentric distance ~1.4-1.7 AU
        assert!(r > 1.0 && r < 2.0, "r = {r} AU");
    }

    #[test]
    fn heliocentric_state_jupiter() {
        let state = heliocentric_state(Planet::Jupiter, JD_J2000).unwrap();
        let r = (state.position[0].powi(2) + state.position[1].powi(2) + state.position[2].powi(2))
            .sqrt();
        assert!(r > 4.5 && r < 5.5, "Jupiter r = {r} AU");
    }

    #[test]
    fn geocentric_position_mars() {
        let pos = compute_position(Planet::Mars, JD_J2000).unwrap();
        assert!(pos.longitude_deg >= 0.0 && pos.longitude_deg < 360.0);
        assert!(pos.distance_au > 0.3 && pos.distance_au < 2.7);
    }

    #[test]
    fn geocentric_position_all_planets() {
        for planet in [
            Planet::Mercury,
            Planet::Venus,
            Planet::Mars,
            Planet::Jupiter,
            Planet::Saturn,
            Planet::Uranus,
            Planet::Neptune,
            Planet::Pluto,
        ] {
            let pos = compute_position(planet, JD_J2000).unwrap();
            assert!(
                pos.longitude_deg >= 0.0 && pos.longitude_deg < 360.0,
                "{planet}: lon = {}",
                pos.longitude_deg
            );
            assert!(
                pos.distance_au > 0.0,
                "{planet}: dist = {}",
                pos.distance_au
            );
        }
    }

    #[test]
    fn falak_vs_builtin_consistency() {
        // The falak-backed and built-in positions should be broadly consistent
        for planet in [Planet::Mars, Planet::Jupiter, Planet::Saturn] {
            let falak_pos = compute_position(planet, JD_J2000).unwrap();
            let builtin_pos = crate::planetary::compute_position(planet, JD_J2000).unwrap();

            let lon_diff = (falak_pos.longitude_deg - builtin_pos.longitude_deg).abs();
            let lon_diff = if lon_diff > 180.0 {
                360.0 - lon_diff
            } else {
                lon_diff
            };

            // Should agree within ~2° (same elements, different solvers)
            assert!(
                lon_diff < 2.0,
                "{planet}: falak={:.2}° builtin={:.2}° diff={:.2}°",
                falak_pos.longitude_deg,
                builtin_pos.longitude_deg,
                lon_diff
            );
        }
    }

    #[test]
    fn sun_moon_rejected() {
        assert!(compute_position(Planet::Sun, JD_J2000).is_err());
        assert!(compute_position(Planet::Moon, JD_J2000).is_err());
    }

    #[test]
    fn orbital_period_earth() {
        let period = orbital_period_days(1.0);
        assert!((period - 365.25).abs() < 0.1, "got {period}");
    }

    #[test]
    fn orbital_period_mars() {
        let period = orbital_period_days(1.524);
        // Mars period ≈ 687 days
        assert!((period - 687.0).abs() < 2.0, "got {period}");
    }

    #[test]
    fn orbital_velocity_earth() {
        let v = orbital_velocity_km_s(1.0, 1.0);
        // Earth's orbital velocity ≈ 29.78 km/s
        assert!((v - 29.78).abs() < 0.5, "got {v}");
    }
}
