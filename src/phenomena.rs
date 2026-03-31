//! Planetary phenomena — elongation, opposition, conjunction, station detection.
//!
//! Identifies notable astronomical events for planets: greatest elongation
//! (Mercury/Venus), opposition (outer planets), conjunction (all planets),
//! and stationary points (all planets except Sun/Moon).
//!
//! All functions accept Julian Dates in TT/TDB and search forward in time
//! from the given epoch.

use crate::coords::normalize_degrees;
use crate::error::{JyotishError, Result};
use crate::planet::Planet;
use crate::sun::solar_longitude;
use crate::transit::daily_motion;
use serde::{Deserialize, Serialize};
use std::fmt;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Maximum number of coarse search steps before giving up.
const MAX_SEARCH_STEPS: usize = 500;

/// Bisection convergence tolerance in Julian days (~0.1 seconds).
const BISECT_TOL: f64 = 1e-6;

/// Maximum bisection iterations.
const BISECT_ITERS: usize = 60;

// ---------------------------------------------------------------------------
// Phenomenon enum
// ---------------------------------------------------------------------------

/// A type of planetary phenomenon.
///
/// Represents notable geometric configurations between a planet and the Sun
/// as seen from Earth.
///
/// # Examples
///
/// ```
/// use jyotish::phenomena::Phenomenon;
///
/// let p = Phenomenon::Opposition;
/// assert_eq!(p.to_string(), "Opposition");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[non_exhaustive]
pub enum Phenomenon {
    /// Inner planet reaches maximum eastern elongation (visible after sunset).
    GreatestElongationEast,
    /// Inner planet reaches maximum western elongation (visible before sunrise).
    GreatestElongationWest,
    /// Outer planet opposite the Sun (elongation ~180°).
    Opposition,
    /// Planet at same ecliptic longitude as the Sun (elongation ~0°).
    Conjunction,
    /// Planet's daily motion passes through zero (direction reversal).
    Station,
    /// Planet at closest approach to the Sun (placeholder — not fully computed).
    Perihelion,
    /// Planet at farthest point from the Sun (placeholder — not fully computed).
    Aphelion,
}

impl fmt::Display for Phenomenon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GreatestElongationEast => write!(f, "Greatest Elongation East"),
            Self::GreatestElongationWest => write!(f, "Greatest Elongation West"),
            Self::Opposition => write!(f, "Opposition"),
            Self::Conjunction => write!(f, "Conjunction"),
            Self::Station => write!(f, "Station"),
            Self::Perihelion => write!(f, "Perihelion"),
            Self::Aphelion => write!(f, "Aphelion"),
        }
    }
}

// ---------------------------------------------------------------------------
// PhenomenonEvent struct
// ---------------------------------------------------------------------------

/// A detected planetary phenomenon event.
///
/// Contains the planet, the type of phenomenon, the Julian Date of the event,
/// and the Sun-planet elongation at that moment.
///
/// # Examples
///
/// ```
/// use jyotish::phenomena::{Phenomenon, PhenomenonEvent};
/// use jyotish::planet::Planet;
///
/// let evt = PhenomenonEvent {
///     planet: Planet::Venus,
///     phenomenon: Phenomenon::GreatestElongationEast,
///     jd: 2_451_600.0,
///     elongation: 46.0,
/// };
/// assert_eq!(evt.planet, Planet::Venus);
/// ```
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PhenomenonEvent {
    /// The planet involved.
    pub planet: Planet,
    /// The type of phenomenon detected.
    pub phenomenon: Phenomenon,
    /// Julian Date (TT) of the event.
    pub jd: f64,
    /// Sun-planet elongation in degrees at the event.
    ///
    /// East is positive (0..180), west is negative (-180..0) for signed
    /// elongation. For opposition this is near 180, for conjunction near 0.
    pub elongation: f64,
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Return true if this planet is an inner planet (orbit inside Earth's).
fn is_inner_planet(planet: Planet) -> bool {
    matches!(planet, Planet::Mercury | Planet::Venus)
}

/// Return true if this planet is an outer planet (orbit outside Earth's).
fn is_outer_planet(planet: Planet) -> bool {
    matches!(
        planet,
        Planet::Mars
            | Planet::Jupiter
            | Planet::Saturn
            | Planet::Uranus
            | Planet::Neptune
            | Planet::Pluto
    )
}

/// Get the ecliptic longitude of a planet at a given JD.
fn longitude_at(planet: Planet, jd: f64) -> Result<f64> {
    match planet {
        Planet::Sun => Ok(solar_longitude(jd)),
        Planet::Moon => Ok(crate::moon::lunar_longitude(jd)),
        _ => Ok(crate::planetary::compute_position(planet, jd)?.longitude_deg),
    }
}

// ---------------------------------------------------------------------------
// elongation_from_sun
// ---------------------------------------------------------------------------

/// Compute the signed elongation of a planet from the Sun in degrees.
///
/// Returns a value in the range (-180, 180]. Positive values indicate eastern
/// elongation (planet east of the Sun — visible in the evening), negative
/// values indicate western elongation (visible in the morning).
///
/// # Errors
///
/// Returns [`JyotishError::InvalidParameter`] if called with `Planet::Sun`
/// or `Planet::Moon`.
///
/// # Examples
///
/// ```
/// # use jyotish::phenomena::elongation_from_sun;
/// # use jyotish::planet::Planet;
/// let elong = elongation_from_sun(Planet::Venus, 2_451_545.0).unwrap();
/// assert!(elong.abs() <= 180.0, "elongation out of range: {elong}");
/// ```
pub fn elongation_from_sun(planet: Planet, jd: f64) -> Result<f64> {
    if matches!(planet, Planet::Sun | Planet::Moon) {
        return Err(JyotishError::InvalidParameter(
            "elongation is not defined for Sun or Moon".into(),
        ));
    }

    let sun_lon = solar_longitude(jd);
    let planet_lon = longitude_at(planet, jd)?;

    // Signed difference: planet_lon - sun_lon, normalized to (-180, 180]
    let mut diff = normalize_degrees(planet_lon - sun_lon);
    if diff > 180.0 {
        diff -= 360.0;
    }

    Ok(diff)
}

// ---------------------------------------------------------------------------
// next_greatest_elongation
// ---------------------------------------------------------------------------

/// Find the next greatest elongation of an inner planet after the given Julian Date.
///
/// For Mercury and Venus only. Searches forward in 1-day increments for a
/// local maximum of |elongation|, then refines with bisection on the derivative
/// of |elongation|.
///
/// Returns a [`PhenomenonEvent`] with the phenomenon set to either
/// [`Phenomenon::GreatestElongationEast`] or [`Phenomenon::GreatestElongationWest`]
/// depending on the sign of the elongation at the peak.
///
/// # Errors
///
/// Returns [`JyotishError::InvalidParameter`] if called with an outer planet,
/// the Sun, or the Moon.
///
/// # Examples
///
/// ```
/// # use jyotish::phenomena::next_greatest_elongation;
/// # use jyotish::planet::Planet;
/// let evt = next_greatest_elongation(Planet::Mercury, 2_451_545.0).unwrap();
/// assert!(evt.elongation.abs() >= 17.0 && evt.elongation.abs() <= 28.5,
///     "Mercury elongation {} out of expected range", evt.elongation);
/// ```
pub fn next_greatest_elongation(planet: Planet, jd: f64) -> Result<PhenomenonEvent> {
    if !is_inner_planet(planet) {
        return Err(JyotishError::InvalidParameter(format!(
            "greatest elongation is only defined for inner planets, not {planet}"
        )));
    }

    let step = 1.0; // 1-day steps for inner planets

    let mut prev_abs = elongation_from_sun(planet, jd)?.abs();
    let mut curr_jd = jd + step;
    let mut curr_abs = elongation_from_sun(planet, curr_jd)?.abs();

    // Walk forward looking for a peak: prev_abs < curr_abs > next_abs
    for _ in 0..MAX_SEARCH_STEPS {
        let next_jd = curr_jd + step;
        let next_abs = elongation_from_sun(planet, next_jd)?.abs();

        if curr_abs > prev_abs && curr_abs > next_abs {
            // Found a peak bracketed by [curr_jd - step, curr_jd + step].
            // Refine: find where d|elong|/dt = 0 using golden section search.
            let peak_jd = golden_section_max(curr_jd - step, curr_jd + step, |t| {
                elongation_from_sun(planet, t).map(|e| e.abs())
            })?;

            let elong = elongation_from_sun(planet, peak_jd)?;
            let phenomenon = if elong > 0.0 {
                Phenomenon::GreatestElongationEast
            } else {
                Phenomenon::GreatestElongationWest
            };

            return Ok(PhenomenonEvent {
                planet,
                phenomenon,
                jd: peak_jd,
                elongation: elong,
            });
        }

        prev_abs = curr_abs;
        curr_abs = next_abs;
        curr_jd = next_jd;
    }

    Err(JyotishError::InvalidParameter(format!(
        "greatest elongation search did not converge for {planet} within {MAX_SEARCH_STEPS} days"
    )))
}

/// Golden section search for maximum of `f` on `[a, b]`.
///
/// Returns the JD at which `f` attains its maximum.
fn golden_section_max(mut a: f64, mut b: f64, f: impl Fn(f64) -> Result<f64>) -> Result<f64> {
    let gr = 0.618_033_988_749_895; // (sqrt(5) - 1) / 2
    let mut c = b - gr * (b - a);
    let mut d = a + gr * (b - a);

    for _ in 0..BISECT_ITERS {
        if (b - a).abs() < BISECT_TOL {
            break;
        }

        let fc = f(c)?;
        let fd = f(d)?;

        if fc < fd {
            a = c;
            c = d;
            d = a + gr * (b - a);
        } else {
            b = d;
            d = c;
            c = b - gr * (b - a);
        }
    }

    Ok((a + b) / 2.0)
}

// ---------------------------------------------------------------------------
// next_station
// ---------------------------------------------------------------------------

/// Find the next stationary point of a planet after the given Julian Date.
///
/// A station occurs when the planet's daily motion crosses zero — the planet
/// appears to pause before reversing direction. Searches forward in 5-day
/// increments, then refines with bisection.
///
/// # Errors
///
/// Returns [`JyotishError::InvalidParameter`] if called with the Sun or Moon
/// (which never go retrograde), or if the search does not converge.
///
/// # Examples
///
/// ```
/// # use jyotish::phenomena::next_station;
/// # use jyotish::planet::Planet;
/// let evt = next_station(Planet::Mars, 2_451_545.0).unwrap();
/// assert!(evt.jd > 2_451_545.0);
/// assert!(evt.elongation.abs() <= 180.0);
/// ```
pub fn next_station(planet: Planet, jd: f64) -> Result<PhenomenonEvent> {
    if matches!(planet, Planet::Sun | Planet::Moon) {
        return Err(JyotishError::InvalidParameter(
            "Sun and Moon do not have stations".into(),
        ));
    }

    let step = 5.0;
    let mut jd_a = jd;
    let mut motion_a = daily_motion(planet, jd_a)?;

    for _ in 0..MAX_SEARCH_STEPS {
        let jd_b = jd_a + step;
        let motion_b = daily_motion(planet, jd_b)?;

        // Sign change means daily motion crossed zero
        if motion_a.signum() != motion_b.signum() {
            // Bisect to refine
            let mut lo = jd_a;
            let mut hi = jd_b;
            for _ in 0..BISECT_ITERS {
                let mid = (lo + hi) / 2.0;
                let m = daily_motion(planet, mid)?;
                if m.signum() == motion_a.signum() {
                    lo = mid;
                } else {
                    hi = mid;
                }
                if (hi - lo) < BISECT_TOL {
                    break;
                }
            }

            let station_jd = (lo + hi) / 2.0;
            let elong = elongation_from_sun(planet, station_jd)?;

            return Ok(PhenomenonEvent {
                planet,
                phenomenon: Phenomenon::Station,
                jd: station_jd,
                elongation: elong,
            });
        }

        jd_a = jd_b;
        motion_a = motion_b;
    }

    Err(JyotishError::InvalidParameter(format!(
        "station search did not converge for {planet}"
    )))
}

// ---------------------------------------------------------------------------
// next_opposition
// ---------------------------------------------------------------------------

/// Find the next opposition of an outer planet after the given Julian Date.
///
/// An opposition occurs when the planet's elongation from the Sun reaches
/// 180°. Only defined for Mars through Pluto. Searches forward in 5-day
/// increments, then refines with bisection.
///
/// # Errors
///
/// Returns [`JyotishError::InvalidParameter`] if called with an inner planet,
/// the Sun, or the Moon.
///
/// # Examples
///
/// ```
/// # use jyotish::phenomena::next_opposition;
/// # use jyotish::planet::Planet;
/// let evt = next_opposition(Planet::Jupiter, 2_451_545.0).unwrap();
/// assert!(evt.jd > 2_451_545.0);
/// assert!((evt.elongation.abs() - 180.0).abs() < 1.0,
///     "elongation at opposition: {}", evt.elongation);
/// ```
pub fn next_opposition(planet: Planet, jd: f64) -> Result<PhenomenonEvent> {
    if !is_outer_planet(planet) {
        return Err(JyotishError::InvalidParameter(format!(
            "opposition is only defined for outer planets, not {planet}"
        )));
    }

    // We want elongation ≈ 180°. Track (elongation - 180) and look for sign
    // change through zero. Since elongation is signed (-180..180], opposition
    // is when |elongation| ≈ 180, i.e. signed elongation is near ±180.
    //
    // Use the raw (0..360) difference to detect the 180° crossing cleanly.
    let raw_elong = |t: f64| -> Result<f64> {
        let sun_lon = solar_longitude(t);
        let planet_lon = longitude_at(planet, t)?;
        Ok(normalize_degrees(planet_lon - sun_lon))
    };

    let step = 5.0;
    let mut jd_a = jd;
    let mut ea = raw_elong(jd_a)? - 180.0;

    for _ in 0..MAX_SEARCH_STEPS {
        let jd_b = jd_a + step;
        let eb = raw_elong(jd_b)? - 180.0;

        // Sign change through zero means we crossed 180° elongation.
        // But we must avoid false crossings at 0° (conjunction). The raw
        // elongation near opposition will be close to 180, so |ea| and |eb|
        // should both be small (< 90) near a genuine opposition crossing.
        if ea.signum() != eb.signum() && ea.abs() < 90.0 && eb.abs() < 90.0 {
            let mut lo = jd_a;
            let mut hi = jd_b;
            let mut val_lo = ea;

            for _ in 0..BISECT_ITERS {
                let mid = (lo + hi) / 2.0;
                let vm = raw_elong(mid)? - 180.0;
                if vm.signum() == val_lo.signum() {
                    lo = mid;
                    val_lo = vm;
                } else {
                    hi = mid;
                }
                if (hi - lo) < BISECT_TOL {
                    break;
                }
            }

            let opp_jd = (lo + hi) / 2.0;
            let elong = elongation_from_sun(planet, opp_jd)?;

            return Ok(PhenomenonEvent {
                planet,
                phenomenon: Phenomenon::Opposition,
                jd: opp_jd,
                elongation: elong,
            });
        }

        jd_a = jd_b;
        ea = eb;
    }

    Err(JyotishError::InvalidParameter(format!(
        "opposition search did not converge for {planet}"
    )))
}

// ---------------------------------------------------------------------------
// next_conjunction
// ---------------------------------------------------------------------------

/// Find the next conjunction of a planet with the Sun after the given Julian Date.
///
/// A conjunction occurs when the planet's elongation from the Sun is ~0°.
/// Defined for all planets except the Sun and Moon. Searches forward in 5-day
/// increments, then refines with bisection.
///
/// # Errors
///
/// Returns [`JyotishError::InvalidParameter`] if called with the Sun or Moon.
///
/// # Examples
///
/// ```
/// # use jyotish::phenomena::next_conjunction;
/// # use jyotish::planet::Planet;
/// let evt = next_conjunction(Planet::Venus, 2_451_545.0).unwrap();
/// assert!(evt.jd > 2_451_545.0);
/// assert!(evt.elongation.abs() < 5.0,
///     "elongation at conjunction: {}", evt.elongation);
/// ```
pub fn next_conjunction(planet: Planet, jd: f64) -> Result<PhenomenonEvent> {
    if matches!(planet, Planet::Sun | Planet::Moon) {
        return Err(JyotishError::InvalidParameter(
            "conjunction with Sun is not defined for Sun or Moon".into(),
        ));
    }

    // Track the signed elongation; conjunction is when it crosses zero.
    // For inner planets this can happen frequently (~once per synodic period).
    // We need to distinguish conjunction (elongation crosses through 0)
    // from opposition (elongation crosses through ±180).
    let step = 5.0;
    let mut jd_a = jd;
    let mut ea = elongation_from_sun(planet, jd_a)?;

    for _ in 0..MAX_SEARCH_STEPS {
        let jd_b = jd_a + step;
        let eb = elongation_from_sun(planet, jd_b)?;

        // Sign change means elongation crossed zero or ±180.
        // Near conjunction (0°), both |ea| and |eb| are small (< 90).
        // Near opposition (180°), both are large (> 90).
        if ea.signum() != eb.signum() && ea.abs() < 90.0 && eb.abs() < 90.0 {
            let mut lo = jd_a;
            let mut hi = jd_b;
            let mut val_lo = ea;

            for _ in 0..BISECT_ITERS {
                let mid = (lo + hi) / 2.0;
                let vm = elongation_from_sun(planet, mid)?;
                if vm.signum() == val_lo.signum() {
                    lo = mid;
                    val_lo = vm;
                } else {
                    hi = mid;
                }
                if (hi - lo) < BISECT_TOL {
                    break;
                }
            }

            let conj_jd = (lo + hi) / 2.0;
            let elong = elongation_from_sun(planet, conj_jd)?;

            return Ok(PhenomenonEvent {
                planet,
                phenomenon: Phenomenon::Conjunction,
                jd: conj_jd,
                elongation: elong,
            });
        }

        jd_a = jd_b;
        ea = eb;
    }

    Err(JyotishError::InvalidParameter(format!(
        "conjunction search did not converge for {planet}"
    )))
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    /// J2000.0 epoch.
    const JD_J2000: f64 = 2_451_545.0;

    // ----- elongation_from_sun -----

    #[test]
    fn elongation_sun_errors() {
        assert!(elongation_from_sun(Planet::Sun, JD_J2000).is_err());
        assert!(elongation_from_sun(Planet::Moon, JD_J2000).is_err());
    }

    #[test]
    fn elongation_venus_at_j2000() {
        let elong = elongation_from_sun(Planet::Venus, JD_J2000).unwrap();
        // Venus elongation should be in valid range
        assert!(elong.abs() <= 180.0, "elongation out of range: {elong}");
    }

    #[test]
    fn elongation_mars_at_j2000() {
        let elong = elongation_from_sun(Planet::Mars, JD_J2000).unwrap();
        assert!(elong.abs() <= 180.0, "elongation out of range: {elong}");
    }

    #[test]
    fn elongation_jupiter_at_known_date() {
        // At J2000.0, Jupiter was at ~34° Taurus, Sun at ~280° Capricorn.
        // Rough elongation ~ 80-90° east.
        let elong = elongation_from_sun(Planet::Jupiter, JD_J2000).unwrap();
        assert!(
            elong > 0.0,
            "Jupiter should be east of Sun at J2000: {elong}"
        );
    }

    // ----- greatest elongation -----

    #[test]
    fn mercury_greatest_elongation_after_j2000() {
        let evt = next_greatest_elongation(Planet::Mercury, JD_J2000).unwrap();
        assert!(evt.jd > JD_J2000);

        // Mercury's synodic period is ~116 days, so greatest elongation
        // should occur within that window.
        assert!(
            evt.jd - JD_J2000 < 116.0,
            "took too long: {} days",
            evt.jd - JD_J2000
        );

        // Mercury's greatest elongation ranges from ~18° to ~28°.
        assert!(
            evt.elongation.abs() >= 17.0 && evt.elongation.abs() <= 28.5,
            "unexpected Mercury elongation: {}",
            evt.elongation
        );

        assert!(matches!(
            evt.phenomenon,
            Phenomenon::GreatestElongationEast | Phenomenon::GreatestElongationWest
        ));
    }

    #[test]
    fn venus_greatest_elongation() {
        let evt = next_greatest_elongation(Planet::Venus, JD_J2000).unwrap();
        assert!(evt.jd > JD_J2000);

        // Venus greatest elongation is ~45-47°.
        assert!(
            evt.elongation.abs() >= 44.0 && evt.elongation.abs() <= 48.0,
            "unexpected Venus elongation: {}",
            evt.elongation
        );
    }

    #[test]
    fn greatest_elongation_mars_errors() {
        assert!(next_greatest_elongation(Planet::Mars, JD_J2000).is_err());
        assert!(next_greatest_elongation(Planet::Jupiter, JD_J2000).is_err());
        assert!(next_greatest_elongation(Planet::Sun, JD_J2000).is_err());
    }

    // ----- opposition -----

    #[test]
    fn jupiter_opposition_after_j2000() {
        let evt = next_opposition(Planet::Jupiter, JD_J2000).unwrap();
        assert!(evt.jd > JD_J2000);

        // Jupiter's synodic period is ~399 days (~13 months).
        assert!(
            evt.jd - JD_J2000 < 400.0,
            "took too long: {} days",
            evt.jd - JD_J2000
        );

        // At opposition, elongation should be near ±180°.
        assert!(
            evt.elongation.abs() > 170.0,
            "elongation at opposition too low: {}",
            evt.elongation
        );

        assert_eq!(evt.phenomenon, Phenomenon::Opposition);
    }

    #[test]
    fn mars_opposition() {
        // Mars opposition occurs roughly every 780 days.
        let evt = next_opposition(Planet::Mars, JD_J2000).unwrap();
        assert!(evt.jd > JD_J2000);
        assert!(
            evt.jd - JD_J2000 < 800.0,
            "took too long: {} days",
            evt.jd - JD_J2000
        );
        assert!(evt.elongation.abs() > 170.0);
    }

    #[test]
    fn opposition_venus_errors() {
        assert!(next_opposition(Planet::Venus, JD_J2000).is_err());
        assert!(next_opposition(Planet::Mercury, JD_J2000).is_err());
        assert!(next_opposition(Planet::Sun, JD_J2000).is_err());
        assert!(next_opposition(Planet::Moon, JD_J2000).is_err());
    }

    // ----- station -----

    #[test]
    fn mars_station_after_j2000() {
        let evt = next_station(Planet::Mars, JD_J2000).unwrap();
        assert!(evt.jd > JD_J2000);

        // Mars station should occur within ~2.5 years.
        assert!(
            evt.jd - JD_J2000 < 900.0,
            "took too long: {} days",
            evt.jd - JD_J2000
        );

        assert_eq!(evt.phenomenon, Phenomenon::Station);
        assert!(evt.elongation.abs() <= 180.0);
    }

    #[test]
    fn saturn_station() {
        let evt = next_station(Planet::Saturn, JD_J2000).unwrap();
        assert!(evt.jd > JD_J2000);
        // Saturn stations roughly every ~1 year.
        assert!(evt.jd - JD_J2000 < 400.0);
    }

    #[test]
    fn station_sun_moon_errors() {
        assert!(next_station(Planet::Sun, JD_J2000).is_err());
        assert!(next_station(Planet::Moon, JD_J2000).is_err());
    }

    // ----- conjunction -----

    #[test]
    fn venus_conjunction_after_j2000() {
        let evt = next_conjunction(Planet::Venus, JD_J2000).unwrap();
        assert!(evt.jd > JD_J2000);

        // Venus synodic period is ~584 days; conjunction within that window.
        assert!(
            evt.jd - JD_J2000 < 585.0,
            "took too long: {} days",
            evt.jd - JD_J2000
        );

        // At conjunction, elongation should be near 0°.
        assert!(
            evt.elongation.abs() < 5.0,
            "elongation at conjunction: {}",
            evt.elongation
        );

        assert_eq!(evt.phenomenon, Phenomenon::Conjunction);
    }

    #[test]
    fn mars_conjunction() {
        let evt = next_conjunction(Planet::Mars, JD_J2000).unwrap();
        assert!(evt.jd > JD_J2000);
        assert!(evt.elongation.abs() < 5.0);
    }

    #[test]
    fn conjunction_sun_moon_errors() {
        assert!(next_conjunction(Planet::Sun, JD_J2000).is_err());
        assert!(next_conjunction(Planet::Moon, JD_J2000).is_err());
    }

    // ----- Display / serde -----

    #[test]
    fn phenomenon_display() {
        assert_eq!(
            Phenomenon::GreatestElongationEast.to_string(),
            "Greatest Elongation East"
        );
        assert_eq!(
            Phenomenon::GreatestElongationWest.to_string(),
            "Greatest Elongation West"
        );
        assert_eq!(Phenomenon::Opposition.to_string(), "Opposition");
        assert_eq!(Phenomenon::Conjunction.to_string(), "Conjunction");
        assert_eq!(Phenomenon::Station.to_string(), "Station");
        assert_eq!(Phenomenon::Perihelion.to_string(), "Perihelion");
        assert_eq!(Phenomenon::Aphelion.to_string(), "Aphelion");
    }

    #[test]
    fn phenomenon_serde_roundtrip() {
        let p = Phenomenon::Opposition;
        let json = serde_json::to_string(&p).unwrap();
        let restored: Phenomenon = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, p);
    }

    #[test]
    fn phenomenon_event_serde_roundtrip() {
        let evt = PhenomenonEvent {
            planet: Planet::Jupiter,
            phenomenon: Phenomenon::Opposition,
            jd: 2_451_800.0,
            elongation: 179.5,
        };
        let json = serde_json::to_string(&evt).unwrap();
        let restored: PhenomenonEvent = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.planet, Planet::Jupiter);
        assert_eq!(restored.phenomenon, Phenomenon::Opposition);
        assert!((restored.jd - 2_451_800.0).abs() < f64::EPSILON);
    }
}
