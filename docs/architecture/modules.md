# Module Architecture

## Core

- `src/error.rs` — `JyotishError` enum with `thiserror`
- `src/num.rs` — Kahan compensated summation and numerical utilities
- `src/planet.rs` — `Planet` enum, `PlanetaryPosition` struct

## Time & Calendar

- `src/delta_t.rs` — Delta T (TT−UT1) via Espenak & Meeus polynomials, time scale conversions
- `src/calendar.rs` — Julian/Gregorian/sidereal time/JDN conversions

## Coordinates & Corrections

- `src/coords.rs` — degree/radian conversion, ecliptic↔equatorial (pole-safe), mean obliquity
- `src/nutation.rs` — IAU 2000B nutation (77 terms), IAU 2006 precession, true obliquity
- `src/aberration.rs` — annual aberration and light-time correction (Meeus Ch. 23/36)
- `src/parallax.rs` — topocentric lunar parallax, horizontal parallax, Observer struct
- `src/refraction.rs` — atmospheric refraction (Bennett/Saemundsson), pressure/temperature correction

## Celestial Bodies

- `src/sun.rs` — solar longitude, distance, equation of time (VSOP87/Meeus)
- `src/moon.rs` — lunar longitude, latitude, distance (Meeus Ch. 47, 60 periodic terms)
- `src/planetary.rs` — Mercury–Pluto geocentric positions (VSOP87 + Keplerian fallback)
- `src/star.rs` — 58 navigational stars with proper motion, J2000.0 catalog

## High-Accuracy Theories

- `src/vsop87/` — VSOP87D heliocentric ecliptic coordinates (J2000.0), per-planet data files (Mercury–Neptune)
- `src/apparent.rs` — apparent position pipeline (geometric → aberration → nutation → apparent)

## Physical Ephemerides

- `src/physical.rs` — apparent diameter, phase angle, illuminated fraction, elongation

## Astrological Systems

- `src/zodiac.rs` — tropical/sidereal zodiac, Lahiri ayanamsa, signs, elements, modalities
- `src/house.rs` — Placidus, Equal, Whole Sign, Porphyry house systems
- `src/aspect.rs` — conjunction, opposition, trine, square, sextile with configurable orbs

## Events & Motion

- `src/event.rs` — equinox/solstice search, conjunction/opposition detection
- `src/transit.rs` — daily motion, ingress, retrograde station detection
- `src/riseset.rs` — rise/set/transit times (Meeus Ch. 15 with interpolation)
- `src/phase.rs` — lunar phases (New/Full/Quarter moon times, lunation numbers)
- `src/phenomena.rs` — greatest elongation, opposition, conjunction, station detection
- `src/eclipse.rs` — solar/lunar eclipse prediction, classification, magnitude

## Feature-Gated

- `src/orbital.rs` — falak integration: enhanced Kepler solver, state vectors (`orbital` feature)
- `src/logging.rs` — tracing-subscriber init (`logging` feature)

## Dependencies

- **hisab** — trigonometry, linear algebra, numerical methods
- **chrono** — date/time handling
- **falak** (optional, `orbital` feature) — orbital mechanics
