//! VSOP87D coefficients for Earth.
//!
//! Heliocentric ecliptic longitude (L), latitude (B), and radius vector (R)
//! referred to the J2000.0 ecliptic and equinox.
//!
//! Source: Bretagnon & Francou (1988), Bureau des Longitudes VSOP87D.ear.

use super::VsopTerm;

// ===========================================================================
// Longitude (L) series
// ===========================================================================

#[rustfmt::skip]
pub(super) const L0: &[VsopTerm] = &[
    VsopTerm { a: 175347046.0e-8, b: 0.0000000, c: 0.0000000 },
    VsopTerm { a: 3341656.0e-8, b: 4.6692568, c: 6283.0758500 },
    VsopTerm { a: 34894.0e-8, b: 4.62610, c: 12566.15170 },
    VsopTerm { a: 3497.0e-8, b: 2.7441, c: 5753.3849 },
    VsopTerm { a: 3418.0e-8, b: 2.8289, c: 3.5232 },
    VsopTerm { a: 3136.0e-8, b: 3.6277, c: 77713.7715 },
    VsopTerm { a: 2676.0e-8, b: 4.4181, c: 7860.4194 },
    VsopTerm { a: 2343.0e-8, b: 6.1352, c: 3930.2097 },
    VsopTerm { a: 1324.0e-8, b: 0.7425, c: 11506.7698 },
    VsopTerm { a: 1273.0e-8, b: 2.0371, c: 529.6910 },
    VsopTerm { a: 1199.0e-8, b: 1.1096, c: 1577.3435 },
    VsopTerm { a: 990.0e-8, b: 5.233, c: 5884.927 },
    VsopTerm { a: 902.0e-8, b: 2.045, c: 26.298 },
    VsopTerm { a: 857.0e-8, b: 3.508, c: 398.149 },
    VsopTerm { a: 780.0e-8, b: 1.179, c: 5223.694 },
    VsopTerm { a: 753.0e-8, b: 2.533, c: 5507.553 },
    VsopTerm { a: 505.0e-8, b: 4.583, c: 18849.228 },
    VsopTerm { a: 492.0e-8, b: 4.205, c: 775.522 },
    VsopTerm { a: 357.0e-8, b: 2.920, c: 0.067 },
    VsopTerm { a: 317.0e-8, b: 5.849, c: 11790.629 },
    VsopTerm { a: 284.0e-8, b: 1.899, c: 796.298 },
    VsopTerm { a: 271.0e-8, b: 0.315, c: 10977.079 },
    VsopTerm { a: 243.0e-8, b: 0.345, c: 5486.778 },
    VsopTerm { a: 206.0e-8, b: 4.806, c: 2544.314 },
    VsopTerm { a: 205.0e-8, b: 1.869, c: 5573.143 },
    VsopTerm { a: 202.0e-8, b: 2.458, c: 6069.777 },
    VsopTerm { a: 156.0e-8, b: 0.833, c: 213.299 },
    VsopTerm { a: 132.0e-8, b: 3.411, c: 2942.463 },
    VsopTerm { a: 126.0e-8, b: 1.083, c: 20.775 },
    VsopTerm { a: 115.0e-8, b: 0.645, c: 0.980 },
    VsopTerm { a: 103.0e-8, b: 0.636, c: 4694.003 },
    VsopTerm { a: 99.0e-8, b: 6.21, c: 2146.17 },
    VsopTerm { a: 98.0e-8, b: 0.68, c: 155.42 },
    VsopTerm { a: 86.0e-8, b: 5.98, c: 161000.69 },
    VsopTerm { a: 85.0e-8, b: 1.30, c: 6275.96 },
    VsopTerm { a: 85.0e-8, b: 3.67, c: 71430.70 },
    VsopTerm { a: 80.0e-8, b: 1.81, c: 17260.15 },
];

#[rustfmt::skip]
pub(super) const L1: &[VsopTerm] = &[
    VsopTerm { a: 628331966747.0e-8, b: 0.000000000, c: 0.000000000 },
    VsopTerm { a: 206059.0e-8, b: 2.678235, c: 6283.075850 },
    VsopTerm { a: 4303.0e-8, b: 2.6351, c: 12566.1517 },
    VsopTerm { a: 425.0e-8, b: 1.590, c: 3.523 },
    VsopTerm { a: 119.0e-8, b: 5.796, c: 26.298 },
    VsopTerm { a: 109.0e-8, b: 2.966, c: 1577.344 },
    VsopTerm { a: 93.0e-8, b: 2.59, c: 18849.23 },
    VsopTerm { a: 72.0e-8, b: 1.14, c: 529.69 },
    VsopTerm { a: 68.0e-8, b: 1.87, c: 398.15 },
    VsopTerm { a: 67.0e-8, b: 4.41, c: 5507.55 },
    VsopTerm { a: 59.0e-8, b: 2.89, c: 5223.69 },
    VsopTerm { a: 56.0e-8, b: 2.17, c: 155.42 },
    VsopTerm { a: 45.0e-8, b: 0.40, c: 796.30 },
    VsopTerm { a: 36.0e-8, b: 0.47, c: 775.52 },
    VsopTerm { a: 29.0e-8, b: 2.65, c: 7.11 },
    VsopTerm { a: 21.0e-8, b: 5.34, c: 0.98 },
    VsopTerm { a: 19.0e-8, b: 1.85, c: 5486.78 },
    VsopTerm { a: 19.0e-8, b: 4.97, c: 213.30 },
    VsopTerm { a: 17.0e-8, b: 2.99, c: 6275.96 },
    VsopTerm { a: 16.0e-8, b: 0.03, c: 2544.31 },
    VsopTerm { a: 16.0e-8, b: 1.43, c: 2146.17 },
    VsopTerm { a: 15.0e-8, b: 1.21, c: 10977.08 },
    VsopTerm { a: 12.0e-8, b: 2.83, c: 1748.02 },
    VsopTerm { a: 12.0e-8, b: 3.26, c: 5088.63 },
    VsopTerm { a: 12.0e-8, b: 5.27, c: 1194.45 },
    VsopTerm { a: 12.0e-8, b: 2.08, c: 4694.00 },
    VsopTerm { a: 11.0e-8, b: 0.77, c: 553.57 },
    VsopTerm { a: 10.0e-8, b: 1.30, c: 6286.60 },
    VsopTerm { a: 10.0e-8, b: 4.24, c: 1349.87 },
    VsopTerm { a: 9.0e-8, b: 2.70, c: 242.73 },
    VsopTerm { a: 9.0e-8, b: 5.64, c: 951.72 },
    VsopTerm { a: 8.0e-8, b: 5.30, c: 2352.87 },
    VsopTerm { a: 6.0e-8, b: 2.65, c: 9437.76 },
    VsopTerm { a: 6.0e-8, b: 4.67, c: 4690.48 },
];

#[rustfmt::skip]
pub(super) const L2: &[VsopTerm] = &[
    VsopTerm { a: 52919.0e-8, b: 0.00000, c: 0.00000 },
    VsopTerm { a: 8720.0e-8, b: 1.0721, c: 6283.0758 },
    VsopTerm { a: 309.0e-8, b: 0.867, c: 12566.152 },
    VsopTerm { a: 27.0e-8, b: 0.05, c: 3.52 },
    VsopTerm { a: 16.0e-8, b: 5.19, c: 26.30 },
    VsopTerm { a: 16.0e-8, b: 3.68, c: 155.42 },
    VsopTerm { a: 10.0e-8, b: 0.76, c: 18849.23 },
    VsopTerm { a: 9.0e-8, b: 2.06, c: 77713.77 },
    VsopTerm { a: 7.0e-8, b: 0.83, c: 775.52 },
    VsopTerm { a: 5.0e-8, b: 4.66, c: 1577.34 },
    VsopTerm { a: 4.0e-8, b: 1.03, c: 7.11 },
    VsopTerm { a: 4.0e-8, b: 3.44, c: 5573.14 },
    VsopTerm { a: 3.0e-8, b: 5.14, c: 796.30 },
    VsopTerm { a: 3.0e-8, b: 6.05, c: 5507.55 },
    VsopTerm { a: 3.0e-8, b: 1.19, c: 242.73 },
    VsopTerm { a: 3.0e-8, b: 6.12, c: 529.69 },
    VsopTerm { a: 3.0e-8, b: 0.31, c: 398.15 },
    VsopTerm { a: 3.0e-8, b: 2.28, c: 553.57 },
    VsopTerm { a: 2.0e-8, b: 4.38, c: 5223.69 },
    VsopTerm { a: 2.0e-8, b: 3.75, c: 0.98 },
];

#[rustfmt::skip]
pub(super) const L3: &[VsopTerm] = &[
    VsopTerm { a: 289.0e-8, b: 5.844, c: 6283.076 },
    VsopTerm { a: 35.0e-8, b: 0.00, c: 0.00 },
    VsopTerm { a: 17.0e-8, b: 5.49, c: 12566.15 },
    VsopTerm { a: 3.0e-8, b: 5.20, c: 155.42 },
    VsopTerm { a: 1.0e-8, b: 4.72, c: 3.52 },
    VsopTerm { a: 1.0e-8, b: 5.30, c: 18849.23 },
    VsopTerm { a: 1.0e-8, b: 5.97, c: 242.73 },
];

#[rustfmt::skip]
pub(super) const L4: &[VsopTerm] = &[
    VsopTerm { a: 114.0e-8, b: 3.142, c: 0.000 },
    VsopTerm { a: 8.0e-8, b: 4.13, c: 6283.08 },
    VsopTerm { a: 1.0e-8, b: 3.84, c: 12566.15 },
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
    VsopTerm { a: 280.0e-8, b: 3.199, c: 84334.662 },
    VsopTerm { a: 102.0e-8, b: 5.422, c: 5507.553 },
    VsopTerm { a: 80.0e-8, b: 3.88, c: 5223.69 },
    VsopTerm { a: 44.0e-8, b: 3.70, c: 2352.87 },
    VsopTerm { a: 32.0e-8, b: 4.00, c: 1577.34 },
];

#[rustfmt::skip]
pub(super) const B1: &[VsopTerm] = &[
    VsopTerm { a: 9.0e-8, b: 3.90, c: 5507.55 },
    VsopTerm { a: 6.0e-8, b: 1.73, c: 5223.69 },
];

/// Latitude sub-series references.
pub(super) const B_SERIES: [&[VsopTerm]; 2] = [B0, B1];

// ===========================================================================
// Radius vector (R) series
// ===========================================================================

#[rustfmt::skip]
pub(super) const R0: &[VsopTerm] = &[
    VsopTerm { a: 100013989.0e-8, b: 0.0000000, c: 0.0000000 },
    VsopTerm { a: 1670700.0e-8, b: 3.0984635, c: 6283.0758500 },
    VsopTerm { a: 13956.0e-8, b: 3.05525, c: 12566.15170 },
    VsopTerm { a: 3084.0e-8, b: 5.1985, c: 77713.7715 },
    VsopTerm { a: 1628.0e-8, b: 1.1739, c: 5753.3849 },
    VsopTerm { a: 1576.0e-8, b: 2.8469, c: 7860.4194 },
    VsopTerm { a: 925.0e-8, b: 5.453, c: 11506.770 },
    VsopTerm { a: 542.0e-8, b: 4.564, c: 3930.210 },
    VsopTerm { a: 472.0e-8, b: 3.661, c: 5884.927 },
    VsopTerm { a: 346.0e-8, b: 0.964, c: 5507.553 },
    VsopTerm { a: 329.0e-8, b: 5.900, c: 5223.694 },
    VsopTerm { a: 307.0e-8, b: 0.299, c: 5573.143 },
    VsopTerm { a: 243.0e-8, b: 4.273, c: 11790.629 },
    VsopTerm { a: 212.0e-8, b: 5.847, c: 1577.344 },
    VsopTerm { a: 186.0e-8, b: 5.022, c: 10977.079 },
    VsopTerm { a: 175.0e-8, b: 3.012, c: 18849.228 },
    VsopTerm { a: 110.0e-8, b: 5.055, c: 5486.778 },
    VsopTerm { a: 98.0e-8, b: 0.89, c: 6069.78 },
    VsopTerm { a: 86.0e-8, b: 5.69, c: 15720.84 },
    VsopTerm { a: 86.0e-8, b: 1.27, c: 161000.69 },
    VsopTerm { a: 65.0e-8, b: 0.27, c: 17260.15 },
    VsopTerm { a: 63.0e-8, b: 0.92, c: 529.69 },
    VsopTerm { a: 57.0e-8, b: 2.01, c: 83996.85 },
    VsopTerm { a: 56.0e-8, b: 5.24, c: 71430.70 },
    VsopTerm { a: 49.0e-8, b: 3.25, c: 2544.31 },
    VsopTerm { a: 47.0e-8, b: 2.58, c: 775.52 },
    VsopTerm { a: 45.0e-8, b: 5.54, c: 9437.76 },
    VsopTerm { a: 43.0e-8, b: 6.01, c: 6275.96 },
    VsopTerm { a: 39.0e-8, b: 5.36, c: 4694.00 },
    VsopTerm { a: 38.0e-8, b: 2.39, c: 8827.39 },
    VsopTerm { a: 37.0e-8, b: 0.83, c: 19651.05 },
    VsopTerm { a: 37.0e-8, b: 4.90, c: 12139.55 },
    VsopTerm { a: 36.0e-8, b: 1.67, c: 12036.46 },
    VsopTerm { a: 35.0e-8, b: 1.84, c: 2942.46 },
    VsopTerm { a: 33.0e-8, b: 0.24, c: 7084.90 },
    VsopTerm { a: 32.0e-8, b: 0.18, c: 5088.63 },
    VsopTerm { a: 32.0e-8, b: 1.78, c: 398.15 },
    VsopTerm { a: 28.0e-8, b: 1.21, c: 6286.60 },
    VsopTerm { a: 28.0e-8, b: 1.90, c: 6279.55 },
    VsopTerm { a: 26.0e-8, b: 4.59, c: 10447.39 },
];

#[rustfmt::skip]
pub(super) const R1: &[VsopTerm] = &[
    VsopTerm { a: 103019.0e-8, b: 1.107490, c: 6283.075850 },
    VsopTerm { a: 1721.0e-8, b: 1.0644, c: 12566.1517 },
    VsopTerm { a: 702.0e-8, b: 3.142, c: 0.000 },
    VsopTerm { a: 32.0e-8, b: 1.02, c: 18849.23 },
    VsopTerm { a: 31.0e-8, b: 2.84, c: 5507.55 },
    VsopTerm { a: 25.0e-8, b: 1.32, c: 5223.69 },
    VsopTerm { a: 18.0e-8, b: 1.42, c: 1577.34 },
    VsopTerm { a: 10.0e-8, b: 5.91, c: 10977.08 },
    VsopTerm { a: 9.0e-8, b: 1.42, c: 6275.96 },
    VsopTerm { a: 9.0e-8, b: 0.27, c: 5486.78 },
];

#[rustfmt::skip]
pub(super) const R2: &[VsopTerm] = &[
    VsopTerm { a: 4359.0e-8, b: 5.7846, c: 6283.0758 },
    VsopTerm { a: 124.0e-8, b: 5.579, c: 12566.152 },
    VsopTerm { a: 12.0e-8, b: 3.14, c: 0.00 },
    VsopTerm { a: 9.0e-8, b: 3.63, c: 77713.77 },
    VsopTerm { a: 6.0e-8, b: 1.87, c: 5573.14 },
    VsopTerm { a: 3.0e-8, b: 5.47, c: 18849.23 },
];

#[rustfmt::skip]
pub(super) const R3: &[VsopTerm] = &[
    VsopTerm { a: 145.0e-8, b: 4.273, c: 6283.076 },
    VsopTerm { a: 7.0e-8, b: 3.92, c: 12566.15 },
];

#[rustfmt::skip]
pub(super) const R4: &[VsopTerm] = &[
    VsopTerm { a: 4.0e-8, b: 2.56, c: 6283.08 },
];

/// Radius sub-series references.
pub(super) const R_SERIES: [&[VsopTerm]; 5] = [R0, R1, R2, R3, R4];
