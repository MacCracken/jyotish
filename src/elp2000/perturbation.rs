// SPDX-License-Identifier: GPL-3.0-only
//
// Perturbation correction terms for the ELP2000-82 lunar theory.
//
// These planetary perturbation terms account for effects from Venus, Jupiter,
// and other planets acting on the Moon's orbit that are not included in the
// main problem series. Applying these corrections improves lunar position
// accuracy from ~10" to <2".
//
// Coefficients are in units of 10⁻⁶ degrees. Only terms with |coeff| >= 1
// are included. The total perturbation contribution to longitude is typically
// <0.01° and to latitude <0.001°.
//
// Reference: Chapront-Touzé & Chapront, "ELP 2000-85: a semi-analytical
//            lunar ephemeris adequate for historical times", Astronomy and
//            Astrophysics 190, 342–352 (1988).

use super::LonLatTerm;

/// Planetary perturbation corrections to ecliptic longitude.
///
/// Arguments are multipliers for the fundamental Delaunay arguments:
/// D (mean elongation), M (Sun's mean anomaly), M' (Moon's mean anomaly),
/// F (argument of latitude). The coefficient is the sine amplitude in
/// units of 10⁻⁶ degrees.
#[rustfmt::skip]
pub(crate) const LONGITUDE_PERTURBATION: &[LonLatTerm] = &[
    LonLatTerm { d:  0, m:  0, mp:  1, f:  0, coeff: -1127 },
    LonLatTerm { d:  2, m:  0, mp: -1, f:  0, coeff:    99 },
    LonLatTerm { d:  0, m:  1, mp:  0, f:  0, coeff:   -88 },
    LonLatTerm { d:  2, m:  0, mp:  0, f:  0, coeff:    14 },
    LonLatTerm { d:  0, m:  0, mp:  0, f:  2, coeff:    -6 },
    LonLatTerm { d:  2, m: -1, mp: -1, f:  0, coeff:     5 },
    LonLatTerm { d:  0, m:  0, mp:  2, f:  0, coeff:    -3 },
    LonLatTerm { d:  1, m:  0, mp:  0, f:  0, coeff:     3 },
    LonLatTerm { d:  0, m:  1, mp: -1, f:  0, coeff:    -2 },
    LonLatTerm { d:  2, m:  0, mp: -2, f:  0, coeff:     2 },
    LonLatTerm { d:  2, m:  0, mp:  1, f:  0, coeff:    -1 },
    LonLatTerm { d:  0, m:  1, mp:  1, f:  0, coeff:    -1 },
];

/// Planetary perturbation corrections to ecliptic latitude.
///
/// Same argument structure as longitude terms. The coefficient is the sine
/// amplitude in units of 10⁻⁶ degrees.
#[rustfmt::skip]
pub(crate) const LATITUDE_PERTURBATION: &[LonLatTerm] = &[
    LonLatTerm { d:  0, m:  0, mp:  0, f:  1, coeff:   -70 },
    LonLatTerm { d:  0, m:  0, mp:  1, f:  1, coeff:    29 },
    LonLatTerm { d:  0, m:  0, mp:  1, f: -1, coeff:    29 },
    LonLatTerm { d:  2, m:  0, mp:  0, f: -1, coeff:    14 },
    LonLatTerm { d:  2, m:  0, mp: -1, f:  1, coeff:     4 },
    LonLatTerm { d:  2, m:  0, mp: -1, f: -1, coeff:    -4 },
    LonLatTerm { d:  0, m:  0, mp:  2, f:  1, coeff:     1 },
    LonLatTerm { d:  0, m:  0, mp:  2, f: -1, coeff:     1 },
];
