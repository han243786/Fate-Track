use crate::calendar::civil::CivilDate;

const GAN: [&str; 10] = ["甲", "乙", "丙", "丁", "戊", "己", "庚", "辛", "壬", "癸"];
const ZHI: [&str; 12] = [
    "子", "丑", "寅", "卯", "辰", "巳", "午", "未", "申", "酉", "戌", "亥",
];
const EPOCH_GZ_INDEX: i32 = 10;
const MONTH_ZHI: [usize; 12] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0, 1];

pub fn day_ganzhi_index(date: CivilDate) -> usize {
    let epoch = CivilDate {
        year: 1900,
        month: 1,
        day: 1,
    };
    let days = date.days_since(epoch);
    (EPOCH_GZ_INDEX + days).rem_euclid(60) as usize
}

pub fn day_ganzhi(date: CivilDate) -> String {
    let index = day_ganzhi_index(date);
    ganzhi_from_index(index)
}

pub fn month_ganzhi(date: CivilDate, current_year_terms: &[u16], solar_year_gan: usize) -> String {
    let solar_term_month =
        solar_term_month_from_terms(date.day_of_year() as u16, current_year_terms);
    let month_gan = (solar_year_gan * 2 + 2 + solar_term_month) % 10;
    let month_zhi = MONTH_ZHI[solar_term_month];
    format!("{}{}", GAN[month_gan], ZHI[month_zhi])
}

fn solar_term_month_from_terms(doy: u16, terms: &[u16]) -> usize {
    if terms.len() < 24 {
        return fallback_solar_term_month(doy);
    }

    if doy < terms[0] {
        return 10;
    }

    if doy < terms[2] {
        return 11;
    }

    for i in 1..11 {
        let start = terms[i * 2];
        let end = terms[(i + 1) * 2];
        if doy >= start && doy < end {
            return i - 1;
        }
    }

    10
}

fn fallback_solar_term_month(doy: u16) -> usize {
    const APPROX_TERMS: [(u16, u16); 12] = [
        (5, 6),
        (34, 35),
        (64, 65),
        (95, 96),
        (125, 126),
        (156, 157),
        (187, 188),
        (218, 219),
        (249, 250),
        (280, 281),
        (310, 311),
        (341, 342),
    ];

    if doy < APPROX_TERMS[0].0 {
        return 10;
    }

    for (i, (lo, _hi)) in APPROX_TERMS.iter().enumerate() {
        let next = if i + 1 < APPROX_TERMS.len() {
            APPROX_TERMS[i + 1].0
        } else {
            366
        };
        if doy >= *lo && doy < next {
            return i;
        }
    }

    10
}

pub fn solar_year_gan_index(
    date: CivilDate,
    current_year_terms: &[u16],
    current_year_gan: Option<usize>,
    previous_year_gan: Option<usize>,
) -> usize {
    if current_year_terms.len() >= 3 && date.day_of_year() as u16 >= current_year_terms[2] {
        current_year_gan.unwrap_or(0)
    } else {
        previous_year_gan.unwrap_or_else(|| current_year_gan.unwrap_or(0))
    }
}

pub fn year_gan_index(ganzhi_year: &str) -> Option<usize> {
    let gan = ganzhi_year.chars().next()?.to_string();
    GAN.iter().position(|item| *item == gan)
}

fn ganzhi_from_index(index: usize) -> String {
    format!("{}{}", GAN[index % 10], ZHI[index % 12])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day_ganzhi_matches_android_edge_cases() {
        assert_eq!(
            day_ganzhi(CivilDate::parse_iso("2025-01-01").unwrap()),
            "庚午"
        );
        assert_eq!(
            day_ganzhi(CivilDate::parse_iso("2024-02-29").unwrap()),
            "癸亥"
        );
        assert_eq!(
            day_ganzhi(CivilDate::parse_iso("1901-02-19").unwrap()),
            "戊辰"
        );
    }
}
