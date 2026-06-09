#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CivilDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl CivilDate {
    pub fn parse_iso(value: &str) -> Option<Self> {
        let mut parts = value.split('-');
        let year = parts.next()?.parse::<i32>().ok()?;
        let month = parts.next()?.parse::<u8>().ok()?;
        let day = parts.next()?.parse::<u8>().ok()?;
        if parts.next().is_some() {
            return None;
        }

        let date = Self { year, month, day };
        date.is_valid().then_some(date)
    }

    pub fn is_valid(self) -> bool {
        if !(1..=12).contains(&self.month) {
            return false;
        }
        (1..=days_in_month(self.year, self.month)).contains(&self.day)
    }

    pub fn days_since(self, other: Self) -> i32 {
        days_from_civil(self.year, self.month, self.day)
            - days_from_civil(other.year, other.month, other.day)
    }

    pub fn day_of_year(self) -> u16 {
        let jan1 = Self {
            year: self.year,
            month: 1,
            day: 1,
        };
        (self.days_since(jan1) + 1) as u16
    }
}

pub fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => 0,
    }
}

pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

pub fn days_from_civil(year: i32, month: u8, day: u8) -> i32 {
    let mut y = year;
    let m = month as i32;
    let d = day as i32;
    y -= (m <= 2) as i32;
    let era = if y >= 0 { y } else { y - 399 } / 400;
    let yoe = y - era * 400;
    let mp = m + if m > 2 { -3 } else { 9 };
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146097 + doe - 719468
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_leap_day() {
        assert!(CivilDate::parse_iso("2024-02-29").is_some());
        assert!(CivilDate::parse_iso("2025-02-29").is_none());
    }

    #[test]
    fn computes_day_difference_across_years() {
        let jan1 = CivilDate::parse_iso("2025-01-01").unwrap();
        let dec31 = CivilDate::parse_iso("2024-12-31").unwrap();

        assert_eq!(jan1.days_since(dec31), 1);
        assert_eq!(jan1.day_of_year(), 1);
    }
}
