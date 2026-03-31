# Changelog

All notable changes to jyotish are documented in this file.

## [Unreleased]

### Phase 1 — Numerical Foundation

- **num** — Kahan compensated summation (`KahanSum`) for all periodic term series
- **delta_t** — Delta T (TT−UT1) via Espenak & Meeus (2006) polynomials, 14 segments from −500 CE to 2150+; `ut1_to_tt()`, `tt_to_ut1()` time scale conversions
- Converted all polynomials to Horner's method (~15 sites across coords, calendar, sun, moon, nutation)
- Applied Kahan summation to moon (3 loops, ~180 terms) and nutation (63 terms)
- Range reduction for secondary moon arguments before trig calls

### Rise/Set, Parallax, and Falak Integration

- **riseset** — rise/set/transit times using Meeus Ch. 15 with iterative interpolation refinement
- **parallax** — topocentric lunar parallax correction, `Observer` struct, horizontal parallax computation
- **orbital** — falak integration (feature-gated): enhanced Kepler solver (Danby+NR), heliocentric state vectors, geocentric positions via falak's orbital mechanics

### Core Modules (full implementation replacing stubs)

- **calendar** — JDN/JD conversions, Gregorian/Julian calendar, sidereal time (GMST/LST), leap year, day of week, proper day-of-month validation
- **coords** — degree/radian conversion, ecliptic↔equatorial, mean obliquity, angle normalization
- **sun** — solar longitude/distance/position (Meeus Ch. 25), equation of time
- **moon** — lunar longitude/latitude/distance (Meeus Ch. 47, 60+60 periodic terms with eccentricity correction)
- **planetary** — Mercury–Pluto geocentric ecliptic positions via Keplerian elements + Kepler solver
- **nutation** — IAU 1980 nutation (63 terms), general precession, equatorial precession parameters, true obliquity
- **event** — equinox/solstice search, planetary conjunction/opposition detection with bisection refinement
- **zodiac** — tropical/sidereal zodiac signs, Lahiri ayanamsa, elements, modalities, sign position lookup
- **house** — Placidus, Equal, Whole Sign, Porphyry house systems; ascendant and midheaven computation
- **aspect** — conjunction, opposition, trine, square, sextile with configurable orbs and aspect strength
- **transit** — daily motion computation, sign ingress search, retrograde station detection, motion state

### Infrastructure

- `.gitignore` — created; removed `target/` from git tracking
- `deny.toml` — cargo-deny license/advisory configuration
- Release workflow tag pattern tightened to semver-only
- Benchmark history output path aligned with `.gitignore`

## [0.1.0] — 2026-03-25

Initial scaffold of the jyotish astronomical computation engine.

### Core modules

- **error** — `JyotishError` enum with 5 variants (InvalidParameter, MathError, DateError, EphemerisError, Io)
- **planet** — `Planet` enum (Sun through Pluto), `PlanetaryPosition` struct with ecliptic coordinates

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
