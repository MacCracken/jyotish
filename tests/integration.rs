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
