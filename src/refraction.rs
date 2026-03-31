//! Atmospheric refraction corrections.
//!
//! Implements Bennett's and Saemundsson's formulae for atmospheric refraction,
//! as described in Meeus, *Astronomical Algorithms*, Chapter 16. These correct
//! for the bending of light by Earth's atmosphere, which causes celestial
//! objects to appear higher than their true geometric position.

use crate::coords::deg_to_rad;

/// Minimum apparent/true altitude (degrees) below which refraction is zero.
const HORIZON_CUTOFF: f64 = -0.575;

/// Standard pressure in millibars for refraction formulae.
const STANDARD_PRESSURE_MBAR: f64 = 1010.0;

/// Bennett's atmospheric refraction from apparent (observed) altitude.
///
/// Given the apparent altitude `h` in degrees (the altitude as measured by an
/// observer), returns the refraction correction in degrees. The true
/// (geometric) altitude is `h - R`.
///
/// Uses Meeus equation 16.4:
///
/// ```text
/// R = 1 / tan(h + 7.31 / (h + 4.4))   [arcminutes]
/// ```
///
/// Returns `0.0` when the apparent altitude is at or below −0.575° (object
/// is below the astronomical horizon).
///
/// # Examples
///
/// ```
/// use jyotish::refraction::refraction_bennett;
///
/// // At the horizon, refraction is roughly 0.575° (≈ 34.5 arcminutes)
/// let r = refraction_bennett(0.0);
/// assert!((r - 0.575).abs() < 0.02);
///
/// // At the zenith, refraction is essentially zero
/// let r = refraction_bennett(90.0);
/// assert!(r.abs() < 0.001);
/// ```
pub fn refraction_bennett(apparent_alt_deg: f64) -> f64 {
    if apparent_alt_deg <= HORIZON_CUTOFF {
        return 0.0;
    }
    let h = apparent_alt_deg;
    let arg = h + 7.31 / (h + 4.4);
    let arg_rad = deg_to_rad(arg);
    let tan_val = arg_rad.tan();
    if tan_val.abs() < 1e-15 {
        return 0.0;
    }
    let r_arcmin = 1.0 / tan_val;
    r_arcmin / 60.0
}

/// Saemundsson's atmospheric refraction from true (geometric) altitude.
///
/// Given the true altitude `h` in degrees, returns the refraction correction
/// in degrees. The apparent (observed) altitude is `h + R`.
///
/// Uses Saemundsson's formula:
///
/// ```text
/// R = 1.02 / tan(h + 10.3 / (h + 5.11))   [arcminutes]
/// ```
///
/// Returns `0.0` when the true altitude is at or below −0.575°.
///
/// # Examples
///
/// ```
/// use jyotish::refraction::refraction_saemundsson;
///
/// // At the horizon (~29 arcminutes ≈ 0.48°)
/// let r = refraction_saemundsson(0.0);
/// assert!((r - 0.483).abs() < 0.03);
///
/// // At the zenith
/// let r = refraction_saemundsson(90.0);
/// assert!(r.abs() < 0.001);
/// ```
pub fn refraction_saemundsson(true_alt_deg: f64) -> f64 {
    if true_alt_deg <= HORIZON_CUTOFF {
        return 0.0;
    }
    let h = true_alt_deg;
    let arg = h + 10.3 / (h + 5.11);
    let arg_rad = deg_to_rad(arg);
    let tan_val = arg_rad.tan();
    if tan_val.abs() < 1e-15 {
        return 0.0;
    }
    let r_arcmin = 1.02 / tan_val;
    r_arcmin / 60.0
}

/// Convert apparent (observed) altitude to true (geometric) altitude.
///
/// Subtracts Bennett's refraction from the apparent altitude:
/// `true = apparent - refraction_bennett(apparent)`.
///
/// # Examples
///
/// ```
/// use jyotish::refraction::apparent_to_true;
///
/// let true_alt = apparent_to_true(1.0);
/// assert!(true_alt < 1.0); // true altitude is always lower
/// ```
pub fn apparent_to_true(apparent_alt_deg: f64) -> f64 {
    apparent_alt_deg - refraction_bennett(apparent_alt_deg)
}

/// Convert true (geometric) altitude to apparent (observed) altitude.
///
/// Adds Saemundsson's refraction to the true altitude:
/// `apparent = true + refraction_saemundsson(true)`.
///
/// # Examples
///
/// ```
/// use jyotish::refraction::true_to_apparent;
///
/// let apparent_alt = true_to_apparent(1.0);
/// assert!(apparent_alt > 1.0); // apparent altitude is always higher
/// ```
pub fn true_to_apparent(true_alt_deg: f64) -> f64 {
    true_alt_deg + refraction_saemundsson(true_alt_deg)
}

/// Bennett's refraction adjusted for non-standard atmospheric conditions.
///
/// Multiplies the base Bennett refraction by the correction factor:
///
/// ```text
/// (P / 1010) × (283 / (273 + T))
/// ```
///
/// where `P` is pressure in millibars and `T` is temperature in degrees Celsius.
///
/// # Arguments
///
/// * `alt_deg` — apparent altitude in degrees
/// * `pressure_mbar` — atmospheric pressure in millibars
/// * `temp_celsius` — air temperature in degrees Celsius
///
/// # Examples
///
/// ```
/// use jyotish::refraction::{refraction_bennett, refraction_with_conditions};
///
/// // Standard conditions (1010 mbar, 10 °C) should match base formula
/// let base = refraction_bennett(0.0);
/// let cond = refraction_with_conditions(0.0, 1010.0, 10.0);
/// assert!((base - cond).abs() < 1e-10);
///
/// // Higher pressure increases refraction
/// let high_p = refraction_with_conditions(10.0, 1050.0, 10.0);
/// let std_p = refraction_with_conditions(10.0, 1010.0, 10.0);
/// assert!(high_p > std_p);
/// ```
pub fn refraction_with_conditions(alt_deg: f64, pressure_mbar: f64, temp_celsius: f64) -> f64 {
    let base = refraction_bennett(alt_deg);
    let factor = (pressure_mbar / STANDARD_PRESSURE_MBAR) * (283.0 / (273.0 + temp_celsius));
    base * factor
}

#[cfg(test)]
mod tests {
    use super::*;

    const ARCMIN_TO_DEG: f64 = 1.0 / 60.0;

    #[test]
    fn bennett_at_horizon() {
        let r = refraction_bennett(0.0);
        // Expect ≈ 34.5 arcminutes ≈ 0.575°
        assert!(
            (r - 34.5 * ARCMIN_TO_DEG).abs() < 0.02,
            "At horizon: expected ~0.575°, got {r}"
        );
    }

    #[test]
    fn bennett_at_zenith() {
        let r = refraction_bennett(90.0);
        assert!(r.abs() < 0.001, "At zenith: expected ~0°, got {r}");
    }

    #[test]
    fn bennett_at_10_degrees() {
        let r = refraction_bennett(10.0);
        let r_arcmin = r * 60.0;
        // Expect ≈ 5.3 arcminutes
        assert!(
            (r_arcmin - 5.3).abs() < 0.3,
            "At 10°: expected ~5.3', got {r_arcmin}'"
        );
    }

    #[test]
    fn bennett_at_45_degrees() {
        let r = refraction_bennett(45.0);
        let r_arcmin = r * 60.0;
        // Expect ≈ 1 arcminute
        assert!(
            (r_arcmin - 1.0).abs() < 0.1,
            "At 45°: expected ~1.0', got {r_arcmin}'"
        );
    }

    #[test]
    fn bennett_below_horizon() {
        assert_eq!(refraction_bennett(-1.0), 0.0);
        assert_eq!(refraction_bennett(-0.575), 0.0);
        assert_eq!(refraction_bennett(-10.0), 0.0);
    }

    #[test]
    fn saemundsson_at_horizon() {
        let r = refraction_saemundsson(0.0);
        let r_arcmin = r * 60.0;
        // Saemundsson at true alt 0° gives ~28.98 arcmin (~0.483°).
        // This differs from Bennett (which takes apparent alt) — the two
        // formulae are approximate inverses, not identical at 0°.
        assert!(
            (r_arcmin - 29.0).abs() < 1.0,
            "At horizon: expected ~29', got {r_arcmin}'"
        );
    }

    #[test]
    fn saemundsson_at_zenith() {
        let r = refraction_saemundsson(90.0);
        assert!(r.abs() < 0.001, "At zenith: expected ~0°, got {r}");
    }

    #[test]
    fn saemundsson_below_horizon() {
        assert_eq!(refraction_saemundsson(-1.0), 0.0);
        assert_eq!(refraction_saemundsson(-0.575), 0.0);
    }

    #[test]
    fn roundtrip_true_to_apparent_to_true() {
        for alt in [0.5, 1.0, 5.0, 10.0, 30.0, 45.0, 60.0, 89.0] {
            let apparent = true_to_apparent(alt);
            let recovered = apparent_to_true(apparent);
            let err = (recovered - alt).abs();
            assert!(
                err < 0.01,
                "Roundtrip failed at {alt}°: recovered {recovered}°, err {err}°"
            );
        }
    }

    #[test]
    fn apparent_to_true_lowers_altitude() {
        for alt in [0.0, 5.0, 30.0, 60.0, 89.0] {
            let true_alt = apparent_to_true(alt);
            assert!(
                true_alt <= alt,
                "apparent_to_true should lower altitude: {alt}° -> {true_alt}°"
            );
        }
    }

    #[test]
    fn true_to_apparent_raises_altitude() {
        for alt in [0.0, 5.0, 30.0, 60.0, 89.0] {
            let app = true_to_apparent(alt);
            assert!(
                app >= alt,
                "true_to_apparent should raise altitude: {alt}° -> {app}°"
            );
        }
    }

    #[test]
    fn conditions_standard_matches_base() {
        for alt in [0.0, 10.0, 45.0, 90.0] {
            let base = refraction_bennett(alt);
            let cond = refraction_with_conditions(alt, 1010.0, 10.0);
            assert!(
                (base - cond).abs() < 1e-12,
                "Standard conditions should match base at {alt}°"
            );
        }
    }

    #[test]
    fn conditions_higher_pressure_increases_refraction() {
        let std = refraction_with_conditions(10.0, 1010.0, 10.0);
        let high = refraction_with_conditions(10.0, 1050.0, 10.0);
        assert!(
            high > std,
            "Higher pressure should increase refraction: {std} vs {high}"
        );
    }

    #[test]
    fn conditions_higher_temperature_decreases_refraction() {
        let cool = refraction_with_conditions(10.0, 1010.0, 0.0);
        let warm = refraction_with_conditions(10.0, 1010.0, 30.0);
        assert!(
            cool > warm,
            "Higher temperature should decrease refraction: cool={cool} vs warm={warm}"
        );
    }

    #[test]
    fn conditions_below_horizon() {
        assert_eq!(refraction_with_conditions(-1.0, 1013.0, 15.0), 0.0);
    }

    #[test]
    fn refraction_monotonically_decreasing() {
        // Refraction should decrease as altitude increases
        let mut prev = refraction_bennett(0.0);
        for alt in (1..=90).map(|a| a as f64) {
            let r = refraction_bennett(alt);
            assert!(
                r <= prev + 1e-12,
                "Refraction should decrease: at {alt}° got {r}, prev was {prev}"
            );
            prev = r;
        }
    }
}
