//! Fixed star catalog — navigational stars with proper motion.
//!
//! Provides a catalog of the 57 navigational stars traditionally used in
//! celestial navigation, with J2000.0 positions and proper motion data for
//! computing apparent positions at any epoch.
//!
//! # Examples
//!
//! ```
//! use jyotish::star::{find_star, brightest_stars, NAVIGATIONAL_STARS};
//! use jyotish::calendar::J2000_0;
//!
//! // Look up Sirius
//! let sirius = find_star("Sirius").unwrap();
//! assert_eq!(sirius.name, "Sirius");
//! assert!(sirius.magnitude < 0.0); // brightest star in the sky
//!
//! // Position at J2000.0
//! let (ra, dec) = sirius.position_at(J2000_0);
//! assert!((ra - 101.2872).abs() < 0.001);
//!
//! // The three brightest navigational stars
//! let top3 = brightest_stars(3);
//! assert_eq!(top3[0].name, "Sirius");
//! assert_eq!(top3[1].name, "Canopus");
//! assert_eq!(top3[2].name, "Arcturus");
//! ```

use crate::calendar::J2000_0;
use crate::coords::{deg_to_rad, normalize_degrees};
use serde::{Deserialize, Serialize};
use std::fmt;

/// A fixed star with J2000.0 position and proper motion.
///
/// Positions are given in the ICRS (essentially FK5/J2000.0) reference frame.
/// Proper motion allows computing the star's position at any epoch via
/// [`Star::position_at`].
///
/// # Examples
///
/// ```
/// use jyotish::star::NAVIGATIONAL_STARS;
/// use jyotish::calendar::J2000_0;
///
/// let star = &NAVIGATIONAL_STARS[0];
/// let (ra, dec) = star.position_at(J2000_0);
/// assert!((ra - star.ra_j2000).abs() < 1e-10);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Star {
    /// Common name (e.g. "Sirius").
    pub name: &'static str,
    /// Bayer designation (e.g. "α CMa").
    pub bayer: &'static str,
    /// Right ascension at J2000.0, in degrees [0, 360).
    pub ra_j2000: f64,
    /// Declination at J2000.0, in degrees [-90, +90].
    pub dec_j2000: f64,
    /// Proper motion in right ascension, in arcseconds per year.
    ///
    /// This value already includes the cos(δ) factor (i.e. it is
    /// μα* = μα·cos(δ)).
    pub pm_ra: f64,
    /// Proper motion in declination, in arcseconds per year.
    pub pm_dec: f64,
    /// Apparent visual magnitude.
    pub magnitude: f64,
}

impl fmt::Display for Star {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({}, mag {:.2})",
            self.name, self.bayer, self.magnitude
        )
    }
}

impl Star {
    /// Compute the star's right ascension and declination at the given Julian
    /// Date, accounting for proper motion.
    ///
    /// Returns `(ra_degrees, dec_degrees)` where RA is normalised to [0, 360).
    ///
    /// This applies linear proper motion only (no precession, nutation, or
    /// aberration). For most purposes this is sufficient for stars over spans
    /// of a few centuries around J2000.0.
    ///
    /// # Examples
    ///
    /// ```
    /// use jyotish::star::find_star;
    /// use jyotish::calendar::J2000_0;
    ///
    /// let sirius = find_star("Sirius").unwrap();
    /// let (ra, dec) = sirius.position_at(J2000_0 + 365.25 * 50.0);
    /// // 50 years of proper motion should shift the position noticeably
    /// assert!((ra - sirius.ra_j2000).abs() > 0.001);
    /// ```
    pub fn position_at(&self, jd: f64) -> (f64, f64) {
        let years = (jd - J2000_0) / 365.25;
        let cos_dec = deg_to_rad(self.dec_j2000).cos();

        // pm_ra already has cos(dec) baked in; divide by cos(dec) to get the
        // actual RA shift in arcseconds, then convert to degrees.
        let ra = if cos_dec.abs() > 1e-12 {
            self.ra_j2000 + self.pm_ra * years / 3600.0 / cos_dec
        } else {
            // Near the pole, proper motion in RA is degenerate; keep J2000 RA
            self.ra_j2000
        };

        let dec = self.dec_j2000 + self.pm_dec * years / 3600.0;

        (normalize_degrees(ra), dec.clamp(-90.0, 90.0))
    }
}

// ---------------------------------------------------------------------------
// Navigational star catalog
// ---------------------------------------------------------------------------

/// The 58 navigational stars used in celestial navigation (57 plus Polaris).
///
/// Coordinates are ICRS J2000.0. Proper motions are in arcseconds per year
/// (milliarcsecond catalog values divided by 1000). Stars are ordered roughly
/// by traditional navigational star number.
#[rustfmt::skip]
pub static NAVIGATIONAL_STARS: &[Star] = &[
    //                          name              bayer        RA(°)       Dec(°)     pm_ra(″/yr)  pm_dec(″/yr)  mag
    Star { name: "Alpheratz",   bayer: "α And",  ra_j2000:   2.0965, dec_j2000:  29.0904, pm_ra:  0.13578, pm_dec: -0.16285, magnitude:  2.06 },
    Star { name: "Ankaa",       bayer: "α Phe",  ra_j2000:   6.5710, dec_j2000: -42.3061, pm_ra:  0.23250, pm_dec: -0.35610, magnitude:  2.39 },
    Star { name: "Schedar",     bayer: "α Cas",  ra_j2000:  10.1268, dec_j2000:  56.5374, pm_ra:  0.05037, pm_dec: -0.03222, magnitude:  2.23 },
    Star { name: "Diphda",      bayer: "β Cet",  ra_j2000:  14.6600, dec_j2000: -17.9866, pm_ra:  0.23239, pm_dec:  0.03277, magnitude:  2.02 },
    Star { name: "Achernar",    bayer: "α Eri",  ra_j2000:  24.4285, dec_j2000: -57.2368, pm_ra:  0.08782, pm_dec: -0.03994, magnitude:  0.46 },
    Star { name: "Hamal",       bayer: "α Ari",  ra_j2000:  31.7934, dec_j2000:  23.4624, pm_ra:  0.19042, pm_dec: -0.14836, magnitude:  2.00 },
    Star { name: "Polaris",     bayer: "α UMi",  ra_j2000:  37.9546, dec_j2000:  89.2641, pm_ra:  0.04422, pm_dec: -0.01174, magnitude:  1.98 },
    Star { name: "Acamar",      bayer: "θ Eri",  ra_j2000:  44.5653, dec_j2000: -40.3047, pm_ra: -0.00524, pm_dec:  0.01992, magnitude:  2.91 },
    Star { name: "Menkar",      bayer: "α Cet",  ra_j2000:  45.5700, dec_j2000:   4.0897, pm_ra: -0.01120, pm_dec: -0.07801, magnitude:  2.53 },
    Star { name: "Mirfak",      bayer: "α Per",  ra_j2000:  51.0809, dec_j2000:  49.8612, pm_ra:  0.02399, pm_dec: -0.02609, magnitude:  1.80 },
    Star { name: "Aldebaran",   bayer: "α Tau",  ra_j2000:  68.9802, dec_j2000:  16.5093, pm_ra:  0.06278, pm_dec: -0.18936, magnitude:  0.86 },
    Star { name: "Rigel",       bayer: "β Ori",  ra_j2000:  78.6345, dec_j2000:  -8.2016, pm_ra:  0.00187, pm_dec: -0.00056, magnitude:  0.13 },
    Star { name: "Capella",     bayer: "α Aur",  ra_j2000:  79.1723, dec_j2000:  45.9980, pm_ra:  0.07552, pm_dec: -0.42711, magnitude:  0.08 },
    Star { name: "Bellatrix",   bayer: "γ Ori",  ra_j2000:  81.2828, dec_j2000:   6.3497, pm_ra: -0.00847, pm_dec: -0.01299, magnitude:  1.64 },
    Star { name: "Elnath",      bayer: "β Tau",  ra_j2000:  81.5728, dec_j2000:  28.6074, pm_ra:  0.02342, pm_dec: -0.17440, magnitude:  1.65 },
    Star { name: "Alnilam",     bayer: "ε Ori",  ra_j2000:  84.0533, dec_j2000:  -1.2019, pm_ra:  0.00126, pm_dec: -0.00105, magnitude:  1.70 },
    Star { name: "Betelgeuse",  bayer: "α Ori",  ra_j2000:  88.7929, dec_j2000:   7.4071, pm_ra:  0.02733, pm_dec:  0.01086, magnitude:  0.50 },
    Star { name: "Canopus",     bayer: "α Car",  ra_j2000:  95.9880, dec_j2000: -52.6957, pm_ra:  0.01999, pm_dec:  0.02367, magnitude: -0.74 },
    Star { name: "Sirius",      bayer: "α CMa",  ra_j2000: 101.2872, dec_j2000: -16.7161, pm_ra: -0.54601, pm_dec: -1.22307, magnitude: -1.46 },
    Star { name: "Adhara",      bayer: "ε CMa",  ra_j2000: 104.6565, dec_j2000: -28.9721, pm_ra:  0.00339, pm_dec:  0.00223, magnitude:  1.50 },
    Star { name: "Procyon",     bayer: "α CMi",  ra_j2000: 114.8255, dec_j2000:   5.2250, pm_ra: -0.71459, pm_dec: -1.03680, magnitude:  0.34 },
    Star { name: "Pollux",      bayer: "β Gem",  ra_j2000: 116.3289, dec_j2000:  28.0262, pm_ra: -0.62569, pm_dec: -0.04595, magnitude:  1.14 },
    Star { name: "Avior",       bayer: "ε Car",  ra_j2000: 125.6285, dec_j2000: -59.5095, pm_ra: -0.02564, pm_dec:  0.01386, magnitude:  1.86 },
    Star { name: "Suhail",      bayer: "λ Vel",  ra_j2000: 136.9990, dec_j2000: -43.4326, pm_ra: -0.02388, pm_dec:  0.01400, magnitude:  2.21 },
    Star { name: "Miaplacidus", bayer: "β Car",  ra_j2000: 138.3000, dec_j2000: -69.7172, pm_ra: -0.15686, pm_dec:  0.10852, magnitude:  1.68 },
    Star { name: "Alphard",     bayer: "α Hya",  ra_j2000: 141.8968, dec_j2000:  -8.6586, pm_ra: -0.01493, pm_dec:  0.03346, magnitude:  1.98 },
    Star { name: "Regulus",     bayer: "α Leo",  ra_j2000: 152.0929, dec_j2000:  11.9672, pm_ra: -0.24940, pm_dec:  0.00559, magnitude:  1.40 },
    Star { name: "Dubhe",       bayer: "α UMa",  ra_j2000: 165.9320, dec_j2000:  61.7510, pm_ra: -0.13428, pm_dec: -0.03488, magnitude:  1.79 },
    Star { name: "Denebola",    bayer: "β Leo",  ra_j2000: 177.2649, dec_j2000:  14.5720, pm_ra: -0.49900, pm_dec: -0.11381, magnitude:  2.13 },
    Star { name: "Gienah",      bayer: "γ Crv",  ra_j2000: 183.9516, dec_j2000: -17.5419, pm_ra: -0.15907, pm_dec:  0.02295, magnitude:  2.59 },
    Star { name: "Acrux",       bayer: "α Cru",  ra_j2000: 186.6496, dec_j2000: -63.0990, pm_ra: -0.03541, pm_dec: -0.01200, magnitude:  0.76 },
    Star { name: "Gacrux",      bayer: "γ Cru",  ra_j2000: 187.7915, dec_j2000: -57.1132, pm_ra:  0.02768, pm_dec: -0.26486, magnitude:  1.63 },
    Star { name: "Alioth",      bayer: "ε UMa",  ra_j2000: 193.5073, dec_j2000:  55.9598, pm_ra:  0.11182, pm_dec: -0.00881, magnitude:  1.77 },
    Star { name: "Spica",       bayer: "α Vir",  ra_j2000: 201.2983, dec_j2000: -11.1613, pm_ra: -0.04250, pm_dec: -0.03173, magnitude:  0.97 },
    Star { name: "Alkaid",      bayer: "η UMa",  ra_j2000: 206.8852, dec_j2000:  49.3133, pm_ra: -0.12124, pm_dec: -0.01502, magnitude:  1.86 },
    Star { name: "Hadar",       bayer: "β Cen",  ra_j2000: 210.9559, dec_j2000: -60.3730, pm_ra: -0.03321, pm_dec: -0.02547, magnitude:  0.61 },
    Star { name: "Menkent",     bayer: "θ Cen",  ra_j2000: 211.6706, dec_j2000: -36.3700, pm_ra: -0.51972, pm_dec: -0.51785, magnitude:  2.06 },
    Star { name: "Arcturus",    bayer: "α Boo",  ra_j2000: 213.9153, dec_j2000:  19.1824, pm_ra: -1.09345, pm_dec: -1.99940, magnitude: -0.05 },
    Star { name: "Rigil Kent",  bayer: "α Cen",  ra_j2000: 219.9021, dec_j2000: -60.8354, pm_ra: -3.67861, pm_dec:  0.48181, magnitude: -0.01 },
    Star { name: "Zubenelgenubi", bayer: "α Lib", ra_j2000: 222.7196, dec_j2000: -16.0418, pm_ra: -0.10556, pm_dec: -0.06930, magnitude:  2.75 },
    Star { name: "Kochab",      bayer: "β UMi",  ra_j2000: 222.6764, dec_j2000:  74.1555, pm_ra: -0.03229, pm_dec:  0.01192, magnitude:  2.08 },
    Star { name: "Alphecca",    bayer: "α CrB",  ra_j2000: 233.6721, dec_j2000:  26.7147, pm_ra:  0.12003, pm_dec: -0.08963, magnitude:  2.23 },
    Star { name: "Antares",     bayer: "α Sco",  ra_j2000: 247.3519, dec_j2000: -26.4320, pm_ra: -0.01211, pm_dec: -0.02330, magnitude:  1.09 },
    Star { name: "Atria",       bayer: "α TrA",  ra_j2000: 252.1662, dec_j2000: -69.0277, pm_ra:  0.01764, pm_dec: -0.03219, magnitude:  1.92 },
    Star { name: "Sabik",       bayer: "η Oph",  ra_j2000: 257.5946, dec_j2000: -15.7249, pm_ra:  0.04117, pm_dec:  0.09867, magnitude:  2.43 },
    Star { name: "Shaula",      bayer: "λ Sco",  ra_j2000: 263.4022, dec_j2000: -37.1038, pm_ra: -0.00860, pm_dec: -0.02940, magnitude:  1.63 },
    Star { name: "Rasalhague",  bayer: "α Oph",  ra_j2000: 263.7339, dec_j2000:  12.5600, pm_ra:  0.10864, pm_dec: -0.22264, magnitude:  2.07 },
    Star { name: "Eltanin",     bayer: "γ Dra",  ra_j2000: 269.1515, dec_j2000:  51.4890, pm_ra: -0.00822, pm_dec: -0.02284, magnitude:  2.23 },
    Star { name: "Kaus Australis", bayer: "ε Sgr", ra_j2000: 276.0430, dec_j2000: -34.3847, pm_ra: -0.03945, pm_dec: -0.12414, magnitude:  1.85 },
    Star { name: "Vega",        bayer: "α Lyr",  ra_j2000: 279.2347, dec_j2000:  38.7837, pm_ra:  0.20094, pm_dec:  0.28623, magnitude:  0.03 },
    Star { name: "Nunki",       bayer: "σ Sgr",  ra_j2000: 283.8163, dec_j2000: -26.2967, pm_ra:  0.01399, pm_dec: -0.05318, magnitude:  2.02 },
    Star { name: "Altair",      bayer: "α Aql",  ra_j2000: 297.6958, dec_j2000:   8.8683, pm_ra:  0.53682, pm_dec:  0.38554, magnitude:  0.77 },
    Star { name: "Peacock",     bayer: "α Pav",  ra_j2000: 306.4119, dec_j2000: -56.7350, pm_ra:  0.00727, pm_dec: -0.08645, magnitude:  1.94 },
    Star { name: "Deneb",       bayer: "α Cyg",  ra_j2000: 310.3580, dec_j2000:  45.2803, pm_ra:  0.00201, pm_dec:  0.00185, magnitude:  1.25 },
    Star { name: "Enif",        bayer: "ε Peg",  ra_j2000: 326.0465, dec_j2000:   9.8750, pm_ra:  0.02615, pm_dec:  0.00060, magnitude:  2.39 },
    Star { name: "Alnair",      bayer: "α Gru",  ra_j2000: 332.0583, dec_j2000: -46.9610, pm_ra:  0.12700, pm_dec: -0.14762, magnitude:  1.74 },
    Star { name: "Fomalhaut",   bayer: "α PsA",  ra_j2000: 344.4127, dec_j2000: -29.6222, pm_ra:  0.32922, pm_dec: -0.16422, magnitude:  1.16 },
    Star { name: "Markab",      bayer: "α Peg",  ra_j2000: 346.1900, dec_j2000:  15.2053, pm_ra:  0.06190, pm_dec: -0.04253, magnitude:  2.49 },
];

/// Find a navigational star by common name (case-insensitive).
///
/// # Examples
///
/// ```
/// use jyotish::star::find_star;
///
/// assert!(find_star("sirius").is_some());
/// assert!(find_star("VEGA").is_some());
/// assert!(find_star("nonexistent").is_none());
/// ```
pub fn find_star(name: &str) -> Option<&'static Star> {
    let needle = name.to_ascii_lowercase();
    NAVIGATIONAL_STARS
        .iter()
        .find(|s| s.name.to_ascii_lowercase() == needle)
}

/// Return the *n* brightest navigational stars, sorted by ascending magnitude
/// (brightest first).
///
/// If `n` exceeds the catalog size, all stars are returned.
///
/// # Examples
///
/// ```
/// use jyotish::star::brightest_stars;
///
/// let top5 = brightest_stars(5);
/// assert_eq!(top5[0].name, "Sirius");
/// assert!(top5[0].magnitude < top5[4].magnitude);
/// ```
pub fn brightest_stars(n: usize) -> Vec<&'static Star> {
    let mut stars: Vec<&Star> = NAVIGATIONAL_STARS.iter().collect();
    stars.sort_by(|a, b| {
        a.magnitude
            .partial_cmp(&b.magnitude)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    stars.truncate(n);
    stars
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::J2000_0;

    #[test]
    fn catalog_has_58_stars() {
        // 57 navigational stars plus Polaris (listed separately in the
        // Nautical Almanac but included here for completeness).
        assert_eq!(NAVIGATIONAL_STARS.len(), 58);
    }

    #[test]
    fn sirius_position_at_j2000() {
        let sirius = find_star("Sirius").unwrap();
        let (ra, dec) = sirius.position_at(J2000_0);
        assert!((ra - 101.2872).abs() < 1e-10, "RA mismatch: {ra}");
        assert!((dec - -16.7161).abs() < 1e-10, "Dec mismatch: {dec}");
    }

    #[test]
    fn sirius_proper_motion_over_100_years() {
        let sirius = find_star("Sirius").unwrap();
        let (ra_0, dec_0) = sirius.position_at(J2000_0);
        let (ra_100, dec_100) = sirius.position_at(J2000_0 + 365.25 * 100.0);

        // Sirius has very large proper motion; over 100 years we expect
        // a shift of many arcseconds — easily detectable in degrees.
        let delta_ra = (ra_100 - ra_0).abs();
        let delta_dec = (dec_100 - dec_0).abs();
        assert!(
            delta_ra > 0.01,
            "RA should shift noticeably: delta={delta_ra}"
        );
        assert!(
            delta_dec > 0.01,
            "Dec should shift noticeably: delta={delta_dec}"
        );
    }

    #[test]
    fn find_star_case_insensitive() {
        assert!(find_star("sirius").is_some());
        assert!(find_star("SIRIUS").is_some());
        assert!(find_star("Sirius").is_some());
        assert!(find_star("sIrIuS").is_some());
    }

    #[test]
    fn find_star_nonexistent() {
        assert!(find_star("nonexistent").is_none());
        assert!(find_star("").is_none());
    }

    #[test]
    fn brightest_stars_top3() {
        let top3 = brightest_stars(3);
        assert_eq!(top3.len(), 3);
        assert_eq!(top3[0].name, "Sirius");
        assert_eq!(top3[1].name, "Canopus");
        assert_eq!(top3[2].name, "Arcturus");
    }

    #[test]
    fn brightest_stars_sorted() {
        let all = brightest_stars(NAVIGATIONAL_STARS.len());
        for w in all.windows(2) {
            assert!(
                w[0].magnitude <= w[1].magnitude,
                "{} (mag {}) should be <= {} (mag {})",
                w[0].name,
                w[0].magnitude,
                w[1].name,
                w[1].magnitude,
            );
        }
    }

    #[test]
    fn brightest_stars_exceeding_catalog() {
        let result = brightest_stars(1000);
        assert_eq!(result.len(), NAVIGATIONAL_STARS.len());
    }

    #[test]
    fn all_stars_have_valid_coordinates() {
        for star in NAVIGATIONAL_STARS {
            assert!(
                (0.0..360.0).contains(&star.ra_j2000),
                "{}: RA {} out of range",
                star.name,
                star.ra_j2000,
            );
            assert!(
                (-90.0..=90.0).contains(&star.dec_j2000),
                "{}: Dec {} out of range",
                star.name,
                star.dec_j2000,
            );
        }
    }

    #[test]
    fn display_format() {
        let sirius = find_star("Sirius").unwrap();
        let s = format!("{sirius}");
        assert!(s.contains("Sirius"));
        assert!(s.contains("α CMa"));
        assert!(s.contains("-1.46"));
    }

    #[test]
    fn polaris_near_pole_position() {
        // Polaris is near the north pole; position_at should handle
        // the near-zero cos(dec) gracefully.
        let polaris = find_star("Polaris").unwrap();
        let (ra, dec) = polaris.position_at(J2000_0 + 365.25 * 50.0);
        // Dec should still be very close to 90
        assert!(dec > 89.0, "Polaris dec should remain near pole: {dec}");
        // RA can be anything near the pole, just check it's normalised
        assert!((0.0..360.0).contains(&ra), "RA not normalised: {ra}");
    }
}
