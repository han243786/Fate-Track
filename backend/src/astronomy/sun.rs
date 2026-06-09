// Solar position computation using Meeus (1998) standard solar theory.
// Provides geocentric ecliptic longitude to <1 arcminute accuracy.
// Simpler and more robust than truncated VSOP87 for our purposes.

use crate::astronomy::time::{JulianDate, deg_to_rad, normalize_deg};

#[derive(Copy, Clone, Debug)]
pub struct SolarPosition {
    pub longitude_deg: f64,
    pub latitude_deg: f64,
    pub distance_au: f64,
    pub apparent_longitude_deg: f64,
}

/// Compute the Sun's geocentric position at a given Julian Date (TT).
pub fn solar_position(jd: JulianDate) -> SolarPosition {
    let t = jd.centuries_since_j2000();

    // Mean solar longitude (Meeus eq 25.2), degrees
    let l0 = normalize_deg(280.46646 + 36000.76983 * t + 0.0003032 * t * t);

    // Mean anomaly of the Sun (Meeus eq 25.3), degrees
    let m = normalize_deg(357.52911 + 35999.05029 * t - 0.0001537 * t * t);

    // Eccentricity of Earth's orbit
    let e = 0.016708634 - 0.000042037 * t - 0.0000001267 * t * t;

    // Equation of center (Meeus eq 25.4)
    let m_rad = deg_to_rad(m);
    let c = (1.914602 - 0.004817 * t - 0.000014 * t * t) * m_rad.sin()
        + (0.019993 - 0.000101 * t) * (2.0 * m_rad).sin()
        + 0.000289 * (3.0 * m_rad).sin();

    // True solar longitude
    let true_lon = normalize_deg(l0 + c);

    // Nutation and aberration (simplified)
    let omega = normalize_deg(125.04 - 1934.136 * t);
    let nutation = -17.20 * deg_to_rad(omega).sin() / 3600.0; // arcseconds → degrees
    let aberration = -0.00569; // degrees (~20.5 arcseconds)

    let apparent_lon = normalize_deg(true_lon + nutation + aberration);

    // Distance in AU (Meeus eq 25.5 simplified)
    let distance = 1.000001018 * (1.0 - e * e) / (1.0 + e * m_rad.cos());

    // Solar latitude is always very small (<1.2 arcseconds)
    let latitude = 0.0;

    SolarPosition {
        longitude_deg: true_lon,
        latitude_deg: latitude,
        distance_au: distance,
        apparent_longitude_deg: apparent_lon,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solar_longitude_jan2025() {
        // J2000 + 25 years ≈ JD 2460674.5 (2025-01-01 00:00)
        let jd = JulianDate::from_jd(2460674.5);
        let pos = solar_position(jd);
        // Jan 1 should have solar longitude around 280-281°
        assert!(
            pos.longitude_deg > 278.0 && pos.longitude_deg < 282.0,
            "solar longitude at 2025-01-01: {}",
            pos.longitude_deg
        );
    }

    #[test]
    fn solar_longitude_progression() {
        let jd_jan1 = JulianDate::from_jd(2460674.5);
        let jd_jan2 = JulianDate::from_jd(2460674.5 + 1.0);
        let pos1 = solar_position(jd_jan1);
        let pos2 = solar_position(jd_jan2);
        // Should advance by ~0.9856°/day
        let diff = normalize_deg(pos2.longitude_deg - pos1.longitude_deg + 540.0) - 180.0;
        assert!(
            diff > 0.9 && diff < 1.1,
            "solar longitude should advance ~1°/day, got {}",
            diff
        );
    }

    #[test]
    fn solar_distance_range() {
        for day in (0..365).step_by(30) {
            let jd = JulianDate::from_jd(2460674.5 + day as f64);
            let pos = solar_position(jd);
            assert!(
                pos.distance_au > 0.98 && pos.distance_au < 1.02,
                "distance out of range at day {}: {}",
                day,
                pos.distance_au
            );
        }
    }

    #[test]
    fn solar_longitude_monotonic() {
        // Over 10 days, longitude should strictly increase
        let jd0 = JulianDate::from_jd(2460674.5);
        let mut prev = solar_position(jd0).longitude_deg;
        for d in 1..=10 {
            let jd = JulianDate::from_jd(2460674.5 + d as f64);
            let curr = solar_position(jd).longitude_deg;
            // Handle 360° wrap: if prev>350 and curr<10, that's fine
            if prev > 350.0 && curr < 10.0 {
                // wrapped around, OK
            } else {
                assert!(
                    curr > prev,
                    "longitude should increase: day {} lon={} < prev={}",
                    d,
                    curr,
                    prev
                );
            }
            prev = curr;
        }
    }
}
