# Jyotish Roadmap

## Pre-1.0 Review Repairs

- [ ] Add `#[non_exhaustive]` to public enums: `AspectKind`, `MotionState`, `Season`, `HouseSystem`, `PositionType`
- [ ] Add `#[must_use]` to pure computational functions across sun, moon, nutation, aberration, eclipse, phase, phenomena, transit, aspect, event, orbital
- [ ] Add `#[inline]` to hot-path math: calendar, coords, zodiac sign lookups, `angular_separation`, `horizontal_parallax`, `evaluate_series`
- [ ] Fix `calendar.rs` seconds validation — `0.0..61.0` allows 60.5–60.99; tighten to `0.0..=60.0`
- [ ] Add `PartialEq` to `PhaseInfo` and `PhenomenonEvent`
- [ ] Add benchmarks for eclipse, phase, phenomena, refraction, star modules
- [ ] Replace `partial_cmp().unwrap_or()` with `total_cmp()` in aspect.rs and event.rs sorts
- [ ] Add doc note on `delta_t()` extrapolation uncertainty for years > 2150

## 1.1.0

- [ ] VSOP2013/TOP2013 upgrade path (0.1-0.5" planetary accuracy)
- [ ] IAU 2000A full nutation (2052 terms, 0.1 mas)
- [ ] FK5/ICRS frame tie
- [ ] JPL DE ephemeris file support (optional feature)

## Accuracy Targets

| Body | Achieved | Method |
|------|----------|--------|
| Sun | <1" | Full VSOP87 |
| Planets | <1" | Full VSOP87 |
| Moon | <2" | ELP2000-82 |
| Nutation | ~1 mas | IAU 2000B |
