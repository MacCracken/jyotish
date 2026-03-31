//! Zodiac signs and constellations — tropical, sidereal, bounds, cusps.
//!
//! Provides tropical (Western) and sidereal (Vedic/IAU) zodiac sign lookup,
//! sign boundary computation, and the Lahiri ayanamsa for converting between
//! the two systems.
//!
//! All longitudes are ecliptic, in degrees \[0, 360).

use crate::calendar::julian_centuries;
use crate::coords::normalize_degrees;
use serde::{Deserialize, Serialize};
use std::fmt;

// ---------------------------------------------------------------------------
// Zodiac sign enum
// ---------------------------------------------------------------------------

/// The twelve zodiac signs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Sign {
    /// Aries (0°–30°)
    Aries,
    /// Taurus (30°–60°)
    Taurus,
    /// Gemini (60°–90°)
    Gemini,
    /// Cancer (90°–120°)
    Cancer,
    /// Leo (120°–150°)
    Leo,
    /// Virgo (150°–180°)
    Virgo,
    /// Libra (180°–210°)
    Libra,
    /// Scorpio (210°–240°)
    Scorpio,
    /// Sagittarius (240°–270°)
    Sagittarius,
    /// Capricorn (270°–300°)
    Capricorn,
    /// Aquarius (300°–330°)
    Aquarius,
    /// Pisces (330°–360°)
    Pisces,
}

/// All signs in order, starting from Aries.
pub const SIGNS: [Sign; 12] = [
    Sign::Aries,
    Sign::Taurus,
    Sign::Gemini,
    Sign::Cancer,
    Sign::Leo,
    Sign::Virgo,
    Sign::Libra,
    Sign::Scorpio,
    Sign::Sagittarius,
    Sign::Capricorn,
    Sign::Aquarius,
    Sign::Pisces,
];

impl Sign {
    /// The zero-based index of this sign (Aries = 0, Pisces = 11).
    pub fn index(self) -> usize {
        match self {
            Self::Aries => 0,
            Self::Taurus => 1,
            Self::Gemini => 2,
            Self::Cancer => 3,
            Self::Leo => 4,
            Self::Virgo => 5,
            Self::Libra => 6,
            Self::Scorpio => 7,
            Self::Sagittarius => 8,
            Self::Capricorn => 9,
            Self::Aquarius => 10,
            Self::Pisces => 11,
        }
    }

    /// The ecliptic longitude where this sign begins (tropical, in degrees).
    pub fn cusp_longitude(self) -> f64 {
        self.index() as f64 * 30.0
    }

    /// The element associated with this sign.
    pub fn element(self) -> Element {
        match self.index() % 4 {
            0 => Element::Fire,
            1 => Element::Earth,
            2 => Element::Air,
            _ => Element::Water,
        }
    }

    /// The modality (quality) of this sign.
    pub fn modality(self) -> Modality {
        match self.index() % 3 {
            0 => Modality::Cardinal,
            1 => Modality::Fixed,
            _ => Modality::Mutable,
        }
    }
}

impl fmt::Display for Sign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Aries => "Aries",
            Self::Taurus => "Taurus",
            Self::Gemini => "Gemini",
            Self::Cancer => "Cancer",
            Self::Leo => "Leo",
            Self::Virgo => "Virgo",
            Self::Libra => "Libra",
            Self::Scorpio => "Scorpio",
            Self::Sagittarius => "Sagittarius",
            Self::Capricorn => "Capricorn",
            Self::Aquarius => "Aquarius",
            Self::Pisces => "Pisces",
        };
        write!(f, "{name}")
    }
}

// ---------------------------------------------------------------------------
// Element and Modality
// ---------------------------------------------------------------------------

/// Classical elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Element {
    /// Fire (Aries, Leo, Sagittarius)
    Fire,
    /// Earth (Taurus, Virgo, Capricorn)
    Earth,
    /// Air (Gemini, Libra, Aquarius)
    Air,
    /// Water (Cancer, Scorpio, Pisces)
    Water,
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Fire => write!(f, "Fire"),
            Self::Earth => write!(f, "Earth"),
            Self::Air => write!(f, "Air"),
            Self::Water => write!(f, "Water"),
        }
    }
}

/// Sign modalities (qualities).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Modality {
    /// Cardinal (Aries, Cancer, Libra, Capricorn)
    Cardinal,
    /// Fixed (Taurus, Leo, Scorpio, Aquarius)
    Fixed,
    /// Mutable (Gemini, Virgo, Sagittarius, Pisces)
    Mutable,
}

impl fmt::Display for Modality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Cardinal => write!(f, "Cardinal"),
            Self::Fixed => write!(f, "Fixed"),
            Self::Mutable => write!(f, "Mutable"),
        }
    }
}

// ---------------------------------------------------------------------------
// Sign lookup
// ---------------------------------------------------------------------------

/// Position within a zodiac sign.
#[derive(Debug, Clone, Copy)]
pub struct SignPosition {
    /// The sign the longitude falls in.
    pub sign: Sign,
    /// Degrees within the sign (0.0..30.0).
    pub degrees_in_sign: f64,
}

/// Determine the tropical zodiac sign for a given ecliptic longitude.
///
/// # Examples
///
/// ```
/// # use jyotish::zodiac::{tropical_sign, Sign};
/// let pos = tropical_sign(45.0);
/// assert_eq!(pos.sign, Sign::Taurus);
/// assert!((pos.degrees_in_sign - 15.0).abs() < 1e-10);
/// ```
pub fn tropical_sign(longitude_deg: f64) -> SignPosition {
    let lon = normalize_degrees(longitude_deg);
    let index = (lon / 30.0) as usize;
    let index = index.min(11); // safety clamp for exactly 360.0
    SignPosition {
        sign: SIGNS[index],
        degrees_in_sign: lon - index as f64 * 30.0,
    }
}

/// Determine the sidereal zodiac sign for a given ecliptic longitude at a given Julian Date.
///
/// Uses the Lahiri ayanamsa to convert from tropical to sidereal.
///
/// # Examples
///
/// ```
/// # use jyotish::zodiac::{sidereal_sign, Sign};
/// // At J2000.0, the ayanamsa is ~23.85°, so 45° tropical → ~21° sidereal → Aries
/// let pos = sidereal_sign(45.0, 2_451_545.0);
/// assert_eq!(pos.sign, Sign::Aries);
/// ```
pub fn sidereal_sign(longitude_deg: f64, jd: f64) -> SignPosition {
    let sidereal_lon = normalize_degrees(longitude_deg - lahiri_ayanamsa(jd));
    tropical_sign(sidereal_lon)
}

// ---------------------------------------------------------------------------
// Ayanamsa
// ---------------------------------------------------------------------------

/// Lahiri ayanamsa in degrees for a given Julian Date.
///
/// The ayanamsa is the angular difference between the tropical and sidereal
/// zodiacs, caused by the precession of the equinoxes. The Lahiri ayanamsa
/// is the standard used by the Indian government's *Rashtriya Panchang*.
///
/// Uses the polynomial approximation:
/// ψ = 23.85° + 0.013972° × (JD - J2000.0) / 365.25
///
/// # Examples
///
/// ```
/// # use jyotish::zodiac::lahiri_ayanamsa;
/// let aya = lahiri_ayanamsa(2_451_545.0); // J2000.0
/// assert!((aya - 23.85).abs() < 0.1, "got {aya}");
/// ```
pub fn lahiri_ayanamsa(jd: f64) -> f64 {
    let t = julian_centuries(jd);
    // Lahiri ayanamsa: polynomial fit to official values
    // At J2000.0: ~23.85°, rate: ~50.29"/year = ~0.013969°/year
    23.85 + 1.396_971 * t + 0.000_308_6 * t * t
}

/// Convert a tropical ecliptic longitude to sidereal at a given epoch.
pub fn tropical_to_sidereal(longitude_deg: f64, jd: f64) -> f64 {
    normalize_degrees(longitude_deg - lahiri_ayanamsa(jd))
}

/// Convert a sidereal ecliptic longitude to tropical at a given epoch.
pub fn sidereal_to_tropical(longitude_deg: f64, jd: f64) -> f64 {
    normalize_degrees(longitude_deg + lahiri_ayanamsa(jd))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tropical_sign_boundaries() {
        for (i, &sign) in SIGNS.iter().enumerate() {
            let lon = i as f64 * 30.0 + 0.001;
            let pos = tropical_sign(lon);
            assert_eq!(pos.sign, sign, "wrong sign at {lon}°");
        }
    }

    #[test]
    fn tropical_sign_degrees_in_sign() {
        let pos = tropical_sign(45.0);
        assert_eq!(pos.sign, Sign::Taurus);
        assert!((pos.degrees_in_sign - 15.0).abs() < 1e-10);
    }

    #[test]
    fn tropical_sign_zero() {
        let pos = tropical_sign(0.0);
        assert_eq!(pos.sign, Sign::Aries);
        assert!(pos.degrees_in_sign.abs() < 1e-10);
    }

    #[test]
    fn tropical_sign_near_360() {
        let pos = tropical_sign(359.99);
        assert_eq!(pos.sign, Sign::Pisces);
    }

    #[test]
    fn sidereal_sign_j2000() {
        // At J2000.0, ayanamsa ≈ 23.85°
        // 45° tropical - 23.85° = ~21.15° sidereal → Aries
        let pos = sidereal_sign(45.0, 2_451_545.0);
        assert_eq!(pos.sign, Sign::Aries);
    }

    #[test]
    fn lahiri_ayanamsa_j2000() {
        let aya = lahiri_ayanamsa(2_451_545.0);
        assert!((aya - 23.85).abs() < 0.1, "got {aya}");
    }

    #[test]
    fn lahiri_ayanamsa_increases() {
        let aya_2000 = lahiri_ayanamsa(2_451_545.0);
        let aya_2100 = lahiri_ayanamsa(2_451_545.0 + 36525.0);
        assert!(aya_2100 > aya_2000, "ayanamsa should increase over time");
    }

    #[test]
    fn tropical_sidereal_roundtrip() {
        let jd = 2_451_545.0;
        let lon = 123.456;
        let sid = tropical_to_sidereal(lon, jd);
        let restored = sidereal_to_tropical(sid, jd);
        assert!((restored - lon).abs() < 1e-10);
    }

    #[test]
    fn sign_properties() {
        assert_eq!(Sign::Aries.element(), Element::Fire);
        assert_eq!(Sign::Aries.modality(), Modality::Cardinal);
        assert_eq!(Sign::Taurus.element(), Element::Earth);
        assert_eq!(Sign::Taurus.modality(), Modality::Fixed);
        assert_eq!(Sign::Gemini.element(), Element::Air);
        assert_eq!(Sign::Gemini.modality(), Modality::Mutable);
        assert_eq!(Sign::Cancer.element(), Element::Water);
        assert_eq!(Sign::Cancer.modality(), Modality::Cardinal);
    }

    #[test]
    fn sign_display() {
        assert_eq!(Sign::Sagittarius.to_string(), "Sagittarius");
        assert_eq!(Sign::Pisces.to_string(), "Pisces");
    }

    #[test]
    fn sign_cusp_longitudes() {
        assert!((Sign::Aries.cusp_longitude() - 0.0).abs() < 1e-10);
        assert!((Sign::Cancer.cusp_longitude() - 90.0).abs() < 1e-10);
        assert!((Sign::Libra.cusp_longitude() - 180.0).abs() < 1e-10);
        assert!((Sign::Capricorn.cusp_longitude() - 270.0).abs() < 1e-10);
    }

    #[test]
    fn sign_serde_roundtrip() {
        let sign = Sign::Scorpio;
        let json = serde_json::to_string(&sign).unwrap();
        let restored: Sign = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, sign);
    }
}
