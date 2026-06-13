use crate::calendar::civil::CivilDate;
use crate::calendar::ganzhi;
use crate::error::AppError;
use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::time::UNIX_EPOCH;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LunarDataMeta {
    pub path: PathBuf,
    pub bytes: u64,
    pub modified_unix: u64,
    pub version: Option<String>,
    pub epoch: Option<String>,
    pub term_count: usize,
    pub year_count: usize,
    pub min_year: Option<u16>,
    pub max_year: Option<u16>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LunarDataStats {
    pub version: Option<String>,
    pub epoch: Option<String>,
    pub term_count: usize,
    pub year_count: usize,
    pub min_year: Option<u16>,
    pub max_year: Option<u16>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct YearEntry {
    pub gan_zhi: String,
    pub zodiac: String,
    pub base_month: u8,
    pub base_day: u8,
    pub months: Vec<i16>,
    pub solar_terms: Vec<u16>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LunarTable {
    pub term_names: Vec<String>,
    pub years: BTreeMap<i32, YearEntry>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LunarDateResult {
    pub gregorian: CivilDate,
    pub lunar_year: i32,
    pub lunar_month: u8,
    pub lunar_day: u8,
    pub is_leap_month: bool,
    pub gan_zhi_year: String,
    pub gan_zhi_month: String,
    pub gan_zhi_day: String,
    pub zodiac: String,
    pub solar_term: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LunarDataSource {
    path: PathBuf,
}

impl LunarDataSource {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn meta(&self) -> Result<LunarDataMeta, AppError> {
        let content = fs::read_to_string(&self.path).map_err(|error| AppError::Io {
            context: format!("read lunar data {}", self.path.display()),
            message: error.to_string(),
        })?;
        let metadata = fs::metadata(&self.path).map_err(|error| AppError::Io {
            context: format!("inspect lunar data {}", self.path.display()),
            message: error.to_string(),
        })?;
        let stats = parse_lunar_data_stats(&content);
        let modified_unix = metadata
            .modified()
            .ok()
            .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
            .map(|duration| duration.as_secs())
            .unwrap_or(0);

        Ok(LunarDataMeta {
            path: self.path.clone(),
            bytes: metadata.len(),
            modified_unix,
            version: stats.version,
            epoch: stats.epoch,
            term_count: stats.term_count,
            year_count: stats.year_count,
            min_year: stats.min_year,
            max_year: stats.max_year,
        })
    }

    pub fn load_table(&self) -> Result<LunarTable, AppError> {
        let content = fs::read_to_string(&self.path).map_err(|error| AppError::Io {
            context: format!("read lunar data {}", self.path.display()),
            message: error.to_string(),
        })?;
        Ok(parse_lunar_table(&content))
    }
}

pub fn parse_lunar_data_stats(content: &str) -> LunarDataStats {
    let mut version = None;
    let mut epoch = None;
    let mut term_count = 0;
    let mut year_count = 0;
    let mut min_year = None;
    let mut max_year = None;
    let mut in_years = false;

    for line in content.lines() {
        let trimmed = line.trim();

        if let Some(value) = trimmed.strip_prefix("version:") {
            version = Some(value.trim().to_string());
        } else if let Some(value) = trimmed.strip_prefix("epoch:") {
            epoch = Some(value.trim().to_string());
        } else if let Some(value) = trimmed.strip_prefix("term_names:") {
            term_count = value.matches('\'').count() / 2;
        } else if trimmed == "years:" {
            in_years = true;
        } else if in_years
            && line.starts_with("  \"")
            && let Ok(year) = trimmed.trim_matches(':').trim_matches('"').parse::<u16>()
        {
            year_count += 1;
            min_year = Some(min_year.map_or(year, |current: u16| current.min(year)));
            max_year = Some(max_year.map_or(year, |current: u16| current.max(year)));
        }
    }

    LunarDataStats {
        version,
        epoch,
        term_count,
        year_count,
        min_year,
        max_year,
    }
}

pub fn parse_lunar_table(content: &str) -> LunarTable {
    let mut term_names = Vec::new();
    let mut years = BTreeMap::new();
    let mut current_year = None;
    let mut current_entry: Option<YearEntry> = None;

    for line in content.lines() {
        let trimmed = line.trim();

        if let Some(value) = trimmed.strip_prefix("term_names:") {
            term_names = parse_string_array(value);
            continue;
        }

        if let Some(year) = parse_year_header(line) {
            if let (Some(saved_year), Some(saved_entry)) =
                (current_year.take(), current_entry.take())
            {
                years.insert(saved_year, saved_entry);
            }
            current_year = Some(year);
            current_entry = Some(YearEntry {
                gan_zhi: String::new(),
                zodiac: String::new(),
                base_month: 1,
                base_day: 1,
                months: Vec::new(),
                solar_terms: Vec::new(),
            });
            continue;
        }

        let Some(entry) = current_entry.as_mut() else {
            continue;
        };

        if let Some(value) = trimmed.strip_prefix("gz:") {
            entry.gan_zhi = clean_scalar(value);
        } else if let Some(value) = trimmed.strip_prefix("zo:") {
            entry.zodiac = clean_scalar(value);
        } else if let Some(value) = trimmed.strip_prefix("base:") {
            let values = parse_i16_array(value);
            if values.len() >= 2 {
                entry.base_month = values[0] as u8;
                entry.base_day = values[1] as u8;
            }
        } else if let Some(value) = trimmed.strip_prefix("m:") {
            entry.months = parse_i16_array(value);
        } else if let Some(value) = trimmed.strip_prefix("s:") {
            entry.solar_terms = parse_i16_array(value)
                .into_iter()
                .map(|value| value.max(0) as u16)
                .collect();
        }
    }

    if let (Some(saved_year), Some(saved_entry)) = (current_year.take(), current_entry.take()) {
        years.insert(saved_year, saved_entry);
    }

    LunarTable { term_names, years }
}

impl LunarTable {
    pub fn lookup(&self, date: CivilDate) -> Option<LunarDateResult> {
        let year_entry = self.years.get(&date.year)?;
        let (lunar_year, lunar_entry) = if date.month > year_entry.base_month
            || (date.month == year_entry.base_month && date.day >= year_entry.base_day)
        {
            (date.year, year_entry)
        } else {
            let previous_year = date.year - 1;
            (previous_year, self.years.get(&previous_year)?)
        };

        let cny = CivilDate {
            year: lunar_year,
            month: lunar_entry.base_month,
            day: lunar_entry.base_day,
        };
        let mut days = date.days_since(cny);
        if days < 0 {
            return None;
        }

        let mut lunar_day = 0;
        let mut is_leap_month = false;
        let mut found_index = None;

        for (index, month_length) in lunar_entry.months.iter().enumerate() {
            let length = month_length.abs() as i32;
            if days < length {
                lunar_day = (days + 1) as u8;
                is_leap_month = *month_length < 0;
                found_index = Some(index);
                break;
            }
            days -= length;
        }

        let found_index = found_index?;
        let lunar_month = display_month(&lunar_entry.months, found_index, is_leap_month)?;
        let solar_term = self.solar_term(date);

        let current_greg_entry = self.years.get(&date.year);
        let prev_greg_entry = self.years.get(&(date.year - 1));
        let current_year_terms: &[u16] = current_greg_entry
            .map(|e| e.solar_terms.as_slice())
            .unwrap_or(&[]);
        let current_year_gan = current_greg_entry.and_then(|e| ganzhi::year_gan_index(&e.gan_zhi));
        let previous_year_gan = prev_greg_entry.and_then(|e| ganzhi::year_gan_index(&e.gan_zhi));
        let solar_year_gan = ganzhi::solar_year_gan_index(
            date,
            current_year_terms,
            current_year_gan,
            previous_year_gan,
        );

        Some(LunarDateResult {
            gregorian: date,
            lunar_year,
            lunar_month,
            lunar_day,
            is_leap_month,
            gan_zhi_year: lunar_entry.gan_zhi.clone(),
            gan_zhi_month: ganzhi::month_ganzhi(date, current_year_terms, solar_year_gan),
            gan_zhi_day: ganzhi::day_ganzhi(date),
            zodiac: lunar_entry.zodiac.clone(),
            solar_term,
        })
    }

    fn solar_term(&self, date: CivilDate) -> Option<String> {
        let entry = self.years.get(&date.year)?;
        let doy = date.day_of_year();
        entry
            .solar_terms
            .iter()
            .position(|term_day| *term_day == doy && *term_day > 0)
            .and_then(|index| self.term_names.get(index).cloned())
    }
}

impl LunarDateResult {
    pub fn month_name(&self) -> String {
        const MONTH_NAMES: [&str; 13] = [
            "",
            "正月",
            "二月",
            "三月",
            "四月",
            "五月",
            "六月",
            "七月",
            "八月",
            "九月",
            "十月",
            "十一月",
            "十二月",
        ];
        let prefix = if self.is_leap_month { "闰" } else { "" };
        let name = MONTH_NAMES
            .get(self.lunar_month as usize)
            .copied()
            .unwrap_or("");
        format!("{prefix}{name}")
    }

    pub fn day_name(&self) -> String {
        const DAY_NAMES: [&str; 31] = [
            "", "初一", "初二", "初三", "初四", "初五", "初六", "初七", "初八", "初九", "初十",
            "十一", "十二", "十三", "十四", "十五", "十六", "十七", "十八", "十九", "二十", "廿一",
            "廿二", "廿三", "廿四", "廿五", "廿六", "廿七", "廿八", "廿九", "三十",
        ];
        DAY_NAMES
            .get(self.lunar_day as usize)
            .copied()
            .unwrap_or("")
            .to_string()
    }
}

fn display_month(months: &[i16], found_index: usize, is_leap_month: bool) -> Option<u8> {
    let mut display_month = 0_u8;
    for month in months.iter().take(found_index) {
        if *month > 0 {
            display_month += 1;
        }
    }
    if !is_leap_month {
        display_month += 1;
    }
    (1..=12).contains(&display_month).then_some(display_month)
}

fn parse_year_header(line: &str) -> Option<i32> {
    let trimmed = line.trim();
    if !line.starts_with("  \"") || !trimmed.ends_with(':') {
        return None;
    }
    trimmed
        .trim_matches(':')
        .trim_matches('"')
        .parse::<i32>()
        .ok()
}

fn parse_i16_array(value: &str) -> Vec<i16> {
    array_body(value)
        .split(',')
        .filter_map(|item| item.trim().parse::<i16>().ok())
        .collect()
}

fn parse_string_array(value: &str) -> Vec<String> {
    array_body(value)
        .split(',')
        .map(clean_scalar)
        .filter(|value| !value.is_empty())
        .collect()
}

fn array_body(value: &str) -> String {
    value
        .trim()
        .trim_start_matches('[')
        .trim_end_matches(']')
        .to_string()
}

fn clean_scalar(value: &str) -> String {
    value
        .trim()
        .trim_matches('"')
        .trim_matches('\'')
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn parses_lunar_data_stats() {
        let content = r#"
version: 1
epoch: 1901
term_names: ['小寒', '大寒', '立春']
years:
  "1901":
    gz: "辛丑"
  "1902":
    gz: "壬寅"
"#;

        let stats = parse_lunar_data_stats(content);

        assert_eq!(stats.version, Some("1".to_string()));
        assert_eq!(stats.epoch, Some("1901".to_string()));
        assert_eq!(stats.term_count, 3);
        assert_eq!(stats.year_count, 2);
        assert_eq!(stats.min_year, Some(1901));
        assert_eq!(stats.max_year, Some(1902));
    }

    #[test]
    fn parses_table_and_looks_up_lunar_date() {
        let content = r#"
term_names: ['小寒', '大寒']
years:
  "1901":
    gz: "辛丑"
    zo: "牛"
    base: [2,19]
    m: [29,30]
    s: [6,21]
"#;
        let table = parse_lunar_table(content);
        let result = table
            .lookup(CivilDate::parse_iso("1901-02-19").unwrap())
            .unwrap();

        assert_eq!(result.lunar_year, 1901);
        assert_eq!(result.month_name(), "正月");
        assert_eq!(result.day_name(), "初一");
        assert_eq!(result.gan_zhi_day, "戊辰");
    }

    #[derive(Clone, Copy)]
    struct AndroidEdgeCase {
        label: &'static str,
        iso_date: &'static str,
        lunar_month: u8,
        lunar_day: u8,
        is_leap_month: bool,
        year_gz: &'static str,
        month_gz: &'static str,
        day_gz: &'static str,
        solar_term: Option<&'static str>,
    }

    const ANDROID_EDGE_CASES: &[AndroidEdgeCase] = &[
        AndroidEdgeCase {
            label: "Feb29-1904",
            iso_date: "1904-02-29",
            lunar_month: 1,
            lunar_day: 14,
            is_leap_month: false,
            year_gz: "\u{7532}\u{8fb0}",
            month_gz: "\u{4e19}\u{5bc5}",
            day_gz: "\u{7678}\u{5df3}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "Feb29-1932",
            iso_date: "1932-02-29",
            lunar_month: 1,
            lunar_day: 24,
            is_leap_month: false,
            year_gz: "\u{58ec}\u{7533}",
            month_gz: "\u{58ec}\u{5bc5}",
            day_gz: "\u{5e9a}\u{7533}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "Feb29-1960",
            iso_date: "1960-02-29",
            lunar_month: 2,
            lunar_day: 3,
            is_leap_month: false,
            year_gz: "\u{5e9a}\u{5b50}",
            month_gz: "\u{620a}\u{5bc5}",
            day_gz: "\u{4e01}\u{4ea5}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "Feb29-1988",
            iso_date: "1988-02-29",
            lunar_month: 1,
            lunar_day: 13,
            is_leap_month: false,
            year_gz: "\u{620a}\u{8fb0}",
            month_gz: "\u{7532}\u{5bc5}",
            day_gz: "\u{7532}\u{5bc5}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "Feb29-2000",
            iso_date: "2000-02-29",
            lunar_month: 1,
            lunar_day: 25,
            is_leap_month: false,
            year_gz: "\u{5e9a}\u{8fb0}",
            month_gz: "\u{620a}\u{5bc5}",
            day_gz: "\u{4e01}\u{5df3}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "Feb29-2024",
            iso_date: "2024-02-29",
            lunar_month: 1,
            lunar_day: 20,
            is_leap_month: false,
            year_gz: "\u{7532}\u{8fb0}",
            month_gz: "\u{4e19}\u{5bc5}",
            day_gz: "\u{7678}\u{4ea5}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "Feb29-2052",
            iso_date: "2052-02-29",
            lunar_month: 1,
            lunar_day: 29,
            is_leap_month: false,
            year_gz: "\u{58ec}\u{7533}",
            month_gz: "\u{58ec}\u{5bc5}",
            day_gz: "\u{5e9a}\u{5bc5}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "Feb29-2080",
            iso_date: "2080-02-29",
            lunar_month: 2,
            lunar_day: 9,
            is_leap_month: false,
            year_gz: "\u{5e9a}\u{5b50}",
            month_gz: "\u{620a}\u{5bc5}",
            day_gz: "\u{4e01}\u{5df3}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "Feb29-2096",
            iso_date: "2096-02-29",
            lunar_month: 2,
            lunar_day: 6,
            is_leap_month: false,
            year_gz: "\u{4e19}\u{8fb0}",
            month_gz: "\u{5e9a}\u{5bc5}",
            day_gz: "\u{8f9b}\u{5df3}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "LeapMonth-1903",
            iso_date: "1903-07-09",
            lunar_month: 5,
            lunar_day: 15,
            is_leap_month: true,
            year_gz: "\u{7678}\u{536f}",
            month_gz: "\u{5df1}\u{672a}",
            day_gz: "\u{620a}\u{620c}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "LeapMonth-1906",
            iso_date: "1906-06-07",
            lunar_month: 4,
            lunar_day: 16,
            is_leap_month: true,
            year_gz: "\u{4e19}\u{5348}",
            month_gz: "\u{7532}\u{5348}",
            day_gz: "\u{58ec}\u{5348}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "LeapMonth-1909",
            iso_date: "1909-04-05",
            lunar_month: 2,
            lunar_day: 15,
            is_leap_month: true,
            year_gz: "\u{5df1}\u{9149}",
            month_gz: "\u{620a}\u{8fb0}",
            day_gz: "\u{4e59}\u{672a}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "LeapMonth-1911",
            iso_date: "1911-08-09",
            lunar_month: 6,
            lunar_day: 15,
            is_leap_month: true,
            year_gz: "\u{8f9b}\u{4ea5}",
            month_gz: "\u{4e19}\u{7533}",
            day_gz: "\u{8f9b}\u{4ea5}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "LeapMonth-1914",
            iso_date: "1914-07-08",
            lunar_month: 5,
            lunar_day: 16,
            is_leap_month: true,
            year_gz: "\u{7532}\u{5bc5}",
            month_gz: "\u{8f9b}\u{672a}",
            day_gz: "\u{4e59}\u{672a}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "Dec31-1901",
            iso_date: "1901-12-31",
            lunar_month: 11,
            lunar_day: 21,
            is_leap_month: false,
            year_gz: "\u{8f9b}\u{4e11}",
            month_gz: "\u{5e9a}\u{5b50}",
            day_gz: "\u{7678}\u{672a}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "Jan1-1902",
            iso_date: "1902-01-01",
            lunar_month: 11,
            lunar_day: 22,
            is_leap_month: false,
            year_gz: "\u{8f9b}\u{4e11}",
            month_gz: "\u{5e9a}\u{5b50}",
            day_gz: "\u{7532}\u{7533}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "Dec31-1950",
            iso_date: "1950-12-31",
            lunar_month: 11,
            lunar_day: 23,
            is_leap_month: false,
            year_gz: "\u{5e9a}\u{5bc5}",
            month_gz: "\u{620a}\u{5b50}",
            day_gz: "\u{5e9a}\u{5b50}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "Jan1-1951",
            iso_date: "1951-01-01",
            lunar_month: 11,
            lunar_day: 24,
            is_leap_month: false,
            year_gz: "\u{5e9a}\u{5bc5}",
            month_gz: "\u{620a}\u{5b50}",
            day_gz: "\u{8f9b}\u{4e11}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "Dec31-2000",
            iso_date: "2000-12-31",
            lunar_month: 12,
            lunar_day: 6,
            is_leap_month: false,
            year_gz: "\u{5e9a}\u{8fb0}",
            month_gz: "\u{620a}\u{5b50}",
            day_gz: "\u{7678}\u{4ea5}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "Jan1-2001",
            iso_date: "2001-01-01",
            lunar_month: 12,
            lunar_day: 7,
            is_leap_month: false,
            year_gz: "\u{5e9a}\u{8fb0}",
            month_gz: "\u{620a}\u{5b50}",
            day_gz: "\u{7532}\u{5b50}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "Dec31-2023",
            iso_date: "2023-12-31",
            lunar_month: 11,
            lunar_day: 19,
            is_leap_month: false,
            year_gz: "\u{7678}\u{536f}",
            month_gz: "\u{7532}\u{5b50}",
            day_gz: "\u{7678}\u{4ea5}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "Jan1-2024",
            iso_date: "2024-01-01",
            lunar_month: 11,
            lunar_day: 20,
            is_leap_month: false,
            year_gz: "\u{7678}\u{536f}",
            month_gz: "\u{7532}\u{5b50}",
            day_gz: "\u{7532}\u{5b50}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "Dec31-2024",
            iso_date: "2024-12-31",
            lunar_month: 12,
            lunar_day: 1,
            is_leap_month: false,
            year_gz: "\u{7532}\u{8fb0}",
            month_gz: "\u{4e19}\u{5b50}",
            day_gz: "\u{5df1}\u{5df3}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "Jan1-2025",
            iso_date: "2025-01-01",
            lunar_month: 12,
            lunar_day: 2,
            is_leap_month: false,
            year_gz: "\u{7532}\u{8fb0}",
            month_gz: "\u{4e19}\u{5b50}",
            day_gz: "\u{5e9a}\u{5348}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "Dec31-2025",
            iso_date: "2025-12-31",
            lunar_month: 11,
            lunar_day: 12,
            is_leap_month: false,
            year_gz: "\u{4e59}\u{5df3}",
            month_gz: "\u{620a}\u{5b50}",
            day_gz: "\u{7532}\u{620c}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "Jan1-2026",
            iso_date: "2026-01-01",
            lunar_month: 11,
            lunar_day: 13,
            is_leap_month: false,
            year_gz: "\u{4e59}\u{5df3}",
            month_gz: "\u{620a}\u{5b50}",
            day_gz: "\u{4e59}\u{4ea5}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "Dec31-2099",
            iso_date: "2099-12-31",
            lunar_month: 11,
            lunar_day: 20,
            is_leap_month: false,
            year_gz: "\u{5df1}\u{672a}",
            month_gz: "\u{4e19}\u{5b50}",
            day_gz: "\u{58ec}\u{5bc5}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "Jan1-2100",
            iso_date: "2100-01-01",
            lunar_month: 11,
            lunar_day: 21,
            is_leap_month: false,
            year_gz: "\u{5df1}\u{672a}",
            month_gz: "\u{4e19}\u{5b50}",
            day_gz: "\u{7678}\u{536f}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "Dec31-2100",
            iso_date: "2100-12-31",
            lunar_month: 12,
            lunar_day: 1,
            is_leap_month: false,
            year_gz: "\u{5e9a}\u{7533}",
            month_gz: "\u{620a}\u{5b50}",
            day_gz: "\u{4e01}\u{672a}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "PreCNY-2015",
            iso_date: "2015-01-15",
            lunar_month: 11,
            lunar_day: 25,
            is_leap_month: false,
            year_gz: "\u{7532}\u{5348}",
            month_gz: "\u{4e01}\u{4e11}",
            day_gz: "\u{8f9b}\u{536f}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "PreCNY-2020",
            iso_date: "2020-01-15",
            lunar_month: 12,
            lunar_day: 21,
            is_leap_month: false,
            year_gz: "\u{5df1}\u{4ea5}",
            month_gz: "\u{4e01}\u{4e11}",
            day_gz: "\u{4e01}\u{5df3}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "PreCNY-2025",
            iso_date: "2025-01-15",
            lunar_month: 12,
            lunar_day: 16,
            is_leap_month: false,
            year_gz: "\u{7532}\u{8fb0}",
            month_gz: "\u{4e01}\u{4e11}",
            day_gz: "\u{7532}\u{7533}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "PreCNY-2030",
            iso_date: "2030-01-15",
            lunar_month: 12,
            lunar_day: 12,
            is_leap_month: false,
            year_gz: "\u{5df1}\u{9149}",
            month_gz: "\u{4e01}\u{4e11}",
            day_gz: "\u{5e9a}\u{620c}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "CNY-1901",
            iso_date: "1901-02-19",
            lunar_month: 1,
            lunar_day: 1,
            is_leap_month: false,
            year_gz: "\u{8f9b}\u{4e11}",
            month_gz: "\u{5e9a}\u{5bc5}",
            day_gz: "\u{620a}\u{8fb0}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "CNY-1950",
            iso_date: "1950-02-17",
            lunar_month: 1,
            lunar_day: 1,
            is_leap_month: false,
            year_gz: "\u{5e9a}\u{5bc5}",
            month_gz: "\u{620a}\u{5bc5}",
            day_gz: "\u{7678}\u{672a}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "CNY-2000",
            iso_date: "2000-02-05",
            lunar_month: 1,
            lunar_day: 1,
            is_leap_month: false,
            year_gz: "\u{5e9a}\u{8fb0}",
            month_gz: "\u{620a}\u{5bc5}",
            day_gz: "\u{7678}\u{5df3}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "CNY-2020",
            iso_date: "2020-01-25",
            lunar_month: 1,
            lunar_day: 1,
            is_leap_month: false,
            year_gz: "\u{5e9a}\u{5b50}",
            month_gz: "\u{4e01}\u{4e11}",
            day_gz: "\u{4e01}\u{536f}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "CNY-2025",
            iso_date: "2025-01-29",
            lunar_month: 1,
            lunar_day: 1,
            is_leap_month: false,
            year_gz: "\u{4e59}\u{5df3}",
            month_gz: "\u{4e01}\u{4e11}",
            day_gz: "\u{620a}\u{620c}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "CNY-2050",
            iso_date: "2050-01-23",
            lunar_month: 1,
            lunar_day: 1,
            is_leap_month: false,
            year_gz: "\u{5e9a}\u{5348}",
            month_gz: "\u{4e01}\u{4e11}",
            day_gz: "\u{7678}\u{536f}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "CNY-2100",
            iso_date: "2100-02-09",
            lunar_month: 1,
            lunar_day: 1,
            is_leap_month: false,
            year_gz: "\u{5e9a}\u{7533}",
            month_gz: "\u{620a}\u{5bc5}",
            day_gz: "\u{58ec}\u{5348}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "ExtremeCNY-1966",
            iso_date: "1966-01-26",
            lunar_month: 1,
            lunar_day: 6,
            is_leap_month: false,
            year_gz: "\u{4e19}\u{5348}",
            month_gz: "\u{5df1}\u{4e11}",
            day_gz: "\u{4e59}\u{9149}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "ExtremeCNY-2061",
            iso_date: "2061-01-26",
            lunar_month: 1,
            lunar_day: 6,
            is_leap_month: false,
            year_gz: "\u{8f9b}\u{5df3}",
            month_gz: "\u{5df1}\u{4e11}",
            day_gz: "\u{7532}\u{8fb0}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "ExtremeCNY-2099",
            iso_date: "2099-01-26",
            lunar_month: 1,
            lunar_day: 6,
            is_leap_month: false,
            year_gz: "\u{5df1}\u{672a}",
            month_gz: "\u{4e59}\u{4e11}",
            day_gz: "\u{7678}\u{4ea5}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "ExtremeCNY-2072",
            iso_date: "2072-02-24",
            lunar_month: 1,
            lunar_day: 6,
            is_leap_month: false,
            year_gz: "\u{58ec}\u{8fb0}",
            month_gz: "\u{58ec}\u{5bc5}",
            day_gz: "\u{5e9a}\u{5348}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "ExtremeCNY-1920",
            iso_date: "1920-02-25",
            lunar_month: 1,
            lunar_day: 6,
            is_leap_month: false,
            year_gz: "\u{5e9a}\u{7533}",
            month_gz: "\u{620a}\u{5bc5}",
            day_gz: "\u{7678}\u{4e11}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "ExtremeCNY-1985",
            iso_date: "1985-02-25",
            lunar_month: 1,
            lunar_day: 6,
            is_leap_month: false,
            year_gz: "\u{4e59}\u{4e11}",
            month_gz: "\u{620a}\u{5bc5}",
            day_gz: "\u{4e59}\u{672a}",
            solar_term: None,
        },
        AndroidEdgeCase {
            label: "SolarTerm-\u{5c0f}\u{5bd2}-2000",
            iso_date: "2000-01-06",
            lunar_month: 11,
            lunar_day: 30,
            is_leap_month: false,
            year_gz: "\u{5df1}\u{536f}",
            month_gz: "\u{4e01}\u{4e11}",
            day_gz: "\u{7678}\u{4ea5}",
            solar_term: Some("\u{5c0f}\u{5bd2}"),
        },
        AndroidEdgeCase {
            label: "SolarTerm-\u{5c0f}\u{5bd2}-2020",
            iso_date: "2020-01-06",
            lunar_month: 12,
            lunar_day: 12,
            is_leap_month: false,
            year_gz: "\u{5df1}\u{4ea5}",
            month_gz: "\u{4e01}\u{4e11}",
            day_gz: "\u{620a}\u{7533}",
            solar_term: Some("\u{5c0f}\u{5bd2}"),
        },
        AndroidEdgeCase {
            label: "SolarTerm-\u{5c0f}\u{5bd2}-2025",
            iso_date: "2025-01-05",
            lunar_month: 12,
            lunar_day: 6,
            is_leap_month: false,
            year_gz: "\u{7532}\u{8fb0}",
            month_gz: "\u{4e01}\u{4e11}",
            day_gz: "\u{7532}\u{620c}",
            solar_term: Some("\u{5c0f}\u{5bd2}"),
        },
    ];

    #[test]
    fn project_data_matches_android_edge_cases_for_three_pillars() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("data")
            .join("raw")
            .join("lunar_data.yaml");
        let table = LunarDataSource::new(path).load_table().unwrap();

        for case in ANDROID_EDGE_CASES {
            assert_android_edge_case(&table, case);
        }
    }

    fn assert_android_edge_case(table: &LunarTable, case: &AndroidEdgeCase) {
        let result = table
            .lookup(CivilDate::parse_iso(case.iso_date).unwrap())
            .unwrap();
        assert_eq!(
            result.lunar_month, case.lunar_month,
            "{} lunar month",
            case.label
        );
        assert_eq!(result.lunar_day, case.lunar_day, "{} lunar day", case.label);
        assert_eq!(
            result.is_leap_month, case.is_leap_month,
            "{} leap month",
            case.label
        );
        assert_eq!(
            result.gan_zhi_year, case.year_gz,
            "{} year pillar",
            case.label
        );
        assert_eq!(
            result.gan_zhi_month, case.month_gz,
            "{} month pillar",
            case.label
        );
        assert_eq!(result.gan_zhi_day, case.day_gz, "{} day pillar", case.label);
        if let Some(solar_term) = case.solar_term {
            assert_eq!(
                result.solar_term.as_deref(),
                Some(solar_term),
                "{} solar term",
                case.label
            );
        }
    }
}
