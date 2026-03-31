# Jyotish

**Jyotish** (Sanskrit: ज्योतिष — science of light) — astronomical computation engine for the AGNOS science stack.

Provides planetary position computation, calendar system conversions, celestial event prediction, zodiac/constellation mapping, house system calculations, aspect computation, and transit tracking.

## Features

- **Planetary positions** — Sun, Moon, Mercury through Pluto with ecliptic coordinates
- **Calendar systems** — Julian, Gregorian, sidereal time, Julian Day Number conversions
- **Celestial events** — eclipses, conjunctions, oppositions, equinoxes, solstices
- **Zodiac** — tropical and sidereal signs, constellation boundaries, cusps
- **House systems** — Placidus, Koch, Equal, Whole Sign, Porphyry
- **Aspects** — conjunction, opposition, trine, square, sextile with configurable orbs
- **Transits** — ingress, retrograde, station, direct motion tracking

## Optional features

- **`orbital`** — high-fidelity orbital mechanics via [falak](https://github.com/MacCracken/falak)
- **`logging`** — tracing-subscriber initialization via `JYOTISH_LOG` env var

## Usage

```toml
[dependencies]
jyotish = "0.1"
```

```rust
use jyotish::{Planet, PlanetaryPosition};

let pos = PlanetaryPosition::new(
    Planet::Mars,
    145.3,  // longitude
    1.2,    // latitude
    1.524,  // distance AU
    1711324800,
);

println!("{} at {:.1}°", pos.planet, pos.longitude_deg);
```

## License

GPL-3.0-only
