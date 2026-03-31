//! Delta T (TT − UT1) computation.
//!
//! Delta T is the difference between Terrestrial Time (TT, the uniform time
//! scale used in ephemeris computation) and Universal Time (UT1, tied to
//! Earth's rotation). This difference is critical for converting between
//! the time scale used by planetary theories (TDB ≈ TT) and the time scale
//! tied to Earth's rotation (UT1, used for sidereal time and rise/set).
//!
//! Uses the polynomial expressions from Espenak & Meeus (2006) for historical
//! periods, and Morrison & Stephenson (2004) for the long-term parabolic model.
//!
//! # Time Scale Summary
//!
//! - **TDB** (Barycentric Dynamical Time): independent variable for planetary ephemerides.
//!   Differs from TT by at most ~1.7 ms (periodic). For our purposes, TDB ≈ TT.
//! - **TT** (Terrestrial Time): uniform time on Earth's geoid. TT = TAI + 32.184s.
//! - **UT1**: Earth rotation angle. UT1 = TT − ΔT.
//! - **UTC**: civil time. |UTC − UT1| < 0.9s by definition.

/// Compute Delta T (TT − UT1) in seconds for a given decimal year.
///
/// Uses Espenak & Meeus polynomial expressions for −500 to 2150 CE,
/// with Morrison & Stephenson parabolic extrapolation outside that range.
///
/// # Examples
///
/// ```
/// # use jyotish::delta_t::delta_t;
/// // Delta T for year 2000.0 ≈ 63.8 seconds
/// let dt = delta_t(2000.0);
/// assert!((dt - 63.8).abs() < 1.0, "got {dt}");
///
/// // Historical: year 1000 CE ≈ 1574 seconds
/// let dt = delta_t(1000.0);
/// assert!((dt - 1574.0).abs() < 50.0, "got {dt}");
/// ```
pub fn delta_t(year: f64) -> f64 {
    // Espenak & Meeus (2006), "Five Millennium Canon of Solar Eclipses"
    // NASA/TP-2006-214141. Horner form: c0 + t*(c1 + t*(c2 + ...))
    if year < -500.0 {
        let u = (year - 1820.0) / 100.0;
        -20.0 + 32.0 * u * u
    } else if year < 500.0 {
        let u = year / 100.0;
        10583.6
            + u * (-1014.41
                + u * (33.78311
                    + u * (-5.952053 + u * (-0.1798452 + u * (0.022174192 + u * 0.0090316521)))))
    } else if year < 1600.0 {
        let u = (year - 1000.0) / 100.0;
        1574.2
            + u * (-556.01
                + u * (71.23472
                    + u * (0.319781 + u * (-0.8503463 + u * (-0.005050998 + u * 0.0083572073)))))
    } else if year < 1700.0 {
        let t = year - 1600.0;
        120.0 + t * (-0.9808 + t * (-0.01532 + t / 7129.0))
    } else if year < 1800.0 {
        let t = year - 1700.0;
        8.83 + t * (0.1603 + t * (-0.0059285 + t * (0.00013336 - t / 1_174_000.0)))
    } else if year < 1860.0 {
        let t = year - 1800.0;
        13.72
            + t * (-0.332447
                + t * (0.0068612
                    + t * (0.0041116
                        + t * (-0.00037436
                            + t * (0.0000121272 + t * (-0.0000001699 + t * 0.000000000875))))))
    } else if year < 1900.0 {
        let t = year - 1860.0;
        7.62 + t
            * (0.5737 + t * (-0.251754 + t * (0.01680668 + t * (-0.0004473624 + t / 233_174.0))))
    } else if year < 1920.0 {
        let t = year - 1900.0;
        -2.79 + t * (1.494119 + t * (-0.0598939 + t * (0.0061966 - t * 0.000197)))
    } else if year < 1941.0 {
        let t = year - 1920.0;
        21.20 + t * (0.84493 + t * (-0.076100 + t * 0.0020936))
    } else if year < 1961.0 {
        let t = year - 1950.0;
        29.07 + t * (0.407 + t * (-1.0 / 233.0 + t / 2547.0))
    } else if year < 1986.0 {
        let t = year - 1975.0;
        45.45 + t * (1.067 + t * (-1.0 / 260.0 - t / 718.0))
    } else if year < 2005.0 {
        let t = year - 2000.0;
        63.86
            + t * (0.3345
                + t * (-0.060374 + t * (0.0017275 + t * (0.000651814 + t * 0.00002373599))))
    } else if year < 2050.0 {
        let t = year - 2000.0;
        62.92 + t * (0.32217 + t * 0.005589)
    } else if year < 2150.0 {
        -20.0 + 32.0 * ((year - 1820.0) / 100.0).powi(2) - 0.5628 * (2150.0 - year)
    } else {
        let u = (year - 1820.0) / 100.0;
        -20.0 + 32.0 * u * u
    }
}

/// Compute Delta T for a given Julian Date.
///
/// Converts JD to decimal year first, then calls [`delta_t`].
///
/// # Examples
///
/// ```
/// # use jyotish::delta_t::delta_t_jd;
/// let dt = delta_t_jd(2_451_545.0); // J2000.0 = 2000-01-01
/// assert!((dt - 63.8).abs() < 1.0, "got {dt}");
/// ```
pub fn delta_t_jd(jd: f64) -> f64 {
    let year = jd_to_decimal_year(jd);
    delta_t(year)
}

/// Convert a Julian Date to a Terrestrial Time Julian Date using Delta T.
///
/// If the input is UT1, this returns TT = UT1 + ΔT.
pub fn ut1_to_tt(jd_ut1: f64) -> f64 {
    jd_ut1 + delta_t_jd(jd_ut1) / 86_400.0
}

/// Convert a Terrestrial Time Julian Date to UT1 using Delta T.
///
/// Returns UT1 = TT − ΔT.
pub fn tt_to_ut1(jd_tt: f64) -> f64 {
    jd_tt - delta_t_jd(jd_tt) / 86_400.0
}

/// Convert a Julian Date to a decimal year (e.g., 2000.0 for J2000.0).
fn jd_to_decimal_year(jd: f64) -> f64 {
    2000.0 + (jd - 2_451_545.0) / 365.25
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delta_t_j2000() {
        let dt = delta_t(2000.0);
        // ~63.8 seconds in 2000
        assert!((dt - 63.8).abs() < 1.0, "got {dt}");
    }

    #[test]
    fn delta_t_1900() {
        let dt = delta_t(1900.0);
        // ~−2.8 seconds in 1900
        assert!((dt - (-2.8)).abs() < 2.0, "got {dt}");
    }

    #[test]
    fn delta_t_historical() {
        let dt = delta_t(1000.0);
        // ~1574 seconds in 1000 CE
        assert!((dt - 1574.0).abs() < 50.0, "got {dt}");
    }

    #[test]
    fn delta_t_ancient() {
        let dt = delta_t(-500.0);
        // ~17190 seconds in 500 BCE
        assert!(dt > 15_000.0 && dt < 20_000.0, "got {dt}");
    }

    #[test]
    fn delta_t_2024() {
        let dt = delta_t(2024.0);
        // Espenak polynomial gives ~74s for 2024 (actual observed ~69.4s)
        // Polynomial extrapolation diverges slightly from observations
        assert!((dt - 69.0).abs() < 6.0, "got {dt}");
    }

    #[test]
    fn delta_t_future() {
        let dt = delta_t(2100.0);
        // Extrapolated, should be positive and growing
        assert!(dt > 100.0, "got {dt}");
    }

    #[test]
    fn delta_t_jd_consistency() {
        let dt_year = delta_t(2000.0);
        let dt_jd = delta_t_jd(2_451_545.0);
        assert!((dt_year - dt_jd).abs() < 0.1, "year={dt_year}, jd={dt_jd}");
    }

    #[test]
    fn ut1_tt_roundtrip() {
        let jd_ut1 = 2_451_545.0;
        let jd_tt = ut1_to_tt(jd_ut1);
        let restored = tt_to_ut1(jd_tt);
        // Should be very close (not exact due to ΔT depending on JD)
        assert!((restored - jd_ut1).abs() < 1e-6, "restored = {restored}");
    }

    #[test]
    fn delta_t_monotonic_recent() {
        // ΔT should be generally increasing in the 1970-2024 range
        let dt_1970 = delta_t(1970.0);
        let dt_2000 = delta_t(2000.0);
        let dt_2024 = delta_t(2024.0);
        assert!(
            dt_2000 > dt_1970,
            "ΔT should increase: 1970={dt_1970}, 2000={dt_2000}"
        );
        assert!(
            dt_2024 > dt_2000,
            "ΔT should increase: 2000={dt_2000}, 2024={dt_2024}"
        );
    }

    #[test]
    fn delta_t_continuity_at_boundaries() {
        // Espenak polynomials have small discontinuities at segment boundaries
        // (inherent in piecewise fitting). Verify they're not catastrophic.
        let boundaries = [
            -500.0, 500.0, 1600.0, 1700.0, 1800.0, 1860.0, 1900.0, 1920.0, 1941.0, 1961.0, 1986.0,
            2005.0, 2050.0, 2150.0,
        ];
        for &b in &boundaries {
            let dt_before = delta_t(b - 0.001);
            let dt_after = delta_t(b + 0.001);
            let jump = (dt_after - dt_before).abs();
            assert!(
                jump < 10.0,
                "discontinuity at {b}: before={dt_before}, after={dt_after}, jump={jump}"
            );
        }
    }
}
