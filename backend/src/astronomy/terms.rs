// Solar term crossing time finder.
// Uses bisection on solar apparent longitude from the Meeus solar model.

use crate::astronomy::sun::solar_position;
use crate::astronomy::time::{JulianDate, gregorian_to_jd, jd_to_gregorian, normalize_deg};

pub const TERM_NAMES: [&str; 24] = [
    "小寒", "大寒", "立春", "雨水", "驚蟄", "春分", "清明", "穀雨", "立夏", "小滿", "芒種", "夏至",
    "小暑", "大暑", "立秋", "處暑", "白露", "秋分", "寒露", "霜降", "立冬", "小雪", "大雪", "冬至",
];

#[derive(Clone, Debug)]
pub struct SolarTermEntry {
    pub year: i32,
    pub term_index: u8,
    pub term_name: String,
    pub gregorian_utc: String,
    pub jd_tt: f64,
}

pub fn solar_terms_for_year(year: i32) -> Vec<SolarTermEntry> {
    let mut entries = Vec::with_capacity(24);
    let jd_year_start = gregorian_to_jd(year, 1, 1, 0.0).whole();

    for term_index in 0..24 {
        let target_lon = (term_index as f64 * 15.0 + 285.0) % 360.0;

        // Approximate crossing: Jan 5 + term_index * 15.22 days
        let approx_jd = jd_year_start + 4.5 + term_index as f64 * 15.22;

        // Search ±8 days around the approximate date
        if let Some(jd) = find_longitude_crossing(target_lon, approx_jd - 8.0, approx_jd + 8.0) {
            let (y, m, d, h) = jd_to_gregorian(JulianDate::from_jd(jd));
            let hh = h as u8;
            let mm = ((h - hh as f64) * 60.0) as u8;
            let greg = format!("{:04}-{:02}-{:02}T{:02}:{:02}:00Z", y, m, d, hh, mm);
            entries.push(SolarTermEntry {
                year: y,
                term_index: term_index as u8,
                term_name: TERM_NAMES[term_index].to_string(),
                gregorian_utc: greg,
                jd_tt: jd,
            });
        }
    }

    entries
}

fn find_longitude_crossing(target_deg: f64, jd_lo: f64, jd_hi: f64) -> Option<f64> {
    let mut lo = jd_lo;
    let mut hi = jd_hi;

    let mut flo = lon_diff(target_deg, lo);
    let mut fhi = lon_diff(target_deg, hi);

    // Ensure crossing: signs should differ
    if flo * fhi > 0.0 {
        // Try expanding window
        for &expand in &[3.0, -3.0, 8.0, -8.0, 15.0, -15.0] {
            let te = if expand > 0.0 {
                hi + expand
            } else {
                lo + expand
            };
            let fe = lon_diff(target_deg, te);
            if flo * fe <= 0.0 {
                hi = te;
                fhi = fe;
                break;
            }
            if fe * fhi <= 0.0 {
                lo = te;
                flo = fe;
                break;
            }
        }
        if flo * fhi > 0.0 {
            return None;
        }
    }

    // Bisection
    for _ in 0..60 {
        let mid = (lo + hi) / 2.0;
        if (hi - lo).abs() < 1e-8 {
            return Some(mid);
        }
        let fmid = lon_diff(target_deg, mid);
        if fmid.abs() < 1e-7 {
            return Some(mid);
        }
        if flo * fmid < 0.0 {
            hi = mid;
            let _ = fmid; // hi side value, symmetry with fhi init
        } else {
            lo = mid;
            flo = fmid;
        }
    }
    Some((lo + hi) / 2.0)
}

fn lon_diff(target_deg: f64, jd: f64) -> f64 {
    let pos = solar_position(JulianDate::from_jd(jd));
    normalize_deg(pos.apparent_longitude_deg - target_deg + 540.0) - 180.0
}

pub fn generate_all_solar_terms() -> Vec<SolarTermEntry> {
    let mut all = Vec::with_capacity(200 * 24);
    for year in 1901..=2100 {
        all.extend(solar_terms_for_year(year));
    }
    all
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solar_terms_2025_count() {
        let terms = solar_terms_for_year(2025);
        assert_eq!(terms.len(), 24);
    }

    #[test]
    fn solar_terms_lichun_2025() {
        let terms = solar_terms_for_year(2025);
        let lichun = terms.iter().find(|t| t.term_name == "立春").unwrap();
        assert!(
            lichun.gregorian_utc.starts_with("2025-02-0"),
            "立春 2025: {}",
            lichun.gregorian_utc
        );
    }

    #[test]
    fn solar_terms_monotonic() {
        let terms = solar_terms_for_year(2025);
        for i in 1..terms.len() {
            assert!(terms[i].jd_tt > terms[i - 1].jd_tt);
        }
    }

    #[test]
    fn each_term_has_unique_name() {
        let terms = solar_terms_for_year(2000);
        assert_eq!(terms.len(), 24);
        for i in 0..24 {
            assert_eq!(terms[i].term_name, TERM_NAMES[i]);
        }
    }
}
