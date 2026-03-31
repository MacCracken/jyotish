# Changelog

All notable changes to jyotish are documented in this file.

## [Unreleased]

## [1.0.0] — 2026-03-31

### Added

#### Core Modules

- **calendar** — JDN/JD conversions, Gregorian/Julian calendar, sidereal time (GMST/LST), leap year, day of week, proper day-of-month validation
- **coords** — degree/radian conversion, ecliptic↔equatorial, mean obliquity, angle normalization
- **sun** — solar longitude/distance/position (Meeus Ch. 25), equation of time
- **moon** — lunar longitude/latitude/distance (Meeus Ch. 47, 60+60 periodic terms with eccentricity correction)
- **planetary** — Mercury–Pluto geocentric ecliptic positions via Keplerian elements + Kepler solver
- **nutation** — IAU 2000B (McCarthy & Luzum 2003); 77 lunisolar terms with planetary offset corrections, ~1 mas accuracy; IAU 2006 precession (Capitaine et al. 2003); ICRS frame bias; true obliquity
- **event** — equinox/solstice search, planetary conjunction/opposition detection with bisection refinement
- **zodiac** — tropical/sidereal zodiac signs, Lahiri ayanamsa, elements, modalities, sign position lookup
- **house** — Placidus, Equal, Whole Sign, Porphyry house systems; ascendant and midheaven computation
- **aspect** — conjunction, opposition, trine, square, sextile with configurable orbs and aspect strength
- **transit** — daily motion computation, sign ingress search, retrograde station detection, motion state
- **error** — `JyotishError` enum with 5 variants (InvalidParameter, MathError, DateError, EphemerisError, Io)
- **planet** — `Planet` enum (Sun through Pluto), `PlanetaryPosition` struct with ecliptic coordinates

#### Accuracy Pipeline

- **aberration** — annual aberration correction (Meeus Ch. 23, ~20.5" solar correction), light-time correction
- **apparent** — apparent position pipeline: geometric → aberration → nutation → apparent; `PositionType`/`TypedPosition` API for geometric vs apparent coordinates
- **vsop87** — VSOP87D heliocentric ecliptic coordinates (J2000.0) for Mercury–Neptune, per-planet data files

#### Lunar Theory

- **elp2000** — ELP2000-82 lunar theory with 4th-order Delaunay arguments (Chapront et al. 2002), main problem terms (60 lon + 60 lat + 46 distance), planetary perturbation corrections, additive corrections; <2" longitude accuracy; `lunar_longitude`, `lunar_latitude`, `lunar_distance_km`, `lunar_coordinates` API

#### Celestial Events and Phenomena

- **eclipse** — solar/lunar eclipse prediction with classification (total, annular, partial, hybrid, penumbral), magnitude, gamma parameter
- **phase** — lunar phase computation (New/Full/Quarter moon times), lunation numbers, phase angle, next-phase search
- **phenomena** — planetary phenomena detection: greatest elongation, opposition, conjunction, station (retrograde/direct)
- **star** — fixed star catalog (58 navigational stars) with J2000.0 coordinates, proper motion, magnitude; position-at-epoch, search, brightness ranking

#### Observation Corrections

- **refraction** — atmospheric refraction (Bennett/Saemundsson), apparent↔true altitude conversion, pressure/temperature correction
- **physical** — physical ephemerides: apparent angular diameter, phase angle, illuminated fraction, elongation from Sun
- **riseset** — rise/set/transit times using Meeus Ch. 15 with iterative interpolation refinement
- **parallax** — topocentric lunar parallax correction, `Observer` struct, horizontal parallax computation

#### Numerical Foundation

- **num** — Kahan compensated summation (`KahanSum`) for all periodic term series
- **delta_t** — Delta T (TT−UT1) via Espenak & Meeus (2006) polynomials, 14 segments from −500 CE to 2150+; `ut1_to_tt()`, `tt_to_ut1()` time scale conversions
- Converted all polynomials to Horner's method (~15 sites across coords, calendar, sun, moon, nutation)
- Applied Kahan summation to moon (3 loops, ~180 terms) and nutation (63 terms)
- Range reduction for secondary moon arguments before trig calls

#### Optional Features

- **`orbital`** (falak) — orbital mechanics integration: enhanced Kepler solver (Danby+NR), heliocentric state vectors, geocentric positions via falak's orbital mechanics
- **`logging`** — tracing-subscriber initialization via `JYOTISH_LOG` env var

#### Infrastructure

- `.gitignore` — created; removed `target/` from git tracking
- `deny.toml` — cargo-deny license/advisory configuration
- Release workflow tag pattern tightened to semver-only
- Benchmark history output path aligned with `.gitignore`
- Cargo.toml with hisab, chrono, serde, thiserror, tracing
- CI workflow (fmt, clippy, audit, deny, test, MSRV, coverage, doc)
- Release workflow (GitHub Release only, no cargo publish)
- Criterion benchmark harness
- Integration test scaffold
- GPL-3.0-only license
- Rust 2024 edition, MSRV 1.89

### Fixed

- **coords** — rewrote `ecliptic_to_equatorial` to avoid `tan(lat)` singularity at ecliptic poles
- **event** — fixed `partial_cmp().unwrap()` panic risk in sort; documented recursion bound
- **riseset** — added division-by-zero guard in altitude refinement loop
- **calendar** — allow leap seconds (second=60.0) in time validation
- **planetary** — added convergence guard in Kepler solver for near-zero denominator
- **apparent** — apply aberration to Moon (previously incorrectly omitted)
- **nutation** — use `rem_euclid()` for proper modulo in Delaunay argument reduction
- **house** — clamp `asin` argument in Placidus to prevent NaN at circumpolar declinations
- **sun** — gate unused `equation_of_center_meeus` behind `meeus` feature to eliminate dead code warning
- Removed orphan `src/elp2000/` directory (incomplete, never wired into crate)
