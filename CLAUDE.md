# Jyotish — Claude Code Instructions

## Project Identity

**Jyotish** (Sanskrit: ज्योतिष — science of light/celestial bodies) — Astronomical computation and celestial event prediction for AGNOS

- **Type**: Flat library crate
- **License**: GPL-3.0-only
- **MSRV**: 1.89
- **Version**: SemVer 0.1.0

## Consumers

hoosh (celestial context), agnosai (astronomical reasoning), joshua (game world astronomy)

## Development Process

### P(-1): Scaffold Hardening (before any new features)

0. Read roadmap, CHANGELOG, and open issues — know what was intended before auditing what was built
1. Test + benchmark sweep of existing code
2. Cleanliness check: `cargo fmt --check`, `cargo clippy --all-features --all-targets -- -D warnings`, `cargo audit`, `cargo deny check`, `RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps`
3. Get baseline benchmarks (`./scripts/bench-history.sh`)
4. Internal deep review — gaps, optimizations, security, logging/errors, docs
5. External research — domain completeness, missing capabilities, best practices, world-class accuracy
6. Cleanliness check — must be clean after review
7. Additional tests/benchmarks from findings
8. Post-review benchmarks — prove the wins
9. Repeat if heavy

### Work Loop / Working Loop (continuous)

1. Work phase — new features, roadmap items, bug fixes
2. Cleanliness check: `cargo fmt --check`, `cargo clippy --all-features --all-targets -- -D warnings`, `cargo audit`, `cargo deny check`, `RUSTDOCFLAGS="-D warnings" cargo doc --all-features --no-deps`
3. Test + benchmark additions for new code
4. Run benchmarks (`./scripts/bench-history.sh`)
5. Internal review — performance, memory, security, throughput, correctness
6. Cleanliness check — must be clean after audit
7. Deeper tests/benchmarks from audit observations
8. Run benchmarks again — prove the wins

## Architecture

- `src/error.rs` — `JyotishError` enum with `thiserror`
- `src/planet.rs` — `Planet` enum, `PlanetaryPosition` struct
- `src/calendar.rs` — Julian/Gregorian/sidereal time/JDN conversions
- `src/event.rs` — eclipses, conjunctions, oppositions, equinoxes, solstices
- `src/zodiac.rs` — tropical and sidereal zodiac, constellations, cusps
- `src/house.rs` — Placidus, Koch, Equal, Whole Sign, Porphyry house systems
- `src/aspect.rs` — conjunction, opposition, trine, square, sextile, orbs
- `src/transit.rs` — ingress, retrograde, station, direct motion
- `src/logging.rs` — tracing-subscriber init (feature-gated)

## Dependencies

- **hisab** — trigonometry, linear algebra, numerical methods
- **chrono** — date/time handling
- **falak** (optional, `orbital` feature) — orbital mechanics
