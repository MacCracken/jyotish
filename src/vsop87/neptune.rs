//! VSOP87D coefficients for Neptune.
//!
//! Heliocentric ecliptic longitude (L), latitude (B), and radius vector (R)
//! referred to the J2000.0 ecliptic and equinox.
//!
//! Source: Bretagnon & Francou (1988), Bureau des Longitudes VSOP87D.nep.

use super::VsopTerm;

// ===========================================================================
// Longitude (L) series
// ===========================================================================

#[rustfmt::skip]
pub(super) const L0: &[VsopTerm] = &[
    VsopTerm { a: 531188633.0e-8, b: 0.0000000, c: 0.0000000 },
    VsopTerm { a: 1798476.0e-8, b: 2.9010127, c: 38.1330356 },
    VsopTerm { a: 1019728.0e-8, b: 0.4858092, c: 1.4844727 },
    VsopTerm { a: 124532.0e-8, b: 4.830081, c: 36.648563 },
    VsopTerm { a: 42064.0e-8, b: 5.41055, c: 2.96895 },
    VsopTerm { a: 37715.0e-8, b: 6.09222, c: 35.16409 },
    VsopTerm { a: 33785.0e-8, b: 1.24489, c: 76.26607 },
    VsopTerm { a: 16482.0e-8, b: 0.00000, c: 491.55793 },
    VsopTerm { a: 9198.0e-8, b: 4.9375, c: 39.6175 },
    VsopTerm { a: 8994.0e-8, b: 0.2746, c: 175.1661 },
    VsopTerm { a: 4216.0e-8, b: 1.9871, c: 73.2971 },
    VsopTerm { a: 3365.0e-8, b: 1.0360, c: 33.6796 },
    VsopTerm { a: 2285.0e-8, b: 4.2061, c: 4.4534 },
    VsopTerm { a: 1434.0e-8, b: 2.7834, c: 74.7816 },
    VsopTerm { a: 900.0e-8, b: 2.076, c: 109.946 },
    VsopTerm { a: 745.0e-8, b: 3.190, c: 71.813 },
    VsopTerm { a: 506.0e-8, b: 5.748, c: 114.399 },
    VsopTerm { a: 400.0e-8, b: 0.350, c: 1021.249 },
    VsopTerm { a: 345.0e-8, b: 3.462, c: 41.102 },
    VsopTerm { a: 340.0e-8, b: 3.304, c: 77.751 },
];

#[rustfmt::skip]
pub(super) const L1: &[VsopTerm] = &[
    VsopTerm { a: 3837687717.0e-8, b: 0.0000000000, c: 0.0000000000 },
    VsopTerm { a: 16604.0e-8, b: 4.86319, c: 38.13304 },
    VsopTerm { a: 15807.0e-8, b: 2.27923, c: 1.48447 },
    VsopTerm { a: 3335.0e-8, b: 3.6820, c: 76.2661 },
    VsopTerm { a: 1306.0e-8, b: 3.6732, c: 2.9689 },
    VsopTerm { a: 605.0e-8, b: 1.505, c: 35.164 },
    VsopTerm { a: 179.0e-8, b: 3.453, c: 39.618 },
    VsopTerm { a: 107.0e-8, b: 2.451, c: 4.453 },
];

#[rustfmt::skip]
pub(super) const L2: &[VsopTerm] = &[
    VsopTerm { a: 53893.0e-8, b: 0.00000, c: 0.00000 },
    VsopTerm { a: 296.0e-8, b: 1.855, c: 1.484 },
    VsopTerm { a: 281.0e-8, b: 1.191, c: 38.133 },
    VsopTerm { a: 270.0e-8, b: 5.721, c: 76.266 },
    VsopTerm { a: 23.0e-8, b: 1.21, c: 2.97 },
];

#[rustfmt::skip]
pub(super) const L3: &[VsopTerm] = &[
    VsopTerm { a: 31.0e-8, b: 0.00, c: 0.00 },
    VsopTerm { a: 15.0e-8, b: 1.35, c: 76.27 },
    VsopTerm { a: 12.0e-8, b: 6.04, c: 1.48 },
    VsopTerm { a: 12.0e-8, b: 6.11, c: 38.13 },
];

#[rustfmt::skip]
pub(super) const L4: &[VsopTerm] = &[
    VsopTerm { a: 114.0e-8, b: 3.1416, c: 0.0000 },
];

/// Longitude sub-series references.
pub(super) const L_SERIES: [&[VsopTerm]; 5] = [L0, L1, L2, L3, L4];

// ===========================================================================
// Latitude (B) series
// ===========================================================================

#[rustfmt::skip]
pub(super) const B0: &[VsopTerm] = &[
    VsopTerm { a: 3088623.0e-8, b: 1.4410437, c: 38.1330356 },
    VsopTerm { a: 27780.0e-8, b: 5.91272, c: 76.26607 },
    VsopTerm { a: 27624.0e-8, b: 0.00000, c: 0.00000 },
    VsopTerm { a: 15449.0e-8, b: 3.50877, c: 39.61751 },
    VsopTerm { a: 15355.0e-8, b: 2.52124, c: 36.64856 },
    VsopTerm { a: 2000.0e-8, b: 1.5100, c: 74.7816 },
    VsopTerm { a: 1968.0e-8, b: 4.3778, c: 1.4845 },
    VsopTerm { a: 1015.0e-8, b: 3.2156, c: 35.1641 },
    VsopTerm { a: 606.0e-8, b: 2.802, c: 73.297 },
    VsopTerm { a: 597.0e-8, b: 4.212, c: 41.102 },
    VsopTerm { a: 570.0e-8, b: 3.851, c: 114.399 },
    VsopTerm { a: 549.0e-8, b: 5.573, c: 2.969 },
];

#[rustfmt::skip]
pub(super) const B1: &[VsopTerm] = &[
    VsopTerm { a: 227279.0e-8, b: 3.807931, c: 38.133036 },
    VsopTerm { a: 1803.0e-8, b: 1.9758, c: 76.2661 },
    VsopTerm { a: 1433.0e-8, b: 3.1416, c: 0.0000 },
    VsopTerm { a: 1386.0e-8, b: 4.6256, c: 36.6486 },
    VsopTerm { a: 1073.0e-8, b: 6.0805, c: 39.6175 },
    VsopTerm { a: 148.0e-8, b: 3.858, c: 74.782 },
    VsopTerm { a: 136.0e-8, b: 0.478, c: 1.484 },
];

#[rustfmt::skip]
pub(super) const B2: &[VsopTerm] = &[
    VsopTerm { a: 9691.0e-8, b: 5.5712, c: 38.1330 },
    VsopTerm { a: 79.0e-8, b: 3.63, c: 76.27 },
    VsopTerm { a: 72.0e-8, b: 0.45, c: 36.65 },
    VsopTerm { a: 59.0e-8, b: 3.14, c: 0.00 },
    VsopTerm { a: 30.0e-8, b: 1.61, c: 39.62 },
];

#[rustfmt::skip]
pub(super) const B3: &[VsopTerm] = &[
    VsopTerm { a: 273.0e-8, b: 1.017, c: 38.133 },
    VsopTerm { a: 2.0e-8, b: 0.00, c: 0.00 },
];

/// Latitude sub-series references.
pub(super) const B_SERIES: [&[VsopTerm]; 4] = [B0, B1, B2, B3];

// ===========================================================================
// Radius vector (R) series
// ===========================================================================

#[rustfmt::skip]
pub(super) const R0: &[VsopTerm] = &[
    VsopTerm { a: 3007013206.0e-8, b: 0.0000000, c: 0.0000000 },
    VsopTerm { a: 27062259.0e-8, b: 1.32999459, c: 38.13303564 },
    VsopTerm { a: 1691764.0e-8, b: 3.2518614, c: 36.6485629 },
    VsopTerm { a: 807831.0e-8, b: 5.185928, c: 1.484473 },
    VsopTerm { a: 537761.0e-8, b: 4.521139, c: 35.164090 },
    VsopTerm { a: 495726.0e-8, b: 1.571057, c: 491.557929 },
    VsopTerm { a: 274572.0e-8, b: 1.845523, c: 175.166060 },
    VsopTerm { a: 135134.0e-8, b: 3.372206, c: 39.617508 },
    VsopTerm { a: 121802.0e-8, b: 5.797544, c: 76.266071 },
    VsopTerm { a: 100895.0e-8, b: 0.377027, c: 73.297126 },
    VsopTerm { a: 69792.0e-8, b: 3.79617, c: 2.96895 },
    VsopTerm { a: 46688.0e-8, b: 5.74938, c: 33.67962 },
    VsopTerm { a: 24594.0e-8, b: 0.50802, c: 109.94569 },
    VsopTerm { a: 16939.0e-8, b: 1.59422, c: 71.81265 },
    VsopTerm { a: 14230.0e-8, b: 1.07786, c: 74.78160 },
    VsopTerm { a: 12012.0e-8, b: 1.92062, c: 1021.24889 },
    VsopTerm { a: 8395.0e-8, b: 0.6782, c: 146.5943 },
    VsopTerm { a: 7572.0e-8, b: 1.0715, c: 388.4652 },
    VsopTerm { a: 5721.0e-8, b: 2.5906, c: 4.4534 },
    VsopTerm { a: 4840.0e-8, b: 1.9069, c: 41.1020 },
    VsopTerm { a: 4483.0e-8, b: 2.9057, c: 529.6910 },
    VsopTerm { a: 4421.0e-8, b: 1.7499, c: 108.4612 },
    VsopTerm { a: 4354.0e-8, b: 0.6799, c: 114.3991 },
    VsopTerm { a: 4270.0e-8, b: 3.4134, c: 453.4249 },
];

#[rustfmt::skip]
pub(super) const R1: &[VsopTerm] = &[
    VsopTerm { a: 236339.0e-8, b: 0.704980, c: 38.133036 },
    VsopTerm { a: 13220.0e-8, b: 3.32015, c: 1.48447 },
    VsopTerm { a: 8622.0e-8, b: 6.21630, c: 35.16409 },
    VsopTerm { a: 2702.0e-8, b: 1.8814, c: 39.6175 },
    VsopTerm { a: 2155.0e-8, b: 2.0943, c: 2.9689 },
    VsopTerm { a: 2153.0e-8, b: 5.1687, c: 76.2661 },
    VsopTerm { a: 1603.0e-8, b: 0.0000, c: 0.0000 },
    VsopTerm { a: 1464.0e-8, b: 1.1842, c: 33.6796 },
    VsopTerm { a: 1136.0e-8, b: 3.9189, c: 36.6486 },
];

#[rustfmt::skip]
pub(super) const R2: &[VsopTerm] = &[
    VsopTerm { a: 4247.0e-8, b: 5.8991, c: 38.1330 },
    VsopTerm { a: 218.0e-8, b: 0.346, c: 1.484 },
    VsopTerm { a: 163.0e-8, b: 2.239, c: 76.266 },
    VsopTerm { a: 156.0e-8, b: 4.594, c: 36.649 },
    VsopTerm { a: 127.0e-8, b: 2.848, c: 35.164 },
];

#[rustfmt::skip]
pub(super) const R3: &[VsopTerm] = &[
    VsopTerm { a: 166.0e-8, b: 4.552, c: 38.133 },
    VsopTerm { a: 15.0e-8, b: 4.85, c: 1.48 },
    VsopTerm { a: 4.0e-8, b: 5.47, c: 36.65 },
];

/// Radius sub-series references.
pub(super) const R_SERIES: [&[VsopTerm]; 4] = [R0, R1, R2, R3];
