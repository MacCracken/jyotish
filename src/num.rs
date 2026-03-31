//! Numerical utilities for high-precision astronomical computation.
//!
//! Provides compensated summation (Kahan) and other numerical primitives
//! used throughout the library to minimize floating-point error accumulation.

/// Kahan compensated summation accumulator.
///
/// Tracks a running compensation term to reduce summation error from
/// O(n·ε) to O(ε) where ε is machine epsilon.
#[derive(Debug, Clone, Copy)]
pub struct KahanSum {
    sum: f64,
    compensation: f64,
}

impl KahanSum {
    /// Create a new accumulator initialized to zero.
    pub fn new() -> Self {
        Self {
            sum: 0.0,
            compensation: 0.0,
        }
    }

    /// Add a value to the running sum with compensation.
    #[inline]
    pub fn add(&mut self, value: f64) {
        let y = value - self.compensation;
        let t = self.sum + y;
        self.compensation = (t - self.sum) - y;
        self.sum = t;
    }

    /// Return the accumulated sum.
    #[inline]
    pub fn sum(self) -> f64 {
        self.sum
    }
}

impl Default for KahanSum {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kahan_sum_basic() {
        let mut k = KahanSum::new();
        k.add(1.0);
        k.add(2.0);
        k.add(3.0);
        assert!((k.sum() - 6.0).abs() < 1e-15);
    }

    #[test]
    fn kahan_vs_naive_precision() {
        // Sum many small values where naive summation loses precision
        let n = 1_000_000;
        let value = 0.1;

        let mut naive = 0.0_f64;
        let mut kahan = KahanSum::new();

        for _ in 0..n {
            naive += value;
            kahan.add(value);
        }

        let expected = n as f64 * value;
        let naive_err = (naive - expected).abs();
        let kahan_err = (kahan.sum() - expected).abs();

        // Kahan should be significantly more accurate
        assert!(
            kahan_err < naive_err,
            "kahan_err={kahan_err}, naive_err={naive_err}"
        );
    }
}
