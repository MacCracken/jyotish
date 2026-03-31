//! Calendar systems — Julian, Gregorian, sidereal time, Julian Day Number conversions.
//!
//! Implements the standard astronomical calendar algorithms from Meeus
//! (*Astronomical Algorithms*, 2nd ed.) for converting between civil calendar
//! dates, Julian Day Numbers, and sidereal time.
//!
//! # Terminology
//!
//! - **JDN** (Julian Day Number) — the integer day count from the epoch
//!   January 1, 4713 BC (Julian) / November 24, 4714 BC (Gregorian).
//! - **JD** (Julian Date) — JDN plus fractional day (noon = .0, midnight = .5).
//! - **GMST** — Greenwich Mean Sidereal Time.
//! - **LST** — Local Sidereal Time.

use crate::coords::normalize_degrees;
use crate::error::{JyotishError, Result};

/// Julian Date of the J2000.0 epoch (2000-01-01T12:00:00 TT).
pub const J2000_0: f64 = 2_451_545.0;

/// Julian Date of the Unix epoch (1970-01-01T00:00:00 UTC).
pub const JD_UNIX_EPOCH: f64 = 2_440_587.5;

/// Days per Julian century.
pub const DAYS_PER_JULIAN_CENTURY: f64 = 36_525.0;

/// JDN of the Gregorian calendar reform date (October 15, 1582).
pub const GREGORIAN_REFORM_JDN: i64 = 2_299_161;

// ---------------------------------------------------------------------------
// Gregorian ↔ JDN
// ---------------------------------------------------------------------------

/// Convert a Gregorian calendar date to a Julian Day Number.
///
/// Valid for any Gregorian date (proleptic Gregorian for dates before 1582-10-15).
///
/// # Errors
///
/// Returns [`JyotishError::InvalidParameter`] if month is not in 1..=12
/// or day is not in 1..=31.
///
/// # Examples
///
/// ```
/// # use jyotish::calendar::gregorian_to_jdn;
/// // J2000.0 epoch: 2000-01-01 at noon → JDN 2_451_545
/// assert_eq!(gregorian_to_jdn(2000, 1, 1).unwrap(), 2_451_545);
///
/// // Unix epoch: 1970-01-01
/// assert_eq!(gregorian_to_jdn(1970, 1, 1).unwrap(), 2_440_588);
/// ```
pub fn gregorian_to_jdn(year: i32, month: u32, day: u32) -> Result<i64> {
    validate_date_parts(year, month, day)?;

    let a = (14 - month as i64) / 12;
    let y = year as i64 + 4800 - a;
    let m = month as i64 + 12 * a - 3;

    let jdn = day as i64 + (153 * m + 2) / 5 + 365 * y + y / 4 - y / 100 + y / 400 - 32045;

    Ok(jdn)
}

/// Convert a Julian Day Number to a Gregorian calendar date.
///
/// Returns `(year, month, day)`. Works for any JDN (proleptic Gregorian
/// for dates before the reform).
///
/// # Examples
///
/// ```
/// # use jyotish::calendar::jdn_to_gregorian;
/// assert_eq!(jdn_to_gregorian(2_451_545), (2000, 1, 1));
/// assert_eq!(jdn_to_gregorian(2_440_588), (1970, 1, 1));
/// ```
pub fn jdn_to_gregorian(jdn: i64) -> (i32, u32, u32) {
    // Richards algorithm
    let a = jdn + 32044;
    let b = (4 * a + 3) / 146097;
    let c = a - 146097 * b / 4;
    let d = (4 * c + 3) / 1461;
    let e = c - 1461 * d / 4;
    let m = (5 * e + 2) / 153;

    let day = (e - (153 * m + 2) / 5 + 1) as u32;
    let month = (m + 3 - 12 * (m / 10)) as u32;
    let year = (100 * b + d - 4800 + m / 10) as i32;

    (year, month, day)
}

// ---------------------------------------------------------------------------
// Julian ↔ JDN
// ---------------------------------------------------------------------------

/// Convert a Julian calendar date to a Julian Day Number.
///
/// # Errors
///
/// Returns [`JyotishError::InvalidParameter`] if month is not in 1..=12
/// or day is not in 1..=31.
///
/// # Examples
///
/// ```
/// # use jyotish::calendar::julian_to_jdn;
/// // October 4, 1582 (last day of Julian calendar) → JDN 2_299_160
/// assert_eq!(julian_to_jdn(1582, 10, 4).unwrap(), 2_299_160);
/// ```
pub fn julian_to_jdn(year: i32, month: u32, day: u32) -> Result<i64> {
    validate_date_parts(year, month, day)?;

    let a = (14 - month as i64) / 12;
    let y = year as i64 + 4800 - a;
    let m = month as i64 + 12 * a - 3;

    let jdn = day as i64 + (153 * m + 2) / 5 + 365 * y + y / 4 - 32083;

    Ok(jdn)
}

/// Convert a Julian Day Number to a Julian calendar date.
///
/// Returns `(year, month, day)`.
///
/// # Examples
///
/// ```
/// # use jyotish::calendar::jdn_to_julian;
/// assert_eq!(jdn_to_julian(2_299_160), (1582, 10, 4));
/// ```
pub fn jdn_to_julian(jdn: i64) -> (i32, u32, u32) {
    let b = 0_i64;
    let c = jdn + 32082;
    let d = (4 * c + 3) / 1461;
    let e = c - 1461 * d / 4;
    let m = (5 * e + 2) / 153;

    let day = (e - (153 * m + 2) / 5 + 1) as u32;
    let month = (m + 3 - 12 * (m / 10)) as u32;
    let year = (100 * b + d - 4800 + m / 10) as i32;

    (year, month, day)
}

// ---------------------------------------------------------------------------
// Julian Date ↔ Unix timestamp
// ---------------------------------------------------------------------------

/// Convert a Unix timestamp (seconds since 1970-01-01T00:00:00 UTC) to a Julian Date.
///
/// # Examples
///
/// ```
/// # use jyotish::calendar::unix_to_jd;
/// let jd = unix_to_jd(0);
/// assert!((jd - 2_440_587.5).abs() < 1e-10);
/// ```
pub fn unix_to_jd(timestamp: i64) -> f64 {
    JD_UNIX_EPOCH + (timestamp as f64) / 86_400.0
}

/// Convert a Julian Date to a Unix timestamp (seconds since 1970-01-01T00:00:00 UTC).
///
/// Fractional seconds are truncated.
///
/// # Examples
///
/// ```
/// # use jyotish::calendar::jd_to_unix;
/// assert_eq!(jd_to_unix(2_440_587.5), 0);
/// ```
pub fn jd_to_unix(jd: f64) -> i64 {
    ((jd - JD_UNIX_EPOCH) * 86_400.0) as i64
}

// ---------------------------------------------------------------------------
// Julian Date ↔ calendar date (with fractional day)
// ---------------------------------------------------------------------------

/// Convert a Gregorian date and time to a Julian Date.
///
/// `hour`, `minute`, `second` represent UTC time of day.
///
/// # Errors
///
/// Returns [`JyotishError::InvalidParameter`] for invalid date or time parts.
///
/// # Examples
///
/// ```
/// # use jyotish::calendar::gregorian_to_jd;
/// // J2000.0: 2000-01-01 12:00:00 TT
/// let jd = gregorian_to_jd(2000, 1, 1, 12, 0, 0.0).unwrap();
/// assert!((jd - 2_451_545.0).abs() < 1e-10);
/// ```
pub fn gregorian_to_jd(
    year: i32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
    second: f64,
) -> Result<f64> {
    validate_time_parts(hour, minute, second)?;
    let jdn = gregorian_to_jdn(year, month, day)?;
    let day_fraction = time_to_day_fraction(hour, minute, second);
    // JDN is defined at noon, so subtract 0.5 to get midnight, then add fraction
    Ok(jdn as f64 - 0.5 + day_fraction)
}

/// Convert a Julian Date to a Gregorian date and time.
///
/// Returns `(year, month, day, hour, minute, second)`.
///
/// # Examples
///
/// ```
/// # use jyotish::calendar::jd_to_gregorian;
/// let (y, m, d, h, min, s) = jd_to_gregorian(2_451_545.0);
/// assert_eq!((y, m, d, h, min), (2000, 1, 1, 12, 0));
/// assert!(s.abs() < 1e-6);
/// ```
pub fn jd_to_gregorian(jd: f64) -> (i32, u32, u32, u32, u32, f64) {
    let jdn = (jd + 0.5).floor() as i64;
    let day_fraction = jd + 0.5 - jdn as f64;

    let (year, month, day) = jdn_to_gregorian(jdn);
    let (hour, minute, second) = day_fraction_to_time(day_fraction);

    (year, month, day, hour, minute, second)
}

// ---------------------------------------------------------------------------
// Julian centuries
// ---------------------------------------------------------------------------

/// Julian centuries elapsed since J2000.0 for a given Julian Date.
///
/// This is the fundamental time argument `T` used in most astronomical formulae.
///
/// # Examples
///
/// ```
/// # use jyotish::calendar::{julian_centuries, J2000_0};
/// assert!((julian_centuries(J2000_0)).abs() < 1e-15);
/// assert!((julian_centuries(J2000_0 + 36525.0) - 1.0).abs() < 1e-15);
/// ```
pub fn julian_centuries(jd: f64) -> f64 {
    (jd - J2000_0) / DAYS_PER_JULIAN_CENTURY
}

// ---------------------------------------------------------------------------
// Sidereal time
// ---------------------------------------------------------------------------

/// Greenwich Mean Sidereal Time in degrees for a given Julian Date.
///
/// Uses the IAU 1982 formula (Meeus, *Astronomical Algorithms*, eq. 12.4).
/// The result is normalized to the range \[0, 360).
///
/// # Examples
///
/// ```
/// # use jyotish::calendar::gmst_degrees;
/// // Meeus example 12.a: 1987-04-10 at 0h UT → GMST ≈ 197.693°
/// let jd = 2_446_895.5;
/// let gmst = gmst_degrees(jd);
/// assert!((gmst - 197.693).abs() < 0.01, "got {gmst}");
/// ```
pub fn gmst_degrees(jd: f64) -> f64 {
    let t = julian_centuries(jd);

    // IAU 1982 formula (Meeus eq. 12.4)
    let gmst = 280.460_618_37 + 360.985_647_366_29 * (jd - J2000_0) + 0.000_387_933 * t * t
        - t * t * t / 38_710_000.0;

    normalize_degrees(gmst)
}

/// Greenwich Mean Sidereal Time in hours for a given Julian Date.
///
/// # Examples
///
/// ```
/// # use jyotish::calendar::gmst_hours;
/// let jd = 2_446_895.5;
/// let hours = gmst_hours(jd);
/// assert!((hours - 13.1795).abs() < 0.001, "got {hours}");
/// ```
pub fn gmst_hours(jd: f64) -> f64 {
    gmst_degrees(jd) / 15.0
}

/// Local Sidereal Time in degrees for a given Julian Date and observer longitude.
///
/// `longitude_deg` is positive east, negative west.
///
/// # Examples
///
/// ```
/// # use jyotish::calendar::local_sidereal_time;
/// let jd = 2_446_895.5;
/// let lst = local_sidereal_time(jd, -71.0583); // Boston
/// let expected = (197.693 - 71.0583 + 360.0) % 360.0;
/// assert!((lst - expected).abs() < 0.01, "got {lst}");
/// ```
pub fn local_sidereal_time(jd: f64, longitude_deg: f64) -> f64 {
    normalize_degrees(gmst_degrees(jd) + longitude_deg)
}

// ---------------------------------------------------------------------------
// Leap year
// ---------------------------------------------------------------------------

/// Returns `true` if the given year is a leap year in the Gregorian calendar.
///
/// # Examples
///
/// ```
/// # use jyotish::calendar::is_gregorian_leap_year;
/// assert!(is_gregorian_leap_year(2000));
/// assert!(!is_gregorian_leap_year(1900));
/// assert!(is_gregorian_leap_year(2024));
/// assert!(!is_gregorian_leap_year(2023));
/// ```
pub fn is_gregorian_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// Returns `true` if the given year is a leap year in the Julian calendar.
///
/// # Examples
///
/// ```
/// # use jyotish::calendar::is_julian_leap_year;
/// assert!(is_julian_leap_year(4));
/// assert!(!is_julian_leap_year(3));
/// ```
pub fn is_julian_leap_year(year: i32) -> bool {
    year % 4 == 0
}

// ---------------------------------------------------------------------------
// Day of week
// ---------------------------------------------------------------------------

/// Day of the week for a given JDN.
///
/// Returns 0 = Monday, 1 = Tuesday, ..., 6 = Sunday.
///
/// # Examples
///
/// ```
/// # use jyotish::calendar::{day_of_week, gregorian_to_jdn};
/// // 2000-01-01 was a Saturday (5)
/// assert_eq!(day_of_week(gregorian_to_jdn(2000, 1, 1).unwrap()), 5);
/// ```
pub fn day_of_week(jdn: i64) -> u32 {
    (jdn.rem_euclid(7)) as u32
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Convert h/m/s to fractional day.
fn time_to_day_fraction(hour: u32, minute: u32, second: f64) -> f64 {
    (hour as f64 + minute as f64 / 60.0 + second / 3600.0) / 24.0
}

/// Convert fractional day to h/m/s.
fn day_fraction_to_time(frac: f64) -> (u32, u32, f64) {
    let total_seconds = frac * 86_400.0;
    let hour = (total_seconds / 3600.0) as u32;
    let minute = ((total_seconds - hour as f64 * 3600.0) / 60.0) as u32;
    let second = total_seconds - hour as f64 * 3600.0 - minute as f64 * 60.0;
    (hour, minute, second)
}

fn validate_date_parts(year: i32, month: u32, day: u32) -> Result<()> {
    if !(1..=12).contains(&month) {
        return Err(JyotishError::InvalidParameter(format!(
            "month {month} not in 1..=12"
        )));
    }
    let max_day = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_gregorian_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => unreachable!(),
    };
    if !(1..=max_day).contains(&day) {
        return Err(JyotishError::InvalidParameter(format!(
            "day {day} not valid for month {month} (max {max_day})"
        )));
    }
    Ok(())
}

fn validate_time_parts(hour: u32, minute: u32, second: f64) -> Result<()> {
    if hour > 23 {
        return Err(JyotishError::InvalidParameter(format!(
            "hour {hour} not in 0..=23"
        )));
    }
    if minute > 59 {
        return Err(JyotishError::InvalidParameter(format!(
            "minute {minute} not in 0..=59"
        )));
    }
    if !(0.0..60.0).contains(&second) {
        return Err(JyotishError::InvalidParameter(format!(
            "second {second} not in 0.0..60.0"
        )));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- Gregorian ↔ JDN ---

    #[test]
    fn gregorian_jdn_j2000() {
        assert_eq!(gregorian_to_jdn(2000, 1, 1).unwrap(), 2_451_545);
    }

    #[test]
    fn gregorian_jdn_unix_epoch() {
        assert_eq!(gregorian_to_jdn(1970, 1, 1).unwrap(), 2_440_588);
    }

    #[test]
    fn gregorian_jdn_reform_date() {
        // First day of Gregorian calendar
        assert_eq!(
            gregorian_to_jdn(1582, 10, 15).unwrap(),
            GREGORIAN_REFORM_JDN
        );
    }

    #[test]
    fn jdn_to_gregorian_roundtrip() {
        let dates = [
            (2000, 1, 1),
            (1970, 1, 1),
            (1582, 10, 15),
            (2024, 2, 29),
            (-4713, 11, 24), // JDN 0 in proleptic Gregorian
        ];
        for (y, m, d) in dates {
            let jdn = gregorian_to_jdn(y, m, d).unwrap();
            assert_eq!(
                jdn_to_gregorian(jdn),
                (y, m, d),
                "roundtrip failed for {y}-{m}-{d}"
            );
        }
    }

    // --- Julian ↔ JDN ---

    #[test]
    fn julian_jdn_last_julian_day() {
        assert_eq!(julian_to_jdn(1582, 10, 4).unwrap(), 2_299_160);
    }

    #[test]
    fn jdn_to_julian_roundtrip() {
        let dates = [(1582, 10, 4), (100, 3, 1), (0, 1, 1)];
        for (y, m, d) in dates {
            let jdn = julian_to_jdn(y, m, d).unwrap();
            assert_eq!(
                jdn_to_julian(jdn),
                (y, m, d),
                "roundtrip failed for {y}-{m}-{d}"
            );
        }
    }

    // --- Unix ↔ JD ---

    #[test]
    fn unix_jd_epoch() {
        assert!((unix_to_jd(0) - JD_UNIX_EPOCH).abs() < 1e-10);
    }

    #[test]
    fn jd_unix_roundtrip() {
        let ts = 1_711_324_800_i64; // 2024-03-25T00:00:00 UTC
        let jd = unix_to_jd(ts);
        assert_eq!(jd_to_unix(jd), ts);
    }

    // --- Gregorian ↔ JD (with time) ---

    #[test]
    fn gregorian_jd_j2000_noon() {
        let jd = gregorian_to_jd(2000, 1, 1, 12, 0, 0.0).unwrap();
        assert!((jd - J2000_0).abs() < 1e-10);
    }

    #[test]
    fn gregorian_jd_midnight() {
        let jd = gregorian_to_jd(2000, 1, 1, 0, 0, 0.0).unwrap();
        assert!((jd - 2_451_544.5).abs() < 1e-10);
    }

    #[test]
    fn jd_to_gregorian_roundtrip_with_time() {
        let (y, m, d, h, min, s) = jd_to_gregorian(2_451_545.0);
        assert_eq!((y, m, d, h, min), (2000, 1, 1, 12, 0));
        assert!(s.abs() < 1e-6);
    }

    // --- Julian centuries ---

    #[test]
    fn julian_centuries_at_j2000() {
        assert!(julian_centuries(J2000_0).abs() < 1e-15);
    }

    #[test]
    fn julian_centuries_one_century() {
        let t = julian_centuries(J2000_0 + DAYS_PER_JULIAN_CENTURY);
        assert!((t - 1.0).abs() < 1e-15);
    }

    // --- Sidereal time ---

    #[test]
    fn gmst_meeus_example_12a() {
        // Meeus example 12.a: 1987-04-10 at 0h UT
        let jd = 2_446_895.5;
        let gmst = gmst_degrees(jd);
        assert!((gmst - 197.693).abs() < 0.01, "got {gmst}");
    }

    #[test]
    fn gmst_hours_meeus() {
        let jd = 2_446_895.5;
        let hours = gmst_hours(jd);
        // 197.693° / 15 ≈ 13.1795h
        assert!((hours - 13.1795).abs() < 0.001, "got {hours}");
    }

    #[test]
    fn lst_with_longitude() {
        let jd = 2_446_895.5;
        let gmst = gmst_degrees(jd);
        let lst = local_sidereal_time(jd, -71.0583);
        let expected = normalize_degrees(gmst - 71.0583);
        assert!((lst - expected).abs() < 1e-10);
    }

    // --- Leap year ---

    #[test]
    fn gregorian_leap_years() {
        assert!(is_gregorian_leap_year(2000));
        assert!(is_gregorian_leap_year(2024));
        assert!(is_gregorian_leap_year(1600));
        assert!(!is_gregorian_leap_year(1900));
        assert!(!is_gregorian_leap_year(2100));
        assert!(!is_gregorian_leap_year(2023));
    }

    #[test]
    fn julian_leap_years() {
        assert!(is_julian_leap_year(4));
        assert!(is_julian_leap_year(100));
        assert!(!is_julian_leap_year(3));
    }

    // --- Day of week ---

    #[test]
    fn day_of_week_known_dates() {
        // 2000-01-01 was Saturday (5)
        assert_eq!(day_of_week(gregorian_to_jdn(2000, 1, 1).unwrap()), 5);
        // 2024-01-01 was Monday (0)
        assert_eq!(day_of_week(gregorian_to_jdn(2024, 1, 1).unwrap()), 0);
    }

    // --- Validation ---

    #[test]
    fn invalid_month() {
        assert!(gregorian_to_jdn(2000, 0, 1).is_err());
        assert!(gregorian_to_jdn(2000, 13, 1).is_err());
    }

    #[test]
    fn invalid_day() {
        assert!(gregorian_to_jdn(2000, 1, 0).is_err());
        assert!(gregorian_to_jdn(2000, 1, 32).is_err());
        // Month-specific day limits
        assert!(gregorian_to_jdn(2000, 2, 30).is_err()); // Feb in leap year max 29
        assert!(gregorian_to_jdn(2001, 2, 29).is_err()); // Feb in non-leap year max 28
        assert!(gregorian_to_jdn(2000, 4, 31).is_err()); // Apr max 30
        // Valid edge cases
        assert!(gregorian_to_jdn(2000, 2, 29).is_ok()); // Leap year Feb 29
        assert!(gregorian_to_jdn(2001, 2, 28).is_ok()); // Non-leap Feb 28
    }

    #[test]
    fn invalid_time() {
        assert!(gregorian_to_jd(2000, 1, 1, 24, 0, 0.0).is_err());
        assert!(gregorian_to_jd(2000, 1, 1, 0, 60, 0.0).is_err());
        assert!(gregorian_to_jd(2000, 1, 1, 0, 0, 60.0).is_err());
    }
}
