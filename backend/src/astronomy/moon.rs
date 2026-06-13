// Simplified lunar theory for new moon time finding.
// Uses Meeus Chapter 47 periodic terms (~60 terms) for ~10 arcminute accuracy.

use crate::astronomy::sun::solar_position;
use crate::astronomy::time::{JulianDate, deg_to_rad, jd_to_gregorian, normalize_deg};

/// Lunar position result.
#[derive(Copy, Clone, Debug)]
pub struct LunarPosition {
    /// Geocentric ecliptic longitude in degrees [0, 360).
    pub longitude_deg: f64,
    /// Geocentric ecliptic latitude in degrees.
    pub latitude_deg: f64,
    /// Distance from Earth to Moon in Earth radii.
    pub distance_earth_radii: f64,
}

/// Compute the Moon's geocentric position at a given Julian Date (TT).
/// Uses simplified periodic terms from Meeus Chapter 47.
pub fn lunar_position(jd: JulianDate) -> LunarPosition {
    let t = jd.centuries_since_j2000();

    // Fundamental arguments (Meeus Chapter 47)
    // Mean longitude of the Moon
    let l_p = normalize_deg(
        218.3164591 + 481267.88134236 * t - 0.0013268 * t * t + t * t * t / 538841.0
            - t * t * t * t / 65194000.0,
    );
    // Mean elongation of the Moon from the Sun
    let d = normalize_deg(
        297.8502042 + 445267.1115168 * t - 0.0016300 * t * t + t * t * t / 545868.0
            - t * t * t * t / 113065000.0,
    );
    // Sun's mean anomaly
    let m =
        normalize_deg(357.5291092 + 35999.0502909 * t - 0.0001536 * t * t + t * t * t / 24490000.0);
    // Moon's mean anomaly
    let m_p = normalize_deg(
        134.9634114 + 477198.8676313 * t + 0.0089970 * t * t + t * t * t / 69699.0
            - t * t * t * t / 14712000.0,
    );
    // Moon's argument of latitude
    let f = normalize_deg(
        93.2720993 + 483202.0175273 * t - 0.0034029 * t * t - t * t * t / 3526000.0
            + t * t * t * t / 863310000.0,
    );

    let l_p = deg_to_rad(l_p);
    let d = deg_to_rad(d);
    let m = deg_to_rad(m);
    let m_p = deg_to_rad(m_p);
    let f = deg_to_rad(f);

    // Periodic terms for lunar longitude (dominant ~50 terms from Meeus Table 47.A)
    // Each term: (coefficient_km, D_mult, M_mult, M'_mult, F_mult)
    let lon_terms: &[(f64, f64, f64, f64, f64)] = &[
        // Largest terms (amplitude > 1000")
        (6288774.0, 0.0, 0.0, 1.0, 0.0),
        (1274027.0, 2.0, 0.0, -1.0, 0.0),
        (658314.0, 2.0, 0.0, 0.0, 0.0),
        (213618.0, 0.0, 0.0, 2.0, 0.0),
        (-185116.0, 0.0, 1.0, 0.0, 0.0),
        (-114332.0, 2.0, 0.0, -2.0, 0.0),
        (58793.0, 2.0, -1.0, -1.0, 0.0),
        (57066.0, 2.0, 0.0, 1.0, 0.0),
        (53322.0, 2.0, -1.0, 0.0, 0.0),
        (45758.0, 0.0, 1.0, -1.0, 0.0),
        (-40923.0, 0.0, 1.0, 1.0, 0.0),
        (-34720.0, 0.0, 0.0, 3.0, 0.0),
        (-30383.0, 2.0, 1.0, -1.0, 0.0),
        (15327.0, 2.0, 0.0, -3.0, 0.0),
        (-12528.0, 0.0, 0.0, 2.0, -2.0),
        (10980.0, 2.0, 0.0, 2.0, 0.0),
        (10675.0, 0.0, 0.0, 4.0, 0.0),
        (10034.0, 2.0, 0.0, -4.0, 0.0),
        (8547.0, 4.0, 0.0, -1.0, 0.0),
        (-7888.0, 2.0, 1.0, 0.0, 0.0),
        (-6766.0, 2.0, -1.0, -2.0, 0.0),
        (-5163.0, 0.0, 2.0, 0.0, 0.0),
        (4987.0, 4.0, 0.0, -2.0, 0.0),
        (4036.0, 2.0, -2.0, -1.0, 0.0),
        (3994.0, 2.0, 1.0, -2.0, 0.0),
        (3861.0, 0.0, 1.0, -2.0, 0.0),
        (3665.0, 2.0, -2.0, 0.0, 0.0),
        (-2689.0, 2.0, 0.0, 3.0, 0.0),
        (-2602.0, 2.0, 0.0, -5.0, 0.0),
        (2390.0, 0.0, 1.0, -3.0, 0.0),
        (-2348.0, 2.0, -1.0, 1.0, 0.0),
        (2236.0, 0.0, 2.0, -1.0, 0.0),
        (-2120.0, 2.0, -2.0, 1.0, 0.0),
        (-2069.0, 2.0, 1.0, -3.0, 0.0),
        (2048.0, 2.0, -2.0, -2.0, 0.0),
        (-1773.0, 0.0, 1.0, 2.0, 0.0),
        (-1595.0, 4.0, 0.0, 0.0, 0.0),
        (1215.0, 2.0, -1.0, -3.0, 0.0),
        (-1110.0, 0.0, 2.0, 1.0, 0.0),
        (-892.0, 1.0, 0.0, 0.0, 0.0),
        (-810.0, 0.0, 1.0, -4.0, 0.0),
        (759.0, 3.0, 0.0, 0.0, 0.0),
        (-713.0, 4.0, -1.0, -1.0, 0.0),
        (-700.0, 0.0, 2.0, -2.0, 0.0),
    ];

    let mut sigma_l: f64 = 0.0;
    for &(coeff, d_mul, m_mul, mp_mul, f_mul) in lon_terms {
        let arg = d_mul * d + m_mul * m + mp_mul * m_p + f_mul * f;
        sigma_l += coeff * arg.sin();
    }

    // Convert from 0.0001 arcseconds to degrees
    let lon = normalize_deg(l_p.to_degrees() + sigma_l * 1e-7 / 3600.0);

    // Simplified latitude (dominant terms)
    let lat_terms: &[(f64, f64, f64, f64, f64)] = &[
        (5128122.0, 0.0, 0.0, 0.0, 1.0),
        (280602.0, 0.0, 0.0, 1.0, 1.0),
        (277693.0, 0.0, 0.0, 1.0, -1.0),
        (173237.0, 2.0, 0.0, 0.0, -1.0),
        (55413.0, 2.0, 0.0, -1.0, 1.0),
        (46271.0, 2.0, 0.0, -1.0, -1.0),
        (32573.0, 2.0, 0.0, 0.0, 1.0),
        (17198.0, 0.0, 0.0, 2.0, 1.0),
        (9266.0, 2.0, 0.0, 1.0, -1.0),
        (8822.0, 0.0, 0.0, 2.0, -1.0),
    ];

    let mut sigma_b: f64 = 0.0;
    for &(coeff, d_mul, m_mul, mp_mul, f_mul) in lat_terms {
        let arg = d_mul * d + m_mul * m + mp_mul * m_p + f_mul * f;
        sigma_b += coeff * arg.sin();
    }
    let lat = sigma_b * 1e-7 / 3600.0; // degrees

    // Simplified distance (dominant terms, in km → Earth radii)
    let dist_terms: &[(f64, f64, f64, f64, f64)] = &[
        (-20905355.0, 0.0, 0.0, 1.0, 0.0),
        (-3699111.0, 2.0, 0.0, -1.0, 0.0),
        (-2955968.0, 2.0, 0.0, 0.0, 0.0),
        (-569925.0, 0.0, 0.0, 2.0, 0.0),
        (48888.0, 0.0, 1.0, 0.0, 0.0),
        (-3149.0, 2.0, 0.0, -2.0, 0.0),
        (246158.0, 2.0, -1.0, -1.0, 0.0),
        (-152138.0, 2.0, 0.0, 1.0, 0.0),
        (-170733.0, 2.0, -1.0, 0.0, 0.0),
        (-204586.0, 0.0, 1.0, -1.0, 0.0),
    ];

    let mut sigma_r: f64 = 0.0;
    for &(coeff, d_mul, m_mul, mp_mul, f_mul) in dist_terms {
        let arg = d_mul * d + m_mul * m + mp_mul * m_p + f_mul * f;
        sigma_r += coeff * arg.cos();
    }
    // 385000.56 km mean distance, Earth radius ~6378.137 km
    // sigma_r is in 0.001 km
    let dist_km = 385000.56 + sigma_r * 0.001;
    let dist_earth_radii = dist_km / 6378.137;

    LunarPosition {
        longitude_deg: lon,
        latitude_deg: lat,
        distance_earth_radii: dist_earth_radii,
    }
}

/// Find the new moon (Sun-Moon conjunction) times for a given year.
/// Uses known epoch (2000-01-06 18:14 UT ≈ JD 2451549.26) and mean synodic month.
pub fn new_moons_for_year(year: i32) -> Vec<(f64, String)> {
    let mut results = Vec::new();
    let synodic = 29.530588;
    // Known new moon: 2000-01-06 ~18:14 UT
    let epoch_nm = crate::astronomy::time::gregorian_to_jd(2000, 1, 6, 18.23).whole();

    // Find which new moon number corresponds to Jan 1 of target year
    let jan1_jd = crate::astronomy::time::gregorian_to_jd(year, 1, 1, 0.0).whole();
    let n_approx = ((jan1_jd - epoch_nm) / synodic).round() as i64;

    // Search ±2 synodic months around the approximate
    for n in (n_approx - 2)..=(n_approx + 15) {
        let approx_jd = epoch_nm + n as f64 * synodic;
        // Search ±2 days around the approximate time
        if let Some(jd) = find_conjunction(approx_jd - 2.0, approx_jd + 2.0)
            && jd >= jan1_jd
            && jd < jan1_jd + 366.0
        {
            let (y, m, d, h) = jd_to_gregorian(JulianDate::from_jd(jd));
            if y == year {
                let hh = h as u8;
                let mm = ((h - hh as f64) * 60.0) as u8;
                results.push((
                    jd,
                    format!("{:04}-{:02}-{:02}T{:02}:{:02}:00Z", y, m, d, hh, mm),
                ));
            }
        }
    }
    results
}

/// Find the moment of Sun-Moon conjunction (longitude difference crosses zero).
fn find_conjunction(jd_lo: f64, jd_hi: f64) -> Option<f64> {
    let diff_lo = lon_diff(jd_lo);
    let diff_hi = lon_diff(jd_hi);

    if diff_lo * diff_hi > 0.0 {
        // Expand window aggressively to catch crossings
        for &expand in &[2.0, -2.0, 4.0, -4.0, 8.0, -8.0, 15.0, -15.0] {
            let te = if expand > 0.0 {
                jd_hi + expand
            } else {
                jd_lo + expand
            };
            let fe = lon_diff(te);
            if diff_lo * fe <= 0.0 {
                return find_conjunction_bounded(jd_lo, te, diff_lo, fe);
            }
            if fe * diff_hi <= 0.0 {
                return find_conjunction_bounded(te, jd_hi, fe, diff_hi);
            }
        }
        return None;
    }

    find_conjunction_bounded(jd_lo, jd_hi, diff_lo, diff_hi)
}

fn lon_diff(jd: f64) -> f64 {
    let moon = lunar_position(JulianDate::from_jd(jd));
    let sun = solar_position(JulianDate::from_jd(jd));
    normalize_deg(moon.longitude_deg - sun.longitude_deg + 540.0) - 180.0
}

fn find_conjunction_bounded(lo: f64, hi: f64, diff_lo: f64, _diff_hi: f64) -> Option<f64> {
    // Secant/bisection method
    let mut x_lo = lo;
    let mut x_hi = hi;
    let mut f_lo = diff_lo;

    for _ in 0..30 {
        let x_mid = (x_lo + x_hi) / 2.0;
        let f_mid = lon_diff(x_mid);

        if f_mid.abs() < 0.001 {
            return Some(x_mid);
        }

        if f_lo * f_mid < 0.0 {
            x_hi = x_mid;
        } else {
            x_lo = x_mid;
            f_lo = f_mid;
        }

        if (x_hi - x_lo).abs() < 1e-7 {
            return Some((x_lo + x_hi) / 2.0);
        }
    }

    Some((x_lo + x_hi) / 2.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_moons_2025_count() {
        let moons = new_moons_for_year(2025);
        // Should have 12 new moons in 2025
        assert!(
            moons.len() >= 11 && moons.len() <= 13,
            "expected 11-13 new moons in 2025, got {}",
            moons.len()
        );
    }

    #[test]
    fn new_moons_monotonic() {
        let moons = new_moons_for_year(2025);
        for i in 1..moons.len() {
            assert!(
                moons[i].0 > moons[i - 1].0,
                "new moons should be monotonic in time"
            );
        }
    }

    #[test]
    fn lunar_longitude_range() {
        for day in (0..30).step_by(5) {
            let jd = JulianDate::from_jd(2451545.0 + day as f64 * 29.5);
            let pos = lunar_position(jd);
            assert!(pos.longitude_deg >= 0.0 && pos.longitude_deg < 360.0);
        }
    }
}
