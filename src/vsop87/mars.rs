//! VSOP87D coefficients for Mars.
//!
//! Heliocentric ecliptic longitude (L), latitude (B), and radius vector (R)
//! referred to the J2000.0 ecliptic and equinox.
//!
//! Source: Bretagnon & Francou (1988), Bureau des Longitudes VSOP87D.mar.

use super::VsopTerm;

// ===========================================================================
// Longitude (L) series
// ===========================================================================

#[rustfmt::skip]
pub(super) const L0: &[VsopTerm] = &[
    VsopTerm { a: 620347712.0e-8, b: 0.0000000, c: 0.0000000 },
    VsopTerm { a: 18656368.0e-8, b: 5.0503710, c: 3340.6124267 },
    VsopTerm { a: 1108217.0e-8, b: 5.4009984, c: 6681.2248534 },
    VsopTerm { a: 91798.0e-8, b: 5.75479, c: 10021.83728 },
    VsopTerm { a: 27745.0e-8, b: 5.97050, c: 3.52312 },
    VsopTerm { a: 12316.0e-8, b: 0.84956, c: 2810.92146 },
    VsopTerm { a: 10610.0e-8, b: 2.93959, c: 2281.23050 },
    VsopTerm { a: 8927.0e-8, b: 4.1570, c: 0.0173 },
    VsopTerm { a: 8716.0e-8, b: 6.1101, c: 13362.4497 },
    VsopTerm { a: 7775.0e-8, b: 3.3397, c: 5621.8429 },
    VsopTerm { a: 6798.0e-8, b: 0.3646, c: 398.1490 },
    VsopTerm { a: 4161.0e-8, b: 0.2281, c: 2942.4634 },
    VsopTerm { a: 3575.0e-8, b: 1.6619, c: 2544.3144 },
    VsopTerm { a: 3075.0e-8, b: 0.8570, c: 191.4483 },
    VsopTerm { a: 2938.0e-8, b: 6.0789, c: 0.0673 },
    VsopTerm { a: 2628.0e-8, b: 0.6481, c: 3337.0893 },
    VsopTerm { a: 2580.0e-8, b: 0.0300, c: 3344.1355 },
    VsopTerm { a: 2389.0e-8, b: 5.0390, c: 796.2983 },
    VsopTerm { a: 1799.0e-8, b: 0.6563, c: 529.6910 },
    VsopTerm { a: 1546.0e-8, b: 2.9158, c: 1751.5395 },
    VsopTerm { a: 1528.0e-8, b: 1.1498, c: 6151.5339 },
    VsopTerm { a: 1286.0e-8, b: 3.0680, c: 2146.1654 },
    VsopTerm { a: 1264.0e-8, b: 3.6228, c: 5092.1520 },
    VsopTerm { a: 1025.0e-8, b: 3.6933, c: 8962.4553 },
    VsopTerm { a: 892.0e-8, b: 0.183, c: 16703.062 },
    VsopTerm { a: 859.0e-8, b: 2.401, c: 2914.014 },
    VsopTerm { a: 833.0e-8, b: 4.495, c: 3340.630 },
    VsopTerm { a: 833.0e-8, b: 2.464, c: 3340.595 },
    VsopTerm { a: 749.0e-8, b: 3.822, c: 155.420 },
    VsopTerm { a: 724.0e-8, b: 0.675, c: 3738.761 },
    VsopTerm { a: 713.0e-8, b: 3.663, c: 1059.382 },
    VsopTerm { a: 655.0e-8, b: 0.489, c: 3127.313 },
    VsopTerm { a: 636.0e-8, b: 2.922, c: 8432.764 },
    VsopTerm { a: 553.0e-8, b: 4.475, c: 1748.016 },
    VsopTerm { a: 550.0e-8, b: 3.810, c: 0.980 },
    VsopTerm { a: 472.0e-8, b: 3.625, c: 1194.447 },
    VsopTerm { a: 426.0e-8, b: 0.554, c: 6283.076 },
    VsopTerm { a: 415.0e-8, b: 0.497, c: 213.299 },
    VsopTerm { a: 312.0e-8, b: 0.999, c: 6677.702 },
    VsopTerm { a: 307.0e-8, b: 0.381, c: 6684.748 },
    VsopTerm { a: 302.0e-8, b: 4.486, c: 3532.061 },
    VsopTerm { a: 299.0e-8, b: 2.783, c: 6254.627 },
    VsopTerm { a: 293.0e-8, b: 4.221, c: 20.775 },
    VsopTerm { a: 284.0e-8, b: 5.769, c: 3149.164 },
    VsopTerm { a: 281.0e-8, b: 3.721, c: 735.877 },
    VsopTerm { a: 274.0e-8, b: 0.134, c: 2544.314 },
    VsopTerm { a: 274.0e-8, b: 0.542, c: 1349.867 },
    VsopTerm { a: 239.0e-8, b: 5.372, c: 4136.910 },
    VsopTerm { a: 236.0e-8, b: 5.755, c: 3930.210 },
    VsopTerm { a: 231.0e-8, b: 1.282, c: 3870.303 },
    VsopTerm { a: 221.0e-8, b: 3.505, c: 382.897 },
];

#[rustfmt::skip]
pub(super) const L1: &[VsopTerm] = &[
    VsopTerm { a: 334085627474.0e-8, b: 0.00000000000, c: 0.00000000000 },
    VsopTerm { a: 1458227.0e-8, b: 3.6042605, c: 3340.6124267 },
    VsopTerm { a: 164901.0e-8, b: 3.92631, c: 6681.22485 },
    VsopTerm { a: 19963.0e-8, b: 4.26594, c: 10021.83728 },
    VsopTerm { a: 3452.0e-8, b: 4.7321, c: 3.5231 },
    VsopTerm { a: 2485.0e-8, b: 4.6128, c: 13362.4497 },
    VsopTerm { a: 842.0e-8, b: 4.459, c: 2281.230 },
    VsopTerm { a: 538.0e-8, b: 5.016, c: 398.149 },
    VsopTerm { a: 521.0e-8, b: 4.994, c: 3344.136 },
    VsopTerm { a: 433.0e-8, b: 2.561, c: 191.448 },
    VsopTerm { a: 430.0e-8, b: 5.316, c: 155.420 },
    VsopTerm { a: 382.0e-8, b: 3.539, c: 796.298 },
    VsopTerm { a: 314.0e-8, b: 4.963, c: 16703.062 },
    VsopTerm { a: 283.0e-8, b: 3.160, c: 2544.314 },
    VsopTerm { a: 206.0e-8, b: 4.569, c: 2146.165 },
    VsopTerm { a: 169.0e-8, b: 1.329, c: 3337.089 },
    VsopTerm { a: 158.0e-8, b: 4.185, c: 1751.540 },
    VsopTerm { a: 134.0e-8, b: 2.233, c: 0.980 },
    VsopTerm { a: 134.0e-8, b: 5.974, c: 1748.016 },
    VsopTerm { a: 118.0e-8, b: 6.024, c: 6151.534 },
    VsopTerm { a: 117.0e-8, b: 2.213, c: 1059.382 },
    VsopTerm { a: 114.0e-8, b: 2.129, c: 1194.447 },
    VsopTerm { a: 114.0e-8, b: 5.428, c: 3738.761 },
    VsopTerm { a: 91.0e-8, b: 1.10, c: 1349.87 },
    VsopTerm { a: 85.0e-8, b: 3.91, c: 553.57 },
    VsopTerm { a: 83.0e-8, b: 5.30, c: 6684.75 },
    VsopTerm { a: 81.0e-8, b: 4.43, c: 529.69 },
    VsopTerm { a: 80.0e-8, b: 2.25, c: 8962.46 },
    VsopTerm { a: 73.0e-8, b: 2.50, c: 951.72 },
    VsopTerm { a: 73.0e-8, b: 5.84, c: 242.73 },
    VsopTerm { a: 71.0e-8, b: 3.86, c: 2914.01 },
];

#[rustfmt::skip]
pub(super) const L2: &[VsopTerm] = &[
    VsopTerm { a: 58016.0e-8, b: 2.04979, c: 3340.61243 },
    VsopTerm { a: 54188.0e-8, b: 0.00000, c: 0.00000 },
    VsopTerm { a: 13908.0e-8, b: 2.45742, c: 6681.22485 },
    VsopTerm { a: 2465.0e-8, b: 2.8000, c: 10021.8373 },
    VsopTerm { a: 398.0e-8, b: 3.141, c: 13362.450 },
    VsopTerm { a: 222.0e-8, b: 3.194, c: 3.523 },
    VsopTerm { a: 121.0e-8, b: 0.543, c: 155.420 },
    VsopTerm { a: 62.0e-8, b: 3.49, c: 16703.06 },
    VsopTerm { a: 54.0e-8, b: 3.54, c: 3344.14 },
    VsopTerm { a: 34.0e-8, b: 6.00, c: 2281.23 },
    VsopTerm { a: 32.0e-8, b: 4.14, c: 191.45 },
    VsopTerm { a: 30.0e-8, b: 2.00, c: 796.30 },
    VsopTerm { a: 23.0e-8, b: 4.33, c: 242.73 },
    VsopTerm { a: 22.0e-8, b: 3.45, c: 398.15 },
    VsopTerm { a: 20.0e-8, b: 5.42, c: 553.57 },
    VsopTerm { a: 16.0e-8, b: 0.66, c: 0.98 },
    VsopTerm { a: 16.0e-8, b: 6.11, c: 2146.17 },
    VsopTerm { a: 16.0e-8, b: 1.22, c: 1748.02 },
    VsopTerm { a: 15.0e-8, b: 6.10, c: 3185.19 },
    VsopTerm { a: 14.0e-8, b: 4.02, c: 951.72 },
    VsopTerm { a: 14.0e-8, b: 2.62, c: 1349.87 },
    VsopTerm { a: 13.0e-8, b: 0.60, c: 1194.45 },
    VsopTerm { a: 12.0e-8, b: 3.86, c: 6684.75 },
];

#[rustfmt::skip]
pub(super) const L3: &[VsopTerm] = &[
    VsopTerm { a: 1482.0e-8, b: 0.4443, c: 3340.6124 },
    VsopTerm { a: 662.0e-8, b: 0.885, c: 6681.225 },
    VsopTerm { a: 188.0e-8, b: 1.288, c: 10021.837 },
    VsopTerm { a: 41.0e-8, b: 1.65, c: 13362.45 },
    VsopTerm { a: 26.0e-8, b: 0.00, c: 0.00 },
    VsopTerm { a: 23.0e-8, b: 2.05, c: 155.42 },
    VsopTerm { a: 10.0e-8, b: 1.58, c: 3.52 },
    VsopTerm { a: 8.0e-8, b: 2.00, c: 16703.06 },
    VsopTerm { a: 5.0e-8, b: 2.82, c: 242.73 },
    VsopTerm { a: 4.0e-8, b: 2.02, c: 3344.14 },
    VsopTerm { a: 3.0e-8, b: 4.59, c: 3185.19 },
];

#[rustfmt::skip]
pub(super) const L4: &[VsopTerm] = &[
    VsopTerm { a: 114.0e-8, b: 3.1416, c: 0.0000 },
    VsopTerm { a: 29.0e-8, b: 5.64, c: 3340.61 },
    VsopTerm { a: 24.0e-8, b: 5.14, c: 6681.22 },
    VsopTerm { a: 11.0e-8, b: 6.03, c: 10021.84 },
    VsopTerm { a: 3.0e-8, b: 0.13, c: 13362.45 },
    VsopTerm { a: 3.0e-8, b: 0.76, c: 155.42 },
    VsopTerm { a: 1.0e-8, b: 0.49, c: 16703.06 },
];

#[rustfmt::skip]
pub(super) const L5: &[VsopTerm] = &[
    VsopTerm { a: 1.0e-8, b: 3.14, c: 0.00 },
    VsopTerm { a: 1.0e-8, b: 4.04, c: 3340.61 },
];

/// Longitude sub-series references.
pub(super) const L_SERIES: [&[VsopTerm]; 6] = [L0, L1, L2, L3, L4, L5];

// ===========================================================================
// Latitude (B) series
// ===========================================================================

#[rustfmt::skip]
pub(super) const B0: &[VsopTerm] = &[
    VsopTerm { a: 3197135.0e-8, b: 3.7683204, c: 3340.6124267 },
    VsopTerm { a: 298033.0e-8, b: 4.10617, c: 6681.22485 },
    VsopTerm { a: 289105.0e-8, b: 0.00000, c: 0.00000 },
    VsopTerm { a: 31366.0e-8, b: 4.44651, c: 10021.83728 },
    VsopTerm { a: 3484.0e-8, b: 4.7881, c: 13362.4497 },
    VsopTerm { a: 443.0e-8, b: 5.026, c: 3344.136 },
    VsopTerm { a: 443.0e-8, b: 5.652, c: 3337.089 },
    VsopTerm { a: 399.0e-8, b: 5.131, c: 16703.062 },
    VsopTerm { a: 293.0e-8, b: 3.793, c: 2281.230 },
    VsopTerm { a: 182.0e-8, b: 6.136, c: 6151.534 },
    VsopTerm { a: 163.0e-8, b: 4.264, c: 529.691 },
    VsopTerm { a: 160.0e-8, b: 2.232, c: 1059.382 },
    VsopTerm { a: 149.0e-8, b: 2.165, c: 5621.843 },
    VsopTerm { a: 143.0e-8, b: 1.182, c: 3340.595 },
    VsopTerm { a: 143.0e-8, b: 3.213, c: 3340.630 },
    VsopTerm { a: 139.0e-8, b: 2.418, c: 8962.455 },
];

#[rustfmt::skip]
pub(super) const B1: &[VsopTerm] = &[
    VsopTerm { a: 350069.0e-8, b: 5.368478, c: 3340.612427 },
    VsopTerm { a: 14116.0e-8, b: 3.14159, c: 0.00000 },
    VsopTerm { a: 9671.0e-8, b: 5.4788, c: 6681.2249 },
    VsopTerm { a: 1472.0e-8, b: 3.2021, c: 10021.8373 },
    VsopTerm { a: 426.0e-8, b: 3.408, c: 13362.450 },
    VsopTerm { a: 102.0e-8, b: 0.776, c: 3337.089 },
    VsopTerm { a: 79.0e-8, b: 3.72, c: 16703.06 },
    VsopTerm { a: 33.0e-8, b: 3.46, c: 5621.84 },
    VsopTerm { a: 26.0e-8, b: 2.48, c: 2281.23 },
];

#[rustfmt::skip]
pub(super) const B2: &[VsopTerm] = &[
    VsopTerm { a: 16727.0e-8, b: 0.60221, c: 3340.61243 },
    VsopTerm { a: 4987.0e-8, b: 4.1416, c: 0.0000 },
    VsopTerm { a: 302.0e-8, b: 3.559, c: 6681.225 },
    VsopTerm { a: 26.0e-8, b: 1.90, c: 10021.84 },
    VsopTerm { a: 21.0e-8, b: 0.92, c: 13362.45 },
    VsopTerm { a: 12.0e-8, b: 2.24, c: 3337.09 },
    VsopTerm { a: 8.0e-8, b: 2.25, c: 16703.06 },
];

#[rustfmt::skip]
pub(super) const B3: &[VsopTerm] = &[
    VsopTerm { a: 607.0e-8, b: 1.981, c: 3340.612 },
    VsopTerm { a: 43.0e-8, b: 0.00, c: 0.00 },
    VsopTerm { a: 14.0e-8, b: 1.80, c: 6681.22 },
    VsopTerm { a: 3.0e-8, b: 3.14, c: 10021.84 },
];

#[rustfmt::skip]
pub(super) const B4: &[VsopTerm] = &[
    VsopTerm { a: 13.0e-8, b: 0.00, c: 0.00 },
    VsopTerm { a: 11.0e-8, b: 3.46, c: 3340.61 },
    VsopTerm { a: 1.0e-8, b: 0.50, c: 6681.22 },
];

/// Latitude sub-series references.
pub(super) const B_SERIES: [&[VsopTerm]; 5] = [B0, B1, B2, B3, B4];

// ===========================================================================
// Radius vector (R) series
// ===========================================================================

#[rustfmt::skip]
pub(super) const R0: &[VsopTerm] = &[
    VsopTerm { a: 153033488.0e-8, b: 0.0000000, c: 0.0000000 },
    VsopTerm { a: 14184953.0e-8, b: 3.4737461, c: 3340.6124267 },
    VsopTerm { a: 660776.0e-8, b: 3.81783, c: 6681.22485 },
    VsopTerm { a: 46179.0e-8, b: 4.15595, c: 10021.83728 },
    VsopTerm { a: 8110.0e-8, b: 5.5596, c: 2810.9215 },
    VsopTerm { a: 7485.0e-8, b: 1.7724, c: 5621.8429 },
    VsopTerm { a: 5523.0e-8, b: 1.3644, c: 2281.2305 },
    VsopTerm { a: 3825.0e-8, b: 4.4941, c: 13362.4497 },
    VsopTerm { a: 2484.0e-8, b: 4.9255, c: 2942.4634 },
    VsopTerm { a: 2307.0e-8, b: 0.0908, c: 2544.3144 },
    VsopTerm { a: 1999.0e-8, b: 5.3606, c: 3337.0893 },
    VsopTerm { a: 1960.0e-8, b: 4.7425, c: 3344.1355 },
    VsopTerm { a: 1167.0e-8, b: 2.1126, c: 5092.1520 },
    VsopTerm { a: 1103.0e-8, b: 5.0091, c: 398.1490 },
    VsopTerm { a: 992.0e-8, b: 5.839, c: 6151.534 },
    VsopTerm { a: 899.0e-8, b: 4.408, c: 529.691 },
    VsopTerm { a: 807.0e-8, b: 2.102, c: 1059.382 },
    VsopTerm { a: 798.0e-8, b: 3.448, c: 796.298 },
    VsopTerm { a: 741.0e-8, b: 1.499, c: 2146.165 },
    VsopTerm { a: 726.0e-8, b: 1.245, c: 8432.764 },
    VsopTerm { a: 692.0e-8, b: 2.134, c: 8962.455 },
    VsopTerm { a: 633.0e-8, b: 0.894, c: 3340.595 },
    VsopTerm { a: 633.0e-8, b: 2.924, c: 3340.630 },
    VsopTerm { a: 630.0e-8, b: 1.287, c: 1751.540 },
    VsopTerm { a: 574.0e-8, b: 0.829, c: 2914.014 },
    VsopTerm { a: 526.0e-8, b: 5.383, c: 3738.761 },
    VsopTerm { a: 473.0e-8, b: 5.199, c: 3127.313 },
    VsopTerm { a: 348.0e-8, b: 4.832, c: 16703.062 },
    VsopTerm { a: 284.0e-8, b: 2.907, c: 3532.061 },
    VsopTerm { a: 280.0e-8, b: 5.257, c: 6283.076 },
    VsopTerm { a: 276.0e-8, b: 1.218, c: 6254.627 },
    VsopTerm { a: 275.0e-8, b: 2.908, c: 1748.016 },
    VsopTerm { a: 270.0e-8, b: 3.764, c: 5884.927 },
    VsopTerm { a: 239.0e-8, b: 2.037, c: 1194.447 },
    VsopTerm { a: 234.0e-8, b: 5.105, c: 5486.778 },
    VsopTerm { a: 228.0e-8, b: 3.255, c: 6872.673 },
    VsopTerm { a: 223.0e-8, b: 4.199, c: 3149.164 },
    VsopTerm { a: 219.0e-8, b: 5.583, c: 191.448 },
    VsopTerm { a: 208.0e-8, b: 5.255, c: 3340.680 },
    VsopTerm { a: 208.0e-8, b: 4.846, c: 3340.545 },
    VsopTerm { a: 186.0e-8, b: 5.699, c: 6677.702 },
    VsopTerm { a: 183.0e-8, b: 5.081, c: 6684.748 },
    VsopTerm { a: 179.0e-8, b: 4.184, c: 3333.499 },
    VsopTerm { a: 176.0e-8, b: 5.953, c: 3870.303 },
    VsopTerm { a: 164.0e-8, b: 3.799, c: 4136.910 },
];

#[rustfmt::skip]
pub(super) const R1: &[VsopTerm] = &[
    VsopTerm { a: 1107434.0e-8, b: 2.0325052, c: 3340.6124267 },
    VsopTerm { a: 103176.0e-8, b: 2.37072, c: 6681.22485 },
    VsopTerm { a: 12877.0e-8, b: 0.00000, c: 0.00000 },
    VsopTerm { a: 10816.0e-8, b: 2.70888, c: 10021.83728 },
    VsopTerm { a: 1195.0e-8, b: 3.0470, c: 13362.4497 },
    VsopTerm { a: 439.0e-8, b: 2.888, c: 2281.230 },
    VsopTerm { a: 396.0e-8, b: 3.423, c: 3344.136 },
    VsopTerm { a: 183.0e-8, b: 1.584, c: 2544.314 },
    VsopTerm { a: 136.0e-8, b: 3.385, c: 16703.062 },
    VsopTerm { a: 128.0e-8, b: 6.043, c: 3337.089 },
    VsopTerm { a: 119.0e-8, b: 3.541, c: 796.298 },
    VsopTerm { a: 96.0e-8, b: 1.98, c: 2146.17 },
    VsopTerm { a: 82.0e-8, b: 3.03, c: 398.15 },
    VsopTerm { a: 79.0e-8, b: 4.64, c: 3340.595 },
    VsopTerm { a: 79.0e-8, b: 2.63, c: 3340.630 },
    VsopTerm { a: 72.0e-8, b: 2.77, c: 529.69 },
    VsopTerm { a: 69.0e-8, b: 0.16, c: 1059.38 },
    VsopTerm { a: 65.0e-8, b: 3.67, c: 2914.01 },
    VsopTerm { a: 54.0e-8, b: 3.54, c: 8962.46 },
    VsopTerm { a: 49.0e-8, b: 1.48, c: 3738.76 },
    VsopTerm { a: 46.0e-8, b: 2.36, c: 2281.23 },
    VsopTerm { a: 45.0e-8, b: 2.54, c: 3127.31 },
];

#[rustfmt::skip]
pub(super) const R2: &[VsopTerm] = &[
    VsopTerm { a: 44242.0e-8, b: 0.47931, c: 3340.61243 },
    VsopTerm { a: 8138.0e-8, b: 0.8700, c: 6681.2249 },
    VsopTerm { a: 1275.0e-8, b: 1.2259, c: 10021.8373 },
    VsopTerm { a: 187.0e-8, b: 1.573, c: 13362.450 },
    VsopTerm { a: 52.0e-8, b: 3.14, c: 0.00 },
    VsopTerm { a: 41.0e-8, b: 1.97, c: 3344.14 },
    VsopTerm { a: 27.0e-8, b: 1.92, c: 16703.06 },
    VsopTerm { a: 18.0e-8, b: 4.43, c: 2281.23 },
    VsopTerm { a: 12.0e-8, b: 2.03, c: 3185.19 },
    VsopTerm { a: 10.0e-8, b: 0.58, c: 3337.09 },
    VsopTerm { a: 9.0e-8, b: 3.67, c: 796.30 },
];

#[rustfmt::skip]
pub(super) const R3: &[VsopTerm] = &[
    VsopTerm { a: 1113.0e-8, b: 5.1499, c: 3340.6124 },
    VsopTerm { a: 424.0e-8, b: 5.613, c: 6681.225 },
    VsopTerm { a: 100.0e-8, b: 5.997, c: 10021.837 },
    VsopTerm { a: 20.0e-8, b: 0.08, c: 13362.45 },
    VsopTerm { a: 5.0e-8, b: 3.14, c: 0.00 },
    VsopTerm { a: 3.0e-8, b: 0.43, c: 16703.06 },
];

/// Radius sub-series references.
pub(super) const R_SERIES: [&[VsopTerm]; 4] = [R0, R1, R2, R3];
