use jyotish::{JyotishError, Planet, PlanetaryPosition};

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
