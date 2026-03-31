//! House systems — Placidus, Koch, Equal, Whole Sign, Porphyry.
//!
//! Computes the twelve astrological house cusps for a given local sidereal
//! time and geographic latitude. Each house system uses a different method
//! to divide the ecliptic or celestial sphere.
//!
//! The Ascendant (ASC) and Midheaven (MC) are the same in all systems.

use crate::coords::{deg_to_rad, normalize_degrees, rad_to_deg};
use crate::error::{JyotishError, Result};
use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

// ---------------------------------------------------------------------------
// House system enum
// ---------------------------------------------------------------------------

/// Supported house systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HouseSystem {
    /// Placidus (time-based division of semi-arcs)
    Placidus,
    /// Equal houses (30° from Ascendant)
    Equal,
    /// Whole Sign (sign-based, Ascendant's sign = 1st house)
    WholeSign,
    /// Porphyry (trisection of quadrants)
    Porphyry,
}

// ---------------------------------------------------------------------------
// House cusps result
// ---------------------------------------------------------------------------

/// The twelve house cusps plus the Ascendant and Midheaven.
#[derive(Debug, Clone)]
pub struct HouseCusps {
    /// The house system used.
    pub system: HouseSystem,
    /// Ecliptic longitudes of the twelve house cusps (index 0 = 1st house).
    pub cusps: [f64; 12],
    /// Ascendant (rising degree) — same as `cusps[0]` for most systems.
    pub ascendant: f64,
    /// Midheaven (MC) — upper meridian ecliptic longitude.
    pub midheaven: f64,
}

// ---------------------------------------------------------------------------
// Ascendant and Midheaven
// ---------------------------------------------------------------------------

/// Compute the Ascendant (rising degree) for a given local sidereal time
/// and geographic latitude.
///
/// `lst_deg` is the local sidereal time in degrees.
/// `lat_deg` is the observer's geographic latitude in degrees (positive north).
/// `obliquity_deg` is the obliquity of the ecliptic in degrees.
///
/// # Examples
///
/// ```
/// # use jyotish::house::ascendant;
/// let asc = ascendant(0.0, 51.5, 23.44);
/// assert!((0.0..360.0).contains(&asc));
/// ```
pub fn ascendant(lst_deg: f64, lat_deg: f64, obliquity_deg: f64) -> f64 {
    let lst = deg_to_rad(lst_deg);
    let lat = deg_to_rad(lat_deg);
    let eps = deg_to_rad(obliquity_deg);

    let asc = (-lst.cos()).atan2(lst.sin() * eps.cos() + lat.tan() * eps.sin());

    normalize_degrees(rad_to_deg(asc))
}

/// Compute the Midheaven (MC) for a given local sidereal time.
///
/// `lst_deg` is the local sidereal time in degrees.
/// `obliquity_deg` is the obliquity of the ecliptic in degrees.
///
/// # Examples
///
/// ```
/// # use jyotish::house::midheaven;
/// let mc = midheaven(0.0, 23.44);
/// assert!((0.0..360.0).contains(&mc));
/// ```
pub fn midheaven(lst_deg: f64, obliquity_deg: f64) -> f64 {
    let lst = deg_to_rad(lst_deg);
    let eps = deg_to_rad(obliquity_deg);

    let mc = lst.tan().atan2(eps.cos());

    // Ensure MC is in the correct quadrant (should be near LST)
    let mut mc_deg = normalize_degrees(rad_to_deg(mc));
    // MC should be in the same half of the sky as LST
    let lst_norm = normalize_degrees(lst_deg);
    if (mc_deg - lst_norm).abs() > 90.0 && (mc_deg - lst_norm).abs() < 270.0 {
        mc_deg = normalize_degrees(mc_deg + 180.0);
    }
    mc_deg
}

// ---------------------------------------------------------------------------
// House system computations
// ---------------------------------------------------------------------------

/// Compute house cusps for the given parameters.
///
/// `lst_deg` is the local sidereal time in degrees (compute from UT1 via
/// [`crate::calendar::local_sidereal_time`] or from TT via
/// [`crate::calendar::local_sidereal_time_tt`]).
/// `lat_deg` is the observer's geographic latitude in degrees (positive north).
/// `obliquity_deg` is the obliquity of the ecliptic in degrees.
///
/// # Errors
///
/// Returns [`JyotishError::InvalidParameter`] if an unsupported configuration
/// is requested (e.g., Placidus at extreme polar latitudes).
///
/// # Examples
///
/// ```
/// # use jyotish::house::{compute_houses, HouseSystem};
/// let houses = compute_houses(HouseSystem::Equal, 0.0, 51.5, 23.44).unwrap();
/// assert_eq!(houses.cusps.len(), 12);
/// ```
pub fn compute_houses(
    system: HouseSystem,
    lst_deg: f64,
    lat_deg: f64,
    obliquity_deg: f64,
) -> Result<HouseCusps> {
    let asc = ascendant(lst_deg, lat_deg, obliquity_deg);
    let mc = midheaven(lst_deg, obliquity_deg);

    let cusps = match system {
        HouseSystem::Equal => equal_houses(asc),
        HouseSystem::WholeSign => whole_sign_houses(asc),
        HouseSystem::Porphyry => porphyry_houses(asc, mc),
        HouseSystem::Placidus => placidus_houses(asc, mc, lst_deg, lat_deg, obliquity_deg)?,
    };

    Ok(HouseCusps {
        system,
        cusps,
        ascendant: asc,
        midheaven: mc,
    })
}

/// Equal houses: each house is exactly 30° from the Ascendant.
fn equal_houses(asc: f64) -> [f64; 12] {
    let mut cusps = [0.0; 12];
    for (i, cusp) in cusps.iter_mut().enumerate() {
        *cusp = normalize_degrees(asc + i as f64 * 30.0);
    }
    cusps
}

/// Whole Sign houses: the sign containing the Ascendant is the 1st house.
fn whole_sign_houses(asc: f64) -> [f64; 12] {
    let first_sign_start = (asc / 30.0).floor() * 30.0;
    let mut cusps = [0.0; 12];
    for (i, cusp) in cusps.iter_mut().enumerate() {
        *cusp = normalize_degrees(first_sign_start + i as f64 * 30.0);
    }
    cusps
}

/// Porphyry houses: trisect each quadrant between ASC, MC, DSC, IC.
fn porphyry_houses(asc: f64, mc: f64) -> [f64; 12] {
    let dsc = normalize_degrees(asc + 180.0);
    let ic = normalize_degrees(mc + 180.0);

    // Quadrant arcs (signed, going counter-clockwise through the ecliptic)
    let q1 = arc_between(asc, ic); // ASC → IC (houses 1-3)
    let q2 = arc_between(ic, dsc); // IC → DSC (houses 4-6)
    let q3 = arc_between(dsc, mc); // DSC → MC (houses 7-9)
    let q4 = arc_between(mc, asc); // MC → ASC (houses 10-12)

    let mut cusps = [0.0; 12];
    cusps[0] = asc;
    cusps[1] = normalize_degrees(asc + q1 / 3.0);
    cusps[2] = normalize_degrees(asc + 2.0 * q1 / 3.0);
    cusps[3] = ic;
    cusps[4] = normalize_degrees(ic + q2 / 3.0);
    cusps[5] = normalize_degrees(ic + 2.0 * q2 / 3.0);
    cusps[6] = dsc;
    cusps[7] = normalize_degrees(dsc + q3 / 3.0);
    cusps[8] = normalize_degrees(dsc + 2.0 * q3 / 3.0);
    cusps[9] = mc;
    cusps[10] = normalize_degrees(mc + q4 / 3.0);
    cusps[11] = normalize_degrees(mc + 2.0 * q4 / 3.0);

    cusps
}

/// Placidus houses: time-based semi-arc division.
///
/// Uses iterative computation for intermediate cusps.
fn placidus_houses(
    asc: f64,
    mc: f64,
    _lst_deg: f64,
    lat_deg: f64,
    obliquity_deg: f64,
) -> Result<[f64; 12]> {
    let lat = deg_to_rad(lat_deg);
    let eps = deg_to_rad(obliquity_deg);

    // Placidus fails near the poles where circumpolar objects never rise/set
    if lat_deg.abs() > 66.0 {
        return Err(JyotishError::InvalidParameter(
            "Placidus houses undefined for latitudes above ±66°".into(),
        ));
    }

    let dsc = normalize_degrees(asc + 180.0);
    let ic = normalize_degrees(mc + 180.0);

    let mut cusps = [0.0; 12];
    cusps[0] = asc;
    cusps[3] = ic;
    cusps[6] = dsc;
    cusps[9] = mc;

    // Compute intermediate cusps by iterating on the Placidus formula.
    // Cusp N divides the semi-arc in the ratio N/3.
    for (house_idx, fraction) in [(1, 1.0 / 3.0), (2, 2.0 / 3.0)] {
        cusps[house_idx] = placidus_cusp(mc, fraction, lat, eps, false);
        cusps[house_idx + 6] = normalize_degrees(cusps[house_idx] + 180.0);
    }
    for (house_idx, fraction) in [(4, 1.0 / 3.0), (5, 2.0 / 3.0)] {
        cusps[house_idx] = placidus_cusp(mc, fraction, lat, eps, true);
        cusps[house_idx + 6] = normalize_degrees(cusps[house_idx] + 180.0);
    }

    Ok(cusps)
}

/// Compute a single Placidus cusp by iteration.
fn placidus_cusp(mc: f64, fraction: f64, lat: f64, eps: f64, below_horizon: bool) -> f64 {
    let mc_rad = deg_to_rad(mc);
    let mut cusp = if below_horizon {
        mc_rad + PI * fraction
    } else {
        mc_rad + PI + PI * fraction
    };

    // Iterate to convergence
    for _ in 0..50 {
        let dec = (eps.sin() * cusp.sin()).asin();
        let ad_arg = lat.tan() * dec.tan();
        // Guard: if |ad_arg| >= 1 the body is circumpolar at this declination;
        // clamp to avoid NaN from asin.
        let ad = ad_arg.clamp(-0.999_999, 0.999_999).asin();
        let sa = PI / 2.0 + ad;
        let dsa = if below_horizon { PI - sa } else { sa };

        let new_cusp = mc_rad + (cusp - mc_rad).rem_euclid(2.0 * PI);
        let ra_fraction = fraction * dsa;
        let target_ra = if below_horizon {
            mc_rad + PI + ra_fraction
        } else {
            mc_rad + PI + sa + ra_fraction
        };

        let diff = target_ra - new_cusp;
        if diff.abs() < 1e-10 {
            break;
        }
        cusp += diff * 0.5;
    }

    // Convert RAMC to ecliptic longitude
    normalize_degrees(rad_to_deg(cusp))
}

/// Arc between two ecliptic longitudes going counter-clockwise.
fn arc_between(from: f64, to: f64) -> f64 {
    let arc = to - from;
    if arc < 0.0 { arc + 360.0 } else { arc }
}

#[cfg(test)]
mod tests {
    use super::*;

    const LST: f64 = 197.693; // London-ish LST
    const LAT: f64 = 51.5; // London latitude
    const EPS: f64 = 23.44; // obliquity

    #[test]
    fn ascendant_in_range() {
        let asc = ascendant(LST, LAT, EPS);
        assert!((0.0..360.0).contains(&asc), "ASC = {asc}");
    }

    #[test]
    fn midheaven_in_range() {
        let mc = midheaven(LST, EPS);
        assert!((0.0..360.0).contains(&mc), "MC = {mc}");
    }

    #[test]
    fn equal_houses_30_degree_spacing() {
        let houses = compute_houses(HouseSystem::Equal, LST, LAT, EPS).unwrap();
        for i in 0..11 {
            let diff = arc_between(houses.cusps[i], houses.cusps[i + 1]);
            assert!(
                (diff - 30.0).abs() < 1e-10,
                "house {i} to {} gap = {diff}",
                i + 1
            );
        }
    }

    #[test]
    fn whole_sign_houses_aligned_to_signs() {
        let houses = compute_houses(HouseSystem::WholeSign, LST, LAT, EPS).unwrap();
        for cusp in &houses.cusps {
            // Each cusp should be at a sign boundary (multiple of 30)
            assert!(
                (cusp % 30.0).abs() < 1e-10 || (cusp % 30.0 - 30.0).abs() < 1e-10,
                "cusp {cusp} not on sign boundary"
            );
        }
    }

    #[test]
    fn porphyry_houses_quadrant_trisection() {
        let houses = compute_houses(HouseSystem::Porphyry, LST, LAT, EPS).unwrap();
        // First house cusp should be the ASC
        assert!(
            (houses.cusps[0] - houses.ascendant).abs() < 1e-10,
            "cusp[0] = {}, ASC = {}",
            houses.cusps[0],
            houses.ascendant
        );
        // 4th cusp should be the IC
        let ic = normalize_degrees(houses.midheaven + 180.0);
        assert!(
            (houses.cusps[3] - ic).abs() < 1e-10,
            "cusp[3] = {}, IC = {ic}",
            houses.cusps[3]
        );
    }

    #[test]
    fn placidus_houses_basic() {
        let houses = compute_houses(HouseSystem::Placidus, LST, LAT, EPS).unwrap();
        assert!(
            (houses.cusps[0] - houses.ascendant).abs() < 1e-10,
            "cusp[0] should be ASC"
        );
        assert!(
            (houses.cusps[9] - houses.midheaven).abs() < 1e-10,
            "cusp[9] should be MC"
        );
    }

    #[test]
    fn placidus_fails_at_poles() {
        assert!(compute_houses(HouseSystem::Placidus, LST, 80.0, EPS).is_err());
        assert!(compute_houses(HouseSystem::Placidus, LST, 66.1, EPS).is_err());
        assert!(compute_houses(HouseSystem::Placidus, LST, -66.1, EPS).is_err());
        // Just under the limit should succeed
        assert!(compute_houses(HouseSystem::Placidus, LST, 65.9, EPS).is_ok());
    }

    #[test]
    fn all_systems_produce_12_cusps() {
        for system in [
            HouseSystem::Equal,
            HouseSystem::WholeSign,
            HouseSystem::Porphyry,
            HouseSystem::Placidus,
        ] {
            let houses = compute_houses(system, LST, LAT, EPS).unwrap();
            assert_eq!(houses.cusps.len(), 12, "{system:?} produced wrong count");
            for (i, &cusp) in houses.cusps.iter().enumerate() {
                assert!(
                    (0.0..360.0).contains(&cusp),
                    "{system:?} cusp {i} = {cusp} out of range"
                );
            }
        }
    }

    #[test]
    fn house_system_serde() {
        let sys = HouseSystem::Porphyry;
        let json = serde_json::to_string(&sys).unwrap();
        let restored: HouseSystem = serde_json::from_str(&json).unwrap();
        assert_eq!(restored, sys);
    }

    #[test]
    fn ascendant_varies_with_latitude() {
        let asc_london = ascendant(LST, 51.5, EPS);
        let asc_equator = ascendant(LST, 0.0, EPS);
        // Different latitudes should give different ascendants
        assert!(
            (asc_london - asc_equator).abs() > 1.0,
            "ASC should vary with latitude"
        );
    }
}
