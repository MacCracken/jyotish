# Contributing to jyotish

## Getting Started

```bash
git clone https://github.com/MacCracken/jyotish.git
cd jyotish
cargo test --all-features
```

## Guidelines

- All astronomical functions must validate inputs (JD ranges, coordinate bounds) and return `Result` for fallible operations.
- Use `crate::constants` for astronomical constants — never define them locally.
- Periodic term tables (nutation, lunar, planetary) must cite their source (e.g., Meeus table number, IAU reference).
- Use Kahan compensated summation (`crate::num`) for all multi-term series evaluations.
- Angles must be normalized to their expected range before return (0..360 for longitude, -90..90 for latitude).
- Tracing:
  - `error!` for computational failures (Kepler solver divergence, interpolation failure)
  - `warn!` for domain boundary violations (out-of-epoch extrapolation, below-horizon conditions)
  - `#[instrument]` on significant public functions (`level = "trace"` for computations, `level = "debug"` for aggregators)
  - Skip `#[instrument]` on trivial one-liners, tight-loop helpers, and hot constructors
- Every public function needs a doc comment with the formula it implements.
- Tests must validate against known astronomical values (JPL Horizons, SOFA/ERFA, Meeus worked examples) — not arbitrary numbers.
- Zero `unsafe`. Zero `println!`. Zero clippy warnings.

## Testing

```bash
cargo test --all-features       # all tests
cargo clippy --all-features     # lint
cargo bench                     # benchmarks
```

## License

By contributing you agree that your contributions will be licensed under GPL-3.0-only.
