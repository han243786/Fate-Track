use crate::domain::bazi::{ChartResult, Pillar, TimePrecision};

pub const ANALYSIS_ALGO_VERSION: &str = "structured-analysis-v1";
pub const DISCLAIMER_ID: &str = "traditional-interpretation-not-professional-advice-v1";

const FORBIDDEN_PATTERNS: [&str; 13] = [
    "diagnosis",
    "disease",
    "death",
    "fertility",
    "lawsuit",
    "legal advice",
    "investment advice",
    "guaranteed wealth",
    "guaranteed loss",
    "divorce is certain",
    "criminal",
    "abuse certainty",
    "destiny requires",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WeightedMetric {
    pub id: &'static str,
    pub weight_x2: u16,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnalysisCard {
    pub id: &'static str,
    pub title: &'static str,
    pub severity: &'static str,
    pub body: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnalysisSnapshot {
    pub algo_version: &'static str,
    pub disclaimer_id: &'static str,
    pub day_master: String,
    pub element_metrics: Vec<WeightedMetric>,
    pub ten_god_metrics: Vec<WeightedMetric>,
    pub hidden_stem_metrics: Vec<WeightedMetric>,
    pub relation_flags: Vec<&'static str>,
    pub sensitivity_flags: Vec<&'static str>,
    pub cards: Vec<AnalysisCard>,
    pub forbidden_output_audit: ForbiddenOutputAudit,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForbiddenOutputAudit {
    pub status: &'static str,
    pub checked_patterns: usize,
}

impl AnalysisSnapshot {
    pub fn build(chart: &ChartResult) -> Self {
        let day_master = chart.chart.day.stem.clone();
        let mut element_metrics = metric_seed(&["wood", "fire", "earth", "metal", "water"]);
        let mut ten_god_metrics = metric_seed(&[
            "peer",
            "rob_wealth",
            "eating_god",
            "hurting_officer",
            "direct_wealth",
            "indirect_wealth",
            "direct_officer",
            "seven_killings",
            "direct_resource",
            "indirect_resource",
        ]);
        let mut hidden_stem_metrics = metric_seed(&[
            "year_branch_hidden",
            "month_branch_hidden",
            "day_branch_hidden",
            "hour_branch_hidden",
        ]);

        let visible = visible_pillars(chart);
        // Each visible stem counts as 1 (not 2) — 4 stems total
        for pillar in &visible {
            add_metric(&mut element_metrics, stem_element(&pillar.stem), 1);
            add_metric(&mut ten_god_metrics, ten_god(&day_master, &pillar.stem), 1);
        }

        // Only the primary qi (本气, first hidden stem) of each branch
        // contributes to element and ten-god metrics.
        // All hidden stems still recorded in hidden_stem_metrics for display.
        for (index, pillar) in visible.iter().enumerate() {
            let hidden_metric = match index {
                0 => "year_branch_hidden",
                1 => "month_branch_hidden",
                2 => "day_branch_hidden",
                _ => "hour_branch_hidden",
            };
            let all_stems = hidden_stems(&pillar.branch);
            // Primary qi only (first hidden stem) for element/ten-god counting
            if let Some(&primary) = all_stems.first() {
                add_metric(&mut element_metrics, stem_element(primary), 1);
                add_metric(&mut ten_god_metrics, ten_god(&day_master, primary), 1);
            }
            // All hidden stems recorded in hidden_stem_metrics for transparency
            for stem in all_stems {
                add_metric(&mut hidden_stem_metrics, hidden_metric, 1);
            }
        }

        // With 4 stems + 4 primary branch qi = 8 total, concentration > half
        let mut relation_flags = Vec::new();
        if max_metric(&element_metrics) >= 4 {
            relation_flags.push("element_concentration");
        }
        if min_positive_metric(&element_metrics) == 0 {
            relation_flags.push("element_low_presence");
        }

        let mut sensitivity_flags = Vec::new();
        if matches!(
            chart.basis.request.birth_profile.time_precision,
            TimePrecision::Unknown
        ) {
            sensitivity_flags.push("unknown_hour_affects_hour_pillar_and_hidden_stems");
        }
        if !chart.ambiguity_flags.is_empty() {
            sensitivity_flags.push("chart_has_ambiguity_flags");
        }

        let cards = vec![
            AnalysisCard {
                id: "structure-summary",
                title: "Structure Summary",
                severity: "info",
                body: "This card summarizes deterministic chart metrics under the selected ruleset.",
            },
            AnalysisCard {
                id: "safety-boundary",
                title: "Safety Boundary",
                severity: "notice",
                body: "This analysis is traditional and interpretive, not professional advice.",
            },
        ];
        let forbidden_output_audit = audit_cards(&cards);

        Self {
            algo_version: ANALYSIS_ALGO_VERSION,
            disclaimer_id: DISCLAIMER_ID,
            day_master,
            element_metrics,
            ten_god_metrics,
            hidden_stem_metrics,
            relation_flags,
            sensitivity_flags,
            cards,
            forbidden_output_audit,
        }
    }
}

pub fn audit_text(value: &str) -> ForbiddenOutputAudit {
    let lower = value.to_ascii_lowercase();
    let status = if FORBIDDEN_PATTERNS
        .iter()
        .any(|pattern| lower.contains(pattern))
    {
        "rejected"
    } else {
        "passed"
    };

    ForbiddenOutputAudit {
        status,
        checked_patterns: FORBIDDEN_PATTERNS.len(),
    }
}

fn audit_cards(cards: &[AnalysisCard]) -> ForbiddenOutputAudit {
    let joined = cards
        .iter()
        .map(|card| format!("{} {} {}", card.title, card.severity, card.body))
        .collect::<Vec<_>>()
        .join("\n");
    audit_text(&joined)
}

fn visible_pillars(chart: &ChartResult) -> Vec<Pillar> {
    let mut pillars = vec![
        chart.chart.year.clone(),
        chart.chart.month.clone(),
        chart.chart.day.clone(),
    ];
    if let Some(hour) = &chart.chart.hour {
        pillars.push(hour.clone());
    }
    pillars
}

fn metric_seed(ids: &[&'static str]) -> Vec<WeightedMetric> {
    ids.iter()
        .map(|id| WeightedMetric { id, weight_x2: 0 })
        .collect()
}

fn add_metric(metrics: &mut [WeightedMetric], id: &'static str, weight_x2: u16) {
    if let Some(metric) = metrics.iter_mut().find(|metric| metric.id == id) {
        metric.weight_x2 += weight_x2;
    }
}

fn max_metric(metrics: &[WeightedMetric]) -> u16 {
    metrics
        .iter()
        .map(|metric| metric.weight_x2)
        .max()
        .unwrap_or(0)
}

fn min_positive_metric(metrics: &[WeightedMetric]) -> u16 {
    metrics
        .iter()
        .filter(|metric| metric.weight_x2 > 0)
        .map(|metric| metric.weight_x2)
        .min()
        .unwrap_or(0)
}

fn stem_element(stem: &str) -> &'static str {
    match stem {
        "\u{7532}" | "\u{4e59}" => "wood",
        "\u{4e19}" | "\u{4e01}" => "fire",
        "\u{620a}" | "\u{5df1}" => "earth",
        "\u{5e9a}" | "\u{8f9b}" => "metal",
        "\u{58ec}" | "\u{7678}" => "water",
        _ => "earth",
    }
}

fn stem_yang(stem: &str) -> bool {
    matches!(
        stem,
        "\u{7532}" | "\u{4e19}" | "\u{620a}" | "\u{5e9a}" | "\u{58ec}"
    )
}

fn ten_god(day_stem: &str, other_stem: &str) -> &'static str {
    let day_element = stem_element(day_stem);
    let other_element = stem_element(other_stem);
    let same_polarity = stem_yang(day_stem) == stem_yang(other_stem);

    if day_element == other_element {
        return if same_polarity { "peer" } else { "rob_wealth" };
    }
    if generates(day_element, other_element) {
        return if same_polarity {
            "eating_god"
        } else {
            "hurting_officer"
        };
    }
    if generates(other_element, day_element) {
        return if same_polarity {
            "indirect_resource"
        } else {
            "direct_resource"
        };
    }
    if controls(day_element, other_element) {
        return if same_polarity {
            "indirect_wealth"
        } else {
            "direct_wealth"
        };
    }
    if same_polarity {
        "seven_killings"
    } else {
        "direct_officer"
    }
}

fn generates(a: &str, b: &str) -> bool {
    matches!(
        (a, b),
        ("wood", "fire")
            | ("fire", "earth")
            | ("earth", "metal")
            | ("metal", "water")
            | ("water", "wood")
    )
}

fn controls(a: &str, b: &str) -> bool {
    matches!(
        (a, b),
        ("wood", "earth")
            | ("earth", "water")
            | ("water", "fire")
            | ("fire", "metal")
            | ("metal", "wood")
    )
}

fn hidden_stems(branch: &str) -> &'static [&'static str] {
    match branch {
        "\u{5b50}" => &["\u{7678}"],
        "\u{4e11}" => &["\u{5df1}", "\u{7678}", "\u{8f9b}"],
        "\u{5bc5}" => &["\u{7532}", "\u{4e19}", "\u{620a}"],
        "\u{536f}" => &["\u{4e59}"],
        "\u{8fb0}" => &["\u{620a}", "\u{4e59}", "\u{7678}"],
        "\u{5df3}" => &["\u{4e19}", "\u{620a}", "\u{5e9a}"],
        "\u{5348}" => &["\u{4e01}", "\u{5df1}"],
        "\u{672a}" => &["\u{5df1}", "\u{4e01}", "\u{4e59}"],
        "\u{7533}" => &["\u{5e9a}", "\u{58ec}", "\u{620a}"],
        "\u{9149}" => &["\u{8f9b}"],
        "\u{620c}" => &["\u{620a}", "\u{8f9b}", "\u{4e01}"],
        "\u{4ea5}" => &["\u{58ec}", "\u{7532}"],
        _ => &[],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::civil::CivilDate;
    use crate::domain::bazi::{
        BirthProfile, BirthTime, CalendarKind, ChartBasis, ChartRequest, DateLayerPillars,
        PrivacyLevel, Sex, TimePrecision,
    };

    fn chart() -> ChartResult {
        let request = ChartRequest {
            birth_profile: BirthProfile {
                display_name: None,
                sex: Sex::Unspecified,
                privacy_level: PrivacyLevel::Private,
                calendar_kind: CalendarKind::Gregorian,
                date: CivilDate::parse_iso("2025-01-01").unwrap(),
                time: Some(BirthTime {
                    hour: 10,
                    minute: 30,
                }),
                time_precision: TimePrecision::Exact,
                timezone: "Asia/Shanghai".to_string(),
                is_leap_month: false,
                use_true_solar_time: false,
            },
        };
        let basis = ChartBasis::build(request).unwrap();
        ChartResult::build(
            basis,
            DateLayerPillars {
                year: "\u{7532}\u{8fb0}".to_string(),
                month: "\u{4e59}\u{4e11}".to_string(),
                day: "\u{5e9a}\u{5348}".to_string(),
            },
        )
        .unwrap()
    }

    #[test]
    fn computes_structured_element_and_ten_god_metrics() {
        let snapshot = AnalysisSnapshot::build(&chart());

        assert_eq!(snapshot.day_master, "\u{5e9a}");
        // New model: 4 stems + 4 primary branch qi = 8 total
        // 甲辰/乙丑/庚午/辛巳 → wood:2, fire:2, earth:2, metal:2, water:0
        assert!(
            snapshot
                .element_metrics
                .iter()
                .any(|metric| metric.id == "metal" && metric.weight_x2 >= 2)
        );
        assert!(
            snapshot
                .element_metrics
                .iter()
                .any(|metric| metric.id == "wood" && metric.weight_x2 >= 2)
        );
        assert!(
            snapshot
                .ten_god_metrics
                .iter()
                .any(|metric| metric.id == "direct_wealth" && metric.weight_x2 > 0)
        );
        assert_eq!(snapshot.forbidden_output_audit.status, "passed");
    }

    #[test]
    fn forbidden_output_audit_rejects_high_risk_claims() {
        let audit = audit_text("This is guaranteed wealth and medical diagnosis.");

        assert_eq!(audit.status, "rejected");
        assert!(audit.checked_patterns > 5);
    }
}
