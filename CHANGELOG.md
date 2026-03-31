# Changelog

All notable changes to jyotish are documented in this file.

## [0.1.0] — 2026-03-25

Initial scaffold of the jyotish astronomical computation engine.

### Core modules

- **error** — `JyotishError` enum with 5 variants (InvalidParameter, MathError, DateError, EphemerisError, Io)
- **planet** — `Planet` enum (Sun through Pluto), `PlanetaryPosition` struct with ecliptic coordinates
- **calendar** — stub for Julian/Gregorian/sidereal time/JDN conversions
- **event** — stub for eclipses, conjunctions, oppositions, equinoxes, solstices
- **zodiac** — stub for tropical/sidereal signs, constellation boundaries, cusps
- **house** — stub for Placidus, Koch, Equal, Whole Sign, Porphyry house systems
- **aspect** — stub for conjunction, opposition, trine, square, sextile, orbs
- **transit** — stub for ingress, retrograde, station, direct motion

### Optional features

- **`orbital`** (falak) — orbital mechanics integration
- **`logging`** — tracing-subscriber initialization via `JYOTISH_LOG` env var

### Infrastructure

- Cargo.toml with hisab, chrono, serde, thiserror, tracing
- CI workflow (fmt, clippy, audit, deny, test, MSRV, coverage, doc)
- Release workflow (GitHub Release only, no cargo publish)
- Criterion benchmark harness
- Integration test scaffold
- GPL-3.0-only license
- Rust 2024 edition, MSRV 1.89
