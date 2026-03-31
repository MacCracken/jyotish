//! Lunar position computation.
//!
//! Implements the lunar position algorithm from Meeus (*Astronomical Algorithms*,
//! Chapter 47). Uses the principal periodic terms to achieve approximately
//! 10 arcseconds accuracy in longitude and ~4 arcseconds in latitude.

use crate::calendar::julian_centuries;
use crate::coords::{deg_to_rad, normalize_degrees};
use crate::planet::{Planet, PlanetaryPosition};

// ---------------------------------------------------------------------------
// Fundamental arguments (Meeus Ch. 47)
// ---------------------------------------------------------------------------

/// Moon's mean longitude (Meeus eq. 47.1).
fn mean_longitude(t: f64) -> f64 {
    normalize_degrees(
        218.316_447_7 + 481_267.881_234_21 * t - 0.001_578_6 * t * t + t * t * t / 538_841.0
            - t * t * t * t / 65_194_000.0,
    )
}

/// Moon's mean elongation (Meeus eq. 47.2).
fn mean_elongation(t: f64) -> f64 {
    normalize_degrees(
        297.850_192_1 + 445_267.111_403_4 * t - 0.001_881_9 * t * t + t * t * t / 545_868.0
            - t * t * t * t / 113_065_000.0,
    )
}

/// Sun's mean anomaly (Meeus eq. 47.3).
fn sun_mean_anomaly(t: f64) -> f64 {
    normalize_degrees(
        357.529_109_2 + 35_999.050_290_9 * t - 0.000_153_6 * t * t + t * t * t / 24_490_000.0,
    )
}

/// Moon's mean anomaly (Meeus eq. 47.4).
fn moon_mean_anomaly(t: f64) -> f64 {
    normalize_degrees(
        134.963_396_4 + 477_198.867_505_5 * t + 0.008_741_4 * t * t + t * t * t / 69_699.0
            - t * t * t * t / 14_712_000.0,
    )
}

/// Moon's argument of latitude (Meeus eq. 47.5).
fn argument_of_latitude(t: f64) -> f64 {
    normalize_degrees(
        93.272_095_0 + 483_202.017_523_3 * t - 0.003_653_9 * t * t - t * t * t / 3_526_000.0
            + t * t * t * t / 863_310_000.0,
    )
}

// ---------------------------------------------------------------------------
// Periodic terms for longitude and distance (Table 47.A)
// ---------------------------------------------------------------------------

/// Each entry: (D_mult, M_mult, Mp_mult, F_mult, sin_coeff_longitude, cos_coeff_distance)
/// Coefficients: longitude in 0.000001 degrees, distance in 0.001 km.
#[rustfmt::skip]
const LONGITUDE_DISTANCE_TERMS: &[(i32, i32, i32, i32, i64, i64)] = &[
    ( 0,  0,  1,  0,  6_288_774, -20_905_355),
    ( 2,  0, -1,  0,  1_274_027,  -3_699_111),
    ( 2,  0,  0,  0,    658_314,  -2_955_968),
    ( 0,  0,  2,  0,    213_618,    -569_925),
    ( 0,  1,  0,  0,   -185_116,      48_888),
    ( 0,  0,  0,  2,   -114_332,      -3_149),
    ( 2,  0, -2,  0,     58_793,     246_158),
    ( 2, -1, -1,  0,     57_066,    -152_138),
    ( 2,  0,  1,  0,     53_322,    -170_733),
    ( 2, -1,  0,  0,     45_758,    -204_586),
    ( 0,  1, -1,  0,    -40_923,    -129_620),
    ( 1,  0,  0,  0,    -34_720,     108_743),
    ( 0,  1,  1,  0,    -30_383,     104_755),
    ( 2,  0,  0, -2,     15_327,      10_321),
    ( 0,  0,  1,  2,    -12_528,           0),
    ( 0,  0,  1, -2,     10_980,      79_661),
    ( 4,  0, -1,  0,     10_675,     -34_782),
    ( 0,  0,  3,  0,     10_034,     -23_210),
    ( 4,  0, -2,  0,      8_548,     -21_636),
    ( 2,  1, -1,  0,     -7_888,      24_208),
    ( 2,  1,  0,  0,     -6_766,      30_824),
    ( 1,  0, -1,  0,     -5_163,      -8_379),
    ( 1,  1,  0,  0,      4_987,     -16_675),
    ( 2, -1,  1,  0,      4_036,     -12_831),
    ( 2,  0,  2,  0,      3_994,     -10_445),
    ( 4,  0,  0,  0,      3_861,     -11_650),
    ( 2,  0, -3,  0,      3_665,      14_403),
    ( 0,  1, -2,  0,     -2_689,      -7_003),
    ( 2,  0, -1,  2,     -2_602,           0),
    ( 2, -1, -2,  0,      2_390,      10_056),
    ( 1,  0,  1,  0,     -2_348,       6_322),
    ( 2, -2,  0,  0,      2_236,      -9_884),
    ( 0,  1,  2,  0,     -2_120,       5_751),
    ( 0,  2,  0,  0,     -2_069,           0),
    ( 2, -2, -1,  0,      2_048,      -4_950),
    ( 2,  0,  1, -2,     -1_773,       4_130),
    ( 2,  0,  0,  2,     -1_595,           0),
    ( 4, -1, -1,  0,      1_215,      -3_958),
    ( 0,  0,  2,  2,     -1_110,           0),
    ( 3,  0, -1,  0,      -892,       3_258),
    ( 2,  1,  1,  0,      -810,       2_616),
    ( 4, -1, -2,  0,       759,      -1_897),
    ( 0,  2, -1,  0,      -713,      -2_117),
    ( 2,  2, -1,  0,      -700,       2_354),
    ( 2,  1, -2,  0,       691,           0),
    ( 2, -1,  0, -2,       596,           0),
    ( 4,  0,  1,  0,       549,      -1_423),
    ( 0,  0,  4,  0,       537,      -1_117),
    ( 4, -1,  0,  0,       520,      -1_571),
    ( 1,  0, -2,  0,      -487,      -1_739),
    ( 2,  1,  0, -2,      -399,           0),
    ( 0,  0,  2, -2,      -381,      -4_421),
    ( 1,  1,  1,  0,       351,           0),
    ( 3,  0, -2,  0,      -340,           0),
    ( 4,  0, -3,  0,       330,           0),
    ( 2, -1,  2,  0,       327,           0),
    ( 0,  2,  1,  0,      -323,       1_165),
    ( 1,  1, -1,  0,       299,           0),
    ( 2,  0,  3,  0,       294,           0),
    ( 2,  0, -1, -2,         0,       8_752),
];

// ---------------------------------------------------------------------------
// Periodic terms for latitude (Table 47.B)
// ---------------------------------------------------------------------------

/// Each entry: (D_mult, M_mult, Mp_mult, F_mult, sin_coeff)
/// Coefficient in 0.000001 degrees.
#[rustfmt::skip]
const LATITUDE_TERMS: &[(i32, i32, i32, i32, i64)] = &[
    ( 0,  0,  0,  1,  5_128_122),
    ( 0,  0,  1,  1,    280_602),
    ( 0,  0,  1, -1,    277_693),
    ( 2,  0,  0, -1,    173_237),
    ( 2,  0, -1,  1,     55_413),
    ( 2,  0, -1, -1,     46_271),
    ( 2,  0,  0,  1,     32_573),
    ( 0,  0,  2,  1,     17_198),
    ( 2,  0,  1, -1,      9_266),
    ( 0,  0,  2, -1,      8_822),
    ( 2, -1,  0, -1,      8_216),
    ( 2,  0, -2, -1,      4_324),
    ( 2,  0,  1,  1,      4_200),
    ( 2,  1,  0, -1,     -3_359),
    ( 2, -1, -1,  1,      2_463),
    ( 2, -1,  0,  1,      2_211),
    ( 2, -1, -1, -1,      2_065),
    ( 0,  1, -1, -1,     -1_870),
    ( 4,  0, -1, -1,      1_828),
    ( 0,  1,  0,  1,     -1_794),
    ( 0,  0,  0,  3,     -1_749),
    ( 0,  1, -1,  1,     -1_565),
    ( 1,  0,  0,  1,     -1_491),
    ( 0,  1,  1,  1,     -1_475),
    ( 0,  1,  1, -1,     -1_410),
    ( 0,  1,  0, -1,     -1_344),
    ( 1,  0,  0, -1,     -1_335),
    ( 0,  0,  3,  1,      1_107),
    ( 4,  0,  0, -1,      1_021),
    ( 4,  0, -1,  1,        833),
    ( 0,  0,  1, -3,        777),
    ( 4,  0, -2,  1,        671),
    ( 2,  0,  0, -3,        607),
    ( 2,  0,  2, -1,        596),
    ( 2, -1,  1, -1,        491),
    ( 2,  0, -2,  1,       -451),
    ( 0,  0,  3, -1,        439),
    ( 2,  0,  2,  1,        422),
    ( 2,  0, -3, -1,        421),
    ( 2,  1, -1,  1,       -366),
    ( 2,  1,  0,  1,       -351),
    ( 4,  0,  0,  1,        331),
    ( 2, -1,  1,  1,        315),
    ( 2, -2,  0, -1,        302),
    ( 0,  0,  1,  3,       -283),
    ( 2,  1,  1, -1,       -229),
    ( 1,  1,  0, -1,        223),
    ( 1,  1,  0,  1,        223),
    ( 0,  1, -2, -1,       -220),
    ( 2,  1, -1, -1,       -220),
    ( 1,  0,  1,  1,       -185),
    ( 2, -1, -2, -1,        181),
    ( 0,  1,  2,  1,       -177),
    ( 4,  0, -2, -1,        176),
    ( 4, -1, -1, -1,        166),
    ( 1,  0,  1, -1,       -164),
    ( 4,  0,  1, -1,        132),
    ( 1,  0, -1, -1,       -119),
    ( 4, -1,  0, -1,        115),
    ( 2, -2,  0,  1,        107),
];

// ---------------------------------------------------------------------------
// Eccentricity correction
// ---------------------------------------------------------------------------

/// Earth's orbital eccentricity correction factor (Meeus eq. 47.6).
fn eccentricity_correction(t: f64) -> f64 {
    1.0 - 0.002_516 * t - 0.000_007_4 * t * t
}

/// Compute the eccentricity power for a given M multiplier.
fn e_power(e: f64, m_mult: i32) -> f64 {
    match m_mult.unsigned_abs() {
        0 => 1.0,
        1 => e,
        2 => e * e,
        _ => e.powi(m_mult.unsigned_abs() as i32),
    }
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Compute the Moon's ecliptic longitude in degrees for a given Julian Date.
///
/// # Examples
///
/// ```
/// # use jyotish::moon::lunar_longitude;
/// // Meeus example 47.a: 1992-04-12 at 0h TD
/// // JD = 2448724.5
/// let lon = lunar_longitude(2_448_724.5);
/// assert!((lon - 133.162).abs() < 0.01, "got {lon}");
/// ```
pub fn lunar_longitude(jd: f64) -> f64 {
    let t = julian_centuries(jd);
    let lp = mean_longitude(t);
    let d = deg_to_rad(mean_elongation(t));
    let m = deg_to_rad(sun_mean_anomaly(t));
    let mp = deg_to_rad(moon_mean_anomaly(t));
    let f = deg_to_rad(argument_of_latitude(t));
    let e = eccentricity_correction(t);

    let mut sigma_l: f64 = 0.0;
    for &(d_m, m_m, mp_m, f_m, sin_coeff, _cos_coeff) in LONGITUDE_DISTANCE_TERMS {
        let arg = d_m as f64 * d + m_m as f64 * m + mp_m as f64 * mp + f_m as f64 * f;
        sigma_l += sin_coeff as f64 * e_power(e, m_m) * arg.sin();
    }

    // Additional correction terms (Meeus)
    let a1 = deg_to_rad(119.75 + 131.849 * t);
    let a2 = deg_to_rad(53.09 + 479_264.290 * t);
    sigma_l += 3958.0 * a1.sin() + 1962.0 * a2.sin() + 318.0 * deg_to_rad(lp).sin();

    // Convert from 0.000001 degrees to degrees and add to mean longitude
    normalize_degrees(lp + sigma_l / 1_000_000.0)
}

/// Compute the Moon's ecliptic latitude in degrees for a given Julian Date.
///
/// # Examples
///
/// ```
/// # use jyotish::moon::lunar_latitude;
/// let lat = lunar_latitude(2_448_724.5);
/// assert!((lat - (-3.229)).abs() < 0.01, "got {lat}");
/// ```
pub fn lunar_latitude(jd: f64) -> f64 {
    let t = julian_centuries(jd);
    let lp = mean_longitude(t);
    let d = deg_to_rad(mean_elongation(t));
    let m = deg_to_rad(sun_mean_anomaly(t));
    let mp = deg_to_rad(moon_mean_anomaly(t));
    let f = deg_to_rad(argument_of_latitude(t));
    let e = eccentricity_correction(t);

    let mut sigma_b: f64 = 0.0;
    for &(d_m, m_m, mp_m, f_m, sin_coeff) in LATITUDE_TERMS {
        let arg = d_m as f64 * d + m_m as f64 * m + mp_m as f64 * mp + f_m as f64 * f;
        sigma_b += sin_coeff as f64 * e_power(e, m_m) * arg.sin();
    }

    // Additional correction terms
    let a1 = deg_to_rad(119.75 + 131.849 * t);
    let a3 = deg_to_rad(313.45 + 481_266.484 * t);
    sigma_b += -2235.0 * deg_to_rad(lp).sin()
        + 382.0 * a3.sin()
        + 175.0 * (a1 - f).sin()
        + 175.0 * (a1 + f).sin()
        + 127.0 * (deg_to_rad(lp) - mp).sin()
        - 115.0 * (deg_to_rad(lp) + mp).sin();

    sigma_b / 1_000_000.0
}

/// Compute the Moon's distance from Earth in kilometers for a given Julian Date.
///
/// # Examples
///
/// ```
/// # use jyotish::moon::lunar_distance_km;
/// let dist = lunar_distance_km(2_448_724.5);
/// assert!((dist - 368_409.7).abs() < 5.0, "got {dist}");
/// ```
pub fn lunar_distance_km(jd: f64) -> f64 {
    let t = julian_centuries(jd);
    let d = deg_to_rad(mean_elongation(t));
    let m = deg_to_rad(sun_mean_anomaly(t));
    let mp = deg_to_rad(moon_mean_anomaly(t));
    let f = deg_to_rad(argument_of_latitude(t));
    let e = eccentricity_correction(t);

    let mut sigma_r: f64 = 0.0;
    for &(d_m, m_m, mp_m, f_m, _sin_coeff, cos_coeff) in LONGITUDE_DISTANCE_TERMS {
        let arg = d_m as f64 * d + m_m as f64 * m + mp_m as f64 * mp + f_m as f64 * f;
        sigma_r += cos_coeff as f64 * e_power(e, m_m) * arg.cos();
    }

    // Mean distance 385000.56 km + corrections (in 0.001 km)
    385_000.56 + sigma_r / 1000.0
}

/// Compute the Moon's distance from Earth in AU for a given Julian Date.
pub fn lunar_distance_au(jd: f64) -> f64 {
    lunar_distance_km(jd) / 149_597_870.7
}

/// Compute the Moon's position as a [`PlanetaryPosition`].
///
/// # Examples
///
/// ```
/// # use jyotish::moon::lunar_position;
/// # use jyotish::planet::Planet;
/// let pos = lunar_position(2_448_724.5);
/// assert_eq!(pos.planet, Planet::Moon);
/// ```
pub fn lunar_position(jd: f64) -> PlanetaryPosition {
    PlanetaryPosition::new(
        Planet::Moon,
        lunar_longitude(jd),
        lunar_latitude(jd),
        lunar_distance_au(jd),
        crate::calendar::jd_to_unix(jd),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    // Meeus example 47.a: 1992-04-12 at 0h TD, JD = 2448724.5
    const JD_MEEUS_47A: f64 = 2_448_724.5;

    #[test]
    fn lunar_longitude_meeus_47a() {
        let lon = lunar_longitude(JD_MEEUS_47A);
        // Meeus gives λ = 133.162°
        assert!((lon - 133.162).abs() < 0.02, "got {lon}");
    }

    #[test]
    fn lunar_latitude_meeus_47a() {
        let lat = lunar_latitude(JD_MEEUS_47A);
        // Meeus gives β = -3.229°
        assert!((lat - (-3.229)).abs() < 0.02, "got {lat}");
    }

    #[test]
    fn lunar_distance_meeus_47a() {
        let dist = lunar_distance_km(JD_MEEUS_47A);
        // Meeus gives Δ = 368409.7 km
        assert!((dist - 368_409.7).abs() < 10.0, "got {dist}");
    }

    #[test]
    fn lunar_position_struct() {
        let pos = lunar_position(JD_MEEUS_47A);
        assert_eq!(pos.planet, Planet::Moon);
        assert!(pos.longitude_deg >= 0.0 && pos.longitude_deg < 360.0);
        assert!(pos.latitude_deg.abs() < 6.0); // Moon's max latitude ~5.3°
    }

    #[test]
    fn lunar_longitude_range() {
        for day in 0..365 {
            let jd = 2_451_545.0 + day as f64;
            let lon = lunar_longitude(jd);
            assert!(
                (0.0..360.0).contains(&lon),
                "longitude {lon} out of range at day {day}"
            );
        }
    }

    #[test]
    fn lunar_distance_range() {
        // Moon distance should be between ~356,000 and ~407,000 km
        for day in 0..365 {
            let jd = 2_451_545.0 + day as f64;
            let dist = lunar_distance_km(jd);
            assert!(
                (350_000.0..=410_000.0).contains(&dist),
                "distance {dist} km out of range at day {day}"
            );
        }
    }
}
