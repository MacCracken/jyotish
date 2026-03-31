//! ELP2000-82 main problem periodic term tables.
//!
//! Truncated ELP2000-82 main problem series (Chapront-Touzé & Chapront 1988)
//! for computing the Moon's geocentric ecliptic coordinates.  The tables are
//! equivalent to Meeus *Astronomical Algorithms* Tables 47.A and 47.B, which
//! are themselves derived from the ELP2000-82 Series 1 (Main Problem).
//!
//! Truncation level targets Swiss Ephemeris / Moshier parity (< 2″ longitude
//! accuracy).
//!
//! # Argument multipliers
//!
//! Each term's argument is  `D·d + M·m + M'·mp + F·f`  where:
//! - **D** — mean elongation of the Moon
//! - **M** — Sun's mean anomaly
//! - **M'** — Moon's mean anomaly
//! - **F** — Moon's argument of latitude
//!
//! # References
//!
//! - Chapront-Touzé, M. & Chapront, J. (1988). *ELP2000-85: a semi-analytical
//!   lunar ephemeris adequate for historical times.* Astron. Astrophys. 190, 342.
//! - Meeus, J. (1998). *Astronomical Algorithms*, 2nd ed., Ch. 47.

use super::{DistTerm, LonLatTerm};

// ===========================================================================
// Table 47.A — Longitude (sine coefficients, unit = 0.000 001°)
// ===========================================================================

/// Main-problem longitude terms (ELP2000-82 / Meeus Table 47.A).
///
/// Sorted by |coefficient| descending.  60 terms from the complete Meeus
/// table, covering all terms with |coeff| ≥ 0 in Table 47.A.
#[rustfmt::skip]
pub(crate) const LONGITUDE_TERMS: &[LonLatTerm] = &[
    LonLatTerm { d:  0, m:  0, mp:  1, f:  0, coeff:  6_288_774 },
    LonLatTerm { d:  2, m:  0, mp: -1, f:  0, coeff:  1_274_027 },
    LonLatTerm { d:  2, m:  0, mp:  0, f:  0, coeff:    658_314 },
    LonLatTerm { d:  0, m:  0, mp:  2, f:  0, coeff:    213_618 },
    LonLatTerm { d:  0, m:  1, mp:  0, f:  0, coeff:   -185_116 },
    LonLatTerm { d:  0, m:  0, mp:  0, f:  2, coeff:   -114_332 },
    LonLatTerm { d:  2, m:  0, mp: -2, f:  0, coeff:     58_793 },
    LonLatTerm { d:  2, m: -1, mp: -1, f:  0, coeff:     57_066 },
    LonLatTerm { d:  2, m:  0, mp:  1, f:  0, coeff:     53_322 },
    LonLatTerm { d:  2, m: -1, mp:  0, f:  0, coeff:     45_758 },
    LonLatTerm { d:  0, m:  1, mp: -1, f:  0, coeff:    -40_923 },
    LonLatTerm { d:  1, m:  0, mp:  0, f:  0, coeff:    -34_720 },
    LonLatTerm { d:  0, m:  1, mp:  1, f:  0, coeff:    -30_383 },
    LonLatTerm { d:  2, m:  0, mp:  0, f: -2, coeff:     15_327 },
    LonLatTerm { d:  0, m:  0, mp:  1, f:  2, coeff:    -12_528 },
    LonLatTerm { d:  0, m:  0, mp:  1, f: -2, coeff:     10_980 },
    LonLatTerm { d:  4, m:  0, mp: -1, f:  0, coeff:     10_675 },
    LonLatTerm { d:  0, m:  0, mp:  3, f:  0, coeff:     10_034 },
    LonLatTerm { d:  4, m:  0, mp: -2, f:  0, coeff:      8_548 },
    LonLatTerm { d:  2, m:  1, mp: -1, f:  0, coeff:     -7_888 },
    LonLatTerm { d:  2, m:  1, mp:  0, f:  0, coeff:     -6_766 },
    LonLatTerm { d:  1, m:  0, mp: -1, f:  0, coeff:     -5_163 },
    LonLatTerm { d:  1, m:  1, mp:  0, f:  0, coeff:      4_987 },
    LonLatTerm { d:  2, m: -1, mp:  1, f:  0, coeff:      4_036 },
    LonLatTerm { d:  2, m:  0, mp:  2, f:  0, coeff:      3_994 },
    LonLatTerm { d:  4, m:  0, mp:  0, f:  0, coeff:      3_861 },
    LonLatTerm { d:  2, m:  0, mp: -3, f:  0, coeff:      3_665 },
    LonLatTerm { d:  0, m:  1, mp: -2, f:  0, coeff:     -2_689 },
    LonLatTerm { d:  2, m:  0, mp: -1, f:  2, coeff:     -2_602 },
    LonLatTerm { d:  2, m: -1, mp: -2, f:  0, coeff:      2_390 },
    LonLatTerm { d:  1, m:  0, mp:  1, f:  0, coeff:     -2_348 },
    LonLatTerm { d:  2, m: -2, mp:  0, f:  0, coeff:      2_236 },
    LonLatTerm { d:  0, m:  1, mp:  2, f:  0, coeff:     -2_120 },
    LonLatTerm { d:  0, m:  2, mp:  0, f:  0, coeff:     -2_069 },
    LonLatTerm { d:  2, m: -2, mp: -1, f:  0, coeff:      2_048 },
    LonLatTerm { d:  2, m:  0, mp:  1, f: -2, coeff:     -1_773 },
    LonLatTerm { d:  2, m:  0, mp:  0, f:  2, coeff:     -1_595 },
    LonLatTerm { d:  4, m: -1, mp: -1, f:  0, coeff:      1_215 },
    LonLatTerm { d:  0, m:  0, mp:  2, f:  2, coeff:     -1_110 },
    LonLatTerm { d:  3, m:  0, mp: -1, f:  0, coeff:       -892 },
    LonLatTerm { d:  2, m:  1, mp:  1, f:  0, coeff:       -810 },
    LonLatTerm { d:  4, m: -1, mp: -2, f:  0, coeff:        759 },
    LonLatTerm { d:  0, m:  2, mp: -1, f:  0, coeff:       -713 },
    LonLatTerm { d:  2, m:  2, mp: -1, f:  0, coeff:       -700 },
    LonLatTerm { d:  2, m:  1, mp: -2, f:  0, coeff:        691 },
    LonLatTerm { d:  2, m: -1, mp:  0, f: -2, coeff:        596 },
    LonLatTerm { d:  4, m:  0, mp:  1, f:  0, coeff:        549 },
    LonLatTerm { d:  0, m:  0, mp:  4, f:  0, coeff:        537 },
    LonLatTerm { d:  4, m: -1, mp:  0, f:  0, coeff:        520 },
    LonLatTerm { d:  1, m:  0, mp: -2, f:  0, coeff:       -487 },
    LonLatTerm { d:  2, m:  1, mp:  0, f: -2, coeff:       -399 },
    LonLatTerm { d:  0, m:  0, mp:  2, f: -2, coeff:       -381 },
    LonLatTerm { d:  1, m:  1, mp:  1, f:  0, coeff:        351 },
    LonLatTerm { d:  3, m:  0, mp: -2, f:  0, coeff:       -340 },
    LonLatTerm { d:  4, m:  0, mp: -3, f:  0, coeff:        330 },
    LonLatTerm { d:  2, m: -1, mp:  2, f:  0, coeff:        327 },
    LonLatTerm { d:  0, m:  2, mp:  1, f:  0, coeff:       -323 },
    LonLatTerm { d:  1, m:  1, mp: -1, f:  0, coeff:        299 },
    LonLatTerm { d:  2, m:  0, mp:  3, f:  0, coeff:        294 },
    LonLatTerm { d:  2, m:  0, mp: -1, f: -2, coeff:          0 },
];

// ===========================================================================
// Table 47.B — Latitude (sine coefficients, unit = 0.000 001°)
// ===========================================================================

/// Main-problem latitude terms (ELP2000-82 / Meeus Table 47.B).
///
/// Sorted by |coefficient| descending.  60 terms from the complete Meeus
/// table, covering all terms with |coeff| ≥ 107.
#[rustfmt::skip]
pub(crate) const LATITUDE_TERMS: &[LonLatTerm] = &[
    LonLatTerm { d:  0, m:  0, mp:  0, f:  1, coeff:  5_128_122 },
    LonLatTerm { d:  0, m:  0, mp:  1, f:  1, coeff:    280_602 },
    LonLatTerm { d:  0, m:  0, mp:  1, f: -1, coeff:    277_693 },
    LonLatTerm { d:  2, m:  0, mp:  0, f: -1, coeff:    173_237 },
    LonLatTerm { d:  2, m:  0, mp: -1, f:  1, coeff:     55_413 },
    LonLatTerm { d:  2, m:  0, mp: -1, f: -1, coeff:     46_271 },
    LonLatTerm { d:  2, m:  0, mp:  0, f:  1, coeff:     32_573 },
    LonLatTerm { d:  0, m:  0, mp:  2, f:  1, coeff:     17_198 },
    LonLatTerm { d:  2, m:  0, mp:  1, f: -1, coeff:      9_266 },
    LonLatTerm { d:  0, m:  0, mp:  2, f: -1, coeff:      8_822 },
    LonLatTerm { d:  2, m: -1, mp:  0, f: -1, coeff:      8_216 },
    LonLatTerm { d:  2, m:  0, mp: -2, f: -1, coeff:      4_324 },
    LonLatTerm { d:  2, m:  0, mp:  1, f:  1, coeff:      4_200 },
    LonLatTerm { d:  2, m:  1, mp:  0, f: -1, coeff:     -3_359 },
    LonLatTerm { d:  2, m: -1, mp: -1, f:  1, coeff:      2_463 },
    LonLatTerm { d:  2, m: -1, mp:  0, f:  1, coeff:      2_211 },
    LonLatTerm { d:  2, m: -1, mp: -1, f: -1, coeff:      2_065 },
    LonLatTerm { d:  0, m:  1, mp: -1, f: -1, coeff:     -1_870 },
    LonLatTerm { d:  4, m:  0, mp: -1, f: -1, coeff:      1_828 },
    LonLatTerm { d:  0, m:  1, mp:  0, f:  1, coeff:     -1_794 },
    LonLatTerm { d:  0, m:  0, mp:  0, f:  3, coeff:     -1_749 },
    LonLatTerm { d:  0, m:  1, mp: -1, f:  1, coeff:     -1_565 },
    LonLatTerm { d:  1, m:  0, mp:  0, f:  1, coeff:     -1_491 },
    LonLatTerm { d:  0, m:  1, mp:  1, f:  1, coeff:     -1_475 },
    LonLatTerm { d:  0, m:  1, mp:  1, f: -1, coeff:     -1_410 },
    LonLatTerm { d:  0, m:  1, mp:  0, f: -1, coeff:     -1_344 },
    LonLatTerm { d:  1, m:  0, mp:  0, f: -1, coeff:     -1_335 },
    LonLatTerm { d:  0, m:  0, mp:  3, f:  1, coeff:      1_107 },
    LonLatTerm { d:  4, m:  0, mp:  0, f: -1, coeff:      1_021 },
    LonLatTerm { d:  4, m:  0, mp: -1, f:  1, coeff:        833 },
    LonLatTerm { d:  0, m:  0, mp:  1, f: -3, coeff:        777 },
    LonLatTerm { d:  4, m:  0, mp: -2, f:  1, coeff:        671 },
    LonLatTerm { d:  2, m:  0, mp:  0, f: -3, coeff:        607 },
    LonLatTerm { d:  2, m:  0, mp:  2, f: -1, coeff:        596 },
    LonLatTerm { d:  2, m: -1, mp:  1, f: -1, coeff:        491 },
    LonLatTerm { d:  2, m:  0, mp: -2, f:  1, coeff:       -451 },
    LonLatTerm { d:  0, m:  0, mp:  3, f: -1, coeff:        439 },
    LonLatTerm { d:  2, m:  0, mp:  2, f:  1, coeff:        422 },
    LonLatTerm { d:  2, m:  0, mp: -3, f: -1, coeff:        421 },
    LonLatTerm { d:  2, m:  1, mp: -1, f:  1, coeff:       -366 },
    LonLatTerm { d:  2, m:  1, mp:  0, f:  1, coeff:       -351 },
    LonLatTerm { d:  4, m:  0, mp:  0, f:  1, coeff:        331 },
    LonLatTerm { d:  2, m: -1, mp:  1, f:  1, coeff:        315 },
    LonLatTerm { d:  2, m: -2, mp:  0, f: -1, coeff:        302 },
    LonLatTerm { d:  0, m:  0, mp:  1, f:  3, coeff:       -283 },
    LonLatTerm { d:  2, m:  1, mp:  1, f: -1, coeff:       -229 },
    LonLatTerm { d:  1, m:  1, mp:  0, f: -1, coeff:        223 },
    LonLatTerm { d:  1, m:  1, mp:  0, f:  1, coeff:        223 },
    LonLatTerm { d:  0, m:  1, mp: -2, f: -1, coeff:       -220 },
    LonLatTerm { d:  2, m:  1, mp: -1, f: -1, coeff:       -220 },
    LonLatTerm { d:  1, m:  0, mp:  1, f:  1, coeff:       -185 },
    LonLatTerm { d:  2, m: -1, mp: -2, f: -1, coeff:        181 },
    LonLatTerm { d:  0, m:  1, mp:  2, f:  1, coeff:       -177 },
    LonLatTerm { d:  4, m:  0, mp: -2, f: -1, coeff:        176 },
    LonLatTerm { d:  4, m: -1, mp: -1, f: -1, coeff:        166 },
    LonLatTerm { d:  1, m:  0, mp:  1, f: -1, coeff:       -164 },
    LonLatTerm { d:  4, m:  0, mp:  1, f: -1, coeff:        132 },
    LonLatTerm { d:  1, m:  0, mp: -1, f: -1, coeff:       -119 },
    LonLatTerm { d:  4, m: -1, mp:  0, f: -1, coeff:        115 },
    LonLatTerm { d:  2, m: -2, mp:  0, f:  1, coeff:        107 },
];

// ===========================================================================
// Table 47.A — Distance (cosine coefficients, unit = 0.001 km)
// ===========================================================================

/// Main-problem distance terms (ELP2000-82 / Meeus Table 47.A).
///
/// Sorted by |coefficient| descending.  60 terms from the complete Meeus
/// table.  Terms with zero distance coefficient are omitted.
#[rustfmt::skip]
pub(crate) const DISTANCE_TERMS: &[DistTerm] = &[
    DistTerm { d:  0, m:  0, mp:  1, f:  0, coeff: -20_905_355 },
    DistTerm { d:  2, m:  0, mp: -1, f:  0, coeff:  -3_699_111 },
    DistTerm { d:  2, m:  0, mp:  0, f:  0, coeff:  -2_955_968 },
    DistTerm { d:  0, m:  0, mp:  2, f:  0, coeff:    -569_925 },
    DistTerm { d:  0, m:  1, mp:  0, f:  0, coeff:      48_888 },
    DistTerm { d:  0, m:  0, mp:  0, f:  2, coeff:      -3_149 },
    DistTerm { d:  2, m:  0, mp: -2, f:  0, coeff:     246_158 },
    DistTerm { d:  2, m: -1, mp: -1, f:  0, coeff:    -152_138 },
    DistTerm { d:  2, m:  0, mp:  1, f:  0, coeff:    -170_733 },
    DistTerm { d:  2, m: -1, mp:  0, f:  0, coeff:    -204_586 },
    DistTerm { d:  0, m:  1, mp: -1, f:  0, coeff:    -129_620 },
    DistTerm { d:  1, m:  0, mp:  0, f:  0, coeff:     108_743 },
    DistTerm { d:  0, m:  1, mp:  1, f:  0, coeff:     104_755 },
    DistTerm { d:  2, m:  0, mp:  0, f: -2, coeff:      10_321 },
    DistTerm { d:  0, m:  0, mp:  1, f: -2, coeff:      79_661 },
    DistTerm { d:  4, m:  0, mp: -1, f:  0, coeff:     -34_782 },
    DistTerm { d:  0, m:  0, mp:  3, f:  0, coeff:     -23_210 },
    DistTerm { d:  4, m:  0, mp: -2, f:  0, coeff:     -21_636 },
    DistTerm { d:  2, m:  1, mp: -1, f:  0, coeff:      24_208 },
    DistTerm { d:  2, m:  1, mp:  0, f:  0, coeff:      30_824 },
    DistTerm { d:  1, m:  0, mp: -1, f:  0, coeff:      -8_379 },
    DistTerm { d:  1, m:  1, mp:  0, f:  0, coeff:     -16_675 },
    DistTerm { d:  2, m: -1, mp:  1, f:  0, coeff:     -12_831 },
    DistTerm { d:  2, m:  0, mp:  2, f:  0, coeff:     -10_445 },
    DistTerm { d:  4, m:  0, mp:  0, f:  0, coeff:     -11_650 },
    DistTerm { d:  2, m:  0, mp: -3, f:  0, coeff:      14_403 },
    DistTerm { d:  0, m:  1, mp: -2, f:  0, coeff:      -7_003 },
    DistTerm { d:  2, m: -1, mp: -2, f:  0, coeff:      10_056 },
    DistTerm { d:  1, m:  0, mp:  1, f:  0, coeff:       6_322 },
    DistTerm { d:  2, m: -2, mp:  0, f:  0, coeff:      -9_884 },
    DistTerm { d:  0, m:  1, mp:  2, f:  0, coeff:       5_751 },
    DistTerm { d:  2, m: -2, mp: -1, f:  0, coeff:      -4_950 },
    DistTerm { d:  2, m:  0, mp:  1, f: -2, coeff:       4_130 },
    DistTerm { d:  4, m: -1, mp: -1, f:  0, coeff:      -3_958 },
    DistTerm { d:  3, m:  0, mp: -1, f:  0, coeff:       3_258 },
    DistTerm { d:  2, m:  1, mp:  1, f:  0, coeff:       2_616 },
    DistTerm { d:  4, m: -1, mp: -2, f:  0, coeff:      -1_897 },
    DistTerm { d:  0, m:  2, mp: -1, f:  0, coeff:      -2_117 },
    DistTerm { d:  2, m:  2, mp: -1, f:  0, coeff:       2_354 },
    DistTerm { d:  4, m:  0, mp:  1, f:  0, coeff:      -1_423 },
    DistTerm { d:  0, m:  0, mp:  4, f:  0, coeff:      -1_117 },
    DistTerm { d:  4, m: -1, mp:  0, f:  0, coeff:      -1_571 },
    DistTerm { d:  1, m:  0, mp: -2, f:  0, coeff:      -1_739 },
    DistTerm { d:  0, m:  0, mp:  2, f: -2, coeff:      -4_421 },
    DistTerm { d:  0, m:  2, mp:  1, f:  0, coeff:       1_165 },
    DistTerm { d:  2, m:  0, mp: -1, f: -2, coeff:       8_752 },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longitude_terms_sorted_by_abs_coeff() {
        for w in LONGITUDE_TERMS.windows(2) {
            // Allow equal absolute values (some terms share magnitudes)
            assert!(
                w[0].coeff.abs() >= w[1].coeff.abs(),
                "longitude terms not sorted: |{}| < |{}|",
                w[0].coeff,
                w[1].coeff,
            );
        }
    }

    #[test]
    fn latitude_terms_sorted_by_abs_coeff() {
        for w in LATITUDE_TERMS.windows(2) {
            assert!(
                w[0].coeff.abs() >= w[1].coeff.abs(),
                "latitude terms not sorted: |{}| < |{}|",
                w[0].coeff,
                w[1].coeff,
            );
        }
    }

    #[test]
    fn largest_longitude_term() {
        assert_eq!(LONGITUDE_TERMS[0].coeff, 6_288_774);
        assert_eq!(LONGITUDE_TERMS[0].d, 0);
        assert_eq!(LONGITUDE_TERMS[0].mp, 1);
    }

    #[test]
    fn largest_latitude_term() {
        assert_eq!(LATITUDE_TERMS[0].coeff, 5_128_122);
        assert_eq!(LATITUDE_TERMS[0].f, 1);
    }

    #[test]
    fn largest_distance_term() {
        assert_eq!(DISTANCE_TERMS[0].coeff, -20_905_355);
    }

    #[test]
    fn term_counts() {
        assert_eq!(LONGITUDE_TERMS.len(), 60);
        assert_eq!(LATITUDE_TERMS.len(), 60);
        assert_eq!(DISTANCE_TERMS.len(), 46);
    }
}
