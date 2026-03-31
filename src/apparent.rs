//! Apparent position pipeline.
//!
//! Converts geometric (raw ephemeris) positions into apparent positions
//! by applying corrections:
//!
//! 1. **Annual aberration** — Earth's orbital velocity effect (~20.5")
//! 2. **Nutation** — short-period axis oscillation (~9" max)
//!
//! Light-time correction is available separately via
//! [`crate::aberration::light_time_correction`] for use cases that need
//! heliocentric position functions as input.
//!
//! The result is the position where the body actually appears on the sky.

use crate::aberration::apply_aberration;
use crate::calendar::julian_centuries;
use crate::coords::{mean_obliquity, normalize_degrees};
use crate::error::{JyotishError, Result};
use crate::nutation::nutation_longitude;
use crate::planet::{Planet, PlanetaryPosition};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Position type
// ---------------------------------------------------------------------------

/// The type of a computed position, indicating which corrections have been applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PositionType {
    /// Raw ephemeris output, no corrections applied.
    Geometric,
    /// Geometric + light-time correction.
    Astrometric,
    /// Geometric + light-time + aberration + nutation = observed position.
    Apparent,
}

/// A position with its type explicitly tagged.
#[derive(Debug, Clone)]
pub struct TypedPosition {
    /// The base position data.
    pub position: PlanetaryPosition,
    /// What corrections have been applied.
    pub position_type: PositionType,
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Compute the apparent ecliptic position of a planet.
///
/// Applies aberration and nutation corrections to the geometric position.
/// This is the position where the body actually appears on the sky.
///
/// # Errors
///
/// Returns an error for Sun/Moon (use dedicated functions) or if computation fails.
///
/// # Examples
///
/// ```
/// # use jyotish::apparent::{apparent_position, PositionType};
/// # use jyotish::planet::Planet;
/// let pos = apparent_position(Planet::Mars, 2_451_545.0).unwrap();
/// assert_eq!(pos.position_type, PositionType::Apparent);
/// assert!(pos.position.longitude_deg >= 0.0 && pos.position.longitude_deg < 360.0);
/// ```
pub fn apparent_position(planet: Planet, jd: f64) -> Result<TypedPosition> {
    if matches!(planet, Planet::Sun | Planet::Moon) {
        return Err(JyotishError::InvalidParameter(
            "use apparent_sun() or apparent_moon() for Sun/Moon".into(),
        ));
    }

    // Step 1: Geometric position (already includes basic geocentric conversion)
    let geometric = crate::planetary::compute_position(planet, jd)?;
    let mut lon = geometric.longitude_deg;
    let mut lat = geometric.latitude_deg;
    let dist = geometric.distance_au;

    // Step 2: Annual aberration
    let t = julian_centuries(jd);
    let sun_lon = crate::sun::solar_longitude(jd);
    let eps = mean_obliquity(t);
    let (ab_lon, ab_lat) = apply_aberration(lon, lat, sun_lon, eps);
    lon = ab_lon;
    lat = ab_lat;

    // Step 3: Nutation in longitude
    lon = normalize_degrees(lon + nutation_longitude(jd));

    Ok(TypedPosition {
        position: PlanetaryPosition::new(planet, lon, lat, dist, crate::calendar::jd_to_unix(jd)),
        position_type: PositionType::Apparent,
    })
}

/// Compute the apparent ecliptic longitude of the Sun.
///
/// Applies solar aberration (−20.4898″/R) and nutation in longitude to
/// the geometric position from VSOP87.
pub fn apparent_sun(jd: f64) -> TypedPosition {
    let mut lon = crate::sun::solar_longitude(jd);
    let lat = crate::sun::solar_latitude(jd);
    let dist = crate::sun::solar_distance_au(jd);

    // Solar aberration: −20.4898″ / R (Meeus eq. 25.10)
    lon -= 20.4898 / (3600.0 * dist);

    // Nutation in longitude
    lon = normalize_degrees(lon + nutation_longitude(jd));

    TypedPosition {
        position: PlanetaryPosition::new(
            Planet::Sun,
            lon,
            lat,
            dist,
            crate::calendar::jd_to_unix(jd),
        ),
        position_type: PositionType::Apparent,
    }
}

/// Compute the apparent ecliptic position of the Moon.
///
/// Applies aberration (~0.2") and nutation (~9") corrections.
pub fn apparent_moon(jd: f64) -> TypedPosition {
    let mut lon = crate::moon::lunar_longitude(jd);
    let mut lat = crate::moon::lunar_latitude(jd);
    let dist = crate::moon::lunar_distance_au(jd);

    // Aberration (small but non-negligible for precision work)
    let t = julian_centuries(jd);
    let sun_lon = crate::sun::solar_longitude(jd);
    let eps = mean_obliquity(t);
    let (ab_lon, ab_lat) = apply_aberration(lon, lat, sun_lon, eps);
    lon = ab_lon;
    lat = ab_lat;

    // Apply nutation to lunar longitude
    let apparent_lon = normalize_degrees(lon + nutation_longitude(jd));

    TypedPosition {
        position: PlanetaryPosition::new(
            Planet::Moon,
            apparent_lon,
            lat,
            dist,
            crate::calendar::jd_to_unix(jd),
        ),
        position_type: PositionType::Apparent,
    }
}

/// Compute the geometric (uncorrected) position of a planet.
///
/// This is a convenience wrapper that tags the built-in position
/// with [`PositionType::Geometric`].
pub fn geometric_position(planet: Planet, jd: f64) -> Result<TypedPosition> {
    let pos = match planet {
        Planet::Sun => crate::sun::solar_position(jd),
        Planet::Moon => crate::moon::lunar_position(jd),
        _ => crate::planetary::compute_position(planet, jd)?,
    };

    Ok(TypedPosition {
        position: pos,
        position_type: PositionType::Geometric,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const JD_J2000: f64 = 2_451_545.0;

    #[test]
    fn apparent_mars() {
        let pos = apparent_position(Planet::Mars, JD_J2000).unwrap();
        assert_eq!(pos.position_type, PositionType::Apparent);
        assert!(pos.position.longitude_deg >= 0.0 && pos.position.longitude_deg < 360.0);
    }

    #[test]
    fn apparent_vs_geometric_differs() {
        let geo = geometric_position(Planet::Jupiter, JD_J2000).unwrap();
        let app = apparent_position(Planet::Jupiter, JD_J2000).unwrap();

        // Apparent and geometric should differ (by aberration + nutation)
        let diff = (geo.position.longitude_deg - app.position.longitude_deg).abs();
        assert!(
            diff > 0.001 && diff < 0.02,
            "diff = {diff}° (should be ~0.006°)"
        );
    }

    #[test]
    fn apparent_sun_tagged() {
        let pos = apparent_sun(JD_J2000);
        assert_eq!(pos.position_type, PositionType::Apparent);
        assert_eq!(pos.position.planet, Planet::Sun);
    }

    #[test]
    fn apparent_moon_tagged() {
        let pos = apparent_moon(JD_J2000);
        assert_eq!(pos.position_type, PositionType::Apparent);
        assert_eq!(pos.position.planet, Planet::Moon);
    }

    #[test]
    fn geometric_all_bodies() {
        for planet in [
            Planet::Sun,
            Planet::Moon,
            Planet::Mars,
            Planet::Jupiter,
            Planet::Saturn,
        ] {
            let pos = geometric_position(planet, JD_J2000).unwrap();
            assert_eq!(pos.position_type, PositionType::Geometric);
        }
    }

    #[test]
    fn sun_moon_rejected_by_apparent() {
        assert!(apparent_position(Planet::Sun, JD_J2000).is_err());
        assert!(apparent_position(Planet::Moon, JD_J2000).is_err());
    }

    #[test]
    fn position_type_serde() {
        let pt = PositionType::Apparent;
        let json = serde_json::to_string(&pt).unwrap();
        let restored: PositionType = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, pt);
    }
}
