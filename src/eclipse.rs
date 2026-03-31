//! Eclipse prediction — solar and lunar eclipse detection.
//!
//! Implements eclipse detection using the lunar node proximity method,
//! based on Meeus (*Astronomical Algorithms*, Chapter 54). The algorithm
//! finds new/full moons via bisection on Sun–Moon elongation, then checks
//! whether the Moon is close enough to the ecliptic for an eclipse to occur.
//!
//! This provides practical approximate eclipse prediction — not full
//! Besselian element computation.
//!
//! # Examples
//!
//! ```
//! # use jyotish::eclipse::{next_solar_eclipse, next_lunar_eclipse, eclipses_in_year};
//! // Find the next solar eclipse after J2000.0
//! let eclipse = next_solar_eclipse(2_451_545.0).unwrap();
//! assert!(eclipse.magnitude > 0.0);
//!
//! // Find all eclipses in a year
//! let eclipses = eclipses_in_year(2000).unwrap();
//! assert!(!eclipses.is_empty());
//! ```

use crate::coords::deg_to_rad;
use crate::error::{JyotishError, Result};
use crate::moon::{lunar_distance_km, lunar_latitude, lunar_longitude};
use crate::sun::{solar_distance_au, solar_longitude};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Maximum number of lunations to search before giving up.
const MAX_SEARCH_LUNATIONS: u32 = 50;

/// Earth's equatorial radius in km.
const EARTH_RADIUS_KM: f64 = 6378.14;

/// Bisection tolerance in days (~0.086 seconds).
const BISECTION_TOL: f64 = 1e-6;

/// Maximum lunar latitude (degrees) for an eclipse to be possible.
const ECLIPSE_LAT_LIMIT: f64 = 1.5;

// ---------------------------------------------------------------------------
// EclipseType
// ---------------------------------------------------------------------------

/// Classification of an eclipse event.
///
/// Solar eclipses occur at new moon when the Moon passes between the Sun and
/// Earth. Lunar eclipses occur at full moon when the Moon passes through
/// Earth's shadow.
///
/// # Examples
///
/// ```
/// # use jyotish::eclipse::EclipseType;
/// let et = EclipseType::SolarTotal;
/// assert_eq!(format!("{et}"), "Total Solar Eclipse");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EclipseType {
    /// Total solar eclipse — Moon completely covers the Sun.
    SolarTotal,
    /// Annular solar eclipse — Moon is too far to cover the Sun completely.
    SolarAnnular,
    /// Partial solar eclipse — Moon only partially covers the Sun.
    SolarPartial,
    /// Hybrid (annular-total) solar eclipse — transitions between annular and total.
    SolarHybrid,
    /// Total lunar eclipse — Moon is fully immersed in Earth's umbral shadow.
    LunarTotal,
    /// Partial lunar eclipse — Moon partially enters Earth's umbral shadow.
    LunarPartial,
    /// Penumbral lunar eclipse — Moon passes through Earth's penumbral shadow only.
    LunarPenumbral,
}

impl fmt::Display for EclipseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SolarTotal => write!(f, "Total Solar Eclipse"),
            Self::SolarAnnular => write!(f, "Annular Solar Eclipse"),
            Self::SolarPartial => write!(f, "Partial Solar Eclipse"),
            Self::SolarHybrid => write!(f, "Hybrid Solar Eclipse"),
            Self::LunarTotal => write!(f, "Total Lunar Eclipse"),
            Self::LunarPartial => write!(f, "Partial Lunar Eclipse"),
            Self::LunarPenumbral => write!(f, "Penumbral Lunar Eclipse"),
        }
    }
}

// ---------------------------------------------------------------------------
// Eclipse struct
// ---------------------------------------------------------------------------

/// An eclipse event with its type, timing, magnitude, and geometry.
///
/// # Fields
///
/// - `eclipse_type` — classification of the eclipse.
/// - `jd_max` — Julian Date of maximum eclipse.
/// - `magnitude` — eclipse magnitude (fraction of the Sun's or Moon's diameter
///   obscured). Typically 0.0–1.0+; values above 1.0 indicate a total eclipse
///   where the obscuring body appears larger than the obscured body.
/// - `gamma` — distance of the Moon's shadow axis from Earth's center, measured
///   in Earth radii. For lunar eclipses, this represents the Moon's offset from
///   the center of Earth's shadow.
///
/// # Examples
///
/// ```
/// # use jyotish::eclipse::{Eclipse, EclipseType};
/// let e = Eclipse {
///     eclipse_type: EclipseType::SolarTotal,
///     jd_max: 2_451_545.0,
///     magnitude: 1.02,
///     gamma: 0.15,
/// };
/// assert!(e.magnitude > 1.0);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Eclipse {
    /// The type/classification of this eclipse.
    pub eclipse_type: EclipseType,
    /// Julian Date of maximum eclipse.
    pub jd_max: f64,
    /// Eclipse magnitude (fraction obscured; >1.0 for total solar eclipses).
    pub magnitude: f64,
    /// Distance of shadow axis from Earth's center in Earth radii.
    pub gamma: f64,
}

impl fmt::Display for Eclipse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} at JD {:.4} (mag={:.4}, gamma={:.4})",
            self.eclipse_type, self.jd_max, self.magnitude, self.gamma
        )
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Signed angular difference, wrapped to [-180, 180).
fn signed_diff(a: f64, b: f64) -> f64 {
    let mut d = a - b;
    if d > 180.0 {
        d -= 360.0;
    }
    if d < -180.0 {
        d += 360.0;
    }
    d
}

/// Raw (unwrapped) elongation: `lunar_longitude - solar_longitude`, in [0, 360).
fn raw_elongation(jd: f64) -> f64 {
    crate::coords::normalize_degrees(lunar_longitude(jd) - solar_longitude(jd))
}

/// Bisect a generic function `f` to find a zero crossing between `lo` and `hi`.
/// `flo` must be `f(lo)`. Returns the midpoint when `|hi - lo| < tol`.
fn bisect(f: impl Fn(f64) -> f64, mut lo: f64, mut hi: f64, mut flo: f64, tol: f64) -> f64 {
    for _ in 0..100 {
        if (hi - lo) < tol {
            break;
        }
        let mid = (lo + hi) / 2.0;
        let fmid = f(mid);
        if flo.signum() == fmid.signum() {
            lo = mid;
            flo = fmid;
        } else {
            hi = mid;
        }
    }
    (lo + hi) / 2.0
}

/// Find the next new moon (elongation ≈ 0°) after `jd` using bisection.
///
/// New moon occurs when the raw elongation wraps from ~360° back to ~0°.
/// We detect this as a large negative jump in raw elongation between steps.
fn find_next_new_moon(jd: f64) -> Result<f64> {
    // Use a step small enough to not skip a lunation (~29.5 days)
    let step = 5.0;
    let mut t0 = jd;
    let mut e0 = raw_elongation(t0);

    for _ in 0..3000 {
        let t1 = t0 + step;
        let e1 = raw_elongation(t1);

        // New moon: elongation wraps from high (~350°+) to low (~0°+)
        // This shows as e1 < e0 with a large difference (> 180°)
        if e0 > 180.0 && e1 < 180.0 && (e0 - e1) > 180.0 {
            // Elongation wrapped through 0; bisect using signed_diff from target 0
            let f = |t: f64| -> f64 { signed_diff(raw_elongation(t), 0.0) };
            let nm = bisect(f, t0, t1, f(t0), BISECTION_TOL);
            if nm > jd {
                return Ok(nm);
            }
        }

        t0 = t1;
        e0 = e1;
    }

    Err(JyotishError::EphemerisError(
        "could not find next new moon within search range".into(),
    ))
}

/// Find the next full moon (elongation ≈ 180°) after `jd` using bisection.
///
/// Full moon occurs when the raw elongation crosses 180° going upward.
fn find_next_full_moon(jd: f64) -> Result<f64> {
    let step = 5.0;
    let mut t0 = jd;
    let mut e0 = raw_elongation(t0);

    for _ in 0..3000 {
        let t1 = t0 + step;
        let e1 = raw_elongation(t1);

        // Full moon: elongation crosses 180° going upward (increasing)
        // e0 < 180 and e1 >= 180, with no wrap (difference < 180)
        if e0 < 180.0 && e1 >= 180.0 && (e1 - e0) < 180.0 {
            // Bisect: f = elongation - 180, find zero
            let f = |t: f64| -> f64 { raw_elongation(t) - 180.0 };
            let fm = bisect(f, t0, t1, f(t0), BISECTION_TOL);
            if fm > jd {
                return Ok(fm);
            }
        }

        t0 = t1;
        e0 = e1;
    }

    Err(JyotishError::EphemerisError(
        "could not find next full moon within search range".into(),
    ))
}

/// Sun semi-diameter in arcseconds for a given solar distance in AU.
fn sun_semi_diameter(dist_au: f64) -> f64 {
    959.63 / dist_au
}

/// Moon semi-diameter in arcseconds for a given lunar distance in km.
fn moon_semi_diameter(dist_km: f64) -> f64 {
    // asin(1737.4 / dist) ≈ 1737.4 / dist (small angle) converted to arcseconds
    // More precisely: 358473400 / dist_km (= 1737.4 * 206265 / dist_km)
    358_473_400.0 / dist_km
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Find the next solar eclipse after the given Julian Date.
///
/// Searches forward from `jd` through successive new moons, checking each
/// for eclipse geometry. Returns the first solar eclipse found.
///
/// # Errors
///
/// Returns [`JyotishError::EphemerisError`] if no eclipse is found within
/// the search limit (approximately 4 years).
///
/// # Examples
///
/// ```
/// # use jyotish::eclipse::next_solar_eclipse;
/// // The first solar eclipse after J2000.0 was on 2000-02-05
/// let eclipse = next_solar_eclipse(2_451_545.0).unwrap();
/// assert!((eclipse.jd_max - 2_451_580.0).abs() < 5.0);
/// assert!(eclipse.magnitude > 0.0);
/// ```
pub fn next_solar_eclipse(jd: f64) -> Result<Eclipse> {
    let mut search_jd = jd;

    for _ in 0..MAX_SEARCH_LUNATIONS {
        let nm = find_next_new_moon(search_jd)?;
        let lat = lunar_latitude(nm);

        if lat.abs() < ECLIPSE_LAT_LIMIT {
            let moon_dist = lunar_distance_km(nm);
            let sun_dist = solar_distance_au(nm);
            let lat_rad = deg_to_rad(lat);

            // Gamma: distance of Moon's shadow axis from Earth's center (Earth radii)
            let gamma = lat_rad.sin() * moon_dist / EARTH_RADIUS_KM;

            if gamma.abs() < 1.5 {
                let s_sd = sun_semi_diameter(sun_dist);
                let m_sd = moon_semi_diameter(moon_dist);

                let (eclipse_type, magnitude) = classify_solar(gamma, s_sd, m_sd);

                return Ok(Eclipse {
                    eclipse_type,
                    jd_max: nm,
                    magnitude,
                    gamma,
                });
            }
        }

        // Advance past this new moon to search for the next one
        search_jd = nm + 1.0;
    }

    Err(JyotishError::EphemerisError(
        "no solar eclipse found within search limit".into(),
    ))
}

/// Classify a solar eclipse and compute its magnitude.
fn classify_solar(gamma: f64, sun_sd: f64, moon_sd: f64) -> (EclipseType, f64) {
    let abs_gamma = gamma.abs();

    // Magnitude: ratio of diameters obscured
    // For central eclipses, magnitude ≈ moon_sd / sun_sd
    // For partial eclipses, magnitude depends on how much of the Sun is covered
    if abs_gamma > 0.9972 {
        // Partial eclipse — only the edge of the shadow crosses Earth
        // Approximate magnitude from geometry
        let mag = (1.5433 - abs_gamma) / (0.5461 + (sun_sd / moon_sd - 1.0).abs() * 0.5);
        let mag = mag.clamp(0.001, 0.999);
        (EclipseType::SolarPartial, mag)
    } else if (moon_sd - sun_sd).abs() < 0.5 {
        // Very close sizes — hybrid eclipse
        let mag = moon_sd / sun_sd;
        (EclipseType::SolarHybrid, mag)
    } else if moon_sd > sun_sd {
        // Moon appears larger → total
        let mag = moon_sd / sun_sd;
        (EclipseType::SolarTotal, mag)
    } else {
        // Moon appears smaller → annular
        let mag = moon_sd / sun_sd;
        (EclipseType::SolarAnnular, mag)
    }
}

/// Find the next lunar eclipse after the given Julian Date.
///
/// Searches forward from `jd` through successive full moons, checking each
/// for eclipse geometry. Returns the first lunar eclipse found.
///
/// # Errors
///
/// Returns [`JyotishError::EphemerisError`] if no eclipse is found within
/// the search limit (approximately 4 years).
///
/// # Examples
///
/// ```
/// # use jyotish::eclipse::next_lunar_eclipse;
/// // The first lunar eclipse after J2000.0 was on 2000-01-21
/// let eclipse = next_lunar_eclipse(2_451_545.0).unwrap();
/// assert!(eclipse.magnitude > 0.0);
/// ```
pub fn next_lunar_eclipse(jd: f64) -> Result<Eclipse> {
    let mut search_jd = jd;

    for _ in 0..MAX_SEARCH_LUNATIONS {
        let fm = find_next_full_moon(search_jd)?;
        let lat = lunar_latitude(fm);

        if lat.abs() < ECLIPSE_LAT_LIMIT {
            let moon_dist = lunar_distance_km(fm);
            let sun_dist = solar_distance_au(fm);
            let lat_rad = deg_to_rad(lat);

            let m_sd = moon_semi_diameter(moon_dist);
            let s_sd = sun_semi_diameter(sun_dist);

            // Earth's shadow radii at the Moon's distance (arcseconds)
            let earth_shadow_angle = 1.02 * (0.998_340 * EARTH_RADIUS_KM / moon_dist) * 206_265.0;
            let penumbral_radius = earth_shadow_angle + s_sd;
            let umbral_radius = earth_shadow_angle - s_sd;

            // Separation of Moon center from shadow center (arcseconds)
            let separation = lat_rad.abs() * 206_265.0; // lat in radians → arcsec

            // Gamma for lunar eclipses: Moon offset in Earth radii
            let gamma = lat_rad.sin() * moon_dist / EARTH_RADIUS_KM;

            if let Some((eclipse_type, magnitude)) =
                classify_lunar(separation, m_sd, penumbral_radius, umbral_radius)
            {
                return Ok(Eclipse {
                    eclipse_type,
                    jd_max: fm,
                    magnitude,
                    gamma,
                });
            }
        }

        search_jd = fm + 1.0;
    }

    Err(JyotishError::EphemerisError(
        "no lunar eclipse found within search limit".into(),
    ))
}

/// Classify a lunar eclipse based on shadow geometry.
///
/// Returns `None` if no eclipse occurs (Moon outside penumbra).
fn classify_lunar(
    separation: f64,
    moon_sd: f64,
    penumbral_r: f64,
    umbral_r: f64,
) -> Option<(EclipseType, f64)> {
    if separation >= penumbral_r + moon_sd {
        // Moon entirely outside the penumbra — no eclipse
        return None;
    }

    if separation + moon_sd <= umbral_r {
        // Moon entirely within the umbra — total lunar eclipse
        let mag = (umbral_r + moon_sd - separation) / (2.0 * moon_sd);
        return Some((EclipseType::LunarTotal, mag));
    }

    if separation < umbral_r + moon_sd && separation + moon_sd > umbral_r {
        // Moon partially in the umbra — partial lunar eclipse
        let mag = (umbral_r + moon_sd - separation) / (2.0 * moon_sd);
        if mag > 0.0 {
            return Some((EclipseType::LunarPartial, mag));
        }
    }

    if separation < penumbral_r + moon_sd {
        // Moon in the penumbra only — penumbral lunar eclipse
        let mag = (penumbral_r + moon_sd - separation) / (2.0 * moon_sd);
        return Some((EclipseType::LunarPenumbral, mag));
    }

    None
}

/// Find all eclipses (solar and lunar) in a given calendar year.
///
/// Returns a vector of [`Eclipse`] events sorted by Julian Date. Searches
/// the full year from January 1 through December 31.
///
/// # Errors
///
/// Returns an error if the year-to-JD conversion fails.
///
/// # Examples
///
/// ```
/// # use jyotish::eclipse::eclipses_in_year;
/// let eclipses = eclipses_in_year(2000).unwrap();
/// assert!(eclipses.len() >= 2, "expected at least 2 eclipses in 2000");
/// for e in &eclipses {
///     assert!(e.magnitude > 0.0);
/// }
/// ```
pub fn eclipses_in_year(year: i32) -> Result<Vec<Eclipse>> {
    let jd_start = crate::calendar::gregorian_to_jd(year, 1, 1, 0, 0, 0.0)?;
    let jd_end = crate::calendar::gregorian_to_jd(year + 1, 1, 1, 0, 0, 0.0)?;

    let mut eclipses = Vec::new();
    let mut search_jd = jd_start;

    // Search for solar eclipses
    loop {
        match next_solar_eclipse(search_jd) {
            Ok(e) if e.jd_max < jd_end => {
                search_jd = e.jd_max + 20.0;
                eclipses.push(e);
            }
            _ => break,
        }
    }

    // Search for lunar eclipses
    search_jd = jd_start;
    loop {
        match next_lunar_eclipse(search_jd) {
            Ok(e) if e.jd_max < jd_end => {
                search_jd = e.jd_max + 20.0;
                eclipses.push(e);
            }
            _ => break,
        }
    }

    eclipses.sort_by(|a, b| a.jd_max.partial_cmp(&b.jd_max).unwrap());
    Ok(eclipses)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// J2000.0 epoch.
    const J2000: f64 = 2_451_545.0;

    #[test]
    fn signed_diff_wrapping() {
        assert!((signed_diff(350.0, 10.0) - (-20.0)).abs() < 1e-10);
        assert!((signed_diff(10.0, 350.0) - 20.0).abs() < 1e-10);
        assert!((signed_diff(180.0, 0.0) - 180.0).abs() < 1e-10);
    }

    #[test]
    fn find_new_moon_near_j2000() {
        // New moon near J2000.0: 2000-01-06 ~18:14 UT → JD ~2451550.26
        let nm = find_next_new_moon(J2000).unwrap();
        assert!(
            (nm - 2_451_550.26).abs() < 1.0,
            "new moon JD = {nm}, expected ~2451550.26"
        );
    }

    #[test]
    fn find_full_moon_near_j2000() {
        // Full moon near J2000.0: 2000-01-21 ~04:41 UT → JD ~2451563.69
        let fm = find_next_full_moon(J2000).unwrap();
        assert!(
            (fm - 2_451_563.7).abs() < 1.0,
            "full moon JD = {fm}, expected ~2451563.7"
        );
    }

    #[test]
    fn next_solar_eclipse_after_j2000() {
        // 2000-02-05: partial solar eclipse, JD ~2451580
        let eclipse = next_solar_eclipse(J2000).unwrap();
        assert!(
            (eclipse.jd_max - 2_451_580.0).abs() < 5.0,
            "solar eclipse JD = {}, expected ~2451580",
            eclipse.jd_max
        );
        assert!(
            eclipse.magnitude > 0.0,
            "magnitude should be positive: {}",
            eclipse.magnitude
        );
        matches!(
            eclipse.eclipse_type,
            EclipseType::SolarTotal
                | EclipseType::SolarAnnular
                | EclipseType::SolarPartial
                | EclipseType::SolarHybrid
        );
    }

    #[test]
    fn next_lunar_eclipse_after_j2000() {
        // 2000-01-21: total lunar eclipse, JD ~2451564
        // Search from just before J2000 to catch this eclipse
        let eclipse = next_lunar_eclipse(J2000 - 30.0).unwrap();
        assert!(
            (eclipse.jd_max - 2_451_564.0).abs() < 5.0,
            "lunar eclipse JD = {}, expected ~2451564",
            eclipse.jd_max
        );
        assert!(
            eclipse.magnitude > 0.0,
            "magnitude should be positive: {}",
            eclipse.magnitude
        );
    }

    #[test]
    fn eclipses_in_year_2000() {
        let eclipses = eclipses_in_year(2000).unwrap();
        assert!(
            eclipses.len() >= 2,
            "expected at least 2 eclipses in 2000, got {}",
            eclipses.len()
        );

        // Verify they are sorted by date
        for w in eclipses.windows(2) {
            assert!(w[0].jd_max <= w[1].jd_max, "eclipses not sorted by date");
        }

        // All magnitudes should be positive
        for e in &eclipses {
            assert!(e.magnitude > 0.0, "eclipse magnitude <= 0: {e}");
        }
    }

    #[test]
    fn eclipse_magnitude_valid_range() {
        let eclipse = next_solar_eclipse(J2000).unwrap();
        assert!(
            eclipse.magnitude > 0.0 && eclipse.magnitude < 2.0,
            "solar eclipse magnitude out of range: {}",
            eclipse.magnitude
        );

        let eclipse = next_lunar_eclipse(J2000).unwrap();
        assert!(
            eclipse.magnitude > 0.0 && eclipse.magnitude < 5.0,
            "lunar eclipse magnitude out of range: {}",
            eclipse.magnitude
        );
    }

    #[test]
    fn eclipse_type_display() {
        assert_eq!(EclipseType::SolarTotal.to_string(), "Total Solar Eclipse");
        assert_eq!(
            EclipseType::SolarAnnular.to_string(),
            "Annular Solar Eclipse"
        );
        assert_eq!(
            EclipseType::SolarPartial.to_string(),
            "Partial Solar Eclipse"
        );
        assert_eq!(EclipseType::SolarHybrid.to_string(), "Hybrid Solar Eclipse");
        assert_eq!(EclipseType::LunarTotal.to_string(), "Total Lunar Eclipse");
        assert_eq!(
            EclipseType::LunarPartial.to_string(),
            "Partial Lunar Eclipse"
        );
        assert_eq!(
            EclipseType::LunarPenumbral.to_string(),
            "Penumbral Lunar Eclipse"
        );
    }

    #[test]
    fn eclipse_type_serde_roundtrip() {
        let types = [
            EclipseType::SolarTotal,
            EclipseType::SolarAnnular,
            EclipseType::SolarPartial,
            EclipseType::SolarHybrid,
            EclipseType::LunarTotal,
            EclipseType::LunarPartial,
            EclipseType::LunarPenumbral,
        ];
        for et in &types {
            let json = serde_json::to_string(et).unwrap();
            let rt: EclipseType = serde_json::from_str(&json).unwrap();
            assert_eq!(*et, rt, "roundtrip failed for {et}");
        }
    }

    #[test]
    fn eclipse_struct_serde_roundtrip() {
        let e = Eclipse {
            eclipse_type: EclipseType::SolarTotal,
            jd_max: 2_451_580.0,
            magnitude: 1.024,
            gamma: 0.123,
        };
        let json = serde_json::to_string(&e).unwrap();
        let rt: Eclipse = serde_json::from_str(&json).unwrap();
        assert_eq!(rt.eclipse_type, e.eclipse_type);
        assert!((rt.jd_max - e.jd_max).abs() < 1e-10);
        assert!((rt.magnitude - e.magnitude).abs() < 1e-10);
        assert!((rt.gamma - e.gamma).abs() < 1e-10);
    }

    #[test]
    fn eclipse_display() {
        let e = Eclipse {
            eclipse_type: EclipseType::SolarTotal,
            jd_max: 2_451_580.123_4,
            magnitude: 1.024,
            gamma: 0.123,
        };
        let s = e.to_string();
        assert!(s.contains("Total Solar Eclipse"));
        assert!(s.contains("2451580.1234"));
    }

    #[test]
    fn solar_eclipse_gamma_in_range() {
        let eclipse = next_solar_eclipse(J2000).unwrap();
        assert!(
            eclipse.gamma.abs() < 1.6,
            "gamma out of expected range: {}",
            eclipse.gamma
        );
    }
}
