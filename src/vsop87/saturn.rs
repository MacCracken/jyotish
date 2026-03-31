//! VSOP87D coefficients for Saturn.
//!
//! Heliocentric ecliptic longitude (L), latitude (B), and radius vector (R)
//! referred to the J2000.0 ecliptic and equinox.
//!
//! Source: Bretagnon & Francou (1988), Bureau des Longitudes VSOP87D.sat.

use super::VsopTerm;

// ===========================================================================
// Longitude (L) series
// ===========================================================================

#[rustfmt::skip]
pub(super) const L0: &[VsopTerm] = &[
    VsopTerm { a: 87401354.0e-8, b: 0.0000000, c: 0.0000000 },
    VsopTerm { a: 11107660.0e-8, b: 3.9620509, c: 213.2990954 },
    VsopTerm { a: 1414151.0e-8, b: 4.5858152, c: 7.1135470 },
    VsopTerm { a: 398379.0e-8, b: 0.52112, c: 206.18555 },
    VsopTerm { a: 350769.0e-8, b: 3.30330, c: 426.59819 },
    VsopTerm { a: 206816.0e-8, b: 0.24658, c: 103.09277 },
    VsopTerm { a: 79271.0e-8, b: 3.84007, c: 220.41264 },
    VsopTerm { a: 23990.0e-8, b: 4.66977, c: 110.20632 },
    VsopTerm { a: 16574.0e-8, b: 0.43719, c: 419.48464 },
    VsopTerm { a: 15820.0e-8, b: 0.93809, c: 632.78374 },
    VsopTerm { a: 15054.0e-8, b: 2.71670, c: 639.89729 },
    VsopTerm { a: 14907.0e-8, b: 5.76904, c: 316.39187 },
    VsopTerm { a: 14610.0e-8, b: 1.56519, c: 3.93215 },
    VsopTerm { a: 13160.0e-8, b: 4.44891, c: 14.22709 },
    VsopTerm { a: 13005.0e-8, b: 5.98119, c: 11.04570 },
    VsopTerm { a: 10725.0e-8, b: 3.12940, c: 202.25340 },
    VsopTerm { a: 6126.0e-8, b: 1.7633, c: 277.0350 },
    VsopTerm { a: 5863.0e-8, b: 0.2366, c: 529.6910 },
    VsopTerm { a: 5228.0e-8, b: 4.2078, c: 3.1814 },
    VsopTerm { a: 5020.0e-8, b: 3.1779, c: 433.7117 },
    VsopTerm { a: 4593.0e-8, b: 0.6198, c: 199.0720 },
    VsopTerm { a: 4006.0e-8, b: 2.2448, c: 63.7359 },
    VsopTerm { a: 3874.0e-8, b: 3.2228, c: 138.5175 },
    VsopTerm { a: 3269.0e-8, b: 0.7749, c: 949.1756 },
    VsopTerm { a: 2954.0e-8, b: 0.9828, c: 95.9792 },
    VsopTerm { a: 2461.0e-8, b: 2.0316, c: 735.8765 },
    VsopTerm { a: 1758.0e-8, b: 3.2658, c: 522.5774 },
    VsopTerm { a: 1640.0e-8, b: 5.5050, c: 846.0828 },
    VsopTerm { a: 1581.0e-8, b: 4.3727, c: 309.2783 },
    VsopTerm { a: 1391.0e-8, b: 4.0233, c: 323.5054 },
    VsopTerm { a: 1124.0e-8, b: 2.8373, c: 415.5525 },
    VsopTerm { a: 1087.0e-8, b: 4.1834, c: 2.4477 },
    VsopTerm { a: 1017.0e-8, b: 3.7170, c: 227.5262 },
    VsopTerm { a: 957.0e-8, b: 0.507, c: 1265.567 },
    VsopTerm { a: 853.0e-8, b: 3.421, c: 175.166 },
    VsopTerm { a: 849.0e-8, b: 3.191, c: 209.367 },
    VsopTerm { a: 789.0e-8, b: 5.007, c: 0.963 },
    VsopTerm { a: 749.0e-8, b: 2.144, c: 853.196 },
    VsopTerm { a: 744.0e-8, b: 5.253, c: 224.345 },
    VsopTerm { a: 687.0e-8, b: 1.747, c: 1052.268 },
    VsopTerm { a: 654.0e-8, b: 1.599, c: 0.048 },
    VsopTerm { a: 634.0e-8, b: 2.299, c: 412.371 },
    VsopTerm { a: 625.0e-8, b: 0.970, c: 210.118 },
    VsopTerm { a: 580.0e-8, b: 3.093, c: 74.782 },
    VsopTerm { a: 546.0e-8, b: 2.127, c: 350.332 },
    VsopTerm { a: 543.0e-8, b: 1.518, c: 9.561 },
    VsopTerm { a: 530.0e-8, b: 4.449, c: 117.320 },
    VsopTerm { a: 478.0e-8, b: 2.965, c: 137.033 },
    VsopTerm { a: 474.0e-8, b: 5.475, c: 742.990 },
    VsopTerm { a: 452.0e-8, b: 1.044, c: 490.334 },
];

#[rustfmt::skip]
pub(super) const L1: &[VsopTerm] = &[
    VsopTerm { a: 21354295596.0e-8, b: 0.0000000000, c: 0.0000000 },
    VsopTerm { a: 1296855.0e-8, b: 1.8282054, c: 213.2990954 },
    VsopTerm { a: 564348.0e-8, b: 2.88500, c: 7.11355 },
    VsopTerm { a: 107679.0e-8, b: 2.27770, c: 206.18555 },
    VsopTerm { a: 98323.0e-8, b: 1.08070, c: 426.59819 },
    VsopTerm { a: 40255.0e-8, b: 2.04128, c: 220.41264 },
    VsopTerm { a: 19942.0e-8, b: 1.27955, c: 103.09277 },
    VsopTerm { a: 10512.0e-8, b: 2.74880, c: 14.22709 },
    VsopTerm { a: 6939.0e-8, b: 0.4049, c: 639.8973 },
    VsopTerm { a: 4803.0e-8, b: 2.4419, c: 419.4846 },
    VsopTerm { a: 4056.0e-8, b: 2.9217, c: 110.2063 },
    VsopTerm { a: 3769.0e-8, b: 3.6497, c: 3.9322 },
    VsopTerm { a: 3385.0e-8, b: 2.4169, c: 3.1814 },
    VsopTerm { a: 3302.0e-8, b: 1.2626, c: 433.7117 },
    VsopTerm { a: 3071.0e-8, b: 2.3274, c: 199.0720 },
    VsopTerm { a: 1953.0e-8, b: 3.5639, c: 11.0457 },
    VsopTerm { a: 1249.0e-8, b: 2.6280, c: 95.9792 },
    VsopTerm { a: 922.0e-8, b: 1.961, c: 227.526 },
    VsopTerm { a: 706.0e-8, b: 4.417, c: 529.691 },
    VsopTerm { a: 650.0e-8, b: 6.174, c: 202.253 },
];

#[rustfmt::skip]
pub(super) const L2: &[VsopTerm] = &[
    VsopTerm { a: 116441.0e-8, b: 1.17988, c: 7.11355 },
    VsopTerm { a: 91921.0e-8, b: 0.07425, c: 213.29910 },
    VsopTerm { a: 90592.0e-8, b: 0.00000, c: 0.00000 },
    VsopTerm { a: 15277.0e-8, b: 4.06492, c: 206.18555 },
    VsopTerm { a: 10631.0e-8, b: 0.25778, c: 220.41264 },
    VsopTerm { a: 10605.0e-8, b: 5.40964, c: 426.59819 },
    VsopTerm { a: 4265.0e-8, b: 1.0460, c: 14.2271 },
    VsopTerm { a: 1216.0e-8, b: 2.9186, c: 103.0928 },
    VsopTerm { a: 1165.0e-8, b: 4.6094, c: 639.8973 },
    VsopTerm { a: 1082.0e-8, b: 5.6913, c: 433.7117 },
    VsopTerm { a: 1045.0e-8, b: 4.0421, c: 199.0720 },
    VsopTerm { a: 1020.0e-8, b: 0.6337, c: 3.1814 },
    VsopTerm { a: 634.0e-8, b: 4.388, c: 419.485 },
    VsopTerm { a: 549.0e-8, b: 5.573, c: 3.932 },
    VsopTerm { a: 457.0e-8, b: 1.268, c: 110.206 },
    VsopTerm { a: 425.0e-8, b: 0.209, c: 227.526 },
];

#[rustfmt::skip]
pub(super) const L3: &[VsopTerm] = &[
    VsopTerm { a: 16039.0e-8, b: 5.73945, c: 7.11355 },
    VsopTerm { a: 4250.0e-8, b: 4.5854, c: 213.2991 },
    VsopTerm { a: 1907.0e-8, b: 4.7608, c: 220.4126 },
    VsopTerm { a: 1466.0e-8, b: 5.9133, c: 206.1856 },
    VsopTerm { a: 1162.0e-8, b: 5.6197, c: 14.2271 },
    VsopTerm { a: 1067.0e-8, b: 3.6082, c: 426.5982 },
    VsopTerm { a: 239.0e-8, b: 3.861, c: 433.712 },
    VsopTerm { a: 237.0e-8, b: 5.768, c: 199.072 },
    VsopTerm { a: 166.0e-8, b: 5.116, c: 3.181 },
    VsopTerm { a: 151.0e-8, b: 2.736, c: 639.897 },
    VsopTerm { a: 131.0e-8, b: 4.743, c: 227.526 },
    VsopTerm { a: 63.0e-8, b: 0.23, c: 419.48 },
];

#[rustfmt::skip]
pub(super) const L4: &[VsopTerm] = &[
    VsopTerm { a: 1662.0e-8, b: 3.9983, c: 7.1135 },
    VsopTerm { a: 257.0e-8, b: 2.984, c: 220.413 },
    VsopTerm { a: 236.0e-8, b: 3.902, c: 14.227 },
    VsopTerm { a: 149.0e-8, b: 2.741, c: 213.299 },
    VsopTerm { a: 114.0e-8, b: 3.142, c: 0.000 },
    VsopTerm { a: 110.0e-8, b: 1.515, c: 206.186 },
    VsopTerm { a: 68.0e-8, b: 1.72, c: 426.598 },
    VsopTerm { a: 40.0e-8, b: 2.05, c: 433.712 },
    VsopTerm { a: 38.0e-8, b: 1.24, c: 199.072 },
];

#[rustfmt::skip]
pub(super) const L5: &[VsopTerm] = &[
    VsopTerm { a: 124.0e-8, b: 2.259, c: 7.114 },
    VsopTerm { a: 34.0e-8, b: 2.16, c: 14.23 },
    VsopTerm { a: 28.0e-8, b: 1.20, c: 220.41 },
    VsopTerm { a: 6.0e-8, b: 1.22, c: 227.53 },
    VsopTerm { a: 5.0e-8, b: 0.24, c: 433.71 },
];

/// Longitude sub-series references.
pub(super) const L_SERIES: [&[VsopTerm]; 6] = [L0, L1, L2, L3, L4, L5];

// ===========================================================================
// Latitude (B) series
// ===========================================================================

#[rustfmt::skip]
pub(super) const B0: &[VsopTerm] = &[
    VsopTerm { a: 4330678.0e-8, b: 3.6028443, c: 213.2990954 },
    VsopTerm { a: 240348.0e-8, b: 2.85238, c: 426.59819 },
    VsopTerm { a: 84746.0e-8, b: 0.00000, c: 0.00000 },
    VsopTerm { a: 34116.0e-8, b: 0.57297, c: 206.18555 },
    VsopTerm { a: 30863.0e-8, b: 3.48442, c: 220.41264 },
    VsopTerm { a: 14734.0e-8, b: 2.11847, c: 639.89729 },
    VsopTerm { a: 9917.0e-8, b: 5.7900, c: 419.4846 },
    VsopTerm { a: 6994.0e-8, b: 4.7360, c: 7.1135 },
    VsopTerm { a: 4808.0e-8, b: 5.4331, c: 316.3919 },
    VsopTerm { a: 4788.0e-8, b: 4.9651, c: 110.2063 },
    VsopTerm { a: 3432.0e-8, b: 2.7326, c: 433.7117 },
    VsopTerm { a: 1506.0e-8, b: 6.0130, c: 103.0928 },
    VsopTerm { a: 1060.0e-8, b: 5.6310, c: 529.6910 },
    VsopTerm { a: 969.0e-8, b: 5.204, c: 632.784 },
    VsopTerm { a: 942.0e-8, b: 1.396, c: 853.196 },
    VsopTerm { a: 708.0e-8, b: 3.803, c: 323.505 },
    VsopTerm { a: 552.0e-8, b: 5.131, c: 202.253 },
    VsopTerm { a: 400.0e-8, b: 3.359, c: 227.526 },
    VsopTerm { a: 319.0e-8, b: 3.626, c: 209.367 },
    VsopTerm { a: 316.0e-8, b: 1.997, c: 647.011 },
    VsopTerm { a: 314.0e-8, b: 0.465, c: 217.231 },
    VsopTerm { a: 284.0e-8, b: 4.886, c: 224.345 },
];

#[rustfmt::skip]
pub(super) const B1: &[VsopTerm] = &[
    VsopTerm { a: 397555.0e-8, b: 5.33290, c: 213.29910 },
    VsopTerm { a: 49479.0e-8, b: 3.14159, c: 0.00000 },
    VsopTerm { a: 18572.0e-8, b: 6.09919, c: 426.59819 },
    VsopTerm { a: 14801.0e-8, b: 2.30586, c: 206.18555 },
    VsopTerm { a: 12174.0e-8, b: 0.37577, c: 220.41264 },
    VsopTerm { a: 6215.0e-8, b: 0.7504, c: 639.8973 },
    VsopTerm { a: 2702.0e-8, b: 3.5769, c: 419.4846 },
    VsopTerm { a: 1566.0e-8, b: 2.4809, c: 7.1135 },
    VsopTerm { a: 1399.0e-8, b: 0.6685, c: 110.2063 },
    VsopTerm { a: 1135.0e-8, b: 4.5204, c: 433.7117 },
    VsopTerm { a: 613.0e-8, b: 2.459, c: 529.691 },
    VsopTerm { a: 495.0e-8, b: 5.544, c: 103.093 },
    VsopTerm { a: 363.0e-8, b: 4.711, c: 316.392 },
];

#[rustfmt::skip]
pub(super) const B2: &[VsopTerm] = &[
    VsopTerm { a: 20630.0e-8, b: 0.50482, c: 213.29910 },
    VsopTerm { a: 3720.0e-8, b: 3.9983, c: 206.1856 },
    VsopTerm { a: 1627.0e-8, b: 6.1819, c: 220.4126 },
    VsopTerm { a: 1346.0e-8, b: 0.0000, c: 0.0000 },
    VsopTerm { a: 706.0e-8, b: 3.039, c: 426.598 },
    VsopTerm { a: 365.0e-8, b: 5.099, c: 639.897 },
    VsopTerm { a: 339.0e-8, b: 3.383, c: 419.485 },
    VsopTerm { a: 219.0e-8, b: 1.937, c: 7.114 },
    VsopTerm { a: 139.0e-8, b: 5.016, c: 110.206 },
];

#[rustfmt::skip]
pub(super) const B3: &[VsopTerm] = &[
    VsopTerm { a: 666.0e-8, b: 1.990, c: 213.299 },
    VsopTerm { a: 632.0e-8, b: 5.698, c: 206.186 },
    VsopTerm { a: 398.0e-8, b: 0.000, c: 0.000 },
    VsopTerm { a: 188.0e-8, b: 4.338, c: 220.413 },
    VsopTerm { a: 92.0e-8, b: 4.84, c: 426.598 },
    VsopTerm { a: 52.0e-8, b: 3.42, c: 639.897 },
];

#[rustfmt::skip]
pub(super) const B4: &[VsopTerm] = &[
    VsopTerm { a: 80.0e-8, b: 1.12, c: 206.19 },
    VsopTerm { a: 32.0e-8, b: 3.12, c: 213.30 },
    VsopTerm { a: 17.0e-8, b: 2.48, c: 220.41 },
    VsopTerm { a: 12.0e-8, b: 3.14, c: 0.00 },
    VsopTerm { a: 9.0e-8, b: 0.38, c: 426.60 },
];

/// Latitude sub-series references.
pub(super) const B_SERIES: [&[VsopTerm]; 5] = [B0, B1, B2, B3, B4];

// ===========================================================================
// Radius vector (R) series
// ===========================================================================

#[rustfmt::skip]
pub(super) const R0: &[VsopTerm] = &[
    VsopTerm { a: 955758136.0e-8, b: 0.0000000, c: 0.0000000 },
    VsopTerm { a: 52921382.0e-8, b: 2.3922620, c: 213.2990954 },
    VsopTerm { a: 1873680.0e-8, b: 5.2354961, c: 206.1855484 },
    VsopTerm { a: 1464664.0e-8, b: 1.6476305, c: 426.5981909 },
    VsopTerm { a: 821891.0e-8, b: 5.93520, c: 316.39187 },
    VsopTerm { a: 547507.0e-8, b: 5.01533, c: 103.09277 },
    VsopTerm { a: 371684.0e-8, b: 2.27114, c: 220.41264 },
    VsopTerm { a: 361778.0e-8, b: 3.13904, c: 7.11355 },
    VsopTerm { a: 140618.0e-8, b: 5.70407, c: 632.78374 },
    VsopTerm { a: 108975.0e-8, b: 3.29314, c: 110.20632 },
    VsopTerm { a: 69007.0e-8, b: 5.94100, c: 419.48464 },
    VsopTerm { a: 61053.0e-8, b: 0.94038, c: 639.89729 },
    VsopTerm { a: 48913.0e-8, b: 1.55733, c: 202.25340 },
    VsopTerm { a: 34144.0e-8, b: 0.19519, c: 277.03499 },
    VsopTerm { a: 32402.0e-8, b: 5.47085, c: 949.17561 },
    VsopTerm { a: 20937.0e-8, b: 0.46349, c: 735.87651 },
    VsopTerm { a: 20839.0e-8, b: 1.52103, c: 433.71174 },
    VsopTerm { a: 20747.0e-8, b: 5.33256, c: 199.07200 },
    VsopTerm { a: 15298.0e-8, b: 3.05944, c: 529.69097 },
    VsopTerm { a: 14296.0e-8, b: 2.60434, c: 323.50542 },
    VsopTerm { a: 12884.0e-8, b: 1.64892, c: 138.51750 },
    VsopTerm { a: 11993.0e-8, b: 5.98051, c: 846.08283 },
    VsopTerm { a: 11380.0e-8, b: 1.73106, c: 522.57742 },
    VsopTerm { a: 9796.0e-8, b: 5.2048, c: 1265.5675 },
    VsopTerm { a: 7753.0e-8, b: 5.8519, c: 95.9792 },
    VsopTerm { a: 6771.0e-8, b: 3.0043, c: 14.2271 },
    VsopTerm { a: 6466.0e-8, b: 0.1773, c: 1052.2684 },
    VsopTerm { a: 5850.0e-8, b: 1.4552, c: 415.5525 },
    VsopTerm { a: 5307.0e-8, b: 0.5974, c: 63.7359 },
    VsopTerm { a: 4696.0e-8, b: 2.1492, c: 227.5262 },
    VsopTerm { a: 4044.0e-8, b: 1.6401, c: 209.3669 },
    VsopTerm { a: 3688.0e-8, b: 0.7802, c: 412.3711 },
    VsopTerm { a: 3461.0e-8, b: 1.8509, c: 175.1661 },
    VsopTerm { a: 3420.0e-8, b: 4.9455, c: 1581.9593 },
    VsopTerm { a: 3401.0e-8, b: 0.5539, c: 350.3321 },
    VsopTerm { a: 3376.0e-8, b: 3.6953, c: 224.3448 },
    VsopTerm { a: 2976.0e-8, b: 5.6847, c: 210.1177 },
    VsopTerm { a: 2885.0e-8, b: 1.3876, c: 838.9693 },
    VsopTerm { a: 2881.0e-8, b: 0.1796, c: 853.1964 },
    VsopTerm { a: 2508.0e-8, b: 3.5385, c: 742.9901 },
    VsopTerm { a: 2448.0e-8, b: 6.1841, c: 1368.6603 },
    VsopTerm { a: 2406.0e-8, b: 2.9656, c: 117.3199 },
    VsopTerm { a: 2174.0e-8, b: 0.0151, c: 340.7709 },
    VsopTerm { a: 2024.0e-8, b: 5.0541, c: 11.0457 },
];

#[rustfmt::skip]
pub(super) const R1: &[VsopTerm] = &[
    VsopTerm { a: 6182981.0e-8, b: 0.2584352, c: 213.2990954 },
    VsopTerm { a: 506578.0e-8, b: 0.71147, c: 206.18555 },
    VsopTerm { a: 341394.0e-8, b: 5.79636, c: 426.59819 },
    VsopTerm { a: 188491.0e-8, b: 0.47216, c: 220.41264 },
    VsopTerm { a: 186262.0e-8, b: 3.14159, c: 0.00000 },
    VsopTerm { a: 143891.0e-8, b: 1.40745, c: 7.11355 },
    VsopTerm { a: 49621.0e-8, b: 6.01744, c: 103.09277 },
    VsopTerm { a: 20928.0e-8, b: 5.09246, c: 639.89729 },
    VsopTerm { a: 19953.0e-8, b: 1.17560, c: 419.48464 },
    VsopTerm { a: 18840.0e-8, b: 1.60820, c: 110.20632 },
    VsopTerm { a: 13877.0e-8, b: 0.75886, c: 199.07200 },
    VsopTerm { a: 12893.0e-8, b: 5.94330, c: 433.71174 },
    VsopTerm { a: 5397.0e-8, b: 1.2885, c: 14.2271 },
    VsopTerm { a: 4869.0e-8, b: 0.8679, c: 323.5054 },
    VsopTerm { a: 4247.0e-8, b: 0.3930, c: 227.5262 },
    VsopTerm { a: 3252.0e-8, b: 1.2585, c: 95.9792 },
    VsopTerm { a: 3081.0e-8, b: 3.4366, c: 522.5774 },
    VsopTerm { a: 2909.0e-8, b: 4.6068, c: 202.2534 },
    VsopTerm { a: 2856.0e-8, b: 2.1673, c: 735.8765 },
    VsopTerm { a: 1988.0e-8, b: 2.4505, c: 412.3711 },
];

#[rustfmt::skip]
pub(super) const R2: &[VsopTerm] = &[
    VsopTerm { a: 436902.0e-8, b: 4.78672, c: 213.29910 },
    VsopTerm { a: 71923.0e-8, b: 2.50070, c: 206.18555 },
    VsopTerm { a: 49767.0e-8, b: 4.97168, c: 220.41264 },
    VsopTerm { a: 43221.0e-8, b: 3.86940, c: 426.59819 },
    VsopTerm { a: 29646.0e-8, b: 5.96310, c: 7.11355 },
    VsopTerm { a: 4721.0e-8, b: 2.4753, c: 199.0720 },
    VsopTerm { a: 4142.0e-8, b: 4.1067, c: 433.7117 },
    VsopTerm { a: 3789.0e-8, b: 3.0977, c: 639.8973 },
    VsopTerm { a: 2964.0e-8, b: 1.3721, c: 103.0928 },
    VsopTerm { a: 2556.0e-8, b: 2.8507, c: 419.4846 },
    VsopTerm { a: 2327.0e-8, b: 0.0000, c: 0.0000 },
    VsopTerm { a: 2208.0e-8, b: 6.2759, c: 110.2063 },
    VsopTerm { a: 2188.0e-8, b: 5.8555, c: 14.2271 },
    VsopTerm { a: 1957.0e-8, b: 4.9245, c: 227.5262 },
    VsopTerm { a: 924.0e-8, b: 5.464, c: 323.505 },
    VsopTerm { a: 706.0e-8, b: 2.971, c: 95.979 },
];

#[rustfmt::skip]
pub(super) const R3: &[VsopTerm] = &[
    VsopTerm { a: 20315.0e-8, b: 3.02187, c: 213.29910 },
    VsopTerm { a: 8924.0e-8, b: 3.1914, c: 220.4126 },
    VsopTerm { a: 6909.0e-8, b: 4.3517, c: 206.1856 },
    VsopTerm { a: 4087.0e-8, b: 4.2227, c: 7.1135 },
    VsopTerm { a: 3879.0e-8, b: 2.0106, c: 426.5982 },
    VsopTerm { a: 1071.0e-8, b: 4.2036, c: 199.0720 },
    VsopTerm { a: 907.0e-8, b: 2.283, c: 433.712 },
    VsopTerm { a: 606.0e-8, b: 3.175, c: 227.526 },
    VsopTerm { a: 597.0e-8, b: 4.135, c: 14.227 },
    VsopTerm { a: 483.0e-8, b: 1.173, c: 639.897 },
    VsopTerm { a: 393.0e-8, b: 0.000, c: 0.000 },
    VsopTerm { a: 229.0e-8, b: 4.698, c: 419.485 },
];

#[rustfmt::skip]
pub(super) const R4: &[VsopTerm] = &[
    VsopTerm { a: 1202.0e-8, b: 1.415, c: 220.413 },
    VsopTerm { a: 708.0e-8, b: 1.162, c: 213.299 },
    VsopTerm { a: 516.0e-8, b: 6.240, c: 206.186 },
    VsopTerm { a: 427.0e-8, b: 2.469, c: 7.114 },
    VsopTerm { a: 268.0e-8, b: 0.187, c: 426.598 },
    VsopTerm { a: 170.0e-8, b: 5.959, c: 199.072 },
    VsopTerm { a: 150.0e-8, b: 0.480, c: 433.712 },
    VsopTerm { a: 145.0e-8, b: 1.442, c: 227.526 },
    VsopTerm { a: 121.0e-8, b: 2.405, c: 14.227 },
    VsopTerm { a: 47.0e-8, b: 5.57, c: 639.90 },
    VsopTerm { a: 19.0e-8, b: 5.86, c: 419.48 },
];

/// Radius sub-series references.
pub(super) const R_SERIES: [&[VsopTerm]; 5] = [R0, R1, R2, R3, R4];
