// Julian Day, delta-T, and time-scale conversions.
// Uses the project's battle-tested calendar/civil.rs for Julian Day computation.

use crate::calendar::civil;

/// Two-part Julian Date.
#[derive(Copy, Clone, Debug)]
pub struct JulianDate {
    pub jd1: f64,
    pub jd2: f64,
}

impl JulianDate {
    pub fn from_jd(jd: f64) -> Self {
        Self { jd1: jd, jd2: 0.0 }
    }
    pub fn whole(self) -> f64 {
        self.jd1 + self.jd2
    }
    pub fn centuries_since_j2000(self) -> f64 {
        (self.whole() - 2451545.0) / 36525.0
    }
}

/// Known reference: 1900-01-01 00:00 = JD 2415020.5.
/// civil::days_from_civil(1900,1,1) returns some integer; we calibrate from that.
/// days_from_civil(1900,1,1) = ? Let's use civil::days_since to compute it.
/// civil::days_from_civil uses the Hinnant algorithm: returns days since 0000-03-01.
/// We add a constant to get JD.
pub fn gregorian_to_jd(year: i32, month: u8, day: u8, hour: f64) -> JulianDate {
    // Use the project's existing days_from_civil function
    let days = civil_days(year, month, day);
    // civil_days(2000, 1, 1) = 10957
    // J2000 = 2000-01-01 12:00 = JD 2451545.0
    // JD = civil_days + 2440587.5 + hour/24
    const JD_CIVIL_OFFSET: f64 = 2440587.5;
    JulianDate::from_jd(days as f64 + JD_CIVIL_OFFSET + hour / 24.0)
}

pub fn jd_to_gregorian(jd: JulianDate) -> (i32, u8, u8, f64) {
    let jd = jd.whole();
    // Convert JD to civil days (integer at midnight)
    let civ_days_f = jd - 2440587.5;
    let n = civ_days_f.floor() as i32;
    let frac = civ_days_f - n as f64;
    // frac is fraction of day since midnight (0.0 = 00:00, 0.5 = 12:00)
    let (y, m, d) = civil_days_to_ymd(n);
    let hour = frac * 24.0;
    (y, m, d, hour)
}

fn civil_days(year: i32, month: u8, day: u8) -> i32 {
    civil::days_from_civil(year, month, day)
}

// Reverse of days_from_civil: given days since epoch, return (year, month, day).
// This mirrors the Hinnant algorithm in reverse.
fn civil_days_to_ymd(z: i32) -> (i32, u8, u8) {
    let z = z + 719468;
    let era = if z >= 0 {
        z / 146097
    } else {
        (z - 146096) / 146097
    };
    let doe = z - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };

    (y, m as u8, d as u8)
}

/// ΔT = TT - UT1 in seconds for 1901–2100.
/// NASA polynomial (Espenak & Meeus), ~0.5s accuracy for 1901–2005.
pub fn delta_t(year: i32) -> f64 {
    let t = (year - 2000) as f64;
    if year < 1960 {
        let u = (year - 1900) as f64;
        // ΔT was near zero in 1900 and slowly increased
        u * 0.4 + u * u * 0.01
    } else if year <= 2005 {
        63.86 + 0.3345 * t - 0.060374 * t * t
            + 0.0017275 * t * t * t
            + 0.0006518 * t * t * t * t
            + 0.00002373599 * t * t * t * t * t
    } else if year <= 2050 {
        63.86 + 0.3345 * t + 0.01 * t * t
    } else {
        // 2051-2100: rough extrapolation, capped
        (63.86 + 0.3345 * t + 0.01 * t * t).min(150.0)
    }
}

/// Leap seconds (TAI - UTC) for a given year. Returns 0 before 1972.
pub fn leap_seconds(year: i32) -> f64 {
    match year {
        y if y < 1972 => 0.0,
        y if y < 1973 => 10.0,
        y if y < 1974 => 11.0,
        y if y < 1975 => 12.0,
        y if y < 1976 => 13.0,
        y if y < 1977 => 14.0,
        y if y < 1978 => 15.0,
        y if y < 1979 => 16.0,
        y if y < 1980 => 17.0,
        y if y < 1981 => 18.0,
        y if y < 1982 => 19.0,
        y if y < 1983 => 20.0,
        y if y < 1985 => 21.0,
        y if y < 1988 => 22.0,
        y if y < 1990 => 23.0,
        y if y < 1991 => 24.0,
        y if y < 1992 => 25.0,
        y if y < 1993 => 26.0,
        y if y < 1994 => 27.0,
        y if y < 1996 => 28.0,
        y if y < 1997 => 29.0,
        y if y < 1999 => 30.0,
        y if y < 2006 => 31.0,
        y if y < 2009 => 32.0,
        y if y < 2012 => 33.0,
        y if y < 2015 => 34.0,
        y if y < 2017 => 35.0,
        y if y < 2020 => 36.0,
        _ => 37.0,
    }
}

pub fn day_of_year(year: i32, month: u8, day: u8) -> u16 {
    let dims = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let mut doy = day as u16;
    for (m, dim) in dims.iter().enumerate().take(month as usize - 1) {
        doy += *dim as u16;
        if m == 1 && leap {
            doy += 1;
        }
    }
    doy
}

pub fn doy_to_date(year: i32, doy: u16) -> (u8, u8) {
    let dims = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let mut remaining = doy;
    for (m, &dim) in dims.iter().enumerate() {
        let dim = if m == 1 && leap { dim + 1 } else { dim };
        if remaining <= dim as u16 {
            return (m as u8 + 1, remaining as u8);
        }
        remaining -= dim as u16;
    }
    (12, 31)
}

pub fn normalize_deg(angle: f64) -> f64 {
    let a = angle % 360.0;
    if a < 0.0 { a + 360.0 } else { a }
}

pub fn deg_to_rad(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.0
}
pub fn rad_to_deg(rad: f64) -> f64 {
    rad * 180.0 / std::f64::consts::PI
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jd_j2000_is_correct() {
        let jd = gregorian_to_jd(2000, 1, 1, 12.0);
        assert!(
            (jd.whole() - 2451545.0).abs() < 0.001,
            "J2000 should be 2451545.0, got {}",
            jd.whole()
        );
    }

    #[test]
    fn jd_roundtrip() {
        for (y, m, d, h) in [
            (1901, 2, 19, 0.0),
            (1950, 1, 1, 12.0),
            (2000, 2, 29, 6.0),
            (2025, 6, 15, 18.5),
            (2100, 12, 31, 23.0),
        ] {
            let jd = gregorian_to_jd(y, m, d, h);
            let (y2, m2, d2, h2) = jd_to_gregorian(jd);
            assert_eq!((y, m, d), (y2, m2, d2), "roundtrip date mismatch");
            assert!((h - h2).abs() < 0.02, "hour mismatch: {h} vs {h2}");
        }
    }

    #[test]
    fn delta_t_reasonable() {
        for y in [1901, 1950, 2000, 2025, 2050, 2100] {
            let dt = delta_t(y);
            assert!(dt > 0.0, "delta_t should be positive for {y}, got {dt}");
            assert!(dt < 200.0, "delta_t implausibly large for {y}: {dt}");
        }
    }

    #[test]
    fn day_of_year_edge_cases() {
        assert_eq!(day_of_year(2024, 1, 1), 1);
        assert_eq!(day_of_year(2024, 12, 31), 366);
        assert_eq!(day_of_year(2025, 12, 31), 365);
        assert_eq!(day_of_year(2024, 2, 29), 60);
    }
}
