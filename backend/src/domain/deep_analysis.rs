// M21: Deep analysis based on 三命通会/子平法 distillation.
// All rules are hard-coded from classical principles. No AI/LLM.
use crate::domain::bazi::ChartResult;

#[derive(Clone, Debug)]
pub struct StrengthAssessment {
    pub score: u8,          // 0-10
    pub level: &'static str, // 极弱/偏弱/中和/偏强/极强
    pub deling: bool,       // 得令 (month branch supports day stem element)
    pub dedi: u8,           // 得地 count (branches supporting day stem)
    pub deshi: u8,          // 得势 count (stems matching/supporting day stem)
}

#[derive(Clone, Debug)]
pub struct PatternInfo {
    pub pattern_name: String,
    pub pattern_type: &'static str, // 正格/变格/特殊
    pub confidence: &'static str,   // 确定/可能
}

#[derive(Clone, Debug)]
pub struct UsefulGodHint {
    pub element: &'static str,
    pub priority: u8,       // 1=首选, 2=次选
    pub reason: String,
}

#[allow(dead_code)]
const STEMS: [&str; 10] = ["甲","乙","丙","丁","戊","己","庚","辛","壬","癸"];
#[allow(dead_code)]
const BRANCHES: [&str; 12] = ["子","丑","寅","卯","辰","巳","午","未","申","酉","戌","亥"];

/// Element of a stem
fn stem_element(stem: &str) -> &'static str {
    match stem {
        "甲"|"乙" => "wood", "丙"|"丁" => "fire", "戊"|"己" => "earth",
        "庚"|"辛" => "metal", "壬"|"癸" => "water", _ => "earth",
    }
}

/// Branch primary element
fn branch_element(branch: &str) -> &'static str {
    match branch {
        "子"|"亥" => "water", "寅"|"卯" => "wood", "巳"|"午" => "fire",
        "申"|"酉" => "metal", "辰"|"戌"|"丑"|"未" => "earth", _ => "earth",
    }
}

/// Element generates another
fn generates(a: &str, b: &str) -> bool {
    matches!((a,b), ("wood","fire")|("fire","earth")|("earth","metal")|("metal","water")|("water","wood"))
}

/// Element controls another
fn controls(a: &str, b: &str) -> bool {
    matches!((a,b), ("wood","earth")|("earth","water")|("water","fire")|("fire","metal")|("metal","wood"))
}

/// Month branch → season → element prospering
fn month_season_element(month_branch: &str) -> &'static str {
    match month_branch {
        "寅"|"卯"|"辰" => "wood",   // spring
        "巳"|"午"|"未" => "fire",   // summer
        "申"|"酉"|"戌" => "metal",  // autumn
        "亥"|"子"|"丑" => "water",  // winter
        _ => "earth",
    }
}

/// Assess day master strength per 三命通会 principles.
pub fn assess_strength(chart: &ChartResult) -> StrengthAssessment {
    let day_stem = &chart.chart.day.stem;
    let day_el = stem_element(day_stem);
    let month_branch = &chart.chart.month.branch;

    // 得令: month branch season matches day stem element
    let season_el = month_season_element(month_branch);
    let deling = day_el == season_el || generates(season_el, day_el);

    // 得地: count branches whose element matches or generates day stem
    let branches = [
        &chart.chart.year.branch,
        &chart.chart.month.branch,
        &chart.chart.day.branch,
        chart.chart.hour.as_ref().map(|h| &h.branch).unwrap_or(&chart.chart.day.branch),
    ];
    let dedi = branches.iter()
        .filter(|b| {
            let be = branch_element(b);
            be == day_el || generates(be, day_el)
        })
        .count() as u8;

    // 得势: count stems that match or generate day stem
    let stems = [
        &chart.chart.year.stem,
        &chart.chart.month.stem,
        &chart.chart.day.stem,
        chart.chart.hour.as_ref().map(|h| &h.stem).unwrap_or(&chart.chart.day.stem),
    ];
    let deshi = stems.iter()
        .filter(|s| {
            let se = stem_element(s);
            se == day_el || generates(se, day_el)
        })
        .count() as u8;

    let score = if deling { 4 } else { 0 }
        + dedi.min(3)
        + deshi.min(3);

    let level = match score {
        0..=2 => "极弱",
        3..=4 => "偏弱",
        5..=6 => "中和",
        7..=8 => "偏强",
        _ => "极强",
    };

    StrengthAssessment { score, level, deling, dedi, deshi }
}

/// Classify 格局 per 子平法.
pub fn classify_pattern(chart: &ChartResult, _strength: &StrengthAssessment) -> PatternInfo {
    let month_stem = &chart.chart.month.stem;
    let day_stem = &chart.chart.day.stem;
    let day_el = stem_element(day_stem);
    let month_el = stem_element(month_stem);

    // Month stem controls day stem → 正官格 or 七杀格
    if controls(month_el, day_el) {
        let yang = ["甲","丙","戊","庚","壬"].contains(&day_stem.as_str());
        let m_yang = ["甲","丙","戊","庚","壬"].contains(&month_stem.as_str());
        if yang == m_yang {
            return PatternInfo { pattern_name: "七杀格".into(), pattern_type: "正格", confidence: "确定" };
        } else {
            return PatternInfo { pattern_name: "正官格".into(), pattern_type: "正格", confidence: "确定" };
        }
    }

    // Day stem controls month stem → 财格
    if controls(day_el, month_el) {
        return PatternInfo { pattern_name: "财格".into(), pattern_type: "正格", confidence: "确定" };
    }

    // Month stem generates day stem → 印格
    if generates(month_el, day_el) {
        return PatternInfo { pattern_name: "印格".into(), pattern_type: "正格", confidence: "确定" };
    }

    // Day stem generates month stem → 食伤格
    if generates(day_el, month_el) {
        return PatternInfo { pattern_name: "食伤格".into(), pattern_type: "正格", confidence: "确定" };
    }

    // Same element → 建禄格 or 月刃格
    if day_el == month_el {
        let yang = ["甲","丙","戊","庚","壬"].contains(&day_stem.as_str());
        if yang {
            return PatternInfo { pattern_name: "建禄格".into(), pattern_type: "正格", confidence: "确定" };
        } else {
            return PatternInfo { pattern_name: "月刃格".into(), pattern_type: "正格", confidence: "确定" };
        }
    }

    PatternInfo { pattern_name: "杂格".into(), pattern_type: "正格", confidence: "可能" }
}

/// Suggest 用神 based on strength and pattern.
pub fn suggest_useful_god(strength: &StrengthAssessment, pattern: &PatternInfo) -> Vec<UsefulGodHint> {
    let mut hints = Vec::new();

    match strength.level {
        "极弱"|"偏弱" => {
            // Weak: support day stem element
            hints.push(UsefulGodHint {
                element: "印星(生我)", priority: 1,
                reason: "日主偏弱，首选印星生扶".into(),
            });
            hints.push(UsefulGodHint {
                element: "比劫(同我)", priority: 2,
                reason: "次选比劫帮扶".into(),
            });
        }
        "极强"|"偏强" => {
            // Strong: control/vent day stem
            if pattern.pattern_name.contains("杀") || pattern.pattern_name.contains("官") {
                hints.push(UsefulGodHint {
                    element: "印星(化杀)", priority: 1,
                    reason: "杀强身旺，首选印星化杀生身".into(),
                });
            } else {
                hints.push(UsefulGodHint {
                    element: "食伤(泄秀)", priority: 1,
                    reason: "日主偏强，首选食伤泄秀".into(),
                });
                hints.push(UsefulGodHint {
                    element: "财星(耗身)", priority: 2,
                    reason: "次选财星耗身".into(),
                });
            }
        }
        _ => {
            // Balanced
            hints.push(UsefulGodHint {
                element: "通关用神", priority: 1,
                reason: "日主中和，取通关五行为用".into(),
            });
        }
    }

    hints
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::civil::CivilDate;
    use crate::domain::bazi::{BirthProfile, BirthTime, CalendarKind, ChartBasis, ChartRequest, DateLayerPillars, PrivacyLevel, Sex, TimePrecision};

    fn sample_chart() -> ChartResult {
        let req = ChartRequest {
            birth_profile: BirthProfile {
                display_name: None, sex: Sex::Unspecified, privacy_level: PrivacyLevel::Private,
                calendar_kind: CalendarKind::Gregorian,
                date: CivilDate::parse_iso("2025-01-01").unwrap(),
                time: Some(BirthTime { hour: 10, minute: 30 }),
                time_precision: TimePrecision::Exact,
                timezone: "Asia/Shanghai".into(),
                is_leap_month: false, use_true_solar_time: false,
            },
        };
        let basis = ChartBasis::build(req).unwrap();
        ChartResult::build(basis, DateLayerPillars {
            year: "甲辰".into(), month: "丙子".into(), day: "庚午".into(),
        }).unwrap()
    }

    #[test]
    fn strength_assessment_runs() {
        let chart = sample_chart();
        let s = assess_strength(&chart);
        assert!(s.score <= 10);
        assert!(!s.level.is_empty());
    }

    #[test]
    fn pattern_classification_runs() {
        let chart = sample_chart();
        let s = assess_strength(&chart);
        let p = classify_pattern(&chart, &s);
        assert!(!p.pattern_name.is_empty());
    }

    #[test]
    fn useful_god_suggestions() {
        let chart = sample_chart();
        let s = assess_strength(&chart);
        let p = classify_pattern(&chart, &s);
        let hints = suggest_useful_god(&s, &p);
        assert!(!hints.is_empty());
    }
}
