// M13: Luck cycle calculation (大运).
// ADR 0020 closes DG-005.

use crate::astronomy::terms::solar_terms_for_year;
use crate::calendar::civil::CivilDate;
use crate::domain::bazi::{BRANCHES, Pillar, STEMS, Sex};

const GAN_YANG: [&str; 5] = ["甲", "丙", "戊", "庚", "壬"];

/// A single luck cycle (one 大运).
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LuckCycle {
    pub label: String,
    pub start_age: u8,
    pub end_age: u8,
    pub pillar: Pillar,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LuckCycleContext {
    pub direction: &'static str,
    pub year_gan: String,
    pub days_to_jie: u16,
    pub starting_age: u8,
    pub cycles: Vec<LuckCycle>,
}

pub fn compute_luck_cycle_context(
    year_gan: &str,
    month_pillar: &Pillar,
    sex: &Sex,
    birth_date: CivilDate,
) -> LuckCycleContext {
    let days_to_jie = compute_days_to_jie(year_gan, sex, birth_date);
    let cycles = compute_luck_cycles(year_gan, month_pillar, sex, days_to_jie);
    let starting_age = cycles.first().map(|cycle| cycle.start_age).unwrap_or(0);

    LuckCycleContext {
        direction: if is_forward(year_gan, sex) {
            "forward"
        } else {
            "reverse"
        },
        year_gan: year_gan.to_string(),
        days_to_jie,
        starting_age,
        cycles,
    }
}

/// Compute luck cycles for a given chart.
/// `year_gan` is the first character of the year pillar (e.g. "甲" from "甲辰").
/// `month_pillar` is the month pillar (e.g. "丙子").
/// `sex` is male or female.
/// `days_to_jie` is the number of days from birth to the next (forward) or previous (reverse) 节.
pub fn compute_luck_cycles(
    year_gan: &str,
    month_pillar: &Pillar,
    sex: &Sex,
    days_to_jie: u16,
) -> Vec<LuckCycle> {
    let is_yang_year = GAN_YANG.contains(&year_gan);
    let is_male = matches!(sex, Sex::Male);

    let forward = (is_yang_year && is_male) || (!is_yang_year && !is_male);

    // Starting age = days_to_jie / 3, rounded up, min 1, max 10
    let start_age = if days_to_jie == 0 {
        1u8
    } else {
        ((days_to_jie as f64 / 3.0).ceil() as u8).clamp(1, 10)
    };

    let month_stem_idx = STEMS
        .iter()
        .position(|s| *s == month_pillar.stem.as_str())
        .unwrap_or(0);
    let month_branch_idx = BRANCHES
        .iter()
        .position(|b| *b == month_pillar.branch.as_str())
        .unwrap_or(0);

    let mut cycles = Vec::with_capacity(8);
    for i in 0..8 {
        let offset = i as usize;
        let (si, bi) = if forward {
            (
                (month_stem_idx + offset) % 10,
                (month_branch_idx + offset) % 12,
            )
        } else {
            (
                (month_stem_idx + 10 - offset % 10) % 10,
                (month_branch_idx + 12 - offset % 12) % 12,
            )
        };
        let cycle_start = if i == 0 {
            start_age
        } else {
            start_age + i as u8 * 10
        };
        cycles.push(LuckCycle {
            label: format!("第{}运", i + 1),
            start_age: cycle_start,
            end_age: cycle_start + 9,
            pillar: Pillar {
                stem: STEMS[si].to_string(),
                branch: BRANCHES[bi].to_string(),
            },
        });
    }
    cycles
}

fn compute_days_to_jie(year_gan: &str, sex: &Sex, birth_date: CivilDate) -> u16 {
    let terms = solar_terms_for_year(birth_date.year);
    let doy = birth_date.day_of_year() as u16;
    let jie_indices = [0usize, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22];

    if is_forward(year_gan, sex) {
        let mut next_jie_doy = 366u16;
        for &index in &jie_indices {
            if let Some(term) = terms.get(index) {
                let td = (term.jd_tt
                    - crate::astronomy::time::gregorian_to_jd(birth_date.year, 1, 1, 0.0).whole())
                    as u16
                    + 1;
                if td > doy && td < next_jie_doy {
                    next_jie_doy = td;
                }
            }
        }
        if next_jie_doy == 366 {
            let next_year_terms = solar_terms_for_year(birth_date.year + 1);
            if let Some(term) = next_year_terms.first() {
                next_jie_doy = (term.jd_tt
                    - crate::astronomy::time::gregorian_to_jd(birth_date.year + 1, 1, 1, 0.0)
                        .whole()) as u16
                    + 365;
            }
        }
        next_jie_doy.saturating_sub(doy)
    } else {
        let mut prev_jie_doy = 0u16;
        for &index in &jie_indices {
            if let Some(term) = terms.get(index) {
                let td = (term.jd_tt
                    - crate::astronomy::time::gregorian_to_jd(birth_date.year, 1, 1, 0.0).whole())
                    as u16
                    + 1;
                if td <= doy && td > prev_jie_doy {
                    prev_jie_doy = td;
                }
            }
        }
        if prev_jie_doy == 0 {
            let prev_year_terms = solar_terms_for_year(birth_date.year - 1);
            if let Some(term) = prev_year_terms.get(11) {
                let td = (term.jd_tt
                    - crate::astronomy::time::gregorian_to_jd(birth_date.year - 1, 1, 1, 0.0)
                        .whole()) as u16
                    + 1;
                prev_jie_doy = td;
            }
            doy + 365u16 - prev_jie_doy
        } else {
            doy - prev_jie_doy
        }
    }
}

fn is_forward(year_gan: &str, sex: &Sex) -> bool {
    let yang = GAN_YANG.contains(&year_gan);
    let male = matches!(sex, Sex::Male);
    (yang && male) || (!yang && !male)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yang_male_forward() {
        // 甲年 (yang) 男 → 顺行
        let mp = Pillar {
            stem: "丙".into(),
            branch: "寅".into(),
        };
        let cycles = compute_luck_cycles("甲", &mp, &Sex::Male, 30); // 30 days → 10 years
        assert_eq!(cycles.len(), 8);
        assert_eq!(cycles[0].pillar.stem, "丙"); // month pillar
        assert_eq!(cycles[0].start_age, 10); // 30/3 = 10
        assert_eq!(cycles[1].pillar.stem, "丁"); // forward: 丙→丁
        assert_eq!(cycles[1].pillar.branch, "卯"); // forward: 寅→卯
    }

    #[test]
    fn yang_female_reverse() {
        // 甲年 (yang) 女 → 逆行
        let mp = Pillar {
            stem: "丙".into(),
            branch: "寅".into(),
        };
        let cycles = compute_luck_cycles("甲", &mp, &Sex::Female, 15); // 15 days → 5 years
        assert_eq!(cycles.len(), 8);
        assert_eq!(cycles[0].pillar.stem, "丙"); // month pillar
        assert_eq!(cycles[0].start_age, 5); // 15/3 = 5
        assert_eq!(cycles[1].pillar.stem, "乙"); // reverse: 丙→乙
        assert_eq!(cycles[1].pillar.branch, "丑"); // reverse: 寅→丑
    }

    #[test]
    fn yin_male_reverse() {
        // 乙年 (yin) 男 → 逆行
        let mp = Pillar {
            stem: "戊".into(),
            branch: "午".into(),
        };
        let cycles = compute_luck_cycles("乙", &mp, &Sex::Male, 9); // 9 days → 3 years
        assert_eq!(cycles[0].start_age, 3);
        assert_eq!(cycles[1].pillar.stem, "丁"); // reverse: 戊→丁
    }

    #[test]
    fn yin_female_forward() {
        // 乙年 (yin) 女 → 顺行
        let mp = Pillar {
            stem: "戊".into(),
            branch: "午".into(),
        };
        let cycles = compute_luck_cycles("乙", &mp, &Sex::Female, 0); // birth on jie day
        assert_eq!(cycles[0].start_age, 1); // clamp to 1
        assert_eq!(cycles[1].pillar.stem, "己"); // forward: 戊→己
    }

    #[test]
    fn all_eight_cycles_present() {
        let mp = Pillar {
            stem: "甲".into(),
            branch: "子".into(),
        };
        let cycles = compute_luck_cycles("甲", &mp, &Sex::Male, 21);
        assert_eq!(cycles.len(), 8);
        // Verify age ranges are contiguous
        for i in 1..cycles.len() {
            assert_eq!(cycles[i].start_age, cycles[i - 1].end_age + 1);
        }
    }
}
