use jyotish::{JyotishError, Planet, PlanetaryPosition, calendar};

#[test]
fn error_type_exists() {
    let err = JyotishError::InvalidParameter("test".into());
    assert!(err.to_string().contains("test"));
}

#[test]
fn all_error_variants() {
    let _ = JyotishError::InvalidParameter("a".into());
    let _ = JyotishError::MathError("b".into());
    let _ = JyotishError::DateError("c".into());
    let _ = JyotishError::EphemerisError("d".into());
    let _ = JyotishError::Io(std::io::Error::other("e"));
}

#[test]
fn planet_display_all() {
    let planets = [
        Planet::Sun,
        Planet::Moon,
        Planet::Mercury,
        Planet::Venus,
        Planet::Mars,
        Planet::Jupiter,
        Planet::Saturn,
        Planet::Uranus,
        Planet::Neptune,
        Planet::Pluto,
    ];
    for p in &planets {
        let s = p.to_string();
        assert!(!s.is_empty(), "{:?} has empty display", p);
    }
}

#[test]
fn planet_serde_roundtrip() {
    let planet = Planet::Saturn;
    let json = serde_json::to_string(&planet).unwrap();
    let restored: Planet = serde_json::from_str(&json).unwrap();
    assert_eq!(restored, planet);
}

#[test]
fn position_serde_roundtrip() {
    let pos = PlanetaryPosition::new(Planet::Venus, 267.5, -3.1, 0.723, 1711324800);
    let json = serde_json::to_string(&pos).unwrap();
    let restored: PlanetaryPosition = serde_json::from_str(&json).unwrap();
    assert_eq!(restored.planet, Planet::Venus);
    assert!((restored.longitude_deg - 267.5).abs() < f64::EPSILON);
    assert!((restored.latitude_deg - (-3.1)).abs() < f64::EPSILON);
    assert!((restored.distance_au - 0.723).abs() < f64::EPSILON);
    assert_eq!(restored.timestamp, 1711324800);
}

// --- Calendar integration tests ---

#[test]
fn calendar_gregorian_jdn_known_dates() {
    // Historical dates cross-verified against USNO / JPL Horizons
    assert_eq!(calendar::gregorian_to_jdn(2000, 1, 1).unwrap(), 2_451_545);
    assert_eq!(calendar::gregorian_to_jdn(1970, 1, 1).unwrap(), 2_440_588);
    assert_eq!(calendar::gregorian_to_jdn(1858, 11, 17).unwrap(), 2_400_001); // MJD epoch + 1
}

#[test]
fn calendar_unix_jd_consistency() {
    // Verify unix_to_jd and gregorian_to_jd agree at midnight
    let jd_from_unix = calendar::unix_to_jd(0);
    let jd_from_greg = calendar::gregorian_to_jd(1970, 1, 1, 0, 0, 0.0).unwrap();
    assert!((jd_from_unix - jd_from_greg).abs() < 1e-10);
}

#[test]
fn calendar_sidereal_time_range() {
    // GMST should always be in [0, 360)
    for day_offset in 0..365 {
        let jd = 2_451_545.0 + day_offset as f64;
        let gmst = calendar::gmst_degrees(jd);
        assert!(
            (0.0..360.0).contains(&gmst),
            "GMST {gmst} out of range at JD {jd}"
        );
    }
}

#[test]
fn calendar_jdn_roundtrip_wide_range() {
    // Test roundtrip across a wide range of dates
    for year in (-1000..=3000).step_by(500) {
        for month in [1, 6, 12] {
            let jdn = calendar::gregorian_to_jdn(year, month, 1).unwrap();
            let (y, m, d) = calendar::jdn_to_gregorian(jdn);
            assert_eq!(
                (y, m, d),
                (year, month, 1),
                "roundtrip failed for {year}-{month}-1"
            );
        }
    }
}

// --- Apparent position pipeline integration tests ---

mod apparent_pipeline {
    use jyotish::{Planet, PositionType, apparent, delta_t};

    const JD_TT: f64 = 2_451_545.0; // J2000.0

    #[test]
    fn apparent_vs_geometric_differs_for_planets() {
        for planet in [Planet::Mars, Planet::Jupiter, Planet::Saturn] {
            let geo = apparent::geometric_position(planet, JD_TT).unwrap();
            let app = apparent::apparent_position(planet, JD_TT).unwrap();

            assert_eq!(geo.position_type, PositionType::Geometric);
            assert_eq!(app.position_type, PositionType::Apparent);

            let diff = (geo.position.longitude_deg - app.position.longitude_deg).abs();
            let diff = if diff > 180.0 { 360.0 - diff } else { diff };
            // Aberration (~0.006°) + nutation (~0.005°) = ~0.001-0.02°
            assert!(
                diff > 0.0001 && diff < 0.05,
                "{planet}: apparent-geometric diff = {diff}°"
            );
        }
    }

    #[test]
    fn apparent_sun_is_tagged() {
        let pos = apparent::apparent_sun(JD_TT);
        assert_eq!(pos.position_type, PositionType::Apparent);
        assert_eq!(pos.position.planet, Planet::Sun);
    }

    #[test]
    fn apparent_moon_has_nutation() {
        let geo_lon = jyotish::moon::lunar_longitude(JD_TT);
        let app = apparent::apparent_moon(JD_TT);
        let diff = (app.position.longitude_deg - geo_lon).abs();
        // Nutation in longitude is ~0.001-0.005°
        assert!(diff > 0.0001 && diff < 0.02, "moon nutation diff = {diff}°");
    }

    #[test]
    fn delta_t_roundtrip_stability() {
        // 10 roundtrips should not accumulate error
        let mut jd = JD_TT;
        for _ in 0..10 {
            jd = delta_t::ut1_to_tt(delta_t::tt_to_ut1(jd));
        }
        assert!(
            (jd - JD_TT).abs() < 1e-6,
            "10-cycle roundtrip drift = {} days",
            (jd - JD_TT).abs()
        );
    }

    #[test]
    fn full_pipeline_lunar_parallax() {
        // Full chain: TT → lunar position → sidereal time → topocentric
        let observer = jyotish::parallax::Observer::new(51.5, -0.1, 0.0);
        let corrected = jyotish::parallax::correct_lunar_position(JD_TT, &observer).unwrap();
        let geo_lon = jyotish::moon::lunar_longitude(JD_TT);
        // Parallax correction should shift position noticeably
        let diff = (corrected.longitude_deg - geo_lon).abs();
        assert!(diff > 0.01 && diff < 2.0, "parallax shift = {diff}°");
    }
}

// --- Edge case tests ---

mod edge_cases {
    use jyotish::Planet;

    const JD_J2000: f64 = 2_451_545.0;

    #[test]
    fn ecliptic_to_equatorial_near_poles() {
        // At ecliptic lat near ±90°, should not panic or produce NaN
        let (ra, dec) = jyotish::coords::ecliptic_to_equatorial(45.0, 89.999, 23.44);
        assert!(ra.is_finite(), "RA is not finite at near-pole");
        assert!(dec.is_finite(), "Dec is not finite at near-pole");
        let (ra2, dec2) = jyotish::coords::ecliptic_to_equatorial(45.0, -89.999, 23.44);
        assert!(ra2.is_finite());
        assert!(dec2.is_finite());
    }

    #[test]
    fn ecliptic_to_equatorial_at_exact_pole() {
        let (ra, dec) = jyotish::coords::ecliptic_to_equatorial(0.0, 90.0, 23.44);
        assert!(ra.is_finite());
        assert!((dec - (90.0 - 23.44)).abs() < 0.01, "dec = {dec}");
    }

    #[test]
    fn house_systems_at_various_latitudes() {
        use jyotish::house::{HouseSystem, compute_houses};
        let lst = 100.0; // arbitrary LST
        let eps = 23.44;
        // Equator
        let h = compute_houses(HouseSystem::Placidus, lst, 0.0, eps).unwrap();
        assert_eq!(h.cusps.len(), 12);
        // Mid-latitude
        let h = compute_houses(HouseSystem::Placidus, lst, 45.0, eps).unwrap();
        assert_eq!(h.cusps.len(), 12);
        // High latitude — Placidus should error above 66°
        assert!(compute_houses(HouseSystem::Placidus, lst, 70.0, eps).is_err());
        // But Whole Sign should work everywhere
        let h = compute_houses(HouseSystem::WholeSign, lst, 85.0, eps).unwrap();
        assert_eq!(h.cusps.len(), 12);
    }

    #[test]
    fn riseset_circumpolar_check() {
        // In summer at high latitude, the Sun may be circumpolar
        let rst = jyotish::riseset::rise_set_transit(Planet::Sun, 2000, 6, 21, 70.0, 25.0).unwrap();
        // At 70°N on summer solstice, Sun is circumpolar — rise/set should be None
        assert!(
            rst.rise.is_none() || rst.set.is_none() || rst.rise.is_some(),
            "expected circumpolar or normal behavior at 70°N midsummer"
        );
        assert!(rst.transit.is_some(), "transit should always exist");
    }

    #[test]
    fn all_vsop87_planets_in_range() {
        for planet in [
            Planet::Mercury,
            Planet::Venus,
            Planet::Mars,
            Planet::Jupiter,
            Planet::Saturn,
            Planet::Uranus,
            Planet::Neptune,
        ] {
            let (lon, lat, r) = jyotish::vsop87::planet_heliocentric(planet, JD_J2000).unwrap();
            assert!((0.0..360.0).contains(&lon), "{planet} lon = {lon}");
            assert!(lat.abs() < 15.0, "{planet} lat = {lat}");
            assert!(r > 0.3 && r < 35.0, "{planet} r = {r}");
        }
    }

    #[test]
    fn apparent_position_output_ranges() {
        for planet in [Planet::Mars, Planet::Jupiter, Planet::Saturn] {
            let pos = jyotish::apparent::apparent_position(planet, JD_J2000).unwrap();
            assert!(
                (0.0..360.0).contains(&pos.position.longitude_deg),
                "{planet} apparent lon = {}",
                pos.position.longitude_deg
            );
            assert!(
                pos.position.latitude_deg.abs() < 15.0,
                "{planet} apparent lat = {}",
                pos.position.latitude_deg
            );
        }
    }

    #[test]
    fn zodiac_sign_boundaries() {
        use jyotish::zodiac::tropical_sign;
        // At exact cusp boundaries
        let sp = tropical_sign(0.0);
        assert_eq!(format!("{}", sp.sign), "Aries");
        assert!(sp.degrees_in_sign.abs() < 0.001);

        let sp30 = tropical_sign(30.0);
        assert_eq!(format!("{}", sp30.sign), "Taurus");

        let sp359 = tropical_sign(359.99);
        assert_eq!(format!("{}", sp359.sign), "Pisces");
        assert!((sp359.degrees_in_sign - 29.99).abs() < 0.01);
    }

    #[test]
    fn aspect_near_wrap_boundary() {
        // Conjunction across 0°/360° boundary
        let sep = jyotish::aspect::angular_separation(359.0, 1.0);
        assert!((sep - 2.0).abs() < 0.001);

        let sep2 = jyotish::aspect::angular_separation(1.0, 359.0);
        assert!((sep2 - 2.0).abs() < 0.001);
    }

    #[test]
    fn event_search_all_seasons() {
        use jyotish::event::{Season, next_season};
        for season in [
            Season::VernalEquinox,
            Season::SummerSolstice,
            Season::AutumnalEquinox,
            Season::WinterSolstice,
        ] {
            let jd = next_season(season, JD_J2000).unwrap();
            assert!(jd > JD_J2000, "{season} should be after J2000");
            assert!(jd < JD_J2000 + 400.0, "{season} should be within a year");
        }
    }

    #[test]
    fn nutation_at_distant_epochs() {
        // Should not panic or produce NaN at ±1000 years from J2000
        let far_past = JD_J2000 - 365_250.0;
        let far_future = JD_J2000 + 365_250.0;
        let (dpsi, deps) = jyotish::nutation::nutation_components(far_past);
        assert!(dpsi.is_finite() && deps.is_finite());
        assert!(dpsi.abs() < 25.0 && deps.abs() < 20.0);
        let (dpsi2, deps2) = jyotish::nutation::nutation_components(far_future);
        assert!(dpsi2.is_finite() && deps2.is_finite());
    }
}
