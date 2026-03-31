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

## Roadmap

### P0 — Blocking (must fix before any further work)

1. **Fix compilation**: `src/num.rs` exists (provides `KahanSum`) but is not declared in `lib.rs`. Both `moon.rs` and `nutation.rs` import `crate::num`. Fix: add `mod num;` to `lib.rs`.
2. **Verify `cargo test --all-features` passes** after the fix.
3. **Run `cargo clippy --all-features`** and fix all warnings.
4. **Update CHANGELOG.md** — currently says modules are "stubs" but they are fully implemented (5,326 lines across 16 modules).

### P1 — Testing & Hardening (v1 gate)

5. Add **inline unit tests** to every module (currently only 10 integration tests — no module-level tests exist).
6. Add **adversarial input tests** — NaN, Inf, negative values, edge dates (year 0, far past/future), invalid lat/lon for every public function.
7. **Input/output validation audit** — ensure all public functions validate inputs (`require_finite` or equivalent) and produce finite outputs.
8. Add `tracing::warn` on physics/domain boundary violations.
9. Validate planetary positions against **JPL Horizons** or **VSOP87** reference data for at least Sun, Moon, and one outer planet.

### P2 — Documentation & Polish (v1 readiness)

10. Doc comments with formulas on all public functions.
11. Doc tests (runnable examples in `/// # Examples` blocks).
12. Working `examples/` directory.
13. API stability audit — confirm all public items are intentional for v1.

### Integration: hisab-mimamsa Scale 3

Once v1-ready, jyotish provides the planetary position + aspect + zodiac + house data needed for hisab-mimamsa's `unified::scale_bridge::bridge_scale_3()` (planetary field → personality manifestation via bhava).
