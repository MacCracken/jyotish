//! Planetary position computation for Mercury through Pluto.
//!
//! Uses VSOP87D theory (Bretagnon & Francou 1988) for Mercury through Neptune,
//! providing <1 arcsecond accuracy over ±4000 years from J2000.0. Pluto uses
//! Keplerian orbital elements from Meeus Table 31.A (~1 arcminute accuracy).
//!
//! All positions are **geometric** (no aberration or nutation corrections).
//! For apparent positions, use [`crate::apparent::apparent_position`].

use crate::calendar::julian_centuries;
use crate::coords::{deg_to_rad, normalize_degrees, rad_to_deg};
use crate::error::{JyotishError, Result};
use crate::planet::{Planet, PlanetaryPosition};

// ---------------------------------------------------------------------------
// Geocentric conversion (shared by VSOP87 and Keplerian paths)
// ---------------------------------------------------------------------------

/// Convert heliocentric positions of a planet and Earth to geocentric ecliptic
/// coordinates for the planet.
///
/// Returns `(geo_lon_deg, geo_lat_deg, geo_dist_au)`.
fn heliocentric_to_geocentric(
    h_lon: f64,
    h_lat: f64,
    h_r: f64,
    e_lon: f64,
    e_lat: f64,
    e_r: f64,
) -> (f64, f64, f64) {
    let h_lon_rad = deg_to_rad(h_lon);
    let h_lat_rad = deg_to_rad(h_lat);
    let e_lon_rad = deg_to_rad(e_lon);
    let e_lat_rad = deg_to_rad(e_lat);

    let x_p = h_r * h_lat_rad.cos() * h_lon_rad.cos() - e_r * e_lat_rad.cos() * e_lon_rad.cos();
    let y_p = h_r * h_lat_rad.cos() * h_lon_rad.sin() - e_r * e_lat_rad.cos() * e_lon_rad.sin();
    let z_p = h_r * h_lat_rad.sin() - e_r * e_lat_rad.sin();

    let geo_lon = normalize_degrees(rad_to_deg(y_p.atan2(x_p)));
    let geo_lat = rad_to_deg(z_p.atan2((x_p * x_p + y_p * y_p).sqrt()));
    let geo_dist = (x_p * x_p + y_p * y_p + z_p * z_p).sqrt();

    (geo_lon, geo_lat, geo_dist)
}

// ---------------------------------------------------------------------------
// Keplerian fallback for Pluto
// ---------------------------------------------------------------------------

/// Orbital elements for Pluto (Meeus Table 31.A).
fn pluto_heliocentric(jd: f64) -> (f64, f64, f64) {
    let t = julian_centuries(jd);

    let l = normalize_degrees(238.928_81 + 145.205_26 * t);
    let a = 39.481_686_77;
    let e = 0.248_808_6 + 0.000_060_16 * t;
    let i = deg_to_rad(17.141_75 + 0.003_075 * t);
    let node = deg_to_rad(110.303_47 + -0.010_139_6 * t);
    let peri = deg_to_rad(224.066_76 + -0.003_442_5 * t);

    let m = deg_to_rad(normalize_degrees(l - rad_to_deg(peri)));
    let ea = solve_kepler(m, e);
    let v = 2.0
        * ((1.0 + e).sqrt() * (ea / 2.0).sin()).atan2((1.0 - e).sqrt() * (ea / 2.0).cos());
    let r = a * (1.0 - e * ea.cos());

    let arg = v + peri - node;
    let lon = (arg.sin() * i.cos()).atan2(arg.cos()) + node;
    let lat = (arg.sin() * i.sin()).asin();

    (normalize_degrees(rad_to_deg(lon)), rad_to_deg(lat), r)
}

/// Solve Kepler's equation M = E - e*sin(E) for eccentric anomaly E.
fn solve_kepler(m_rad: f64, e: f64) -> f64 {
    let mut ea = m_rad + e * m_rad.sin();
    for _ in 0..50 {
        let delta = (ea - e * ea.sin() - m_rad) / (1.0 - e * ea.cos());
        ea -= delta;
        if delta.abs() < 1e-12 {
            break;
        }
    }
    ea
}

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Compute the geocentric ecliptic position of a planet.
///
/// Uses VSOP87D for Mercury–Neptune (<1" accuracy) and Keplerian elements
/// for Pluto (~1' accuracy).
///
/// `jd` should be in TT/TDB. Returns a geometric (uncorrected) position.
/// For apparent positions with aberration and nutation, use
/// [`crate::apparent::apparent_position`].
///
/// # Errors
///
/// Returns [`JyotishError::InvalidParameter`] if called with `Planet::Sun`
/// or `Planet::Moon` (use the dedicated `sun` and `moon` modules instead).
///
/// # Examples
///
/// ```
/// # use jyotish::planetary::compute_position;
/// # use jyotish::planet::Planet;
/// let pos = compute_position(Planet::Venus, 2_451_545.0).unwrap();
/// assert!(pos.longitude_deg >= 0.0 && pos.longitude_deg < 360.0);
/// ```
pub fn compute_position(planet: Planet, jd: f64) -> Result<PlanetaryPosition> {
    if matches!(planet, Planet::Sun | Planet::Moon) {
        return Err(JyotishError::InvalidParameter(
            "use sun/moon module for Sun/Moon positions".into(),
        ));
    }

    // Earth's heliocentric position (VSOP87D)
    let (e_lon, e_lat, e_r) = crate::vsop87::earth_heliocentric(jd);

    // Planet's heliocentric position
    let (h_lon, h_lat, h_r) = if planet == Planet::Pluto {
        pluto_heliocentric(jd)
    } else {
        crate::vsop87::planet_heliocentric(planet, jd)?
    };

    let (geo_lon, geo_lat, geo_dist) =
        heliocentric_to_geocentric(h_lon, h_lat, h_r, e_lon, e_lat, e_r);

    Ok(PlanetaryPosition::new(
        planet,
        geo_lon,
        geo_lat,
        geo_dist,
        crate::calendar::jd_to_unix(jd),
    ))
}

/// Compute geocentric positions of all planets (Mercury through Pluto).
///
/// Convenience function that returns positions for all eight planets.
/// Sun and Moon are excluded — use their dedicated modules.
///
/// # Errors
///
/// Returns an error if any planet's position cannot be computed.
pub fn compute_all_positions(jd: f64) -> Result<Vec<PlanetaryPosition>> {
    let planets = [
        Planet::Mercury,
        Planet::Venus,
        Planet::Mars,
        Planet::Jupiter,
        Planet::Saturn,
        Planet::Uranus,
        Planet::Neptune,
        Planet::Pluto,
    ];

    planets.iter().map(|&p| compute_position(p, jd)).collect()
}

// ---------------------------------------------------------------------------
// Legacy Meeus Keplerian positions (behind feature flag)
// ---------------------------------------------------------------------------

/// Compute position using Meeus Table 31.A Keplerian elements (~1' accuracy).
#[cfg(feature = "meeus")]
pub fn compute_position_meeus(planet: Planet, jd: f64) -> Result<PlanetaryPosition> {
    if matches!(planet, Planet::Sun | Planet::Moon) {
        return Err(JyotishError::InvalidParameter(
            "use sun/moon module for Sun/Moon positions".into(),
        ));
    }
    let (h_lon, h_lat, h_r) = heliocentric_keplerian(planet, jd)?;
    let (e_lon, _e_lat, e_r) = earth_heliocentric_keplerian(jd);
    let (geo_lon, geo_lat, geo_dist) =
        heliocentric_to_geocentric(h_lon, h_lat, h_r, e_lon, 0.0, e_r);
    Ok(PlanetaryPosition::new(
        planet, geo_lon, geo_lat, geo_dist,
        crate::calendar::jd_to_unix(jd),
    ))
}

#[cfg(feature = "meeus")]
fn earth_heliocentric_keplerian(jd: f64) -> (f64, f64, f64) {
    let t = julian_centuries(jd);
    let l = normalize_degrees(100.466_49 + 35_999.372_75 * t);
    let a = 1.000_001_018;
    let e = 0.016_708_634 - 0.000_042_037 * t - 0.000_000_126_7 * t * t;
    let peri = deg_to_rad(102.937_35 + 1.719_526_9 * t);
    let m = deg_to_rad(normalize_degrees(l - rad_to_deg(peri)));
    let ea = solve_kepler(m, e);
    let v = 2.0
        * ((1.0 + e).sqrt() * (ea / 2.0).sin()).atan2((1.0 - e).sqrt() * (ea / 2.0).cos());
    let r = a * (1.0 - e * ea.cos());
    let lon = v + peri;
    (normalize_degrees(rad_to_deg(lon)), 0.0, r)
}

#[cfg(feature = "meeus")]
struct OrbitalElements {
    l: (f64, f64),
    a: (f64, f64),
    e: (f64, f64),
    i: (f64, f64),
    node: (f64, f64),
    peri: (f64, f64),
}

#[cfg(feature = "meeus")]
fn orbital_elements_meeus(planet: Planet) -> Result<OrbitalElements> {
    match planet {
        Planet::Mercury => Ok(OrbitalElements {
            l: (252.250_32, 149_472.674_11), a: (0.387_098_93, 0.0),
            e: (0.205_631_75, 0.000_020_406), i: (7.004_986, 0.001_821_5),
            node: (48.330_893, 1.186_124_4), peri: (77.456_45, 1.556_511_1),
        }),
        Planet::Venus => Ok(OrbitalElements {
            l: (181.979_73, 58_517.815_39), a: (0.723_331_99, 0.0),
            e: (0.006_773_23, -0.000_047_766), i: (3.394_662, 0.001_003_7),
            node: (76.679_920, 0.901_120_6), peri: (131.563_70, 1.402_123_9),
        }),
        Planet::Mars => Ok(OrbitalElements {
            l: (355.433_30, 19_140.299_34), a: (1.523_662_31, 0.000_001_97),
            e: (0.093_412_33, 0.000_090_48), i: (1.849_726, -0.000_601_1),
            node: (49.558_093, 0.772_095_9), peri: (336.060_23, 1.840_758_4),
        }),
        Planet::Jupiter => Ok(OrbitalElements {
            l: (34.351_48, 3_034.905_67), a: (5.202_603_2, 0.000_019_13),
            e: (0.048_497_65, 0.000_163_14), i: (1.303_270, -0.001_987_2),
            node: (100.464_44, 0.176_450_5), peri: (14.331_09, 0.215_520_9),
        }),
        Planet::Saturn => Ok(OrbitalElements {
            l: (50.077_44, 1_222.113_79), a: (9.554_909_6, -0.000_021_39),
            e: (0.055_508_62, -0.000_346_64), i: (2.488_878, 0.002_551_5),
            node: (113.665_24, 0.877_191_6), peri: (93.056_78, 0.565_320_6),
        }),
        Planet::Uranus => Ok(OrbitalElements {
            l: (314.055_01, 428.466_77), a: (19.218_143_4, -0.000_003_72),
            e: (0.046_295_11, -0.000_027_29), i: (0.773_196, 0.000_673_9),
            node: (74.005_947, 0.074_146_1), peri: (173.005_56, 0.089_321_2),
        }),
        Planet::Neptune => Ok(OrbitalElements {
            l: (304.348_67, 218.486_28), a: (30.110_386_9, 0.000_012_63),
            e: (0.008_994_83, 0.000_006_91), i: (1.769_952, -0.009_308_2),
            node: (131.784_06, 1.010_304_4), peri: (48.123_69, 0.029_158_7),
        }),
        Planet::Pluto => Ok(OrbitalElements {
            l: (238.928_81, 145.205_26), a: (39.481_686_77, 0.0),
            e: (0.248_808_6, 0.000_060_16), i: (17.141_75, 0.003_075),
            node: (110.303_47, -0.010_139_6), peri: (224.066_76, -0.003_442_5),
        }),
        Planet::Sun | Planet::Moon => Err(JyotishError::InvalidParameter(
            "use sun/moon module".into(),
        )),
    }
}

#[cfg(feature = "meeus")]
fn heliocentric_keplerian(planet: Planet, jd: f64) -> Result<(f64, f64, f64)> {
    let t = julian_centuries(jd);
    let elts = orbital_elements_meeus(planet)?;
    let l = normalize_degrees(elts.l.0 + elts.l.1 * t);
    let a = elts.a.0 + elts.a.1 * t;
    let e = elts.e.0 + elts.e.1 * t;
    let i = deg_to_rad(elts.i.0 + elts.i.1 * t);
    let node = deg_to_rad(elts.node.0 + elts.node.1 * t);
    let peri = deg_to_rad(elts.peri.0 + elts.peri.1 * t);
    let m = deg_to_rad(normalize_degrees(l - rad_to_deg(peri)));
    let ea = solve_kepler(m, e);
    let v = 2.0
        * ((1.0 + e).sqrt() * (ea / 2.0).sin()).atan2((1.0 - e).sqrt() * (ea / 2.0).cos());
    let r = a * (1.0 - e * ea.cos());
    let arg = v + peri - node;
    let lon = (arg.sin() * i.cos()).atan2(arg.cos()) + node;
    let lat = (arg.sin() * i.sin()).asin();
    Ok((normalize_degrees(rad_to_deg(lon)), rad_to_deg(lat), r))
}

#[cfg(test)]
mod tests {
    use super::*;

    const JD_J2000: f64 = 2_451_545.0;

    #[test]
    fn kepler_circular_orbit() {
        let m = 1.5;
        let e = solve_kepler(m, 0.0);
        assert!((e - m).abs() < 1e-12);
    }

    #[test]
    fn kepler_small_eccentricity() {
        let m = 1.0;
        let ea = solve_kepler(m, 0.01);
        let residual = ea - 0.01 * ea.sin() - m;
        assert!(residual.abs() < 1e-12, "residual {residual}");
    }

    #[test]
    fn kepler_high_eccentricity() {
        let m = 0.5;
        let ea = solve_kepler(m, 0.9);
        let residual = ea - 0.9 * ea.sin() - m;
        assert!(residual.abs() < 1e-12, "residual {residual}");
    }

    #[test]
    fn compute_venus_j2000() {
        let pos = compute_position(Planet::Venus, JD_J2000).unwrap();
        assert!(pos.longitude_deg >= 0.0 && pos.longitude_deg < 360.0);
        assert!(pos.latitude_deg.abs() < 5.0);
        assert!(pos.distance_au > 0.2 && pos.distance_au < 1.8);
    }

    #[test]
    fn compute_mars_j2000() {
        let pos = compute_position(Planet::Mars, JD_J2000).unwrap();
        assert!(pos.longitude_deg >= 0.0 && pos.longitude_deg < 360.0);
        assert!(pos.distance_au > 0.3 && pos.distance_au < 2.7);
    }

    #[test]
    fn compute_jupiter_j2000() {
        let pos = compute_position(Planet::Jupiter, JD_J2000).unwrap();
        assert!(pos.longitude_deg >= 0.0 && pos.longitude_deg < 360.0);
        assert!(pos.distance_au > 3.9 && pos.distance_au < 6.5);
    }

    #[test]
    fn compute_all_planets() {
        let positions = compute_all_positions(JD_J2000).unwrap();
        assert_eq!(positions.len(), 8);
        for pos in &positions {
            assert!(pos.longitude_deg >= 0.0 && pos.longitude_deg < 360.0);
            assert!(pos.distance_au > 0.0);
        }
    }

    #[test]
    fn compute_pluto_high_eccentricity() {
        let pos = compute_position(Planet::Pluto, JD_J2000).unwrap();
        assert!(pos.longitude_deg >= 0.0 && pos.longitude_deg < 360.0);
        assert!(
            pos.distance_au > 25.0 && pos.distance_au < 55.0,
            "Pluto distance = {} AU",
            pos.distance_au
        );
    }

    #[test]
    fn sun_moon_rejected() {
        assert!(compute_position(Planet::Sun, JD_J2000).is_err());
        assert!(compute_position(Planet::Moon, JD_J2000).is_err());
    }

    #[test]
    fn positions_vary_over_time() {
        let pos1 = compute_position(Planet::Mars, JD_J2000).unwrap();
        let pos2 = compute_position(Planet::Mars, JD_J2000 + 30.0).unwrap();
        assert!((pos1.longitude_deg - pos2.longitude_deg).abs() > 0.1);
    }

    #[test]
    fn outer_planets_reasonable_distances() {
        let saturn = compute_position(Planet::Saturn, JD_J2000).unwrap();
        assert!(saturn.distance_au > 7.0 && saturn.distance_au < 12.0);

        let uranus = compute_position(Planet::Uranus, JD_J2000).unwrap();
        assert!(uranus.distance_au > 17.0 && uranus.distance_au < 22.0);

        let neptune = compute_position(Planet::Neptune, JD_J2000).unwrap();
        assert!(neptune.distance_au > 28.0 && neptune.distance_au < 32.0);
    }
}
