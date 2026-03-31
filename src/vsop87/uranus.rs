//! VSOP87D coefficients for Uranus.
//!
//! Heliocentric ecliptic longitude (L), latitude (B), and radius vector (R)
//! referred to the J2000.0 ecliptic and equinox.
//!
//! Source: Bretagnon & Francou (1988), Bureau des Longitudes VSOP87D.ura.

use super::VsopTerm;

// ===========================================================================
// Longitude (L) series
// ===========================================================================

#[rustfmt::skip]
pub(super) const L0: &[VsopTerm] = &[
    VsopTerm { a: 548129294.0e-8, b: 0.0000000, c: 0.0000000 },
    VsopTerm { a: 9260408.0e-8, b: 0.8910642, c: 74.7815986 },
    VsopTerm { a: 1504248.0e-8, b: 3.6271926, c: 1.4844727 },
    VsopTerm { a: 365982.0e-8, b: 1.899622, c: 73.297126 },
    VsopTerm { a: 272328.0e-8, b: 3.358237, c: 149.563197 },
    VsopTerm { a: 70328.0e-8, b: 5.39254, c: 63.73590 },
    VsopTerm { a: 68893.0e-8, b: 6.09292, c: 76.26607 },
    VsopTerm { a: 61999.0e-8, b: 2.26952, c: 2.96895 },
    VsopTerm { a: 61951.0e-8, b: 2.85099, c: 11.04570 },
    VsopTerm { a: 26469.0e-8, b: 3.14152, c: 71.81265 },
    VsopTerm { a: 25711.0e-8, b: 6.11380, c: 454.90937 },
    VsopTerm { a: 21079.0e-8, b: 4.36059, c: 148.07872 },
    VsopTerm { a: 17819.0e-8, b: 1.74437, c: 36.64856 },
    VsopTerm { a: 14613.0e-8, b: 4.73732, c: 3.93215 },
    VsopTerm { a: 11163.0e-8, b: 5.82682, c: 224.34480 },
    VsopTerm { a: 10998.0e-8, b: 0.48865, c: 138.51750 },
    VsopTerm { a: 9527.0e-8, b: 2.9552, c: 35.1641 },
    VsopTerm { a: 7546.0e-8, b: 5.2363, c: 109.9457 },
    VsopTerm { a: 4220.0e-8, b: 3.2333, c: 70.8494 },
    VsopTerm { a: 4052.0e-8, b: 2.2775, c: 151.0477 },
    VsopTerm { a: 3490.0e-8, b: 5.4831, c: 146.5943 },
    VsopTerm { a: 3355.0e-8, b: 1.0655, c: 4.4534 },
    VsopTerm { a: 3144.0e-8, b: 4.7520, c: 77.7505 },
    VsopTerm { a: 2927.0e-8, b: 4.6290, c: 9.5612 },
    VsopTerm { a: 2922.0e-8, b: 5.3524, c: 85.8273 },
    VsopTerm { a: 2273.0e-8, b: 4.3660, c: 70.3282 },
    VsopTerm { a: 2149.0e-8, b: 0.6075, c: 38.1330 },
    VsopTerm { a: 2051.0e-8, b: 1.5177, c: 0.1119 },
    VsopTerm { a: 1992.0e-8, b: 4.9244, c: 277.0350 },
    VsopTerm { a: 1667.0e-8, b: 3.6274, c: 380.1278 },
];

#[rustfmt::skip]
pub(super) const L1: &[VsopTerm] = &[
    VsopTerm { a: 7478166163.0e-8, b: 0.0000000000, c: 0.0000000000 },
    VsopTerm { a: 68346.0e-8, b: 4.66926, c: 74.78160 },
    VsopTerm { a: 24594.0e-8, b: 0.14280, c: 1.48447 },
    VsopTerm { a: 11806.0e-8, b: 5.93706, c: 149.56320 },
    VsopTerm { a: 9523.0e-8, b: 3.5493, c: 73.2971 },
    VsopTerm { a: 3907.0e-8, b: 5.4488, c: 63.7359 },
    VsopTerm { a: 2762.0e-8, b: 3.5585, c: 76.2661 },
    VsopTerm { a: 1820.0e-8, b: 4.9001, c: 2.9689 },
    VsopTerm { a: 1726.0e-8, b: 0.3535, c: 11.0457 },
    VsopTerm { a: 706.0e-8, b: 3.039, c: 454.909 },
];

#[rustfmt::skip]
pub(super) const L2: &[VsopTerm] = &[
    VsopTerm { a: 53033.0e-8, b: 0.00000, c: 0.00000 },
    VsopTerm { a: 2358.0e-8, b: 2.2601, c: 74.7816 },
    VsopTerm { a: 769.0e-8, b: 4.526, c: 1.484 },
    VsopTerm { a: 552.0e-8, b: 3.258, c: 149.563 },
    VsopTerm { a: 542.0e-8, b: 2.276, c: 73.297 },
    VsopTerm { a: 529.0e-8, b: 4.923, c: 63.736 },
    VsopTerm { a: 258.0e-8, b: 3.691, c: 76.266 },
    VsopTerm { a: 239.0e-8, b: 5.858, c: 2.969 },
    VsopTerm { a: 182.0e-8, b: 6.218, c: 11.046 },
];

#[rustfmt::skip]
pub(super) const L3: &[VsopTerm] = &[
    VsopTerm { a: 121.0e-8, b: 0.024, c: 74.782 },
    VsopTerm { a: 68.0e-8, b: 4.12, c: 3.93 },
    VsopTerm { a: 53.0e-8, b: 2.39, c: 11.05 },
    VsopTerm { a: 46.0e-8, b: 0.00, c: 0.00 },
    VsopTerm { a: 45.0e-8, b: 2.04, c: 149.56 },
    VsopTerm { a: 44.0e-8, b: 2.96, c: 1.48 },
];

#[rustfmt::skip]
pub(super) const L4: &[VsopTerm] = &[
    VsopTerm { a: 114.0e-8, b: 3.1416, c: 0.0000 },
    VsopTerm { a: 6.0e-8, b: 4.58, c: 74.78 },
    VsopTerm { a: 3.0e-8, b: 0.35, c: 149.56 },
];

/// Longitude sub-series references.
pub(super) const L_SERIES: [&[VsopTerm]; 5] = [L0, L1, L2, L3, L4];

// ===========================================================================
// Latitude (B) series
// ===========================================================================

#[rustfmt::skip]
pub(super) const B0: &[VsopTerm] = &[
    VsopTerm { a: 1346278.0e-8, b: 2.6187127, c: 74.7815986 },
    VsopTerm { a: 42643.0e-8, b: 5.71050, c: 149.56320 },
    VsopTerm { a: 18656.0e-8, b: 3.90404, c: 73.29713 },
    VsopTerm { a: 14099.0e-8, b: 3.14159, c: 0.00000 },
    VsopTerm { a: 6502.0e-8, b: 2.7863, c: 76.2661 },
    VsopTerm { a: 4633.0e-8, b: 0.0267, c: 148.0787 },
    VsopTerm { a: 3792.0e-8, b: 1.9829, c: 63.7359 },
    VsopTerm { a: 2394.0e-8, b: 1.1208, c: 224.3448 },
    VsopTerm { a: 1933.0e-8, b: 0.1738, c: 71.8127 },
    VsopTerm { a: 1332.0e-8, b: 2.2707, c: 109.9457 },
    VsopTerm { a: 1081.0e-8, b: 0.2143, c: 151.0477 },
    VsopTerm { a: 998.0e-8, b: 4.170, c: 1.484 },
    VsopTerm { a: 917.0e-8, b: 4.401, c: 146.594 },
];

#[rustfmt::skip]
pub(super) const B1: &[VsopTerm] = &[
    VsopTerm { a: 206366.0e-8, b: 4.123943, c: 74.781599 },
    VsopTerm { a: 8563.0e-8, b: 0.3382, c: 149.5632 },
    VsopTerm { a: 1726.0e-8, b: 2.1219, c: 73.2971 },
    VsopTerm { a: 1374.0e-8, b: 0.0000, c: 0.0000 },
    VsopTerm { a: 1369.0e-8, b: 3.0686, c: 76.2661 },
    VsopTerm { a: 451.0e-8, b: 3.777, c: 1.484 },
    VsopTerm { a: 400.0e-8, b: 2.848, c: 224.345 },
    VsopTerm { a: 307.0e-8, b: 1.255, c: 148.079 },
    VsopTerm { a: 298.0e-8, b: 3.775, c: 63.736 },
    VsopTerm { a: 252.0e-8, b: 5.782, c: 71.813 },
];

#[rustfmt::skip]
pub(super) const B2: &[VsopTerm] = &[
    VsopTerm { a: 9212.0e-8, b: 5.8004, c: 74.7816 },
    VsopTerm { a: 557.0e-8, b: 0.000, c: 0.000 },
    VsopTerm { a: 286.0e-8, b: 2.177, c: 149.563 },
    VsopTerm { a: 95.0e-8, b: 3.84, c: 73.30 },
    VsopTerm { a: 45.0e-8, b: 4.88, c: 76.27 },
];

#[rustfmt::skip]
pub(super) const B3: &[VsopTerm] = &[
    VsopTerm { a: 393.0e-8, b: 1.204, c: 74.782 },
    VsopTerm { a: 16.0e-8, b: 3.14, c: 0.00 },
    VsopTerm { a: 12.0e-8, b: 3.88, c: 149.56 },
];

/// Latitude sub-series references.
pub(super) const B_SERIES: [&[VsopTerm]; 4] = [B0, B1, B2, B3];

// ===========================================================================
// Radius vector (R) series
// ===========================================================================

#[rustfmt::skip]
pub(super) const R0: &[VsopTerm] = &[
    VsopTerm { a: 1921264848.0e-8, b: 0.0000000, c: 0.0000000 },
    VsopTerm { a: 88784984.0e-8, b: 5.60377527, c: 74.78159857 },
    VsopTerm { a: 3340687.0e-8, b: 3.2828637, c: 149.5631971 },
    VsopTerm { a: 2173375.0e-8, b: 0.0000000, c: 0.0000000 },
    VsopTerm { a: 1296855.0e-8, b: 3.6720456, c: 1.4844727 },
    VsopTerm { a: 700428.0e-8, b: 3.303133, c: 73.297126 },
    VsopTerm { a: 564798.0e-8, b: 0.104502, c: 63.735898 },
    VsopTerm { a: 496561.0e-8, b: 1.401400, c: 76.266071 },
    VsopTerm { a: 482096.0e-8, b: 2.984040, c: 2.968945 },
    VsopTerm { a: 432339.0e-8, b: 3.381440, c: 11.045700 },
    VsopTerm { a: 344860.0e-8, b: 5.851600, c: 148.078724 },
    VsopTerm { a: 295186.0e-8, b: 0.821400, c: 224.344796 },
    VsopTerm { a: 225458.0e-8, b: 1.537300, c: 71.812653 },
    VsopTerm { a: 191229.0e-8, b: 4.892200, c: 454.909367 },
    VsopTerm { a: 167953.0e-8, b: 1.627500, c: 138.517497 },
    VsopTerm { a: 151048.0e-8, b: 3.363100, c: 109.945689 },
    VsopTerm { a: 102746.0e-8, b: 3.031400, c: 36.648563 },
    VsopTerm { a: 87310.0e-8, b: 4.16990, c: 35.16409 },
    VsopTerm { a: 85284.0e-8, b: 0.04720, c: 151.04767 },
    VsopTerm { a: 82434.0e-8, b: 1.01030, c: 70.84945 },
    VsopTerm { a: 77244.0e-8, b: 2.46420, c: 85.82730 },
    VsopTerm { a: 73249.0e-8, b: 2.51620, c: 146.59434 },
    VsopTerm { a: 64976.0e-8, b: 5.20890, c: 277.03499 },
    VsopTerm { a: 56779.0e-8, b: 0.39760, c: 4.45342 },
    VsopTerm { a: 55888.0e-8, b: 3.58850, c: 9.56123 },
    VsopTerm { a: 48764.0e-8, b: 1.30500, c: 77.75054 },
    VsopTerm { a: 42449.0e-8, b: 3.99430, c: 3.93215 },
    VsopTerm { a: 41480.0e-8, b: 4.16380, c: 70.32818 },
    VsopTerm { a: 40820.0e-8, b: 1.55280, c: 0.11187 },
    VsopTerm { a: 35834.0e-8, b: 2.49930, c: 38.13304 },
    VsopTerm { a: 33601.0e-8, b: 0.45740, c: 380.12777 },
];

#[rustfmt::skip]
pub(super) const R1: &[VsopTerm] = &[
    VsopTerm { a: 1479897.0e-8, b: 3.6719405, c: 74.7815986 },
    VsopTerm { a: 71212.0e-8, b: 6.22601, c: 149.56320 },
    VsopTerm { a: 68627.0e-8, b: 6.13411, c: 73.29713 },
    VsopTerm { a: 24060.0e-8, b: 3.14159, c: 0.00000 },
    VsopTerm { a: 21063.0e-8, b: 1.17385, c: 1.48447 },
    VsopTerm { a: 6198.0e-8, b: 2.5981, c: 63.7359 },
    VsopTerm { a: 4825.0e-8, b: 0.1655, c: 76.2661 },
    VsopTerm { a: 3609.0e-8, b: 5.7764, c: 2.9689 },
    VsopTerm { a: 3549.0e-8, b: 5.2426, c: 11.0457 },
    VsopTerm { a: 3484.0e-8, b: 4.2893, c: 148.0787 },
    VsopTerm { a: 2751.0e-8, b: 4.5017, c: 224.3448 },
    VsopTerm { a: 2319.0e-8, b: 3.0435, c: 454.9094 },
    VsopTerm { a: 2254.0e-8, b: 0.2498, c: 71.8127 },
    VsopTerm { a: 2160.0e-8, b: 4.3868, c: 138.5175 },
    VsopTerm { a: 2023.0e-8, b: 3.7853, c: 109.9457 },
    VsopTerm { a: 1307.0e-8, b: 1.2601, c: 36.6486 },
];

#[rustfmt::skip]
pub(super) const R2: &[VsopTerm] = &[
    VsopTerm { a: 22440.0e-8, b: 0.69953, c: 74.78160 },
    VsopTerm { a: 4728.0e-8, b: 1.6990, c: 73.2971 },
    VsopTerm { a: 1682.0e-8, b: 4.6483, c: 149.5632 },
    VsopTerm { a: 927.0e-8, b: 4.529, c: 1.484 },
    VsopTerm { a: 697.0e-8, b: 0.000, c: 0.000 },
    VsopTerm { a: 551.0e-8, b: 2.726, c: 63.736 },
    VsopTerm { a: 352.0e-8, b: 5.119, c: 76.266 },
];

#[rustfmt::skip]
pub(super) const R3: &[VsopTerm] = &[
    VsopTerm { a: 1164.0e-8, b: 4.7345, c: 74.7816 },
    VsopTerm { a: 212.0e-8, b: 3.343, c: 73.297 },
    VsopTerm { a: 196.0e-8, b: 2.980, c: 1.484 },
    VsopTerm { a: 105.0e-8, b: 0.958, c: 149.563 },
    VsopTerm { a: 73.0e-8, b: 1.00, c: 63.74 },
    VsopTerm { a: 26.0e-8, b: 3.14, c: 0.00 },
];

/// Radius sub-series references.
pub(super) const R_SERIES: [&[VsopTerm]; 4] = [R0, R1, R2, R3];
