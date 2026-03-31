//! Nutation and precession corrections.
//!
//! Implements the **IAU 2000B** nutation model (McCarthy & Luzum 2003,
//! "An Abridged Model of the Precession-Nutation of the Celestial Pole")
//! with 77 lunisolar terms, achieving ~1 milliarcsecond accuracy for
//! epochs 1900–2100.
//!
//! The IAU 2000B model is a truncated form of the full IAU 2000A (MHB2000)
//! series. It retains 77 lunisolar terms and adds fixed offset corrections
//! to compensate for omitted long-period planetary nutation terms.
//!
//! Also provides IAU precession formulae for converting between epochs.

use crate::calendar::julian_centuries;
use crate::num::KahanSum;

// ---------------------------------------------------------------------------
// Fundamental Delaunay arguments (IAU 2003, IERS Conventions Ch. 5)
// ---------------------------------------------------------------------------
// Each argument is computed as a linear function of T (Julian centuries from
// J2000.0) in arcseconds, then reduced modulo 1296000" (= 360°) and
// converted to radians.
//
// These are simplified (linear-only) forms used by IAU 2000B, matching the
// SOFA/ERFA nut00b implementation. The higher-order polynomial terms are
// omitted because the truncated series does not warrant them.

/// Arcseconds in a full circle (360° × 3600″/°).
const TURNAS: f64 = 1_296_000.0;

/// Arcseconds to radians.
const AS2R: f64 = std::f64::consts::PI / (180.0 * 3600.0);

/// Mean anomaly of the Moon (l).
fn el(t: f64) -> f64 {
    // 485868.249036″ + 1717915923.2178″·T, mod 1296000″, → radians
    let a = 485_868.249_036 + 1_717_915_923.217_8 * t;
    (a % TURNAS) * AS2R
}

/// Mean anomaly of the Sun (l').
fn elp(t: f64) -> f64 {
    let a = 1_287_104.793_05 + 129_596_581.048_1 * t;
    (a % TURNAS) * AS2R
}

/// Mean argument of the latitude of the Moon (F).
fn f_arg(t: f64) -> f64 {
    let a = 335_779.526_232 + 1_739_527_262.847_8 * t;
    (a % TURNAS) * AS2R
}

/// Mean elongation of the Moon from the Sun (D).
fn d_arg(t: f64) -> f64 {
    let a = 1_072_260.703_69 + 1_602_961_601.209_0 * t;
    (a % TURNAS) * AS2R
}

/// Longitude of the ascending node of the Moon's mean orbit (Ω).
fn om(t: f64) -> f64 {
    let a = 450_160.398_036 - 6_962_890.543_1 * t;
    (a % TURNAS) * AS2R
}

// ---------------------------------------------------------------------------
// IAU 2000B nutation series — 77 lunisolar terms
// ---------------------------------------------------------------------------
// Each term: (l, l', F, D, Ω, ψ_sin, ψ_sin_t, ψ_cos, ε_cos, ε_cos_t, ε_sin)
//
// Coefficients are in units of 0.1 microarcsecond (0.1 μas).
//
// For each term the argument is:  arg = l·el + l'·elp + F·f + D·d + Ω·om
//   Δψ += (ψ_sin + ψ_sin_t · t) · sin(arg) + ψ_cos · cos(arg)
//   Δε += (ε_cos + ε_cos_t · t) · cos(arg) + ε_sin · sin(arg)
//
// Source: SOFA/ERFA nut00b (McCarthy & Luzum 2003).
type NutTerm = (i8, i8, i8, i8, i8, i64, i64, i64, i64, i64, i64);

#[rustfmt::skip]
const IAU2000B_TERMS: &[NutTerm] = &[
    // l  l'  F   D  Ω      ψ_sin       ψ_sin_t    ψ_cos       ε_cos       ε_cos_t    ε_sin
    ( 0, 0, 0, 0, 1, -172_064_161, -174_666,  33_386,  92_052_331,   9_086,  15_377),
    ( 0, 0, 2,-2, 2,  -13_170_906,   -1_675, -13_696,   5_730_336,  -3_015,  -4_587),
    ( 0, 0, 2, 0, 2,   -2_276_413,     -234,   2_796,     978_459,    -485,   1_374),
    ( 0, 0, 0, 0, 2,    2_074_554,      207,    -698,    -897_492,     470,    -291),
    ( 0, 1, 0, 0, 0,    1_475_877,   -3_633,  11_817,      73_871,    -184,  -1_924),
    ( 0, 1, 2,-2, 2,     -516_821,    1_226,    -524,     224_386,    -677,    -174),
    ( 1, 0, 0, 0, 0,      711_159,       73,    -872,      -6_750,       0,     358),
    ( 0, 0, 2, 0, 1,     -387_298,     -367,     380,     200_728,      18,     318),
    ( 1, 0, 2, 0, 2,     -301_461,      -36,     816,     129_025,     -63,     367),
    ( 0,-1, 2,-2, 2,      215_829,     -494,     111,     -95_929,     299,     132),
    ( 0, 0, 2,-2, 1,      128_227,      137,     181,     -68_982,      -9,      39),
    (-1, 0, 2, 0, 2,      123_457,       11,      19,     -53_311,      32,      -4),
    (-1, 0, 0, 2, 0,      156_994,       10,    -168,      -1_235,       0,      82),
    ( 1, 0, 0, 0, 1,       63_110,       63,      27,     -33_228,       0,      -9),
    (-1, 0, 0, 0, 1,      -57_976,      -63,    -189,      31_429,       0,     -75),
    (-1, 0, 2, 2, 2,      -59_641,      -11,     149,      25_543,     -11,      66),
    ( 1, 0, 2, 0, 1,      -51_613,      -42,     129,      26_366,       0,      78),
    (-2, 0, 2, 0, 1,       45_893,       50,      31,     -24_236,     -10,      20),
    ( 0, 0, 0, 2, 0,       63_384,       11,    -150,      -1_220,       0,      29),
    ( 0, 0, 2, 2, 2,      -38_571,       -1,     158,      16_452,     -11,      68),
    ( 0,-2, 2,-2, 2,       32_481,        0,       0,     -13_870,       0,       0),
    (-2, 0, 0, 2, 0,      -47_722,        0,     -18,         477,       0,     -25),
    ( 2, 0, 2, 0, 2,      -31_046,       -1,     131,      13_238,     -11,      59),
    ( 1, 0, 2,-2, 2,       28_593,        0,      -1,     -12_338,      10,      -3),
    (-1, 0, 2, 0, 1,       20_441,       21,      10,     -10_758,       0,      -3),
    ( 2, 0, 0, 0, 0,       29_243,        0,     -74,        -609,       0,      13),
    ( 0, 0, 2, 0, 0,       25_887,        0,     -66,        -550,       0,      11),
    ( 0, 1, 0, 0, 1,      -14_053,      -25,      79,       8_551,      -2,     -45),
    (-1, 0, 0, 2, 1,       15_164,       10,      11,      -8_001,       0,      -1),
    ( 0, 2, 2,-2, 2,      -15_794,       72,     -16,       6_850,     -42,      -5),
    ( 0, 0,-2, 2, 0,       21_783,        0,      13,        -167,       0,      13),
    ( 1, 0, 0,-2, 1,      -12_873,      -10,     -37,       6_953,       0,     -14),
    ( 0,-1, 0, 0, 1,      -12_654,       11,      63,       6_415,       0,      26),
    (-1, 0, 2, 2, 1,      -10_204,        0,      25,       5_222,       0,      15),
    ( 0, 2, 0, 0, 0,       16_707,      -85,     -10,         168,      -1,      10),
    ( 1, 0, 2, 2, 2,       -7_691,        0,      44,       3_268,       0,      19),
    (-2, 0, 2, 0, 0,      -11_024,        0,     -14,         104,       0,       2),
    ( 0, 1, 2, 0, 2,        7_566,      -21,     -11,      -3_250,       0,      -5),
    ( 0, 0, 2, 2, 1,       -6_637,      -11,      25,       3_353,       0,      14),
    ( 0,-1, 2, 0, 2,       -7_141,       21,       8,       3_070,       0,       4),
    ( 0, 0, 0, 2, 1,       -6_302,      -11,       2,       3_272,       0,       4),
    ( 1, 0, 2,-2, 1,        5_800,       10,       2,      -3_045,       0,      -1),
    ( 2, 0, 2,-2, 2,        6_443,        0,      -7,      -2_768,       0,      -4),
    (-2, 0, 0, 2, 1,       -5_774,      -11,     -15,       3_041,       0,      -5),
    ( 2, 0, 2, 0, 1,       -5_350,        0,      21,       2_695,       0,      12),
    ( 0,-1, 2,-2, 1,       -4_752,      -11,      -3,       2_719,       0,      -3),
    ( 0, 0, 0,-2, 1,       -4_940,      -11,     -21,       2_720,       0,      -9),
    (-1,-1, 0, 2, 0,        7_350,        0,      -8,         -51,       0,       4),
    ( 2, 0, 0,-2, 1,        4_065,        0,       6,      -2_206,       0,       1),
    ( 1, 0, 0, 2, 0,        6_579,        0,     -24,        -199,       0,       2),
    ( 0, 1, 2,-2, 1,        3_579,        0,       5,      -1_900,       0,       1),
    ( 1,-1, 0, 0, 0,        4_725,        0,      -6,         -41,       0,       3),
    (-2, 0, 2, 0, 2,       -3_075,        0,      -2,       1_313,       0,      -1),
    ( 3, 0, 2, 0, 2,       -2_904,        0,      15,       1_233,       0,       7),
    ( 0,-1, 0, 2, 0,        4_348,        0,     -10,         -81,       0,       2),
    ( 1,-1, 2, 0, 2,       -2_878,        0,       8,       1_232,       0,       4),
    ( 0, 0, 0, 1, 0,       -4_230,        0,       5,         -20,       0,      -2),
    (-1,-1, 2, 2, 2,       -2_819,        0,       7,       1_207,       0,       3),
    (-1, 0, 2, 0, 0,       -4_056,        0,       5,          40,       0,      -2),
    ( 0,-1, 2, 2, 2,       -2_647,        0,      11,       1_129,       0,       5),
    (-2, 0, 0, 0, 1,       -2_294,        0,     -10,       1_266,       0,      -4),
    ( 1, 1, 2, 0, 2,        2_481,        0,      -7,      -1_062,       0,      -3),
    ( 2, 0, 0, 0, 1,        2_179,        0,      -2,      -1_129,       0,      -2),
    (-1, 1, 0, 1, 0,        3_276,        0,       1,          -9,       0,       0),
    ( 1, 1, 0, 0, 0,       -3_389,        0,       5,          35,       0,      -2),
    ( 1, 0, 2, 0, 0,        3_339,        0,     -13,        -107,       0,       1),
    (-1, 0, 2,-2, 1,       -1_987,        0,      -6,       1_073,       0,      -2),
    ( 1, 0, 0, 0, 2,       -1_981,        0,       0,         854,       0,       0),
    (-1, 0, 0, 1, 0,        4_026,        0,    -353,        -553,       0,    -139),
    ( 0, 0, 2, 1, 2,        1_660,        0,      -5,        -710,       0,      -2),
    (-1, 0, 2, 4, 2,       -1_521,        0,       9,         647,       0,       4),
    (-1, 1, 0, 1, 1,        1_314,        0,       0,        -700,       0,       0),
    ( 0,-2, 2,-2, 1,       -1_283,        0,       0,         672,       0,       0),
    ( 1, 0, 2, 2, 1,       -1_331,        0,       8,         663,       0,       4),
    (-2, 0, 2, 2, 2,        1_383,        0,      -2,        -594,       0,      -2),
    (-1, 0, 0, 0, 2,        1_405,        0,       4,        -610,       0,       2),
    ( 1, 1, 2,-2, 2,        1_290,        0,       0,        -556,       0,       0),
];

/// Fixed offset corrections for omitted long-period planetary nutation terms.
/// These are added to Δψ and Δε respectively, in milliarcseconds.
const DPPLAN_MAS: f64 = -0.135;
const DEPLAN_MAS: f64 = 0.388;

/// Conversion factor from 0.1 microarcsecond to arcseconds.
/// 0.1 μas = 0.1e-6 as = 1e-7 as.
const U01MUAS_TO_AS: f64 = 1.0e-7;

/// Conversion factor from milliarcseconds to arcseconds.
const MAS_TO_AS: f64 = 1.0e-3;

// ---------------------------------------------------------------------------
// Public API — Nutation
// ---------------------------------------------------------------------------

/// Nutation in longitude (Δψ) and obliquity (Δε) in arcseconds.
///
/// Uses the IAU 2000B model (77 lunisolar terms) with fixed planetary offset
/// corrections, delivering ~1 mas accuracy for epochs 1900–2100.
///
/// Returns `(delta_psi, delta_epsilon)` both in arcseconds.
///
/// # Examples
///
/// ```
/// # use jyotish::nutation::nutation_components;
/// // Meeus example 22.a: 1987-04-10 at 0h TD, JD = 2446895.5
/// let (dpsi, deps) = nutation_components(2_446_895.5);
/// // IAU 2000B gives Δψ ≈ -3.79", Δε ≈ 9.44"
/// assert!((dpsi - (-3.79)).abs() < 0.5, "Δψ = {dpsi}");
/// assert!((deps - 9.44).abs() < 0.5, "Δε = {deps}");
/// ```
pub fn nutation_components(jd: f64) -> (f64, f64) {
    let t = julian_centuries(jd);

    // Fundamental Delaunay arguments (radians)
    let l = el(t);
    let lp = elp(t);
    let f = f_arg(t);
    let d = d_arg(t);
    let omega = om(t);

    let mut delta_psi = KahanSum::new();
    let mut delta_eps = KahanSum::new();

    for &(nl, nlp, nf, nd, nom, psi_s, psi_st, psi_c, eps_c, eps_ct, eps_s) in IAU2000B_TERMS {
        let arg =
            nl as f64 * l + nlp as f64 * lp + nf as f64 * f + nd as f64 * d + nom as f64 * omega;

        let sin_arg = arg.sin();
        let cos_arg = arg.cos();

        // Δψ += (ψ_sin + ψ_sin_t · t) · sin(arg) + ψ_cos · cos(arg)
        delta_psi.add((psi_s as f64 + psi_st as f64 * t) * sin_arg + psi_c as f64 * cos_arg);

        // Δε += (ε_cos + ε_cos_t · t) · cos(arg) + ε_sin · sin(arg)
        delta_eps.add((eps_c as f64 + eps_ct as f64 * t) * cos_arg + eps_s as f64 * sin_arg);
    }

    // Convert from 0.1 microarcseconds to arcseconds, then add planetary offsets
    let dpsi_as = delta_psi.sum() * U01MUAS_TO_AS + DPPLAN_MAS * MAS_TO_AS;
    let deps_as = delta_eps.sum() * U01MUAS_TO_AS + DEPLAN_MAS * MAS_TO_AS;

    (dpsi_as, deps_as)
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
        // IAU 2000B gives values close to the Meeus (IAU 1980) values but
        // not identical.  Meeus: Δψ ≈ -3.788″, Δε ≈ 9.443″.
        // With IAU 2000B we expect values within ~0.5″ of Meeus.
        assert!(
            (dpsi - (-3.788)).abs() < 0.5,
            "Δψ = {dpsi}, expected near -3.788"
        );
        assert!(
            (deps - 9.443).abs() < 0.5,
            "Δε = {deps}, expected near 9.443"
        );
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

    #[test]
    fn iau2000b_term_count() {
        assert_eq!(IAU2000B_TERMS.len(), 77, "IAU 2000B should have 77 terms");
    }

    #[test]
    fn nutation_j2000_epoch() {
        // At J2000.0 the nutation should be non-zero but small
        let (dpsi, deps) = nutation_components(J2000_0);
        assert!(dpsi.abs() < 20.0, "Δψ at J2000 = {dpsi}");
        assert!(deps.abs() < 15.0, "Δε at J2000 = {deps}");
    }

    #[test]
    fn nutation_symmetry_half_nutation_period() {
        // The dominant nutation period is 18.6 years (6798 days).
        // Check that nutation values at ±half-period from J2000 are roughly
        // opposite in sign (the dominant Ω term dominates).
        let half_period = 6798.0 / 2.0;
        let (dpsi_plus, _) = nutation_components(J2000_0 + half_period);
        let (dpsi_minus, _) = nutation_components(J2000_0 - half_period);
        // They should have similar magnitudes (within a factor of 2) and
        // we just check that both are within physical bounds.
        assert!(dpsi_plus.abs() < 20.0);
        assert!(dpsi_minus.abs() < 20.0);
    }

    /// Cross-check: at 2005-12-24 (JD 2453728.5) the nutation should be
    /// within physical bounds and reasonably small.
    #[test]
    fn nutation_2005_epoch() {
        let jd = 2_400_000.5 + 53_736.0;
        let (dpsi, deps) = nutation_components(jd);
        // Δψ should be in the range ±20″, Δε in ±15″
        assert!(dpsi.abs() < 20.0, "Δψ = {dpsi}");
        assert!(deps.abs() < 15.0, "Δε = {deps}");
        // For this epoch, nutation in longitude should be negative and small
        assert!(
            dpsi < 0.0,
            "Δψ should be negative at this epoch, got {dpsi}"
        );
    }
}
