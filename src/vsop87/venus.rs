//! VSOP87D coefficients for Venus.
//!
//! Heliocentric ecliptic longitude (L), latitude (B), and radius vector (R)
//! referred to the J2000.0 ecliptic and equinox.
//!
//! Source: Bretagnon & Francou (1988), Bureau des Longitudes VSOP87D.ven.

use super::VsopTerm;

// ===========================================================================
// Longitude (L) series
// ===========================================================================

#[rustfmt::skip]
pub(super) const L0: &[VsopTerm] = &[
    VsopTerm { a: 317614667.0e-8, b: 0.0000000, c: 0.0000000 },
    VsopTerm { a: 1353968.0e-8, b: 5.5931332, c: 10213.2855462 },
    VsopTerm { a: 89892.0e-8, b: 5.30650, c: 20426.57109 },
    VsopTerm { a: 5477.0e-8, b: 4.4163, c: 7860.4194 },
    VsopTerm { a: 3456.0e-8, b: 2.6996, c: 11790.6291 },
    VsopTerm { a: 2372.0e-8, b: 2.9938, c: 3930.2097 },
    VsopTerm { a: 1664.0e-8, b: 4.2502, c: 1577.3435 },
    VsopTerm { a: 1438.0e-8, b: 4.1575, c: 9683.5946 },
    VsopTerm { a: 1317.0e-8, b: 5.1867, c: 26.2983 },
    VsopTerm { a: 1201.0e-8, b: 6.1536, c: 30639.8566 },
    VsopTerm { a: 769.0e-8, b: 0.816, c: 9437.763 },
    VsopTerm { a: 761.0e-8, b: 1.950, c: 529.691 },
    VsopTerm { a: 708.0e-8, b: 1.065, c: 775.522 },
    VsopTerm { a: 585.0e-8, b: 3.998, c: 191.448 },
    VsopTerm { a: 500.0e-8, b: 4.123, c: 15720.839 },
    VsopTerm { a: 429.0e-8, b: 3.586, c: 19367.189 },
    VsopTerm { a: 327.0e-8, b: 5.677, c: 5507.553 },
    VsopTerm { a: 326.0e-8, b: 4.591, c: 10404.734 },
    VsopTerm { a: 232.0e-8, b: 3.163, c: 9153.904 },
    VsopTerm { a: 180.0e-8, b: 4.653, c: 1109.379 },
    VsopTerm { a: 155.0e-8, b: 5.571, c: 19651.048 },
    VsopTerm { a: 128.0e-8, b: 4.226, c: 20.775 },
    VsopTerm { a: 128.0e-8, b: 0.962, c: 5661.332 },
    VsopTerm { a: 106.0e-8, b: 1.537, c: 801.821 },
    VsopTerm { a: 96.0e-8, b: 3.08, c: 2146.17 },
    VsopTerm { a: 82.0e-8, b: 3.22, c: 40853.14 },
    VsopTerm { a: 79.0e-8, b: 6.19, c: 213.30 },
    VsopTerm { a: 71.0e-8, b: 1.48, c: 9999.99 },
    VsopTerm { a: 62.0e-8, b: 5.48, c: 11015.11 },
    VsopTerm { a: 53.0e-8, b: 2.28, c: 21228.39 },
];

#[rustfmt::skip]
pub(super) const L1: &[VsopTerm] = &[
    VsopTerm { a: 1021352943053.0e-8, b: 0.000000000, c: 0.000000000 },
    VsopTerm { a: 95708.0e-8, b: 2.46424, c: 10213.28555 },
    VsopTerm { a: 14445.0e-8, b: 0.51625, c: 20426.57109 },
    VsopTerm { a: 213.0e-8, b: 1.795, c: 30639.857 },
    VsopTerm { a: 174.0e-8, b: 2.655, c: 26.298 },
    VsopTerm { a: 152.0e-8, b: 5.708, c: 1577.344 },
    VsopTerm { a: 82.0e-8, b: 3.22, c: 9437.76 },
    VsopTerm { a: 70.0e-8, b: 2.27, c: 775.52 },
    VsopTerm { a: 52.0e-8, b: 3.60, c: 40853.14 },
    VsopTerm { a: 38.0e-8, b: 1.03, c: 529.69 },
    VsopTerm { a: 30.0e-8, b: 1.25, c: 5507.55 },
    VsopTerm { a: 25.0e-8, b: 6.11, c: 10404.73 },
    VsopTerm { a: 21.0e-8, b: 0.98, c: 3930.21 },
    VsopTerm { a: 20.0e-8, b: 4.36, c: 11790.63 },
    VsopTerm { a: 12.0e-8, b: 5.39, c: 9153.90 },
];

#[rustfmt::skip]
pub(super) const L2: &[VsopTerm] = &[
    VsopTerm { a: 54127.0e-8, b: 0.00000, c: 0.00000 },
    VsopTerm { a: 3891.0e-8, b: 0.3451, c: 10213.2855 },
    VsopTerm { a: 1338.0e-8, b: 2.0201, c: 20426.5711 },
    VsopTerm { a: 24.0e-8, b: 2.05, c: 26.30 },
    VsopTerm { a: 19.0e-8, b: 3.54, c: 30639.86 },
    VsopTerm { a: 12.0e-8, b: 0.93, c: 1577.34 },
    VsopTerm { a: 8.0e-8, b: 4.14, c: 40853.14 },
    VsopTerm { a: 7.0e-8, b: 4.98, c: 9437.76 },
    VsopTerm { a: 6.0e-8, b: 0.16, c: 775.52 },
];

#[rustfmt::skip]
pub(super) const L3: &[VsopTerm] = &[
    VsopTerm { a: 136.0e-8, b: 4.804, c: 10213.286 },
    VsopTerm { a: 78.0e-8, b: 3.67, c: 20426.57 },
    VsopTerm { a: 26.0e-8, b: 0.00, c: 0.00 },
    VsopTerm { a: 4.0e-8, b: 5.39, c: 30639.86 },
    VsopTerm { a: 3.0e-8, b: 5.02, c: 40853.14 },
];

#[rustfmt::skip]
pub(super) const L4: &[VsopTerm] = &[
    VsopTerm { a: 114.0e-8, b: 3.1416, c: 0.0000 },
    VsopTerm { a: 3.0e-8, b: 5.21, c: 20426.57 },
    VsopTerm { a: 2.0e-8, b: 2.51, c: 10213.29 },
];

#[rustfmt::skip]
pub(super) const L5: &[VsopTerm] = &[
    VsopTerm { a: 1.0e-8, b: 0.92, c: 10213.29 },
    VsopTerm { a: 1.0e-8, b: 3.14, c: 0.00 },
];

/// Longitude sub-series references.
pub(super) const L_SERIES: [&[VsopTerm]; 6] = [L0, L1, L2, L3, L4, L5];

// ===========================================================================
// Latitude (B) series
// ===========================================================================

#[rustfmt::skip]
pub(super) const B0: &[VsopTerm] = &[
    VsopTerm { a: 5923638.0e-8, b: 0.2670278, c: 10213.2855462 },
    VsopTerm { a: 40108.0e-8, b: 1.14737, c: 20426.57109 },
    VsopTerm { a: 32815.0e-8, b: 3.14737, c: 0.00000 },
    VsopTerm { a: 1011.0e-8, b: 1.0895, c: 30639.8566 },
    VsopTerm { a: 149.0e-8, b: 6.254, c: 18073.705 },
    VsopTerm { a: 138.0e-8, b: 0.860, c: 38519.946 },
    VsopTerm { a: 130.0e-8, b: 3.672, c: 9437.763 },
    VsopTerm { a: 120.0e-8, b: 3.705, c: 2352.866 },
    VsopTerm { a: 108.0e-8, b: 4.539, c: 22003.915 },
    VsopTerm { a: 91.0e-8, b: 1.10, c: 40853.14 },
    VsopTerm { a: 52.0e-8, b: 3.58, c: 9153.90 },
    VsopTerm { a: 44.0e-8, b: 4.57, c: 13367.97 },
    VsopTerm { a: 41.0e-8, b: 0.23, c: 7860.42 },
    VsopTerm { a: 40.0e-8, b: 5.95, c: 51066.43 },
];

#[rustfmt::skip]
pub(super) const B1: &[VsopTerm] = &[
    VsopTerm { a: 513348.0e-8, b: 1.803643, c: 10213.285546 },
    VsopTerm { a: 4380.0e-8, b: 3.3862, c: 20426.5711 },
    VsopTerm { a: 199.0e-8, b: 0.000, c: 0.000 },
    VsopTerm { a: 197.0e-8, b: 2.530, c: 30639.857 },
    VsopTerm { a: 14.0e-8, b: 0.32, c: 9437.76 },
    VsopTerm { a: 11.0e-8, b: 4.87, c: 40853.14 },
    VsopTerm { a: 9.0e-8, b: 3.71, c: 19367.19 },
];

#[rustfmt::skip]
pub(super) const B2: &[VsopTerm] = &[
    VsopTerm { a: 22378.0e-8, b: 3.38509, c: 10213.28555 },
    VsopTerm { a: 282.0e-8, b: 0.00, c: 0.00 },
    VsopTerm { a: 173.0e-8, b: 5.256, c: 20426.571 },
    VsopTerm { a: 27.0e-8, b: 3.87, c: 30639.86 },
];

#[rustfmt::skip]
pub(super) const B3: &[VsopTerm] = &[
    VsopTerm { a: 647.0e-8, b: 4.992, c: 10213.286 },
    VsopTerm { a: 20.0e-8, b: 3.14, c: 0.00 },
    VsopTerm { a: 6.0e-8, b: 0.77, c: 20426.57 },
];

#[rustfmt::skip]
pub(super) const B4: &[VsopTerm] = &[
    VsopTerm { a: 14.0e-8, b: 0.32, c: 10213.29 },
];

/// Latitude sub-series references.
pub(super) const B_SERIES: [&[VsopTerm]; 5] = [B0, B1, B2, B3, B4];

// ===========================================================================
// Radius vector (R) series
// ===========================================================================

#[rustfmt::skip]
pub(super) const R0: &[VsopTerm] = &[
    VsopTerm { a: 72334821.0e-8, b: 0.0000000, c: 0.0000000 },
    VsopTerm { a: 489824.0e-8, b: 4.021518, c: 10213.285546 },
    VsopTerm { a: 1658.0e-8, b: 4.9021, c: 20426.5711 },
    VsopTerm { a: 1632.0e-8, b: 2.8455, c: 7860.4194 },
    VsopTerm { a: 1378.0e-8, b: 1.1285, c: 11790.6291 },
    VsopTerm { a: 498.0e-8, b: 2.587, c: 9683.595 },
    VsopTerm { a: 374.0e-8, b: 1.423, c: 3930.210 },
    VsopTerm { a: 264.0e-8, b: 3.917, c: 9437.763 },
    VsopTerm { a: 237.0e-8, b: 2.551, c: 15720.839 },
    VsopTerm { a: 222.0e-8, b: 2.013, c: 19367.189 },
    VsopTerm { a: 126.0e-8, b: 2.728, c: 1577.344 },
    VsopTerm { a: 119.0e-8, b: 3.020, c: 10404.734 },
    VsopTerm { a: 107.0e-8, b: 4.429, c: 30639.857 },
    VsopTerm { a: 93.0e-8, b: 1.44, c: 9153.90 },
    VsopTerm { a: 91.0e-8, b: 1.11, c: 191.45 },
    VsopTerm { a: 52.0e-8, b: 0.95, c: 529.69 },
    VsopTerm { a: 46.0e-8, b: 0.30, c: 775.52 },
    VsopTerm { a: 40.0e-8, b: 2.73, c: 1109.38 },
    VsopTerm { a: 37.0e-8, b: 5.93, c: 5507.55 },
    VsopTerm { a: 35.0e-8, b: 5.08, c: 2352.87 },
    VsopTerm { a: 33.0e-8, b: 6.11, c: 9999.99 },
];

#[rustfmt::skip]
pub(super) const R1: &[VsopTerm] = &[
    VsopTerm { a: 34552.0e-8, b: 0.89199, c: 10213.28555 },
    VsopTerm { a: 234.0e-8, b: 1.772, c: 20426.571 },
    VsopTerm { a: 234.0e-8, b: 3.142, c: 0.000 },
    VsopTerm { a: 11.0e-8, b: 5.38, c: 30639.86 },
    VsopTerm { a: 8.0e-8, b: 3.52, c: 9437.76 },
    VsopTerm { a: 6.0e-8, b: 2.56, c: 19367.19 },
    VsopTerm { a: 5.0e-8, b: 0.79, c: 15720.84 },
];

#[rustfmt::skip]
pub(super) const R2: &[VsopTerm] = &[
    VsopTerm { a: 1407.0e-8, b: 5.0637, c: 10213.2855 },
    VsopTerm { a: 16.0e-8, b: 5.47, c: 20426.57 },
    VsopTerm { a: 13.0e-8, b: 0.00, c: 0.00 },
];

#[rustfmt::skip]
pub(super) const R3: &[VsopTerm] = &[
    VsopTerm { a: 50.0e-8, b: 3.22, c: 10213.29 },
];

/// Radius sub-series references.
pub(super) const R_SERIES: [&[VsopTerm]; 4] = [R0, R1, R2, R3];
