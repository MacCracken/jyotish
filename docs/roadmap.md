# Jyotish Roadmap

## Phase 2 — Accuracy Pipeline (in progress)

- [ ] Light-time correction (iterative, Meeus Ch. 36)
- [ ] Annual aberration (Meeus Ch. 23, ~20.5" correction)
- [ ] IAU 2000B nutation (replace IAU 1980; 77 terms, 1 mas accuracy)
- [ ] Position type specification (geometric vs astrometric vs apparent)
- [ ] Apparent position pipeline (geometric → light-time → aberration → nutation → apparent)

## Phase 3 — Theory Upgrade (Swiss Ephemeris Moshier parity)

- [ ] Full VSOP87 planetary series (2425+ terms, <1" accuracy)
- [ ] ELP2000-82 lunar theory (37K terms, <3" accuracy, replaces Meeus Ch. 47)
- [ ] IAU 2006 precession (Capitaine et al., replaces current model)

## Phase 4 — Feature Completeness (world-class)

- [ ] Eclipse computation with Besselian elements, contact times, magnitude
- [ ] Lunar phases (New/Full/Quarter moon times, lunation numbers)
- [ ] Planetary phenomena (greatest elongation, perihelion, stations)
- [ ] Fixed star catalog with proper motion (~60 navigational stars)
- [ ] Atmospheric refraction model (Bennett/Saemundsson)
- [ ] Physical ephemerides (apparent diameter, illuminated fraction, phase angle)

## Future

- [ ] VSOP2013/TOP2013 upgrade path (0.1-0.5" planetary accuracy)
- [ ] IAU 2000A full nutation (2052 terms, 0.1 mas)
- [ ] FK5/ICRS frame tie
- [ ] JPL DE ephemeris file support (optional feature)

## Accuracy Targets

| Body | Current | Target (no external files) | Method |
|------|---------|---------------------------|--------|
| Sun | ~36" | <1" | Full VSOP87 |
| Planets | ~1-10' | <1" | Full VSOP87 |
| Moon | ~10" | <3" | ELP2000-82 |
| Nutation | ~5 mas | <1 mas | IAU 2000B |
