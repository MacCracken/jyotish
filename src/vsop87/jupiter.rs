//! VSOP87D coefficients for Jupiter.
//!
//! Heliocentric ecliptic longitude (L), latitude (B), and radius vector (R)
//! referred to the J2000.0 ecliptic and equinox.
//!
//! Source: Bretagnon & Francou (1988), Bureau des Longitudes VSOP87D.jup.

use super::VsopTerm;

// ===========================================================================
// Longitude (L) series
// ===========================================================================

#[rustfmt::skip]
pub(super) const L0: &[VsopTerm] = &[
    VsopTerm { a: 59954691.0e-8, b: 0.0000000, c: 0.0000000 },
    VsopTerm { a: 9695899.0e-8, b: 5.0619179, c: 529.6909651 },
    VsopTerm { a: 573610.0e-8, b: 1.44406, c: 1059.38193 },
    VsopTerm { a: 306389.0e-8, b: 5.41735, c: 522.57742 },
    VsopTerm { a: 97178.0e-8, b: 4.14265, c: 1589.07290 },
    VsopTerm { a: 72903.0e-8, b: 3.64043, c: 522.57742 },
    VsopTerm { a: 64264.0e-8, b: 3.41145, c: 103.09277 },
    VsopTerm { a: 39806.0e-8, b: 2.29377, c: 419.48464 },
    VsopTerm { a: 38858.0e-8, b: 1.27232, c: 316.39187 },
    VsopTerm { a: 27965.0e-8, b: 1.78455, c: 536.80451 },
    VsopTerm { a: 13590.0e-8, b: 5.77481, c: 1589.07290 },
    VsopTerm { a: 8769.0e-8, b: 3.6300, c: 949.1756 },
    VsopTerm { a: 8246.0e-8, b: 3.5823, c: 206.1855 },
    VsopTerm { a: 7368.0e-8, b: 5.0810, c: 735.8765 },
    VsopTerm { a: 6263.0e-8, b: 0.0250, c: 213.2991 },
    VsopTerm { a: 6114.0e-8, b: 4.5132, c: 1162.4747 },
    VsopTerm { a: 5305.0e-8, b: 1.3067, c: 14.2271 },
    VsopTerm { a: 5305.0e-8, b: 4.1863, c: 1052.2684 },
    VsopTerm { a: 4905.0e-8, b: 1.3208, c: 110.2063 },
    VsopTerm { a: 4647.0e-8, b: 4.6996, c: 3.9322 },
    VsopTerm { a: 3045.0e-8, b: 4.3168, c: 426.5982 },
    VsopTerm { a: 2610.0e-8, b: 1.5667, c: 846.0828 },
    VsopTerm { a: 2028.0e-8, b: 1.0638, c: 3.1814 },
    VsopTerm { a: 1921.0e-8, b: 0.9717, c: 639.8973 },
    VsopTerm { a: 1765.0e-8, b: 2.1415, c: 1066.4955 },
    VsopTerm { a: 1723.0e-8, b: 3.8804, c: 1265.5675 },
    VsopTerm { a: 1633.0e-8, b: 3.5820, c: 515.4639 },
    VsopTerm { a: 1432.0e-8, b: 4.2968, c: 625.6702 },
    VsopTerm { a: 973.0e-8, b: 4.098, c: 95.979 },
    VsopTerm { a: 884.0e-8, b: 2.437, c: 412.371 },
    VsopTerm { a: 733.0e-8, b: 6.085, c: 838.969 },
    VsopTerm { a: 731.0e-8, b: 3.806, c: 1581.959 },
    VsopTerm { a: 709.0e-8, b: 1.293, c: 742.990 },
    VsopTerm { a: 692.0e-8, b: 6.134, c: 2118.764 },
    VsopTerm { a: 614.0e-8, b: 4.109, c: 1478.867 },
    VsopTerm { a: 582.0e-8, b: 4.540, c: 309.278 },
    VsopTerm { a: 495.0e-8, b: 3.756, c: 323.505 },
    VsopTerm { a: 441.0e-8, b: 2.958, c: 454.909 },
    VsopTerm { a: 417.0e-8, b: 1.036, c: 2.448 },
    VsopTerm { a: 390.0e-8, b: 4.897, c: 1692.166 },
    VsopTerm { a: 376.0e-8, b: 4.703, c: 1368.660 },
    VsopTerm { a: 341.0e-8, b: 5.715, c: 533.623 },
    VsopTerm { a: 330.0e-8, b: 4.740, c: 0.048 },
    VsopTerm { a: 262.0e-8, b: 1.877, c: 0.963 },
    VsopTerm { a: 261.0e-8, b: 0.820, c: 380.128 },
    VsopTerm { a: 257.0e-8, b: 3.724, c: 199.072 },
    VsopTerm { a: 244.0e-8, b: 5.220, c: 728.763 },
    VsopTerm { a: 235.0e-8, b: 1.227, c: 909.819 },
    VsopTerm { a: 220.0e-8, b: 1.651, c: 543.918 },
    VsopTerm { a: 207.0e-8, b: 1.855, c: 525.759 },
];

#[rustfmt::skip]
pub(super) const L1: &[VsopTerm] = &[
    VsopTerm { a: 52993480757.0e-8, b: 0.00000000000, c: 0.00000000000 },
    VsopTerm { a: 489741.0e-8, b: 4.220667, c: 529.690965 },
    VsopTerm { a: 22814.0e-8, b: 3.98454, c: 7.11355 },
    VsopTerm { a: 19928.0e-8, b: 5.36071, c: 522.57742 },
    VsopTerm { a: 17749.0e-8, b: 2.93114, c: 1059.38193 },
    VsopTerm { a: 10606.0e-8, b: 3.70298, c: 1589.07290 },
    VsopTerm { a: 8683.0e-8, b: 1.2722, c: 6283.0758 },
    VsopTerm { a: 4022.0e-8, b: 3.4425, c: 1265.5675 },
    VsopTerm { a: 3629.0e-8, b: 3.0827, c: 110.2063 },
    VsopTerm { a: 2742.0e-8, b: 0.3975, c: 103.0928 },
    VsopTerm { a: 2142.0e-8, b: 1.8075, c: 419.4846 },
    VsopTerm { a: 1772.0e-8, b: 5.2437, c: 316.3919 },
    VsopTerm { a: 1630.0e-8, b: 0.8144, c: 515.4639 },
    VsopTerm { a: 1002.0e-8, b: 3.1493, c: 536.8045 },
    VsopTerm { a: 696.0e-8, b: 0.388, c: 735.877 },
    VsopTerm { a: 623.0e-8, b: 4.338, c: 1478.867 },
    VsopTerm { a: 476.0e-8, b: 2.524, c: 206.186 },
    VsopTerm { a: 455.0e-8, b: 2.470, c: 213.299 },
    VsopTerm { a: 442.0e-8, b: 1.249, c: 1162.475 },
    VsopTerm { a: 422.0e-8, b: 6.109, c: 949.176 },
    VsopTerm { a: 383.0e-8, b: 3.324, c: 1581.959 },
    VsopTerm { a: 340.0e-8, b: 3.211, c: 625.670 },
    VsopTerm { a: 299.0e-8, b: 2.186, c: 846.083 },
    VsopTerm { a: 294.0e-8, b: 4.382, c: 2118.764 },
    VsopTerm { a: 252.0e-8, b: 3.523, c: 1066.495 },
    VsopTerm { a: 188.0e-8, b: 2.073, c: 14.227 },
    VsopTerm { a: 161.0e-8, b: 1.032, c: 1052.268 },
    VsopTerm { a: 129.0e-8, b: 5.779, c: 1045.155 },
    VsopTerm { a: 119.0e-8, b: 4.999, c: 412.371 },
    VsopTerm { a: 109.0e-8, b: 5.089, c: 639.897 },
];

#[rustfmt::skip]
pub(super) const L2: &[VsopTerm] = &[
    VsopTerm { a: 47234.0e-8, b: 4.32148, c: 7.11355 },
    VsopTerm { a: 38966.0e-8, b: 0.00000, c: 0.00000 },
    VsopTerm { a: 30629.0e-8, b: 2.93021, c: 529.69097 },
    VsopTerm { a: 3189.0e-8, b: 1.0555, c: 522.5774 },
    VsopTerm { a: 2729.0e-8, b: 4.8455, c: 1059.3819 },
    VsopTerm { a: 2723.0e-8, b: 3.4141, c: 1589.0729 },
    VsopTerm { a: 1721.0e-8, b: 4.1873, c: 14.2271 },
    VsopTerm { a: 383.0e-8, b: 5.768, c: 419.485 },
    VsopTerm { a: 378.0e-8, b: 0.760, c: 515.464 },
    VsopTerm { a: 367.0e-8, b: 6.055, c: 103.093 },
    VsopTerm { a: 337.0e-8, b: 3.786, c: 3.181 },
    VsopTerm { a: 308.0e-8, b: 0.694, c: 206.186 },
    VsopTerm { a: 218.0e-8, b: 3.814, c: 1265.567 },
    VsopTerm { a: 199.0e-8, b: 5.340, c: 1066.495 },
    VsopTerm { a: 197.0e-8, b: 2.484, c: 3.932 },
    VsopTerm { a: 156.0e-8, b: 1.406, c: 1045.155 },
    VsopTerm { a: 146.0e-8, b: 3.814, c: 110.206 },
    VsopTerm { a: 142.0e-8, b: 1.634, c: 536.805 },
    VsopTerm { a: 130.0e-8, b: 5.837, c: 316.392 },
];

#[rustfmt::skip]
pub(super) const L3: &[VsopTerm] = &[
    VsopTerm { a: 6502.0e-8, b: 2.5986, c: 7.1135 },
    VsopTerm { a: 1357.0e-8, b: 1.3464, c: 529.6910 },
    VsopTerm { a: 471.0e-8, b: 2.475, c: 14.227 },
    VsopTerm { a: 417.0e-8, b: 3.245, c: 536.805 },
    VsopTerm { a: 353.0e-8, b: 2.974, c: 522.577 },
    VsopTerm { a: 155.0e-8, b: 2.076, c: 1059.382 },
    VsopTerm { a: 87.0e-8, b: 2.51, c: 515.46 },
    VsopTerm { a: 44.0e-8, b: 0.00, c: 0.00 },
    VsopTerm { a: 34.0e-8, b: 3.83, c: 1066.50 },
    VsopTerm { a: 28.0e-8, b: 2.45, c: 206.19 },
    VsopTerm { a: 24.0e-8, b: 1.28, c: 110.21 },
    VsopTerm { a: 23.0e-8, b: 2.98, c: 14.23 },
];

#[rustfmt::skip]
pub(super) const L4: &[VsopTerm] = &[
    VsopTerm { a: 669.0e-8, b: 0.853, c: 7.114 },
    VsopTerm { a: 114.0e-8, b: 3.142, c: 0.000 },
    VsopTerm { a: 100.0e-8, b: 0.743, c: 14.227 },
    VsopTerm { a: 50.0e-8, b: 1.65, c: 536.80 },
    VsopTerm { a: 44.0e-8, b: 5.82, c: 529.69 },
    VsopTerm { a: 32.0e-8, b: 4.56, c: 522.58 },
    VsopTerm { a: 15.0e-8, b: 4.55, c: 515.46 },
    VsopTerm { a: 9.0e-8, b: 0.48, c: 7.11 },
];

#[rustfmt::skip]
pub(super) const L5: &[VsopTerm] = &[
    VsopTerm { a: 50.0e-8, b: 5.26, c: 7.11 },
    VsopTerm { a: 16.0e-8, b: 5.25, c: 14.23 },
    VsopTerm { a: 4.0e-8, b: 0.01, c: 536.80 },
    VsopTerm { a: 2.0e-8, b: 3.14, c: 0.00 },
];

/// Longitude sub-series references.
pub(super) const L_SERIES: [&[VsopTerm]; 6] = [L0, L1, L2, L3, L4, L5];

// ===========================================================================
// Latitude (B) series
// ===========================================================================

#[rustfmt::skip]
pub(super) const B0: &[VsopTerm] = &[
    VsopTerm { a: 2268616.0e-8, b: 3.5585261, c: 529.6909651 },
    VsopTerm { a: 110090.0e-8, b: 0.00000, c: 0.00000 },
    VsopTerm { a: 109972.0e-8, b: 3.90809, c: 1059.38193 },
    VsopTerm { a: 8101.0e-8, b: 3.6051, c: 522.5774 },
    VsopTerm { a: 6438.0e-8, b: 0.3063, c: 536.8045 },
    VsopTerm { a: 6044.0e-8, b: 4.2588, c: 1589.0729 },
    VsopTerm { a: 1107.0e-8, b: 2.9853, c: 1162.4747 },
    VsopTerm { a: 944.0e-8, b: 1.675, c: 426.598 },
    VsopTerm { a: 942.0e-8, b: 2.936, c: 1052.268 },
    VsopTerm { a: 894.0e-8, b: 1.754, c: 7.114 },
    VsopTerm { a: 836.0e-8, b: 5.179, c: 103.093 },
    VsopTerm { a: 767.0e-8, b: 2.155, c: 632.784 },
    VsopTerm { a: 684.0e-8, b: 3.678, c: 213.299 },
    VsopTerm { a: 629.0e-8, b: 0.643, c: 1066.495 },
    VsopTerm { a: 559.0e-8, b: 0.014, c: 846.083 },
    VsopTerm { a: 532.0e-8, b: 2.703, c: 110.206 },
    VsopTerm { a: 464.0e-8, b: 1.173, c: 949.176 },
    VsopTerm { a: 431.0e-8, b: 2.608, c: 419.485 },
    VsopTerm { a: 351.0e-8, b: 4.611, c: 2118.764 },
    VsopTerm { a: 132.0e-8, b: 4.778, c: 742.990 },
    VsopTerm { a: 123.0e-8, b: 3.350, c: 1692.166 },
    VsopTerm { a: 116.0e-8, b: 1.387, c: 323.505 },
    VsopTerm { a: 115.0e-8, b: 5.049, c: 316.392 },
    VsopTerm { a: 104.0e-8, b: 3.701, c: 515.464 },
    VsopTerm { a: 103.0e-8, b: 2.319, c: 1478.867 },
];

#[rustfmt::skip]
pub(super) const B1: &[VsopTerm] = &[
    VsopTerm { a: 177352.0e-8, b: 5.70166, c: 529.69097 },
    VsopTerm { a: 3230.0e-8, b: 5.7794, c: 1059.3819 },
    VsopTerm { a: 3081.0e-8, b: 5.4746, c: 522.5774 },
    VsopTerm { a: 2212.0e-8, b: 4.7348, c: 536.8045 },
    VsopTerm { a: 1694.0e-8, b: 3.1416, c: 0.0000 },
    VsopTerm { a: 346.0e-8, b: 4.746, c: 1052.268 },
    VsopTerm { a: 234.0e-8, b: 5.189, c: 1066.495 },
    VsopTerm { a: 196.0e-8, b: 6.186, c: 7.114 },
    VsopTerm { a: 150.0e-8, b: 3.927, c: 1589.073 },
    VsopTerm { a: 114.0e-8, b: 3.439, c: 632.784 },
    VsopTerm { a: 97.0e-8, b: 2.91, c: 949.18 },
    VsopTerm { a: 82.0e-8, b: 5.08, c: 1162.47 },
    VsopTerm { a: 77.0e-8, b: 2.51, c: 103.09 },
    VsopTerm { a: 60.0e-8, b: 1.91, c: 419.48 },
];

#[rustfmt::skip]
pub(super) const B2: &[VsopTerm] = &[
    VsopTerm { a: 8094.0e-8, b: 1.4632, c: 529.6910 },
    VsopTerm { a: 813.0e-8, b: 3.1416, c: 0.0000 },
    VsopTerm { a: 742.0e-8, b: 0.957, c: 522.577 },
    VsopTerm { a: 399.0e-8, b: 2.899, c: 536.805 },
    VsopTerm { a: 342.0e-8, b: 1.447, c: 1059.382 },
    VsopTerm { a: 74.0e-8, b: 0.41, c: 1052.27 },
    VsopTerm { a: 46.0e-8, b: 3.48, c: 1066.50 },
    VsopTerm { a: 30.0e-8, b: 1.93, c: 1589.07 },
    VsopTerm { a: 29.0e-8, b: 0.99, c: 515.46 },
    VsopTerm { a: 23.0e-8, b: 4.27, c: 7.11 },
];

#[rustfmt::skip]
pub(super) const B3: &[VsopTerm] = &[
    VsopTerm { a: 252.0e-8, b: 3.381, c: 529.691 },
    VsopTerm { a: 122.0e-8, b: 2.733, c: 522.577 },
    VsopTerm { a: 49.0e-8, b: 1.04, c: 536.80 },
    VsopTerm { a: 11.0e-8, b: 2.31, c: 1059.38 },
    VsopTerm { a: 8.0e-8, b: 2.77, c: 515.46 },
    VsopTerm { a: 7.0e-8, b: 4.25, c: 1066.50 },
    VsopTerm { a: 6.0e-8, b: 1.78, c: 1052.27 },
];

#[rustfmt::skip]
pub(super) const B4: &[VsopTerm] = &[
    VsopTerm { a: 15.0e-8, b: 4.53, c: 522.58 },
    VsopTerm { a: 5.0e-8, b: 4.47, c: 529.69 },
    VsopTerm { a: 4.0e-8, b: 5.44, c: 536.80 },
    VsopTerm { a: 3.0e-8, b: 0.00, c: 0.00 },
];

/// Latitude sub-series references.
pub(super) const B_SERIES: [&[VsopTerm]; 5] = [B0, B1, B2, B3, B4];

// ===========================================================================
// Radius vector (R) series
// ===========================================================================

#[rustfmt::skip]
pub(super) const R0: &[VsopTerm] = &[
    VsopTerm { a: 520887429.0e-8, b: 0.0000000, c: 0.0000000 },
    VsopTerm { a: 25209327.0e-8, b: 3.4910864, c: 529.6909651 },
    VsopTerm { a: 610600.0e-8, b: 3.84116, c: 1059.38193 },
    VsopTerm { a: 282029.0e-8, b: 2.57420, c: 632.78374 },
    VsopTerm { a: 187647.0e-8, b: 2.07590, c: 522.57742 },
    VsopTerm { a: 86793.0e-8, b: 0.71001, c: 419.48464 },
    VsopTerm { a: 72063.0e-8, b: 0.21466, c: 536.80451 },
    VsopTerm { a: 65517.0e-8, b: 5.97996, c: 316.39187 },
    VsopTerm { a: 30135.0e-8, b: 2.16132, c: 949.17561 },
    VsopTerm { a: 29135.0e-8, b: 1.67759, c: 103.09277 },
    VsopTerm { a: 23947.0e-8, b: 0.27458, c: 7.11355 },
    VsopTerm { a: 23453.0e-8, b: 3.54023, c: 735.87651 },
    VsopTerm { a: 22284.0e-8, b: 4.19363, c: 1589.07290 },
    VsopTerm { a: 13033.0e-8, b: 2.96043, c: 1162.47470 },
    VsopTerm { a: 12749.0e-8, b: 2.71550, c: 1052.26838 },
    VsopTerm { a: 9703.0e-8, b: 1.9067, c: 206.1855 },
    VsopTerm { a: 9161.0e-8, b: 4.4135, c: 213.2991 },
    VsopTerm { a: 7895.0e-8, b: 2.4791, c: 426.5982 },
    VsopTerm { a: 7058.0e-8, b: 2.1818, c: 1265.5675 },
    VsopTerm { a: 6138.0e-8, b: 6.2642, c: 846.0828 },
    VsopTerm { a: 5477.0e-8, b: 5.6573, c: 639.8973 },
    VsopTerm { a: 4170.0e-8, b: 2.0161, c: 515.4639 },
    VsopTerm { a: 4137.0e-8, b: 2.7222, c: 625.6702 },
    VsopTerm { a: 3503.0e-8, b: 0.5653, c: 1066.4955 },
    VsopTerm { a: 2617.0e-8, b: 2.0099, c: 1581.9593 },
    VsopTerm { a: 2500.0e-8, b: 4.5518, c: 838.9693 },
    VsopTerm { a: 2128.0e-8, b: 6.1275, c: 742.9901 },
    VsopTerm { a: 1912.0e-8, b: 0.8562, c: 412.3711 },
    VsopTerm { a: 1611.0e-8, b: 3.0887, c: 1368.6603 },
    VsopTerm { a: 1479.0e-8, b: 2.6803, c: 1478.8666 },
    VsopTerm { a: 1231.0e-8, b: 1.8904, c: 323.5054 },
    VsopTerm { a: 1217.0e-8, b: 1.8017, c: 110.2063 },
    VsopTerm { a: 1015.0e-8, b: 1.3867, c: 454.9094 },
    VsopTerm { a: 999.0e-8, b: 2.872, c: 309.278 },
    VsopTerm { a: 961.0e-8, b: 4.549, c: 2118.764 },
    VsopTerm { a: 886.0e-8, b: 4.148, c: 533.623 },
    VsopTerm { a: 821.0e-8, b: 1.593, c: 1898.351 },
    VsopTerm { a: 812.0e-8, b: 5.941, c: 909.819 },
    VsopTerm { a: 777.0e-8, b: 3.677, c: 728.763 },
    VsopTerm { a: 727.0e-8, b: 3.988, c: 1155.361 },
    VsopTerm { a: 655.0e-8, b: 2.791, c: 1685.052 },
    VsopTerm { a: 654.0e-8, b: 3.382, c: 1692.166 },
    VsopTerm { a: 621.0e-8, b: 4.823, c: 956.289 },
    VsopTerm { a: 615.0e-8, b: 2.276, c: 942.062 },
    VsopTerm { a: 562.0e-8, b: 0.081, c: 543.918 },
    VsopTerm { a: 542.0e-8, b: 0.284, c: 525.759 },
];

#[rustfmt::skip]
pub(super) const R1: &[VsopTerm] = &[
    VsopTerm { a: 1271802.0e-8, b: 2.6493751, c: 529.6909651 },
    VsopTerm { a: 61662.0e-8, b: 3.00076, c: 1059.38193 },
    VsopTerm { a: 53444.0e-8, b: 3.89718, c: 522.57742 },
    VsopTerm { a: 41390.0e-8, b: 0.00000, c: 0.00000 },
    VsopTerm { a: 31185.0e-8, b: 4.88277, c: 536.80451 },
    VsopTerm { a: 11847.0e-8, b: 2.41330, c: 419.48464 },
    VsopTerm { a: 9166.0e-8, b: 4.7598, c: 7.1135 },
    VsopTerm { a: 3404.0e-8, b: 3.3469, c: 1589.0729 },
    VsopTerm { a: 3203.0e-8, b: 5.2108, c: 735.8765 },
    VsopTerm { a: 3176.0e-8, b: 2.7930, c: 103.0928 },
    VsopTerm { a: 2806.0e-8, b: 3.7422, c: 515.4639 },
    VsopTerm { a: 2677.0e-8, b: 4.3305, c: 1066.4955 },
    VsopTerm { a: 2600.0e-8, b: 3.6344, c: 206.1855 },
    VsopTerm { a: 2412.0e-8, b: 1.4695, c: 426.5982 },
    VsopTerm { a: 2174.0e-8, b: 4.8534, c: 949.1756 },
    VsopTerm { a: 1820.0e-8, b: 1.8480, c: 110.2063 },
    VsopTerm { a: 1633.0e-8, b: 2.6234, c: 1162.4747 },
    VsopTerm { a: 1516.0e-8, b: 6.2427, c: 1052.2684 },
    VsopTerm { a: 1399.0e-8, b: 0.7315, c: 625.6702 },
    VsopTerm { a: 1283.0e-8, b: 3.0845, c: 846.0828 },
    VsopTerm { a: 1217.0e-8, b: 2.0115, c: 316.3919 },
    VsopTerm { a: 1086.0e-8, b: 5.6330, c: 309.2783 },
    VsopTerm { a: 1024.0e-8, b: 1.4672, c: 14.2271 },
    VsopTerm { a: 1015.0e-8, b: 3.2165, c: 323.5054 },
];

#[rustfmt::skip]
pub(super) const R2: &[VsopTerm] = &[
    VsopTerm { a: 79645.0e-8, b: 1.35866, c: 529.69097 },
    VsopTerm { a: 8252.0e-8, b: 5.7777, c: 522.5774 },
    VsopTerm { a: 7030.0e-8, b: 3.2748, c: 536.8045 },
    VsopTerm { a: 5314.0e-8, b: 1.8384, c: 1059.3819 },
    VsopTerm { a: 1861.0e-8, b: 2.9768, c: 7.1135 },
    VsopTerm { a: 964.0e-8, b: 5.480, c: 515.464 },
    VsopTerm { a: 836.0e-8, b: 4.199, c: 419.485 },
    VsopTerm { a: 498.0e-8, b: 3.142, c: 0.000 },
    VsopTerm { a: 427.0e-8, b: 2.228, c: 639.897 },
    VsopTerm { a: 406.0e-8, b: 3.783, c: 1066.495 },
    VsopTerm { a: 377.0e-8, b: 2.242, c: 1589.073 },
    VsopTerm { a: 363.0e-8, b: 5.368, c: 206.186 },
    VsopTerm { a: 342.0e-8, b: 6.099, c: 1052.268 },
    VsopTerm { a: 339.0e-8, b: 6.127, c: 625.670 },
    VsopTerm { a: 333.0e-8, b: 0.003, c: 426.598 },
    VsopTerm { a: 280.0e-8, b: 4.262, c: 412.371 },
    VsopTerm { a: 257.0e-8, b: 0.963, c: 632.784 },
    VsopTerm { a: 230.0e-8, b: 0.705, c: 735.877 },
    VsopTerm { a: 201.0e-8, b: 3.069, c: 543.918 },
    VsopTerm { a: 200.0e-8, b: 4.429, c: 103.093 },
    VsopTerm { a: 139.0e-8, b: 2.932, c: 14.227 },
];

#[rustfmt::skip]
pub(super) const R3: &[VsopTerm] = &[
    VsopTerm { a: 3519.0e-8, b: 6.058, c: 529.691 },
    VsopTerm { a: 1073.0e-8, b: 1.673, c: 536.805 },
    VsopTerm { a: 916.0e-8, b: 1.413, c: 522.577 },
    VsopTerm { a: 342.0e-8, b: 0.523, c: 1059.382 },
    VsopTerm { a: 255.0e-8, b: 1.196, c: 7.114 },
    VsopTerm { a: 222.0e-8, b: 0.952, c: 515.464 },
    VsopTerm { a: 90.0e-8, b: 3.14, c: 0.00 },
    VsopTerm { a: 69.0e-8, b: 2.27, c: 1066.50 },
    VsopTerm { a: 58.0e-8, b: 1.41, c: 543.92 },
    VsopTerm { a: 58.0e-8, b: 0.53, c: 639.90 },
    VsopTerm { a: 51.0e-8, b: 5.98, c: 412.37 },
    VsopTerm { a: 47.0e-8, b: 4.57, c: 625.67 },
];

#[rustfmt::skip]
pub(super) const R4: &[VsopTerm] = &[
    VsopTerm { a: 129.0e-8, b: 0.084, c: 536.805 },
    VsopTerm { a: 113.0e-8, b: 4.249, c: 529.691 },
    VsopTerm { a: 83.0e-8, b: 3.30, c: 522.58 },
    VsopTerm { a: 38.0e-8, b: 2.73, c: 515.46 },
    VsopTerm { a: 27.0e-8, b: 5.69, c: 7.11 },
    VsopTerm { a: 18.0e-8, b: 5.40, c: 1059.38 },
    VsopTerm { a: 13.0e-8, b: 6.02, c: 543.92 },
    VsopTerm { a: 9.0e-8, b: 0.77, c: 1066.50 },
    VsopTerm { a: 8.0e-8, b: 5.68, c: 14.23 },
    VsopTerm { a: 7.0e-8, b: 1.43, c: 412.37 },
];

/// Radius sub-series references.
pub(super) const R_SERIES: [&[VsopTerm]; 5] = [R0, R1, R2, R3, R4];
