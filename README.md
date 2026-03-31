# jyotish

> **Jyotish** (Sanskrit: ज्योतिष -- science of light/celestial bodies)

**Astronomical computation engine** -- planetary positions, calendar systems, celestial event prediction, and astrological calculations. Built on the [hisab](https://crates.io/crates/hisab) math foundation.

[![CI](https://github.com/MacCracken/jyotish/actions/workflows/ci.yml/badge.svg)](https://github.com/MacCracken/jyotish/actions/workflows/ci.yml)
[![License: GPL-3.0-only](https://img.shields.io/badge/license-GPL--3.0--only-blue.svg)](LICENSE)
[![MSRV: 1.89](https://img.shields.io/badge/MSRV-1.89-blue.svg)](https://blog.rust-lang.org/)

## Overview

Jyotish is the astronomical computation layer of the AGNOS science stack. It computes geocentric and topocentric positions for the Sun, Moon, and planets using full VSOP87 and ELP2000-82 theories, achieving sub-arcsecond accuracy for all major bodies. Beyond raw positions, it provides calendar conversions, celestial event prediction (eclipses, conjunctions, equinoxes), zodiac and house system calculations, aspect geometry, transit tracking, and physical ephemerides. Jyotish feeds astronomical context into hoosh, agnosai, and joshua.

## Features

### Positions and Bodies

- **Solar position** -- longitude, distance, equation of time via full VSOP87 (Meeus Ch. 25)
- **Lunar position** -- longitude, latitude, distance via ELP2000-82 (~180 periodic terms)
- **Planetary positions** -- Mercury through Neptune via VSOP87D heliocentric theory; Pluto via Keplerian elements
- **Fixed stars** -- 58 navigational stars with proper motion, J2000.0 catalog
- **Apparent positions** -- full pipeline: geometric, aberration, nutation, light-time correction

### Calendar and Time

- **Calendar conversions** -- Julian, Gregorian, Julian Day Number, Unix timestamp
- **Sidereal time** -- GMST and local sidereal time (UT1 and TT variants)
- **Delta T** -- TT-UT1 via Espenak and Meeus polynomials (historical and predictive)

### Coordinates and Corrections

- **Coordinate transforms** -- ecliptic/equatorial conversion, degree/radian utilities
- **Nutation** -- IAU 2000B (77 terms, ~1 mas accuracy)
- **Precession** -- IAU 2006 precession model
- **Aberration** -- annual aberration and light-time correction (Meeus Ch. 23/36)
- **Parallax** -- topocentric lunar parallax, horizontal parallax, Observer struct
- **Refraction** -- atmospheric refraction (Bennett/Saemundsson) with pressure/temperature correction

### Events and Phenomena

- **Eclipses** -- solar and lunar eclipse prediction, classification, magnitude
- **Lunar phases** -- New Moon, First Quarter, Full Moon, Last Quarter times and lunation numbers
- **Equinoxes and solstices** -- iterative search for exact moments
- **Conjunctions and oppositions** -- detection with angular separation
- **Planetary phenomena** -- greatest elongation, opposition, conjunction, station detection
- **Rise/set/transit** -- times via Meeus Ch. 15 with interpolation

### Physical Ephemerides

- **Apparent diameter** -- angular size of solar system bodies
- **Phase angle and illumination** -- illuminated fraction for planets and Moon
- **Elongation** -- angular separation from the Sun

### Astrological Systems

- **Zodiac** -- tropical and sidereal signs, Lahiri ayanamsa, elements, modalities
- **House systems** -- Placidus, Equal, Whole Sign, Porphyry
- **Aspects** -- conjunction, opposition, trine, square, sextile with configurable orbs
- **Transits** -- daily motion, ingress, retrograde station detection

## Accuracy

| Body | Achieved | Method | Reference |
|------|----------|--------|-----------|
| Sun | <1" | Full VSOP87 | Bretagnon & Francou 1988 |
| Planets | <1" | Full VSOP87D | Bretagnon & Francou 1988 |
| Moon | <2" | ELP2000-82 (~180 terms) | Chapront-Touze & Chapront 1988 |
| Nutation | ~1 mas | IAU 2000B (77 terms) | McCarthy & Luzum 2003 |
| Precession | IAU 2006 | Capitaine et al. | IAU 2006 |

## Architecture

Modules are organized by domain:

### Core

| Module | Description |
|--------|-------------|
| `error` | `JyotishError` enum with `thiserror` |
| `num` | Kahan compensated summation, numerical utilities |
| `planet` | `Planet` enum, `PlanetaryPosition` struct |

### Time and Calendar

| Module | Description |
|--------|-------------|
| `calendar` | Julian/Gregorian/JDN/sidereal time conversions |
| `delta_t` | TT-UT1 via Espenak and Meeus polynomials |

### Coordinates and Corrections

| Module | Description |
|--------|-------------|
| `coords` | Ecliptic/equatorial transforms, mean obliquity |
| `nutation` | IAU 2000B nutation, IAU 2006 precession, true obliquity |
| `aberration` | Annual aberration, light-time correction |
| `parallax` | Topocentric parallax, Observer struct |
| `refraction` | Atmospheric refraction (Bennett/Saemundsson) |

### Celestial Bodies

| Module | Description |
|--------|-------------|
| `sun` | Solar longitude, distance, equation of time |
| `moon` | Lunar longitude, latitude, distance (Meeus Ch. 47) |
| `elp2000` | ELP2000-82 lunar theory (~180 terms) |
| `planetary` | Mercury-Pluto geocentric positions |
| `vsop87` | VSOP87D heliocentric ecliptic coordinates |
| `star` | 58 navigational stars with proper motion |
| `apparent` | Apparent position pipeline (geometric through apparent) |
| `physical` | Apparent diameter, phase angle, illumination, elongation |

### Events and Motion

| Module | Description |
|--------|-------------|
| `event` | Equinox/solstice search, conjunction/opposition detection |
| `eclipse` | Solar/lunar eclipse prediction and classification |
| `phase` | Lunar phase times and lunation numbers |
| `phenomena` | Greatest elongation, opposition, conjunction, stations |
| `transit` | Daily motion, ingress, retrograde station detection |
| `riseset` | Rise/set/transit times (Meeus Ch. 15) |

### Astrological

| Module | Description |
|--------|-------------|
| `zodiac` | Tropical/sidereal signs, Lahiri ayanamsa |
| `house` | Placidus, Equal, Whole Sign, Porphyry |
| `aspect` | Major aspects with configurable orbs |

## Quick Start

```toml
[dependencies]
jyotish = "1.0"
```

```rust
use jyotish::calendar::gregorian_to_jd;
use jyotish::apparent::{apparent_sun, apparent_moon, apparent_position};
use jyotish::planet::Planet;
use jyotish::eclipse::next_lunar_eclipse;
use jyotish::phase::phase_info;

// Compute Julian Date for 2026-03-20 12:00 UT
let jd = gregorian_to_jd(2026, 3, 20, 12, 0, 0.0);

// Apparent solar position (includes nutation + aberration)
let sun = apparent_sun(jd);
println!("Sun: {:.4} deg", sun.longitude);

// Apparent lunar position
let moon = apparent_moon(jd);
println!("Moon: {:.4} deg", moon.longitude);

// Apparent planetary position
let mars = apparent_position(Planet::Mars, jd);
println!("Mars: {:.4} deg", mars.longitude);

// Next lunar eclipse after this date
let eclipse = next_lunar_eclipse(jd);
println!("Next lunar eclipse: JD {:.2}", eclipse.jd_max);

// Current lunar phase
let phase = phase_info(jd);
println!("Phase: {:.1}% illuminated", phase.illumination * 100.0);
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `meeus` | no | Select Meeus-chapter algorithms (lower precision, faster) |
| `orbital` | no | High-fidelity orbital mechanics via [falak](https://github.com/MacCracken/falak) |
| `logging` | no | Structured tracing via `JYOTISH_LOG` env var |

```toml
[dependencies]
jyotish = { version = "1.0", features = ["orbital"] }
```

## Consumers

| Crate | Role |
|-------|------|
| **hoosh** | Celestial context for environmental awareness |
| **agnosai** | Astronomical reasoning and interpretation |
| **joshua** | Game world astronomy and sky simulation |

## Relationship to AGNOS Science Stack

```
hisab (math foundation)
  ├── hisab-mimamsa — theoretical physics
  ├── brahmanda — galactic / large-scale structure
  ├── tara — stellar astrophysics
  ├── jyotish (this) — astronomical computation
  ├── falak — orbital mechanics
  ├── kana — quantum mechanics
  └── tanmatra — atomic / subatomic physics
```

## Roadmap

See [docs/roadmap.md](docs/roadmap.md) for the full plan. Key 1.1.0 goals:

- VSOP2013/TOP2013 upgrade path (0.1-0.5" planetary accuracy)
- IAU 2000A full nutation (2052 terms, 0.1 mas)
- FK5/ICRS frame tie
- JPL DE ephemeris file support (optional feature)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

GPL-3.0-only
