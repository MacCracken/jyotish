//! VSOP87D coefficients for Mercury.
//!
//! Heliocentric ecliptic longitude (L), latitude (B), and radius vector (R)
//! referred to the J2000.0 ecliptic and equinox.
//!
//! Source: Bretagnon & Francou (1988), Bureau des Longitudes VSOP87D.mer.

use super::VsopTerm;

// ===========================================================================
// Longitude (L) series
// ===========================================================================

#[rustfmt::skip]
pub(super) const L0: &[VsopTerm] = &[
    VsopTerm { a: 440250710.0e-8, b: 0.00000000, c: 0.00000000 },
    VsopTerm { a: 40989415.0e-8, b: 1.48302034, c: 26087.90314157 },
    VsopTerm { a: 5046294.0e-8, b: 4.4778549, c: 52175.8062831 },
    VsopTerm { a: 855347.0e-8, b: 1.165203, c: 78263.709425 },
    VsopTerm { a: 165590.0e-8, b: 4.119692, c: 104351.612566 },
    VsopTerm { a: 34562.0e-8, b: 0.77931, c: 130439.51571 },
    VsopTerm { a: 7583.0e-8, b: 3.71348, c: 156527.41885 },
    VsopTerm { a: 3560.0e-8, b: 1.51203, c: 1109.09946 },
    VsopTerm { a: 1803.0e-8, b: 4.10333, c: 5661.33205 },
    VsopTerm { a: 1726.0e-8, b: 0.35832, c: 182615.32199 },
    VsopTerm { a: 1590.0e-8, b: 2.99510, c: 25028.52121 },
    VsopTerm { a: 1365.0e-8, b: 4.59918, c: 27197.28169 },
    VsopTerm { a: 1017.0e-8, b: 0.88031, c: 31749.23519 },
    VsopTerm { a: 714.0e-8, b: 1.5414, c: 24978.5246 },
    VsopTerm { a: 644.0e-8, b: 5.3033, c: 21535.9496 },
    VsopTerm { a: 451.0e-8, b: 6.0499, c: 51116.4244 },
    VsopTerm { a: 404.0e-8, b: 3.2824, c: 208703.2251 },
    VsopTerm { a: 352.0e-8, b: 5.2416, c: 20426.5711 },
    VsopTerm { a: 345.0e-8, b: 2.7921, c: 15874.6176 },
    VsopTerm { a: 343.0e-8, b: 5.7652, c: 955.5997 },
    VsopTerm { a: 339.0e-8, b: 5.8633, c: 25558.2122 },
    VsopTerm { a: 325.0e-8, b: 1.3367, c: 53285.1848 },
    VsopTerm { a: 273.0e-8, b: 2.4947, c: 529.6910 },
    VsopTerm { a: 264.0e-8, b: 3.9170, c: 57837.1383 },
    VsopTerm { a: 260.0e-8, b: 0.9874, c: 4551.9535 },
    VsopTerm { a: 239.0e-8, b: 0.1133, c: 1059.3819 },
    VsopTerm { a: 235.0e-8, b: 0.2668, c: 11322.6641 },
    VsopTerm { a: 217.0e-8, b: 0.6601, c: 13521.7514 },
    VsopTerm { a: 209.0e-8, b: 2.0918, c: 47623.8528 },
    VsopTerm { a: 183.0e-8, b: 2.6291, c: 27043.5033 },
    VsopTerm { a: 182.0e-8, b: 2.4344, c: 25661.3050 },
    VsopTerm { a: 176.0e-8, b: 4.5364, c: 51066.4277 },
    VsopTerm { a: 173.0e-8, b: 2.4522, c: 24498.8302 },
    VsopTerm { a: 142.0e-8, b: 3.3600, c: 37410.5672 },
    VsopTerm { a: 138.0e-8, b: 0.2911, c: 10213.2855 },
    VsopTerm { a: 125.0e-8, b: 3.7211, c: 39609.6546 },
    VsopTerm { a: 118.0e-8, b: 2.7813, c: 77204.3275 },
    VsopTerm { a: 106.0e-8, b: 4.2063, c: 19804.8273 },
    VsopTerm { a: 100.0e-8, b: 1.0413, c: 234791.1283 },
];

#[rustfmt::skip]
pub(super) const L1: &[VsopTerm] = &[
    VsopTerm { a: 2608814706223.0e-8, b: 0.00000000000, c: 0.00000000000 },
    VsopTerm { a: 7834132.0e-8, b: 6.1923372, c: 26087.9031416 },
    VsopTerm { a: 1803648.0e-8, b: 3.9990813, c: 52175.8062831 },
    VsopTerm { a: 378831.0e-8, b: 0.772188, c: 78263.709425 },
    VsopTerm { a: 81148.0e-8, b: 3.78586, c: 104351.61257 },
    VsopTerm { a: 17689.0e-8, b: 0.52983, c: 130439.51571 },
    VsopTerm { a: 4028.0e-8, b: 3.56415, c: 156527.41885 },
    VsopTerm { a: 961.0e-8, b: 0.3293, c: 182615.3220 },
    VsopTerm { a: 235.0e-8, b: 3.3627, c: 208703.2251 },
    VsopTerm { a: 152.0e-8, b: 4.3254, c: 27197.2817 },
    VsopTerm { a: 98.0e-8, b: 4.202, c: 25028.521 },
    VsopTerm { a: 81.0e-8, b: 5.927, c: 31749.235 },
    VsopTerm { a: 71.0e-8, b: 0.672, c: 24978.525 },
    VsopTerm { a: 59.0e-8, b: 0.1253, c: 234791.1283 },
    VsopTerm { a: 57.0e-8, b: 5.0197, c: 51116.4244 },
    VsopTerm { a: 51.0e-8, b: 3.2968, c: 21535.9496 },
    VsopTerm { a: 49.0e-8, b: 1.4755, c: 1109.0995 },
    VsopTerm { a: 43.0e-8, b: 1.0466, c: 53285.1848 },
    VsopTerm { a: 39.0e-8, b: 5.8377, c: 5661.3321 },
    VsopTerm { a: 37.0e-8, b: 4.4838, c: 57837.1383 },
];

#[rustfmt::skip]
pub(super) const L2: &[VsopTerm] = &[
    VsopTerm { a: 53050.0e-8, b: 0.00000, c: 0.00000 },
    VsopTerm { a: 16904.0e-8, b: 4.69072, c: 26087.90314 },
    VsopTerm { a: 7397.0e-8, b: 1.3474, c: 52175.8063 },
    VsopTerm { a: 3018.0e-8, b: 4.4564, c: 78263.7094 },
    VsopTerm { a: 1107.0e-8, b: 1.2642, c: 104351.6126 },
    VsopTerm { a: 378.0e-8, b: 4.3200, c: 130439.5157 },
    VsopTerm { a: 123.0e-8, b: 1.0688, c: 156527.4189 },
    VsopTerm { a: 39.0e-8, b: 4.08, c: 182615.32 },
    VsopTerm { a: 15.0e-8, b: 4.63, c: 27197.28 },
    VsopTerm { a: 12.0e-8, b: 0.79, c: 208703.23 },
];

#[rustfmt::skip]
pub(super) const L3: &[VsopTerm] = &[
    VsopTerm { a: 188.0e-8, b: 0.035, c: 52175.806 },
    VsopTerm { a: 142.0e-8, b: 3.125, c: 26087.903 },
    VsopTerm { a: 97.0e-8, b: 3.00, c: 78263.709 },
    VsopTerm { a: 44.0e-8, b: 6.02, c: 104351.613 },
    VsopTerm { a: 35.0e-8, b: 0.00, c: 0.00 },
    VsopTerm { a: 18.0e-8, b: 2.78, c: 130439.516 },
    VsopTerm { a: 7.0e-8, b: 5.82, c: 156527.419 },
    VsopTerm { a: 3.0e-8, b: 2.57, c: 182615.322 },
];

#[rustfmt::skip]
pub(super) const L4: &[VsopTerm] = &[
    VsopTerm { a: 114.0e-8, b: 3.1416, c: 0.0000 },
    VsopTerm { a: 3.0e-8, b: 2.03, c: 26087.90 },
    VsopTerm { a: 2.0e-8, b: 1.42, c: 78263.71 },
    VsopTerm { a: 2.0e-8, b: 4.50, c: 52175.81 },
    VsopTerm { a: 1.0e-8, b: 4.50, c: 104351.61 },
];

#[rustfmt::skip]
pub(super) const L5: &[VsopTerm] = &[
    VsopTerm { a: 1.0e-8, b: 3.14, c: 0.00 },
];

/// Longitude sub-series references.
pub(super) const L_SERIES: [&[VsopTerm]; 6] = [L0, L1, L2, L3, L4, L5];

// ===========================================================================
// Latitude (B) series
// ===========================================================================

#[rustfmt::skip]
pub(super) const B0: &[VsopTerm] = &[
    VsopTerm { a: 11737529.0e-8, b: 1.98357499, c: 26087.90314157 },
    VsopTerm { a: 2388077.0e-8, b: 5.0373896, c: 52175.8062831 },
    VsopTerm { a: 1222840.0e-8, b: 3.1415927, c: 0.0000000 },
    VsopTerm { a: 543252.0e-8, b: 1.796444, c: 78263.709425 },
    VsopTerm { a: 129779.0e-8, b: 4.832325, c: 104351.612566 },
    VsopTerm { a: 31867.0e-8, b: 1.58088, c: 130439.51571 },
    VsopTerm { a: 7963.0e-8, b: 4.60972, c: 156527.41885 },
    VsopTerm { a: 2014.0e-8, b: 1.35324, c: 182615.32199 },
    VsopTerm { a: 514.0e-8, b: 4.37835, c: 208703.22514 },
    VsopTerm { a: 209.0e-8, b: 2.0202, c: 24978.5246 },
    VsopTerm { a: 208.0e-8, b: 4.9185, c: 27197.2817 },
    VsopTerm { a: 132.0e-8, b: 1.1191, c: 234791.1283 },
    VsopTerm { a: 121.0e-8, b: 1.8131, c: 53285.1848 },
    VsopTerm { a: 100.0e-8, b: 5.6572, c: 20426.5711 },
    VsopTerm { a: 94.0e-8, b: 2.8228, c: 25028.5212 },
    VsopTerm { a: 80.0e-8, b: 1.6258, c: 51116.4244 },
    VsopTerm { a: 72.0e-8, b: 3.5590, c: 529.6910 },
    VsopTerm { a: 63.0e-8, b: 2.0568, c: 4551.9535 },
    VsopTerm { a: 60.0e-8, b: 4.1243, c: 21535.9496 },
    VsopTerm { a: 56.0e-8, b: 0.8381, c: 31749.2352 },
];

#[rustfmt::skip]
pub(super) const B1: &[VsopTerm] = &[
    VsopTerm { a: 429151.0e-8, b: 3.501698, c: 26087.903142 },
    VsopTerm { a: 146234.0e-8, b: 3.141593, c: 0.000000 },
    VsopTerm { a: 22675.0e-8, b: 0.01515, c: 52175.80628 },
    VsopTerm { a: 10895.0e-8, b: 0.48540, c: 78263.70942 },
    VsopTerm { a: 6353.0e-8, b: 3.42943, c: 104351.61257 },
    VsopTerm { a: 2496.0e-8, b: 0.16051, c: 130439.51571 },
    VsopTerm { a: 860.0e-8, b: 3.18453, c: 156527.41885 },
    VsopTerm { a: 278.0e-8, b: 6.21021, c: 182615.32199 },
    VsopTerm { a: 86.0e-8, b: 2.95330, c: 208703.22514 },
    VsopTerm { a: 26.0e-8, b: 6.00, c: 234791.13 },
    VsopTerm { a: 14.0e-8, b: 1.75, c: 27197.28 },
    VsopTerm { a: 12.0e-8, b: 5.40, c: 53285.18 },
];

#[rustfmt::skip]
pub(super) const B2: &[VsopTerm] = &[
    VsopTerm { a: 11831.0e-8, b: 4.79066, c: 26087.90314 },
    VsopTerm { a: 4518.0e-8, b: 4.24327, c: 52175.80628 },
    VsopTerm { a: 1554.0e-8, b: 0.00000, c: 0.00000 },
    VsopTerm { a: 1465.0e-8, b: 0.78899, c: 78263.70942 },
    VsopTerm { a: 618.0e-8, b: 3.82461, c: 104351.61257 },
    VsopTerm { a: 227.0e-8, b: 0.54553, c: 130439.51571 },
    VsopTerm { a: 77.0e-8, b: 3.57, c: 156527.42 },
    VsopTerm { a: 25.0e-8, b: 0.37, c: 182615.32 },
    VsopTerm { a: 8.0e-8, b: 3.41, c: 208703.23 },
];

#[rustfmt::skip]
pub(super) const B3: &[VsopTerm] = &[
    VsopTerm { a: 235.0e-8, b: 0.354, c: 26087.903 },
    VsopTerm { a: 161.0e-8, b: 0.506, c: 52175.806 },
    VsopTerm { a: 90.0e-8, b: 1.084, c: 78263.709 },
    VsopTerm { a: 44.0e-8, b: 4.163, c: 104351.613 },
    VsopTerm { a: 19.0e-8, b: 0.92, c: 130439.52 },
    VsopTerm { a: 7.0e-8, b: 4.04, c: 156527.42 },
    VsopTerm { a: 4.0e-8, b: 3.14, c: 0.00 },
];

#[rustfmt::skip]
pub(super) const B4: &[VsopTerm] = &[
    VsopTerm { a: 4.0e-8, b: 1.75, c: 26087.90 },
    VsopTerm { a: 4.0e-8, b: 2.00, c: 52175.81 },
    VsopTerm { a: 3.0e-8, b: 5.26, c: 78263.71 },
    VsopTerm { a: 1.0e-8, b: 2.10, c: 104351.61 },
];

/// Latitude sub-series references.
pub(super) const B_SERIES: [&[VsopTerm]; 5] = [B0, B1, B2, B3, B4];

// ===========================================================================
// Radius vector (R) series
// ===========================================================================

#[rustfmt::skip]
pub(super) const R0: &[VsopTerm] = &[
    VsopTerm { a: 39528272.0e-8, b: 0.00000000, c: 0.00000000 },
    VsopTerm { a: 7834132.0e-8, b: 6.19233723, c: 26087.90314157 },
    VsopTerm { a: 795526.0e-8, b: 2.959897, c: 52175.806283 },
    VsopTerm { a: 121282.0e-8, b: 6.010642, c: 78263.709425 },
    VsopTerm { a: 21922.0e-8, b: 2.77820, c: 104351.61257 },
    VsopTerm { a: 4354.0e-8, b: 5.82894, c: 130439.51571 },
    VsopTerm { a: 918.0e-8, b: 2.5970, c: 156527.4189 },
    VsopTerm { a: 290.0e-8, b: 1.4245, c: 25028.5212 },
    VsopTerm { a: 260.0e-8, b: 3.0282, c: 27197.2817 },
    VsopTerm { a: 202.0e-8, b: 5.6473, c: 182615.3220 },
    VsopTerm { a: 201.0e-8, b: 5.5924, c: 31749.2352 },
    VsopTerm { a: 142.0e-8, b: 6.2530, c: 24978.5246 },
    VsopTerm { a: 100.0e-8, b: 3.7340, c: 21535.9496 },
    VsopTerm { a: 77.0e-8, b: 1.6391, c: 20426.5711 },
    VsopTerm { a: 61.0e-8, b: 4.4927, c: 51116.4244 },
    VsopTerm { a: 49.0e-8, b: 2.3905, c: 208703.2251 },
    VsopTerm { a: 47.0e-8, b: 4.5760, c: 5661.3321 },
    VsopTerm { a: 46.0e-8, b: 0.5312, c: 1109.0995 },
    VsopTerm { a: 40.0e-8, b: 2.7201, c: 53285.1848 },
    VsopTerm { a: 38.0e-8, b: 3.9810, c: 25558.2122 },
    VsopTerm { a: 37.0e-8, b: 5.1025, c: 57837.1383 },
    VsopTerm { a: 34.0e-8, b: 4.2030, c: 47623.8528 },
    VsopTerm { a: 33.0e-8, b: 0.6465, c: 15874.6176 },
    VsopTerm { a: 28.0e-8, b: 5.3900, c: 955.5997 },
    VsopTerm { a: 26.0e-8, b: 0.9043, c: 529.6910 },
];

#[rustfmt::skip]
pub(super) const R1: &[VsopTerm] = &[
    VsopTerm { a: 217348.0e-8, b: 4.656172, c: 26087.903142 },
    VsopTerm { a: 44142.0e-8, b: 1.42386, c: 52175.80628 },
    VsopTerm { a: 10094.0e-8, b: 4.47466, c: 78263.70942 },
    VsopTerm { a: 2433.0e-8, b: 1.24226, c: 104351.61257 },
    VsopTerm { a: 1624.0e-8, b: 0.00000, c: 0.00000 },
    VsopTerm { a: 604.0e-8, b: 4.29303, c: 130439.51571 },
    VsopTerm { a: 153.0e-8, b: 1.06064, c: 156527.41885 },
    VsopTerm { a: 39.0e-8, b: 4.11, c: 182615.32 },
    VsopTerm { a: 15.0e-8, b: 4.30, c: 27197.28 },
    VsopTerm { a: 14.0e-8, b: 2.67, c: 25028.52 },
    VsopTerm { a: 10.0e-8, b: 0.86, c: 208703.23 },
];

#[rustfmt::skip]
pub(super) const R2: &[VsopTerm] = &[
    VsopTerm { a: 3118.0e-8, b: 3.0823, c: 26087.9031 },
    VsopTerm { a: 1245.0e-8, b: 6.1518, c: 52175.8063 },
    VsopTerm { a: 425.0e-8, b: 2.9258, c: 78263.7094 },
    VsopTerm { a: 136.0e-8, b: 5.9799, c: 104351.6126 },
    VsopTerm { a: 42.0e-8, b: 2.7494, c: 130439.5157 },
    VsopTerm { a: 22.0e-8, b: 3.1416, c: 0.0000 },
    VsopTerm { a: 13.0e-8, b: 5.80, c: 156527.42 },
    VsopTerm { a: 4.0e-8, b: 2.58, c: 182615.32 },
];

#[rustfmt::skip]
pub(super) const R3: &[VsopTerm] = &[
    VsopTerm { a: 33.0e-8, b: 1.68, c: 26087.90 },
    VsopTerm { a: 24.0e-8, b: 4.63, c: 52175.81 },
    VsopTerm { a: 12.0e-8, b: 1.39, c: 78263.71 },
    VsopTerm { a: 5.0e-8, b: 4.44, c: 104351.61 },
    VsopTerm { a: 2.0e-8, b: 1.21, c: 130439.52 },
];

/// Radius sub-series references.
pub(super) const R_SERIES: [&[VsopTerm]; 4] = [R0, R1, R2, R3];
