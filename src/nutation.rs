//! Nutation and precession corrections.
//!
//! Implements the IAU 1980 nutation model (Meeus, *Astronomical Algorithms*,
//! Chapter 22) with the principal periodic terms, and the IAU precession
//! formulae for converting between epochs.
//!
//! Nutation is the short-period oscillation of the Earth's axis caused
//! primarily by the Moon's orbital plane. Precession is the long-period
//! ~26,000-year gyroscopic drift of the equinox along the ecliptic.

use crate::calendar::julian_centuries;
use crate::coords::deg_to_rad;

// ---------------------------------------------------------------------------
// Fundamental arguments for nutation (Meeus Table 22.A)
// ---------------------------------------------------------------------------

/// Longitude of the ascending node of the Moon's mean orbit on the ecliptic.
fn omega(t: f64) -> f64 {
    125.044_52 - 1_934.136_261 * t + 0.002_075_4 * t * t + t * t * t / 467_441.0
}

/// Mean elongation of the Moon from the Sun.
fn d_arg(t: f64) -> f64 {
    297.850_36 + 445_267.111_480 * t - 0.001_914_2 * t * t + t * t * t / 189_474.0
}

/// Mean anomaly of the Sun (Earth).
fn m_sun(t: f64) -> f64 {
    357.527_72 + 35_999.050_340 * t - 0.000_160_3 * t * t - t * t * t / 300_000.0
}

/// Mean anomaly of the Moon.
fn m_moon(t: f64) -> f64 {
    134.962_98 + 477_198.867_398 * t + 0.008_697_2 * t * t + t * t * t / 56_250.0
}

/// Moon's argument of latitude.
fn f_arg(t: f64) -> f64 {
    93.271_91 + 483_202.017_538 * t - 0.003_682_5 * t * t + t * t * t / 327_270.0
}

// ---------------------------------------------------------------------------
// Nutation periodic terms (IAU 1980, Meeus Table 22.A)
// ---------------------------------------------------------------------------

/// Nutation term: (D, M_sun, M_moon, F, Omega, psi_sin, psi_sin_t, eps_cos, eps_cos_t).
/// psi/eps coefficients in 0.0001 arcseconds.
type NutationTerm = (i32, i32, i32, i32, i32, i64, i64, i64, i64);

#[rustfmt::skip]
const NUTATION_TERMS: &[NutationTerm] = &[
    ( 0,  0,  0,  0,  1, -171_996, -1742, 92_025,  89),
    (-2,  0,  0,  2,  2,  -13_187,   -16,  5_736, -31),
    ( 0,  0,  0,  2,  2,   -2_274,    -2,    977,  -5),
    ( 0,  0,  0,  0,  2,    2_062,     2,   -895,   5),
    ( 0,  1,  0,  0,  0,    1_426,   -34,     54,  -1),
    ( 0,  0,  1,  0,  0,      712,     1,     -7,   0),
    (-2,  1,  0,  2,  2,     -517,    12,    224,  -6),
    ( 0,  0,  0,  2,  1,     -386,    -4,    200,   0),
    ( 0,  0,  1,  2,  2,     -301,     0,    129,  -1),
    (-2, -1,  0,  2,  2,      217,    -5,    -95,   3),
    (-2,  0,  1,  0,  0,     -158,     0,      0,   0),
    (-2,  0,  0,  2,  1,      129,     1,    -70,   0),
    ( 0,  0, -1,  2,  2,      123,     0,    -53,   0),
    ( 2,  0,  0,  0,  0,       63,     0,      0,   0),
    ( 0,  0,  1,  0,  1,       63,     1,    -33,   0),
    ( 2,  0, -1,  2,  2,      -59,     0,     26,   0),
    ( 0,  0, -1,  0,  1,      -58,    -1,     32,   0),
    ( 0,  0,  1,  2,  1,      -51,     0,     27,   0),
    (-2,  0,  2,  0,  0,       48,     0,      0,   0),
    ( 0,  0, -2,  2,  1,       46,     0,    -24,   0),
    ( 2,  0,  0,  2,  2,      -38,     0,     16,   0),
    ( 0,  0,  2,  2,  2,      -31,     0,     13,   0),
    ( 0,  0,  2,  0,  0,       29,     0,      0,   0),
    (-2,  0,  1,  2,  2,       29,     0,    -12,   0),
    ( 0,  0,  0,  2,  0,       26,     0,      0,   0),
    (-2,  0,  0,  2,  0,      -22,     0,      0,   0),
    ( 0,  0, -1,  2,  1,       21,     0,    -10,   0),
    ( 0,  2,  0,  0,  0,       17,    -1,      0,   0),
    ( 2,  0, -1,  0,  1,       16,     0,     -8,   0),
    (-2,  2,  0,  2,  2,      -16,     1,      7,   0),
    ( 0,  1,  0,  0,  1,      -15,     0,      9,   0),
    (-2,  0,  1,  0,  1,      -13,     0,      7,   0),
    ( 0, -1,  0,  0,  1,      -12,     0,      6,   0),
    ( 0,  0,  2, -2,  0,       11,     0,      0,   0),
    ( 2,  0, -1,  2,  1,      -10,     0,      5,   0),
    ( 2,  0,  1,  2,  2,       -8,     0,      3,   0),
    ( 0,  1,  0,  2,  2,        7,     0,     -3,   0),
    (-2,  1,  1,  0,  0,       -7,     0,      0,   0),
    ( 0, -1,  0,  2,  2,       -7,     0,      3,   0),
    ( 2,  0,  0,  2,  1,       -7,     0,      3,   0),
    ( 2,  0,  1,  0,  0,       -6,     0,      0,   0),
    (-2,  0,  2,  2,  2,       -6,     0,      3,   0),
    (-2,  0,  1,  2,  1,        6,     0,     -3,   0),
    ( 2,  0, -2,  0,  1,       -6,     0,      3,   0),
    ( 2,  0,  0,  0,  1,       -6,     0,      3,   0),
    ( 0, -1,  1,  0,  0,        5,     0,      0,   0),
    (-2, -1,  0,  2,  1,       -5,     0,      3,   0),
    (-2,  0,  0,  0,  1,       -5,     0,      3,   0),
    ( 0,  0,  2,  2,  1,       -5,     0,      3,   0),
    (-2,  0,  2,  0,  1,        4,     0,      0,   0),
    (-2,  1,  0,  2,  1,        4,     0,      0,   0),
    ( 0,  0,  1, -2,  0,        4,     0,      0,   0),
    (-1,  0,  1,  0,  0,       -4,     0,      0,   0),
    (-2,  1,  0,  0,  0,       -4,     0,      0,   0),
    ( 1,  0,  0,  0,  0,       -4,     0,      0,   0),
    ( 0,  0,  1,  2,  0,        3,     0,      0,   0),
    ( 0,  0, -2,  2,  2,       -3,     0,      0,   0),
    (-1, -1,  1,  0,  0,       -3,     0,      0,   0),
    ( 0,  1,  1,  0,  0,       -3,     0,      0,   0),
    ( 0, -1,  1,  2,  2,       -3,     0,      0,   0),
    ( 2, -1, -1,  2,  2,       -3,     0,      0,   0),
    ( 0,  0,  3,  2,  2,       -3,     0,      0,   0),
    ( 2, -1,  0,  2,  2,       -3,     0,      0,   0),
];

// ---------------------------------------------------------------------------
// Public API — Nutation
// ---------------------------------------------------------------------------

/// Nutation in longitude (Δψ) and obliquity (Δε) in arcseconds.
///
/// Returns `(delta_psi, delta_epsilon)` both in arcseconds.
///
/// # Examples
///
/// ```
/// # use jyotish::nutation::nutation_components;
/// // Meeus example 22.a: 1987-04-10 at 0h TD, JD = 2446895.5
/// let (dpsi, deps) = nutation_components(2_446_895.5);
/// // Expected: Δψ ≈ -3.788", Δε ≈ 9.443"
/// assert!((dpsi - (-3.788)).abs() < 0.5, "Δψ = {dpsi}");
/// assert!((deps - 9.443).abs() < 0.5, "Δε = {deps}");
/// ```
pub fn nutation_components(jd: f64) -> (f64, f64) {
    let t = julian_centuries(jd);

    let omega_r = deg_to_rad(omega(t));
    let d_r = deg_to_rad(d_arg(t));
    let m_sun_r = deg_to_rad(m_sun(t));
    let m_moon_r = deg_to_rad(m_moon(t));
    let f_r = deg_to_rad(f_arg(t));

    let mut delta_psi: f64 = 0.0;
    let mut delta_eps: f64 = 0.0;

    for &(d, ms, mm, f, om, psi_s, psi_st, eps_c, eps_ct) in NUTATION_TERMS {
        let arg = d as f64 * d_r
            + ms as f64 * m_sun_r
            + mm as f64 * m_moon_r
            + f as f64 * f_r
            + om as f64 * omega_r;

        delta_psi += (psi_s as f64 + psi_st as f64 * t) * arg.sin();
        delta_eps += (eps_c as f64 + eps_ct as f64 * t) * arg.cos();
    }

    // Convert from 0.0001 arcseconds to arcseconds
    (delta_psi / 10_000.0, delta_eps / 10_000.0)
}

/// Nutation in longitude (Δψ) in degrees.
pub fn nutation_longitude(jd: f64) -> f64 {
    nutation_components(jd).0 / 3600.0
}

/// Nutation in obliquity (Δε) in degrees.
pub fn nutation_obliquity(jd: f64) -> f64 {
    nutation_components(jd).1 / 3600.0
}

/// True obliquity of the ecliptic in degrees (mean obliquity + nutation).
///
/// # Examples
///
/// ```
/// # use jyotish::nutation::true_obliquity;
/// let eps = true_obliquity(2_446_895.5);
/// // Should be close to 23.44°
/// assert!((eps - 23.44).abs() < 0.05, "got {eps}");
/// ```
pub fn true_obliquity(jd: f64) -> f64 {
    crate::coords::mean_obliquity(julian_centuries(jd)) + nutation_obliquity(jd)
}

// ---------------------------------------------------------------------------
// Public API — Precession
// ---------------------------------------------------------------------------

/// General precession in longitude (ψ_A) accumulated from J2000.0.
///
/// `t` is Julian centuries from J2000.0. Returns the precession in degrees.
///
/// Uses the IAU 2006 precession (Lieske constants via Meeus eq. 21.3).
///
/// # Examples
///
/// ```
/// # use jyotish::nutation::precession_longitude;
/// // One century of precession ≈ 1.3972°
/// let prec = precession_longitude(1.0);
/// assert!((prec - 1.3972).abs() < 0.001, "got {prec}");
/// ```
pub fn precession_longitude(t: f64) -> f64 {
    // General precession in longitude (Meeus eq. 21.3)
    // In arcseconds: 5029.0966" * T + 1.11113" * T² - 0.000006" * T³
    (5_029.096_6 * t + 1.111_13 * t * t - 0.000_006 * t * t * t) / 3600.0
}

/// Precession parameters (ζ_A, z_A, θ_A) for equatorial precession from J2000.0.
///
/// Returns `(zeta_a, z_a, theta_a)` in degrees. These are the three Euler angles
/// for rotating equatorial coordinates from J2000.0 to the epoch at `t` Julian
/// centuries from J2000.0.
///
/// # Examples
///
/// ```
/// # use jyotish::nutation::precession_equatorial;
/// let (zeta, z, theta) = precession_equatorial(1.0);
/// assert!(zeta > 0.0 && zeta < 1.0);
/// assert!(z > 0.0 && z < 1.0);
/// assert!(theta > 0.0 && theta < 1.0);
/// ```
pub fn precession_equatorial(t: f64) -> (f64, f64, f64) {
    // Meeus eq. 21.2, coefficients pre-converted to degrees
    let zeta_a = 0.640_616_1 * t + 0.000_083_9 * t * t + 0.000_005_0 * t * t * t;
    let z_a = 0.640_616_1 * t + 0.000_304_1 * t * t + 0.000_005_1 * t * t * t;
    let theta_a = 0.556_753_0 * t - 0.000_118_5 * t * t - 0.000_011_6 * t * t * t;

    (zeta_a, z_a, theta_a)
}

/// Precess ecliptic longitude from J2000.0 to the epoch at the given Julian Date.
///
/// Takes a J2000.0 ecliptic longitude in degrees and returns the longitude
/// precessed to the epoch of `jd`.
pub fn precess_longitude(lon_j2000: f64, jd: f64) -> f64 {
    let t = julian_centuries(jd);
    crate::coords::normalize_degrees(lon_j2000 + precession_longitude(t))
}

/// Precess ecliptic longitude from an arbitrary epoch back to J2000.0.
pub fn deprecess_longitude(lon_epoch: f64, jd: f64) -> f64 {
    let t = julian_centuries(jd);
    crate::coords::normalize_degrees(lon_epoch - precession_longitude(t))
}

/// Annual precession rate in degrees per Julian year at the given epoch.
///
/// The precession rate is approximately 50.29" per year but varies slowly.
pub fn annual_precession_rate(jd: f64) -> f64 {
    let t = julian_centuries(jd);
    // Derivative of precession_longitude with respect to T, converted to per year
    // d/dT [5029.0966*T + 1.11113*T² - 0.000006*T³] / 3600 / 100
    (5_029.096_6 + 2.0 * 1.111_13 * t - 3.0 * 0.000_006 * t * t) / 3600.0 / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::J2000_0;

    const JD_MEEUS_22A: f64 = 2_446_895.5; // 1987-04-10 0h TD

    #[test]
    fn nutation_meeus_22a() {
        let (dpsi, deps) = nutation_components(JD_MEEUS_22A);
        // Meeus gives Δψ ≈ -3.788", Δε ≈ 9.443"
        assert!((dpsi - (-3.788)).abs() < 0.5, "Δψ = {dpsi}");
        assert!((deps - 9.443).abs() < 0.5, "Δε = {deps}");
    }

    #[test]
    fn nutation_longitude_degrees() {
        let dpsi_deg = nutation_longitude(JD_MEEUS_22A);
        // Should be small, ~ -0.001°
        assert!(dpsi_deg.abs() < 0.01);
    }

    #[test]
    fn true_obliquity_reasonable() {
        let eps = true_obliquity(JD_MEEUS_22A);
        // True obliquity should be near 23.44°
        assert!((eps - 23.44).abs() < 0.05, "got {eps}");
    }

    #[test]
    fn true_obliquity_j2000() {
        let eps = true_obliquity(J2000_0);
        assert!((eps - 23.439).abs() < 0.01, "got {eps}");
    }

    #[test]
    fn precession_one_century() {
        let prec = precession_longitude(1.0);
        // ~1.3972° per century
        assert!((prec - 1.3972).abs() < 0.001, "got {prec}");
    }

    #[test]
    fn precession_zero_at_j2000() {
        let prec = precession_longitude(0.0);
        assert!(prec.abs() < 1e-10);
    }

    #[test]
    fn precession_equatorial_params() {
        let (zeta, z, theta) = precession_equatorial(1.0);
        // At T=1 century, all should be ~0.6° range
        assert!(zeta > 0.0 && zeta < 1.0, "zeta = {zeta}");
        assert!(z > 0.0 && z < 1.0, "z = {z}");
        assert!(theta > 0.0 && theta < 1.0, "theta = {theta}");
    }

    #[test]
    fn precess_deprecess_roundtrip() {
        let lon = 45.0;
        let jd = 2_460_000.0; // ~2023
        let precessed = precess_longitude(lon, jd);
        let restored = deprecess_longitude(precessed, jd);
        assert!((restored - lon).abs() < 1e-10, "got {restored}");
    }

    #[test]
    fn annual_precession_rate_value() {
        let rate = annual_precession_rate(J2000_0);
        // ~50.29 arcsec/year = ~0.01397°/year
        assert!((rate - 0.013_97).abs() < 0.0001, "got {rate}");
    }

    #[test]
    fn nutation_range_over_year() {
        // Nutation in longitude should stay within ±20 arcseconds
        for day in 0..365 {
            let jd = J2000_0 + day as f64;
            let (dpsi, deps) = nutation_components(jd);
            assert!(dpsi.abs() < 20.0, "Δψ {dpsi} at day {day}");
            assert!(deps.abs() < 15.0, "Δε {deps} at day {day}");
        }
    }
}
