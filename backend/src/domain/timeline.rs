// M35: Internal timeline lexicon and compositional rule engine.
// This module is intentionally domain-only: no public route, UI, capability
// declaration, or mutation of the raw luck-cycle API is introduced here.

use crate::domain::analysis::{ForbiddenOutputAudit, audit_text};
use crate::domain::bazi::{ChartResult, Pillar, RULESET_ID};
use crate::domain::luck::LuckCycle;

pub const TIMELINE_RULE_VERSION: &str = "timeline-core-v1";
pub const TIMELINE_DISCLAIMER_ID: &str = "timeline-reading-structure-not-prediction-v1";

const TIMELINE_BOUNDARY: &str =
    "这是命理结构阅读，不是现实事件预告，也不构成财务、婚恋、家庭、职业、医疗或法律建议。";

const TIMELINE_FORBIDDEN_PATTERNS: [&str; 39] = [
    "必然结婚",
    "一定结婚",
    "必然离婚",
    "一定离婚",
    "会离婚",
    "出轨",
    "分手",
    "复合",
    "保证发财",
    "必然发财",
    "必定发财",
    "一定发财",
    "一定破财",
    "会破产",
    "稳赚",
    "必然亏损",
    "具体金额",
    "亲属会生病",
    "亲属会死亡",
    "必然生育",
    "家庭离散",
    "必然冲突",
    "必升职",
    "一定升职",
    "必然升职",
    "必然失业",
    "跳槽必成",
    "行业选择是",
    "考试必过",
    "收入必涨",
    "结果保证",
    "确定发生",
    "完整流月",
    "流月运势",
    "流日运势",
    "每日运势",
    "推到台前",
    "走到台前",
    "拿到台前",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimelineRuleVersion {
    pub ruleset_id: &'static str,
    pub version: &'static str,
    pub disclaimer_id: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimelineLexiconEntry {
    pub id: &'static str,
    pub category: &'static str,
    pub label_zh: &'static str,
    pub professional_zh: &'static str,
    pub plain_zh: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimelineSignal {
    pub id: String,
    pub label: String,
    pub category: &'static str,
    pub qualitative_level: &'static str,
    pub source: &'static str,
    pub applies_to_topics: Vec<&'static str>,
    pub risk_tags: Vec<&'static str>,
    pub summary: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimelineEvidence {
    pub id: String,
    pub signal_id: String,
    pub source: &'static str,
    pub relation: String,
    pub chart_anchor: String,
    pub trigger: String,
    pub detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlainReading {
    pub id: String,
    pub signal_id: String,
    pub professional: String,
    pub plain: String,
    pub boundary: &'static str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TimelineReadingDraft {
    pub rule_version: TimelineRuleVersion,
    pub signals: Vec<TimelineSignal>,
    pub evidence: Vec<TimelineEvidence>,
    pub readings: Vec<PlainReading>,
    pub warnings: Vec<&'static str>,
    pub audit: ForbiddenOutputAudit,
}

pub fn timeline_lexicon() -> &'static [TimelineLexiconEntry] {
    &TIMELINE_LEXICON
}

pub fn lexicon_entry(id: &str) -> Option<&'static TimelineLexiconEntry> {
    TIMELINE_LEXICON.iter().find(|entry| entry.id == id)
}

pub fn build_timeline_foundation(
    chart: &ChartResult,
    luck_cycles: &[LuckCycle],
    annual: Option<&Pillar>,
) -> TimelineReadingDraft {
    let mut signals = Vec::new();
    let mut evidence = Vec::new();
    let mut readings = Vec::new();
    let mut warnings = Vec::new();

    if chart.chart.hour.is_none() {
        warnings.push("unknown_hour_timeline_evidence_downgraded");
    }
    if annual.is_none() {
        warnings.push("annual_trigger_not_requested");
    }

    if let Some(first_luck) = luck_cycles.first() {
        let label = format!(
            "{}({}-{}岁)",
            readable_luck_cycle_label(first_luck),
            first_luck.start_age,
            first_luck.end_age
        );
        collect_pillar_trigger(
            chart,
            &first_luck.pillar,
            "major-luck",
            &label,
            &mut signals,
            &mut evidence,
            &mut readings,
        );
    } else {
        warnings.push("major_luck_cycle_missing");
    }

    if let Some(annual_pillar) = annual {
        let label = format!("年柱{}", annual_pillar.ganzhi());
        collect_pillar_trigger(
            chart,
            annual_pillar,
            "annual-trigger",
            &label,
            &mut signals,
            &mut evidence,
            &mut readings,
        );

        if let Some(first_luck) = luck_cycles.first() {
            collect_luck_annual_overlay(
                &first_luck.pillar,
                annual_pillar,
                None,
                &mut signals,
                &mut evidence,
                &mut readings,
            );
        }
    }

    let audit = audit_timeline_text(&draft_text(&signals, &evidence, &readings));

    TimelineReadingDraft {
        rule_version: TimelineRuleVersion {
            ruleset_id: RULESET_ID,
            version: TIMELINE_RULE_VERSION,
            disclaimer_id: TIMELINE_DISCLAIMER_ID,
        },
        signals,
        evidence,
        readings,
        warnings,
        audit,
    }
}

pub fn build_major_luck_stage_foundation(
    chart: &ChartResult,
    cycle: &LuckCycle,
    stage_role: &'static str,
) -> TimelineReadingDraft {
    let mut signals = Vec::new();
    let mut evidence = Vec::new();
    let mut readings = Vec::new();
    let mut warnings = Vec::new();

    if chart.chart.hour.is_none() {
        warnings.push("unknown_hour_timeline_evidence_downgraded");
    }

    let source = match stage_role {
        "previous" => "major-luck-previous",
        "next" => "major-luck-next",
        "current" => "major-luck-current",
        _ => "major-luck-stage",
    };
    let label = match stage_role {
        "previous" => "上一阶段大运",
        "next" => "下一阶段大运",
        "current" => "当前阶段大运",
        _ => "大运阶段",
    };
    let source_label = format!(
        "{}{}({}-{}岁)",
        label,
        readable_luck_cycle_label(cycle),
        cycle.start_age,
        cycle.end_age
    );

    collect_pillar_trigger(
        chart,
        &cycle.pillar,
        source,
        &source_label,
        &mut signals,
        &mut evidence,
        &mut readings,
    );

    let audit = audit_timeline_text(&draft_text(&signals, &evidence, &readings));

    TimelineReadingDraft {
        rule_version: TimelineRuleVersion {
            ruleset_id: RULESET_ID,
            version: TIMELINE_RULE_VERSION,
            disclaimer_id: TIMELINE_DISCLAIMER_ID,
        },
        signals,
        evidence,
        readings,
        warnings,
        audit,
    }
}

pub fn build_annual_trigger_foundation(
    chart: &ChartResult,
    current_luck: &LuckCycle,
    annual: &Pillar,
    year: i32,
) -> TimelineReadingDraft {
    let mut signals = Vec::new();
    let mut evidence = Vec::new();
    let mut readings = Vec::new();
    let mut warnings = Vec::new();

    if chart.chart.hour.is_none() {
        warnings.push("unknown_hour_timeline_evidence_downgraded");
    }

    let annual_label = format!("{}年年柱{}", year, annual.ganzhi());
    collect_pillar_trigger(
        chart,
        annual,
        "annual-trigger",
        &annual_label,
        &mut signals,
        &mut evidence,
        &mut readings,
    );

    let luck_label = format!(
        "当前大运{}({}-{}岁)",
        readable_luck_cycle_label(current_luck),
        current_luck.start_age,
        current_luck.end_age
    );
    collect_pillar_trigger(
        chart,
        &current_luck.pillar,
        "annual-current-luck",
        &luck_label,
        &mut signals,
        &mut evidence,
        &mut readings,
    );
    collect_luck_annual_overlay(
        &current_luck.pillar,
        annual,
        Some(year),
        &mut signals,
        &mut evidence,
        &mut readings,
    );

    let audit = audit_timeline_text(&draft_text(&signals, &evidence, &readings));

    TimelineReadingDraft {
        rule_version: TimelineRuleVersion {
            ruleset_id: RULESET_ID,
            version: TIMELINE_RULE_VERSION,
            disclaimer_id: TIMELINE_DISCLAIMER_ID,
        },
        signals,
        evidence,
        readings,
        warnings,
        audit,
    }
}

pub fn audit_timeline_text(value: &str) -> ForbiddenOutputAudit {
    let base = audit_text(value);
    let local_rejected = TIMELINE_FORBIDDEN_PATTERNS
        .iter()
        .any(|pattern| value.contains(pattern));

    ForbiddenOutputAudit {
        status: if base.status == "rejected" || local_rejected {
            "rejected"
        } else {
            "passed"
        },
        checked_patterns: base.checked_patterns + TIMELINE_FORBIDDEN_PATTERNS.len(),
    }
}

fn readable_luck_cycle_label(cycle: &LuckCycle) -> String {
    format!(
        "「{}·{}」",
        readable_luck_label(&cycle.label),
        cycle.pillar.ganzhi()
    )
}

fn readable_luck_label(label: &str) -> String {
    let mut text = label.to_string();
    for index in 1..=10 {
        text = text.replace(
            &format!("第{index}运"),
            &format!("{}运", ordinal_text(index)),
        );
    }
    text
}

fn ordinal_text(value: usize) -> String {
    match value {
        1 => "第一".to_string(),
        2 => "第二".to_string(),
        3 => "第三".to_string(),
        4 => "第四".to_string(),
        5 => "第五".to_string(),
        6 => "第六".to_string(),
        7 => "第七".to_string(),
        8 => "第八".to_string(),
        9 => "第九".to_string(),
        10 => "第十".to_string(),
        _ => format!("第{value}"),
    }
}

fn collect_pillar_trigger(
    chart: &ChartResult,
    trigger: &Pillar,
    source: &'static str,
    source_label: &str,
    signals: &mut Vec<TimelineSignal>,
    evidence: &mut Vec<TimelineEvidence>,
    readings: &mut Vec<PlainReading>,
) {
    collect_stem_ten_god(
        chart,
        trigger,
        source,
        source_label,
        signals,
        evidence,
        readings,
    );
    collect_stem_element_relation(
        chart,
        trigger,
        source,
        source_label,
        signals,
        evidence,
        readings,
    );
    collect_hidden_stem_signal(
        chart,
        trigger,
        source,
        source_label,
        signals,
        evidence,
        readings,
    );
    collect_branch_relations(
        chart,
        trigger,
        source,
        source_label,
        signals,
        evidence,
        readings,
    );
}

fn collect_stem_ten_god(
    chart: &ChartResult,
    trigger: &Pillar,
    source: &'static str,
    source_label: &str,
    signals: &mut Vec<TimelineSignal>,
    evidence: &mut Vec<TimelineEvidence>,
    readings: &mut Vec<PlainReading>,
) {
    let signal_id = format!("{source}-stem-ten-god");
    let ten_god = ten_god_id(&chart.chart.day.stem, &trigger.stem);
    let ten_god_label = ten_god_label(ten_god);
    let summary = format!(
        "{source_label}天干「{}」对日主「{}」形成「{}」结构。",
        trigger.stem, chart.chart.day.stem, ten_god_label
    );

    signals.push(TimelineSignal {
        id: signal_id.clone(),
        label: format!("{source_label}十神触发"),
        category: "ten-god",
        qualitative_level: "存在",
        source,
        applies_to_topics: topics_for_ten_god(ten_god),
        risk_tags: vec!["interpretation-only", "no-deterministic-claim"],
        summary: summary.clone(),
    });

    evidence.push(TimelineEvidence {
        id: format!("{signal_id}-evidence"),
        signal_id: signal_id.clone(),
        source,
        relation: ten_god_label.to_string(),
        chart_anchor: format!("日主「{}」", chart.chart.day.stem),
        trigger: format!("{source_label}天干「{}」", trigger.stem),
        detail: format!("以日主为中心，{}。", summary),
    });

    readings.push(PlainReading {
        id: format!("{signal_id}-reading"),
        signal_id,
        professional: format!(
            "从命理结构看，{source_label}显干「{}」以日主「{}」为参照，落在「{}」。{}",
            trigger.stem,
            chart.chart.day.stem,
            ten_god_label,
            professional_for_ten_god(ten_god)
        ),
        plain: format!(
            "{}信号偏向{}。{}",
            ten_god_label,
            plain_for_ten_god(ten_god),
            guidance_for_ten_god(ten_god)
        ),
        boundary: TIMELINE_BOUNDARY,
    });
}

fn collect_stem_element_relation(
    chart: &ChartResult,
    trigger: &Pillar,
    source: &'static str,
    source_label: &str,
    signals: &mut Vec<TimelineSignal>,
    evidence: &mut Vec<TimelineEvidence>,
    readings: &mut Vec<PlainReading>,
) {
    let signal_id = format!("{source}-stem-element-relation");
    let day_element = stem_element(&chart.chart.day.stem);
    let trigger_element = stem_element(&trigger.stem);
    let relation = element_relation(day_element, trigger_element);
    let summary = format!(
        "日主五行「{}」与{source_label}天干五行「{}」形成「{}」。",
        element_label(day_element),
        element_label(trigger_element),
        relation
    );

    signals.push(TimelineSignal {
        id: signal_id.clone(),
        label: format!("{source_label}五行流向"),
        category: "five-element",
        qualitative_level: "存在",
        source,
        applies_to_topics: vec!["relationship", "wealth", "family", "career"],
        risk_tags: vec!["qualitative-only", "no-score"],
        summary: summary.clone(),
    });

    evidence.push(TimelineEvidence {
        id: format!("{signal_id}-evidence"),
        signal_id: signal_id.clone(),
        source,
        relation: relation.to_string(),
        chart_anchor: format!("日主五行「{}」", element_label(day_element)),
        trigger: format!(
            "{source_label}天干五行「{}」",
            element_label(trigger_element)
        ),
        detail: summary,
    });

    readings.push(PlainReading {
        id: format!("{signal_id}-reading"),
        signal_id,
        professional: format!(
            "从五行流向看，日主五行「{}」与{source_label}天干五行「{}」形成「{}」。{}",
            element_label(day_element),
            element_label(trigger_element),
            relation,
            professional_for_element_relation(relation)
        ),
        plain: format!(
            "{}信号提示{}。{}",
            relation,
            plain_for_element_relation(relation),
            guidance_for_element_relation(relation)
        ),
        boundary: TIMELINE_BOUNDARY,
    });
}

fn collect_hidden_stem_signal(
    chart: &ChartResult,
    trigger: &Pillar,
    source: &'static str,
    source_label: &str,
    signals: &mut Vec<TimelineSignal>,
    evidence: &mut Vec<TimelineEvidence>,
    readings: &mut Vec<PlainReading>,
) {
    let hidden = hidden_stems(&trigger.branch);
    if hidden.is_empty() {
        return;
    }

    let signal_id = format!("{source}-branch-hidden-stems");
    let hidden_labels = hidden.join("、");
    let hidden_ten_gods = hidden
        .iter()
        .map(|stem| ten_god_label(ten_god_id(&chart.chart.day.stem, stem)))
        .collect::<Vec<_>>()
        .join("、");

    signals.push(TimelineSignal {
        id: signal_id.clone(),
        label: format!("{source_label}藏干背景"),
        category: "hidden-stem",
        qualitative_level: "背景",
        source,
        applies_to_topics: vec!["relationship", "wealth", "family", "career"],
        risk_tags: vec!["background-signal", "not-event"],
        summary: format!(
            "{source_label}地支「{}」藏干为「{}」，折算十神背景为「{}」。",
            trigger.branch, hidden_labels, hidden_ten_gods
        ),
    });

    evidence.push(TimelineEvidence {
        id: format!("{signal_id}-evidence"),
        signal_id: signal_id.clone(),
        source,
        relation: "藏干".to_string(),
        chart_anchor: format!("日主「{}」", chart.chart.day.stem),
        trigger: format!("{source_label}地支「{}」", trigger.branch),
        detail: format!(
            "藏干「{}」按日主换算为「{}」。",
            hidden_labels, hidden_ten_gods
        ),
    });

    readings.push(PlainReading {
        id: format!("{signal_id}-reading"),
        signal_id,
        professional: format!(
            "从藏干看，{source_label}地支「{}」不只看表面地支，也要看内部所藏天干「{}」；这些藏干再折算为「{}」的十神背景。",
            trigger.branch, hidden_labels, hidden_ten_gods
        ),
        plain: "藏干信号像地支内部保留的底色，会把表面关系往更深一层带；您适合把它当作辅证，不宜只凭这一层推出具体事件。".to_string(),
        boundary: TIMELINE_BOUNDARY,
    });
}

fn collect_branch_relations(
    chart: &ChartResult,
    trigger: &Pillar,
    source: &'static str,
    source_label: &str,
    signals: &mut Vec<TimelineSignal>,
    evidence: &mut Vec<TimelineEvidence>,
    readings: &mut Vec<PlainReading>,
) {
    for (anchor_label, anchor_branch) in chart_branch_anchors(chart) {
        if let Some(relation) = branch_relation(&anchor_branch, &trigger.branch) {
            let signal_id = format!(
                "{source}-{}-branch-relation",
                anchor_label_to_id(anchor_label)
            );
            signals.push(TimelineSignal {
                id: signal_id.clone(),
                label: format!("{source_label}{anchor_label}关系"),
                category: "branch-relation",
                qualitative_level: relation_level(relation),
                source,
                applies_to_topics: topics_for_anchor(anchor_label),
                risk_tags: vec!["relation-signal", "no-event-prediction"],
                summary: format!(
                    "原局{anchor_label}「{}」与{source_label}地支「{}」形成「{}」。",
                    anchor_branch, trigger.branch, relation
                ),
            });

            evidence.push(TimelineEvidence {
                id: format!("{signal_id}-evidence"),
                signal_id: signal_id.clone(),
                source,
                relation: relation.to_string(),
                chart_anchor: format!("原局{anchor_label}「{}」", anchor_branch),
                trigger: format!("{source_label}地支「{}」", trigger.branch),
                detail: format!("{anchor_branch} 与 {} = {relation}", trigger.branch),
            });

            readings.push(PlainReading {
                id: format!("{signal_id}-reading"),
                signal_id,
                professional: format!(
                    "从地支关系看，原局{anchor_label}「{}」与{source_label}地支「{}」出现「{}」。{}",
                    anchor_branch,
                    trigger.branch,
                    relation,
                    professional_for_branch_relation(relation)
                ),
                plain: format!(
                    "{}信号提示{}。{}",
                    relation,
                    plain_for_branch_relation(relation),
                    guidance_for_branch_relation(relation)
                ),
                boundary: TIMELINE_BOUNDARY,
            });
        }
    }
}

fn collect_luck_annual_overlay(
    luck: &Pillar,
    annual: &Pillar,
    annual_year: Option<i32>,
    signals: &mut Vec<TimelineSignal>,
    evidence: &mut Vec<TimelineEvidence>,
    readings: &mut Vec<PlainReading>,
) {
    if let Some(relation) = branch_relation(&luck.branch, &annual.branch) {
        let signal_id = "major-luck-annual-branch-relation".to_string();
        let annual_label = annual_year
            .map(|year| format!("{year}年"))
            .unwrap_or_else(|| "流年".to_string());
        signals.push(TimelineSignal {
            id: signal_id.clone(),
            label: "大运与流年地支叠加".to_string(),
            category: "time-overlay",
            qualitative_level: relation_level(relation),
            source: "major-luck+annual-trigger",
            applies_to_topics: vec!["relationship", "wealth", "family", "career"],
            risk_tags: vec!["overlay-only", "no-flow-month-day"],
            summary: format!(
                "大运地支「{}」与{}地支「{}」形成「{}」。",
                luck.branch, annual_label, annual.branch, relation
            ),
        });

        evidence.push(TimelineEvidence {
            id: format!("{signal_id}-evidence"),
            signal_id: signal_id.clone(),
            source: "major-luck+annual-trigger",
            relation: relation.to_string(),
            chart_anchor: format!("大运地支「{}」", luck.branch),
            trigger: format!("{}地支「{}」", annual_label, annual.branch),
            detail: format!(
                "大运「{}」与{}「{}」发生地支关系「{}」。",
                luck.ganzhi(),
                annual_label,
                annual.ganzhi(),
                relation
            ),
        });

        readings.push(PlainReading {
            id: format!("{signal_id}-reading"),
            signal_id,
            professional: format!(
                "从时间叠加看，大运地支「{}」与{}地支「{}」出现「{}」。{}",
                luck.branch,
                annual_label,
                annual.branch,
                relation,
                professional_for_branch_relation(relation)
            ),
            plain: format!(
                "{}信号提示{}。十年背景与{}入口在这里交会，您可以观察阶段底色和年度触发点怎样互相照面；它不提供月份、日期或事件时间表。",
                relation,
                plain_for_branch_relation(relation),
                annual_label
            ),
            boundary: TIMELINE_BOUNDARY,
        });
    }
}

fn draft_text(
    signals: &[TimelineSignal],
    evidence: &[TimelineEvidence],
    readings: &[PlainReading],
) -> String {
    let mut parts = Vec::new();
    parts.extend(signals.iter().map(|signal| signal.summary.clone()));
    parts.extend(evidence.iter().map(|item| item.detail.clone()));
    parts.extend(readings.iter().flat_map(|reading| {
        [
            reading.professional.clone(),
            reading.plain.clone(),
            reading.boundary.to_string(),
        ]
    }));
    parts.join("\n")
}

fn chart_branch_anchors(chart: &ChartResult) -> Vec<(&'static str, String)> {
    let mut anchors = vec![
        ("年支", chart.chart.year.branch.clone()),
        ("月支", chart.chart.month.branch.clone()),
        ("日支", chart.chart.day.branch.clone()),
    ];
    if let Some(hour) = &chart.chart.hour {
        anchors.push(("时支", hour.branch.clone()));
    }
    anchors
}

fn anchor_label_to_id(value: &str) -> &'static str {
    match value {
        "年支" => "year",
        "月支" => "month",
        "日支" => "day",
        "时支" => "hour",
        _ => "unknown",
    }
}

fn topics_for_anchor(value: &str) -> Vec<&'static str> {
    match value {
        "年支" => vec!["family", "career"],
        "月支" => vec!["career", "family"],
        "日支" => vec!["relationship"],
        "时支" => vec!["family", "career"],
        _ => vec!["relationship", "wealth", "family", "career"],
    }
}

fn topics_for_ten_god(id: &str) -> Vec<&'static str> {
    match id {
        "direct_wealth" | "indirect_wealth" => vec!["wealth", "career"],
        "direct_officer" | "seven_killings" => vec!["career", "relationship"],
        "direct_resource" | "indirect_resource" => vec!["family", "career"],
        "eating_god" | "hurting_officer" => vec!["career", "wealth", "relationship"],
        "peer" | "rob_wealth" => vec!["family", "wealth", "career"],
        _ => vec!["relationship", "wealth", "family", "career"],
    }
}

fn ten_god_id(day_stem: &str, other_stem: &str) -> &'static str {
    let day_element = stem_element(day_stem);
    let other_element = stem_element(other_stem);
    let same_polarity = is_yang(day_stem) == is_yang(other_stem);

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

fn ten_god_label(id: &str) -> &'static str {
    match id {
        "peer" => "比肩",
        "rob_wealth" => "劫财",
        "eating_god" => "食神",
        "hurting_officer" => "伤官",
        "direct_wealth" => "正财",
        "indirect_wealth" => "偏财",
        "direct_officer" => "正官",
        "seven_killings" => "七杀",
        "direct_resource" => "正印",
        "indirect_resource" => "偏印",
        _ => "未知",
    }
}

fn professional_for_ten_god(id: &str) -> &'static str {
    match id {
        "peer" => "比肩与日主同五行同阴阳，主同类并行、自我立场与并肩力量。",
        "rob_wealth" => "劫财与日主同五行异阴阳，主同类分流、竞争协作与资源边界。",
        "eating_god" => "食神为日主所生且同阴阳，主稳定输出、滋养表达与持续成形。",
        "hurting_officer" => "伤官为日主所生且异阴阳，主外放表达、破格意识与才气显露。",
        "direct_wealth" => "正财为日主所克且异阴阳，主可管理资源、现实承载与稳定责任。",
        "indirect_wealth" => "偏财为日主所克且同阴阳，主流动资源、外部机会与调度能力。",
        "direct_officer" => "正官为克日主且异阴阳，主规则秩序、职责位置与外部规范。",
        "seven_killings" => "七杀为克日主且同阴阳，主压力挑战、边界试炼与行动张力。",
        "direct_resource" => "正印为生日主且异阴阳，主支持保护、学习吸收与稳定托举。",
        "indirect_resource" => "偏印为生日主且同阴阳，主特殊理解、侧向支持与内在消化。",
        _ => "十神需要结合日主、透藏位置与原局强弱共同辨认。",
    }
}

fn plain_for_ten_god(id: &str) -> &'static str {
    match id {
        "peer" => "自我立场、同类协力和个人边界",
        "rob_wealth" => "同侪竞争、资源分配和边界再确认",
        "eating_god" => "稳定表达、耐心产出和照料感",
        "hurting_officer" => "表达锋芒、突破旧框和才气显露",
        "direct_wealth" => "现实承载、秩序化资源和长期责任",
        "indirect_wealth" => "流动资源、外部机会和连接能力",
        "direct_officer" => "规则职责、位置秩序和外部要求",
        "seven_killings" => "压力挑战、边界试炼和行动张力",
        "direct_resource" => "支持托举、学习吸收和保护力量",
        "indirect_resource" => "特殊理解、侧向支持和内在消化",
        _ => "需要结合上下文辨认的十神主题",
    }
}

fn guidance_for_ten_god(id: &str) -> &'static str {
    match id {
        "peer" => {
            "您可以看哪些位置需要自己站稳，哪些关系是在并肩而行，也看主见相近时是否容易互不退让。"
        }
        "rob_wealth" => {
            "您可以看资源、注意力或话语权怎样被分走，尤其留意合作尺度与自我保护是否需要重新拿捏。"
        }
        "eating_god" => "您可以看哪些能力适合慢慢输出，哪些照料、作品或表达需要靠耐心养成。",
        "hurting_officer" => {
            "您可以看才气如何被看见，也看锋芒是否容易顶到既有规则，先辨明表达方式再谈结果。"
        }
        "direct_wealth" => {
            "您可以看现实责任、稳定资源和日常经营怎样落到身上，不把它直接等同为收入涨跌。"
        }
        "indirect_wealth" => {
            "您可以看外部机会和人情往来如何打开局面，也看流动资源是否需要更稳的承接方式。"
        }
        "direct_officer" => "您可以看哪些责任需要接住，哪些规则需要理清，哪些位置需要站稳。",
        "seven_killings" => "您可以看压力从哪里逼近，也看边界、胆识和执行力是否更需要被看见。",
        "direct_resource" => "您可以看支持从哪里来，学习吸收是否顺畅，也看自己是否需要先稳住根基。",
        "indirect_resource" => {
            "您可以看侧向理解和内在消化怎样发生，也看思路过深时是否需要回到现实秩序。"
        }
        _ => "您可以把它放回原局强弱、事项宫位和时间触发中一起辨认。",
    }
}

fn stem_element(stem: &str) -> &'static str {
    match stem {
        "甲" | "乙" => "wood",
        "丙" | "丁" => "fire",
        "戊" | "己" => "earth",
        "庚" | "辛" => "metal",
        "壬" | "癸" => "water",
        _ => "earth",
    }
}

fn is_yang(stem: &str) -> bool {
    matches!(stem, "甲" | "丙" | "戊" | "庚" | "壬")
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

fn element_relation(day_element: &str, trigger_element: &str) -> &'static str {
    if day_element == trigger_element {
        "同气"
    } else if generates(trigger_element, day_element) {
        "时间层生扶日主"
    } else if generates(day_element, trigger_element) {
        "日主向时间层输出"
    } else if controls(trigger_element, day_element) {
        "时间层制约日主"
    } else if controls(day_element, trigger_element) {
        "日主制约时间层"
    } else {
        "五行关系未明"
    }
}

fn plain_for_element_relation(relation: &str) -> &'static str {
    match relation {
        "同气" => "同类力量被放大，注意力容易回到相近立场、相近资源或重复出现的气质",
        "时间层生扶日主" => "外部背景向日主补充资源，支持、学习、保护或缓冲力量被看见",
        "日主向时间层输出" => "日主把能力、表达或资源向外释放，产出与展示感会更明显",
        "时间层制约日主" => "外部规则、压力或职责向日主靠近，需要先分辨约束来自哪里",
        "日主制约时间层" => "日主需要承接、管理或处理外部资源，现实经营感会被抬高",
        _ => "五行方向尚不清晰，需要放回十神、地支和事项位置一起辨认",
    }
}

fn professional_for_element_relation(relation: &str) -> &'static str {
    match relation {
        "同气" => "同气不是生克变化，而是同类五行并起，力量会被集中或重复强调。",
        "时间层生扶日主" => "时间层五行生日主五行，表示外部背景对日主形成补益或承托。",
        "日主向时间层输出" => "日主五行生时间层五行，表示日主之气向外泄秀、表达或产出。",
        "时间层制约日主" => "时间层五行克日主五行，表示外部规范、压力或挑战进入观察范围。",
        "日主制约时间层" => "日主五行克时间层五行，表示日主对外部资源、事务或责任形成承接。",
        _ => "五行生克需要结合十神定位与原局强弱共同判断。",
    }
}

fn guidance_for_element_relation(relation: &str) -> &'static str {
    match relation {
        "同气" => "您可以看哪里得到顺势放大，也看同类过多时是否需要分辨轻重。",
        "时间层生扶日主" => {
            "您可以看支撑是否真正可用，也看自己是否愿意接受学习、保护或缓冲。"
        }
        "日主向时间层输出" => {
            "您可以看哪些能力正在被拿出来使用，也看输出之后是否需要休整与回收。"
        }
        "时间层制约日主" => "您可以看规则与压力带来的提醒，不急着把它理解成坏事或结果。",
        "日主制约时间层" => {
            "您可以看自己如何处理资源、事务和责任，重点在承接方式，不把它直接当成成败判断。"
        }
        _ => "您可以把它作为背景线索，再结合十神和地支关系确认轻重。",
    }
}

fn element_label(id: &str) -> &'static str {
    match id {
        "wood" => "木",
        "fire" => "火",
        "earth" => "土",
        "metal" => "金",
        "water" => "水",
        _ => "未知",
    }
}

fn hidden_stems(branch: &str) -> &'static [&'static str] {
    match branch {
        "子" => &["癸"],
        "丑" => &["己", "癸", "辛"],
        "寅" => &["甲", "丙", "戊"],
        "卯" => &["乙"],
        "辰" => &["戊", "乙", "癸"],
        "巳" => &["丙", "戊", "庚"],
        "午" => &["丁", "己"],
        "未" => &["己", "丁", "乙"],
        "申" => &["庚", "壬", "戊"],
        "酉" => &["辛"],
        "戌" => &["戊", "辛", "丁"],
        "亥" => &["壬", "甲"],
        _ => &[],
    }
}

fn branch_relation(a: &str, b: &str) -> Option<&'static str> {
    if a == b && matches!(a, "辰" | "午" | "酉" | "亥") {
        return Some("自刑");
    }
    if pair_matches(
        a,
        b,
        &[
            ("子", "丑"),
            ("寅", "亥"),
            ("卯", "戌"),
            ("辰", "酉"),
            ("巳", "申"),
            ("午", "未"),
        ],
    ) {
        return Some("六合");
    }
    if pair_matches(
        a,
        b,
        &[
            ("子", "午"),
            ("丑", "未"),
            ("寅", "申"),
            ("卯", "酉"),
            ("辰", "戌"),
            ("巳", "亥"),
        ],
    ) {
        return Some("六冲");
    }
    if pair_matches(
        a,
        b,
        &[
            ("子", "未"),
            ("丑", "午"),
            ("寅", "巳"),
            ("卯", "辰"),
            ("申", "亥"),
            ("酉", "戌"),
        ],
    ) {
        return Some("六害");
    }
    if pair_matches(
        a,
        b,
        &[
            ("子", "卯"),
            ("寅", "巳"),
            ("巳", "申"),
            ("丑", "戌"),
            ("戌", "未"),
            ("丑", "未"),
        ],
    ) {
        return Some("三刑");
    }
    None
}

fn pair_matches(a: &str, b: &str, pairs: &[(&str, &str)]) -> bool {
    pairs
        .iter()
        .any(|(left, right)| (a == *left && b == *right) || (a == *right && b == *left))
}

fn relation_level(relation: &str) -> &'static str {
    match relation {
        "六冲" | "三刑" => "较明显",
        "六害" | "自刑" => "需留意",
        "六合" => "温和",
        _ => "存在",
    }
}

fn plain_for_branch_relation(relation: &str) -> &'static str {
    match relation {
        "六合" => "两处结构被牵连、靠近或彼此看见，适合观察连接与协调",
        "六冲" => "两个位置之间出现拉扯、方向差或变化感，适合观察调整与移动",
        "六害" => "暗处不顺手、细节消耗或配合不够顺，适合观察隐性摩擦",
        "三刑" => "规则、边界或反复卡点被强调，适合观察哪里需要重新立规矩",
        "自刑" => "同一类议题在内侧反复回响，适合观察自我消耗与重复模式",
        _ => "地支关系尚不清晰，需要结合其他证据辨认",
    }
}

fn professional_for_branch_relation(relation: &str) -> &'static str {
    match relation {
        "六合" => "六合以牵合为主，代表两个地支位置进入同一观察框。",
        "六冲" => "六冲以相冲为主，代表两个地支位置出现方向差、动象或张力。",
        "六害" => "六害以暗损为主，代表两个地支位置之间有不显眼的耗损或牵制。",
        "三刑" => "三刑以刑象为主，代表规则、边界、重复压力或结构性卡点。",
        "自刑" => "自刑以同支内部反复为主，代表同类议题在自身层面循环加重。",
        _ => "地支关系需要结合原局位置、透藏和时间层来源共同判断。",
    }
}

fn guidance_for_branch_relation(relation: &str) -> &'static str {
    match relation {
        "六合" => "您可以看哪些人事位置被拉近，哪些议题有了协调、看见或互相牵引的机会。",
        "六冲" => "您可以看变化感来自哪里，先辨认方向差与节奏差，再考虑如何调身位。",
        "六害" => "您可以看不顺手的细节在哪里，尤其留意长期的小消耗是否正在累积。",
        "三刑" => "您可以看哪些边界需要重新说明，哪些反复出现的问题其实在要求立规则。",
        "自刑" => "您可以看自己是否在同一个结上反复用力，先降低内耗，再看外部关系。",
        _ => "您可以把它当作结构提示，不宜脱离原局位置单独下结论。",
    }
}

const TIMELINE_LEXICON: [TimelineLexiconEntry; 28] = [
    TimelineLexiconEntry {
        id: "peer",
        category: "ten-god",
        label_zh: "比肩",
        professional_zh: "比肩与日主同五行同阴阳，主同类并行、自我立场和并肩力量。",
        plain_zh: "比肩像身边的同路人，也像心里的主见；适合观察独立性、并肩关系和不愿轻易退让的地方。",
    },
    TimelineLexiconEntry {
        id: "rob_wealth",
        category: "ten-god",
        label_zh: "劫财",
        professional_zh: "劫财与日主同五行异阴阳，主同类分流、竞争协作和资源边界。",
        plain_zh: "劫财像同伴、竞争和分配问题一起出现；适合观察合作尺度、资源分界和自我保护是否清楚。",
    },
    TimelineLexiconEntry {
        id: "eating_god",
        category: "ten-god",
        label_zh: "食神",
        professional_zh: "食神为日主所生且同阴阳，主稳定输出、滋养表达和持续成形。",
        plain_zh: "食神像温和而持续的表达；适合观察技能、照料、作品和能否把事情慢慢养成。",
    },
    TimelineLexiconEntry {
        id: "hurting_officer",
        category: "ten-god",
        label_zh: "伤官",
        professional_zh: "伤官为日主所生且异阴阳，主外放表达、破格意识和才气显露。",
        plain_zh: "伤官像锋利的才气和不愿被旧框困住的表达；适合观察创意、话语方式和与规则的摩擦。",
    },
    TimelineLexiconEntry {
        id: "direct_wealth",
        category: "ten-god",
        label_zh: "正财",
        professional_zh: "正财为日主所克且异阴阳，主可管理资源、现实承载和稳定责任。",
        plain_zh: "正财像看得见、管得住、需要长期打理的现实资源；适合观察预算、秩序、责任和经营耐心。",
    },
    TimelineLexiconEntry {
        id: "indirect_wealth",
        category: "ten-god",
        label_zh: "偏财",
        professional_zh: "偏财为日主所克且同阴阳，主流动资源、外部机会和调度能力。",
        plain_zh: "偏财像外部机会、人情往来和流动资源；适合观察机会意识、资源调度和变化能否被稳稳承接。",
    },
    TimelineLexiconEntry {
        id: "direct_officer",
        category: "ten-god",
        label_zh: "正官",
        professional_zh: "正官为克日主且异阴阳，主规则秩序、职责位置和外部规范。",
        plain_zh: "正官像一套清楚的规矩和位置要求；适合观察责任如何被接住、秩序如何建立、边界是否端正。",
    },
    TimelineLexiconEntry {
        id: "seven_killings",
        category: "ten-god",
        label_zh: "七杀",
        professional_zh: "七杀为克日主且同阴阳，主压力挑战、边界试炼和行动张力。",
        plain_zh: "七杀像迎面而来的压力和挑战；适合观察胆识、执行力、边界感，以及压力是否能被转成行动。",
    },
    TimelineLexiconEntry {
        id: "direct_resource",
        category: "ten-god",
        label_zh: "正印",
        professional_zh: "正印为生日主且异阴阳，主支持保护、学习吸收和稳定托举。",
        plain_zh: "正印像可靠的支撑和安定的学习力；适合观察资源是否能滋养自己，也看根基是否被稳住。",
    },
    TimelineLexiconEntry {
        id: "indirect_resource",
        category: "ten-god",
        label_zh: "偏印",
        professional_zh: "偏印为生日主且同阴阳，主特殊理解、侧向支持和内在消化。",
        plain_zh: "偏印像从侧面来的理解和灵感；适合观察独特思路、内在消化和是否需要回到现实秩序。",
    },
    TimelineLexiconEntry {
        id: "wood",
        category: "five-element",
        label_zh: "木",
        professional_zh: "木主生发条达，参与五行相生相克，并呈现计划、伸展与关系生长。",
        plain_zh: "木像枝条向外伸展；适合观察计划如何展开、关系如何生长，以及方向感是否清楚。",
    },
    TimelineLexiconEntry {
        id: "fire",
        category: "five-element",
        label_zh: "火",
        professional_zh: "火主炎上显明，参与五行相生相克，并呈现表达、热度与被看见。",
        plain_zh: "火像灯被点亮；适合观察表达是否显露、情绪是否升温，以及哪些事情需要被看见。",
    },
    TimelineLexiconEntry {
        id: "earth",
        category: "five-element",
        label_zh: "土",
        professional_zh: "土主承载化育，参与五行相生相克，并呈现稳定、消化与边界。",
        plain_zh: "土像一块能承重的地面；适合观察事务能否落地、责任能否消化、边界是否稳固。",
    },
    TimelineLexiconEntry {
        id: "metal",
        category: "five-element",
        label_zh: "金",
        professional_zh: "金主肃降收敛，参与五行相生相克，并呈现规则、整理与执行。",
        plain_zh: "金像清理和定规矩的力量；适合观察取舍、执行、秩序和需要收束的部分。",
    },
    TimelineLexiconEntry {
        id: "water",
        category: "five-element",
        label_zh: "水",
        professional_zh: "水主润下流动，参与五行相生相克，并呈现信息、学习与变通。",
        plain_zh: "水像流动的信息和弹性；适合观察学习、沟通、变化适应，以及情绪是否需要疏通。",
    },
    TimelineLexiconEntry {
        id: "same-qi",
        category: "five-element-relation",
        label_zh: "同气",
        professional_zh: "同气表示日主五行与时间天干五行相同，同类力量并起而不形成生克转化。",
        plain_zh: "同气像同一股力量被放大；适合观察相近立场、重复气质和同类资源是否需要分清轻重。",
    },
    TimelineLexiconEntry {
        id: "time-supports-day-master",
        category: "five-element-relation",
        label_zh: "时间层生扶日主",
        professional_zh: "时间层五行生日主五行，表示外部背景对日主形成补益、承托或缓冲。",
        plain_zh: "生扶像外界递来支撑；适合观察学习、保护、贵人感和缓冲力量是否真正可用。",
    },
    TimelineLexiconEntry {
        id: "day-master-outputs",
        category: "five-element-relation",
        label_zh: "日主向时间层输出",
        professional_zh: "日主五行生时间层五行，表示日主之气向外泄秀、表达、产出或投入。",
        plain_zh: "输出像把能力拿出来使用；适合观察表达、作品、劳动投入和输出后的休整需要。",
    },
    TimelineLexiconEntry {
        id: "time-controls-day-master",
        category: "five-element-relation",
        label_zh: "时间层制约日主",
        professional_zh: "时间层五行克日主五行，表示外部规范、压力或挑战进入观察范围。",
        plain_zh: "制约像外部规则靠近自己；适合观察压力来源、责任边界和能否把压力化成秩序。",
    },
    TimelineLexiconEntry {
        id: "day-master-controls-time",
        category: "five-element-relation",
        label_zh: "日主制约时间层",
        professional_zh: "日主五行克时间层五行，表示日主对外部资源、事务或责任形成处理与承接。",
        plain_zh: "承载像自己需要接住一件事；适合观察资源管理、现实经营和处理事务的方式。",
    },
    TimelineLexiconEntry {
        id: "six-harmony",
        category: "branch-relation",
        label_zh: "六合",
        professional_zh: "六合以牵合为主，表示两个地支位置进入同一观察框。",
        plain_zh: "六合像两处位置被拉近；适合观察连接、协调、彼此看见，不等于现实合作必成。",
    },
    TimelineLexiconEntry {
        id: "six-clash",
        category: "branch-relation",
        label_zh: "六冲",
        professional_zh: "六冲以相冲为主，表示两个地支位置出现方向差、动象或张力。",
        plain_zh: "六冲像两股力量互相顶住；适合观察变化、移动、节奏差和需要调身位的地方。",
    },
    TimelineLexiconEntry {
        id: "six-harm",
        category: "branch-relation",
        label_zh: "六害",
        professional_zh: "六害以暗损为主，表示两个地支位置之间有不显眼的耗损或牵制。",
        plain_zh: "六害像明面不一定冲突、细节却不顺手；适合观察隐性摩擦和长期小消耗。",
    },
    TimelineLexiconEntry {
        id: "three-punishment",
        category: "branch-relation",
        label_zh: "三刑",
        professional_zh: "三刑以刑象为主，表示规则、边界、重复压力或结构性卡点。",
        plain_zh: "三刑像问题反复绕回同一个结；适合观察哪里需要重新立规矩、哪里不能再含糊过去。",
    },
    TimelineLexiconEntry {
        id: "self-punishment",
        category: "branch-relation",
        label_zh: "自刑",
        professional_zh: "自刑以同支内部反复为主，表示同类议题在自身层面循环加重。",
        plain_zh: "自刑像自己在同一个结上反复用力；适合观察内耗、重复模式和需要放松的地方。",
    },
    TimelineLexiconEntry {
        id: "hidden-stem",
        category: "hidden-stem",
        label_zh: "藏干",
        professional_zh: "藏干为地支内部所藏天干，用于补足透出之外的背景层和根气线索。",
        plain_zh: "藏干像地支深处保留的底色；适合做辅证，不能单独推出具体事件。",
    },
    TimelineLexiconEntry {
        id: "pattern",
        category: "pattern-useful-god",
        label_zh: "格局",
        professional_zh: "格局围绕日主、月令、十神组合和气势取向形成结构判断。",
        plain_zh: "格局像整张盘的工作方式；适合观察力量如何组织起来，不是单个字给出的结论。",
    },
    TimelineLexiconEntry {
        id: "useful-god",
        category: "pattern-useful-god",
        label_zh: "用神",
        professional_zh: "用神是用于平衡结构、调和偏枯或承接格局取向的参考五行或十神方向。",
        plain_zh: "用神像调节结构的一盏灯；适合观察哪里需要补足、缓冲或疏通，不是行动命令。",
    },
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::civil::CivilDate;
    use crate::domain::bazi::{
        BirthProfile, BirthTime, CalendarKind, ChartBasis, ChartRequest, DateLayerPillars,
        PrivacyLevel, Sex, TimePrecision,
    };
    use crate::domain::luck::compute_luck_cycles;

    fn exact_chart() -> ChartResult {
        let request = ChartRequest {
            birth_profile: BirthProfile {
                display_name: None,
                sex: Sex::Male,
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
                year: "甲辰".to_string(),
                month: "乙丑".to_string(),
                day: "庚午".to_string(),
            },
        )
        .unwrap()
    }

    fn unknown_hour_chart() -> ChartResult {
        let request = ChartRequest {
            birth_profile: BirthProfile {
                display_name: None,
                sex: Sex::Male,
                privacy_level: PrivacyLevel::Private,
                calendar_kind: CalendarKind::Gregorian,
                date: CivilDate::parse_iso("2025-01-01").unwrap(),
                time: None,
                time_precision: TimePrecision::Unknown,
                timezone: "Asia/Shanghai".to_string(),
                is_leap_month: false,
                use_true_solar_time: false,
            },
        };
        let basis = ChartBasis::build(request).unwrap();
        ChartResult::build(
            basis,
            DateLayerPillars {
                year: "甲辰".to_string(),
                month: "乙丑".to_string(),
                day: "庚午".to_string(),
            },
        )
        .unwrap()
    }

    fn luck_cycles(chart: &ChartResult) -> Vec<LuckCycle> {
        compute_luck_cycles(&chart.chart.year.stem, &chart.chart.month, &Sex::Male, 12)
    }

    fn joined_draft_text(draft: &TimelineReadingDraft) -> String {
        let mut parts = Vec::new();
        parts.extend(draft.signals.iter().map(|signal| signal.summary.clone()));
        parts.extend(draft.evidence.iter().map(|item| item.detail.clone()));
        parts.extend(draft.readings.iter().flat_map(|reading| {
            [
                reading.professional.clone(),
                reading.plain.clone(),
                reading.boundary.to_string(),
            ]
        }));
        parts.join("\n")
    }

    const STIFF_COPY_FORBIDDEN: [&str; 25] = [
        "白话说",
        "白话解释",
        "专业说法",
        "专业解释",
        "指定年份",
        "指定年",
        "这一年",
        "某一年",
        "这条信号",
        "帮助用户",
        "用户",
        "后端返回",
        "前端追加",
        "shared timeline engine",
        "timeline-core-v1",
        "score_internal",
        "0-100",
        "主题里",
        "事业主题里",
        "您在事业中",
        "读盘时",
        "2026年读盘时",
        "机器",
        "模板",
        "变量",
    ];

    fn assert_no_stiff_copy(label: &str, text: &str) {
        for forbidden in STIFF_COPY_FORBIDDEN {
            assert!(
                !text.contains(forbidden),
                "{label} leaked stiff lexicon wording: {forbidden}"
            );
        }
    }

    fn assert_timeline_copy_quality(label: &str, draft: &TimelineReadingDraft) {
        let text = joined_draft_text(draft);
        assert_no_stiff_copy(label, &text);

        for reading in &draft.readings {
            assert!(
                reading.plain.contains("您"),
                "{label} plain reading should address the reader directly: {}",
                reading.plain
            );
            assert_no_stiff_copy(
                &format!("{label}:{}:professional", reading.signal_id),
                &reading.professional,
            );
            assert_no_stiff_copy(
                &format!("{label}:{}:plain", reading.signal_id),
                &reading.plain,
            );
            assert!(
                reading.professional.chars().count() >= 28,
                "{label} professional reading is too thin: {}",
                reading.professional
            );
            assert!(
                reading.plain.chars().count() >= 45,
                "{label} plain reading is too thin: {}",
                reading.plain
            );
        }
    }

    fn assert_m40_bounded_draft(label: &str, draft: &TimelineReadingDraft) {
        assert_eq!(draft.audit.status, "passed", "{label} audit should pass");
        assert!(
            !draft.signals.is_empty(),
            "{label} should keep structural signals"
        );
        assert!(
            !draft.evidence.is_empty(),
            "{label} should keep trace evidence"
        );
        assert!(
            !draft.readings.is_empty(),
            "{label} should keep reading text"
        );
        assert!(
            draft.signals.len() <= 24,
            "{label} expanded too many timeline signals: {}",
            draft.signals.len()
        );
        assert!(
            draft.evidence.len() <= 24,
            "{label} expanded too many timeline evidence rows: {}",
            draft.evidence.len()
        );
        assert!(
            draft.readings.len() <= 24,
            "{label} expanded too many timeline readings: {}",
            draft.readings.len()
        );
        assert!(
            joined_draft_text(draft).len() <= 24000,
            "{label} expanded into an oversized static report"
        );
        assert!(
            draft
                .readings
                .iter()
                .all(|reading| reading.boundary.contains("不是现实事件预告"))
        );
        assert_timeline_copy_quality(label, draft);
    }

    #[test]
    fn timeline_lexicon_explains_core_primitives() {
        let categories = timeline_lexicon()
            .iter()
            .map(|entry| entry.category)
            .collect::<Vec<_>>();

        assert!(categories.contains(&"ten-god"));
        assert!(categories.contains(&"five-element"));
        assert!(categories.contains(&"five-element-relation"));
        assert!(categories.contains(&"branch-relation"));
        assert!(categories.contains(&"hidden-stem"));
        assert!(categories.contains(&"pattern-useful-god"));
        assert!(
            lexicon_entry("direct_wealth")
                .unwrap()
                .plain_zh
                .contains("资源")
        );
        assert!(
            lexicon_entry("six-clash")
                .unwrap()
                .plain_zh
                .contains("变化")
        );
        assert!(lexicon_entry("same-qi").unwrap().plain_zh.contains("同气"));
    }

    #[test]
    fn timeline_lexicon_copy_is_natural_and_guarded() {
        let lexicon = timeline_lexicon();
        assert!(
            (28..=40).contains(&lexicon.len()),
            "timeline lexicon should expand key primitives but stay compositional"
        );

        let mut ids = Vec::new();
        for entry in lexicon {
            assert!(
                !ids.contains(&entry.id),
                "duplicate timeline lexicon id: {}",
                entry.id
            );
            ids.push(entry.id);
            assert!(
                entry.professional_zh.chars().count() >= 18,
                "professional lexicon entry is too thin: {}",
                entry.id
            );
            assert!(
                entry.plain_zh.chars().count() >= 30,
                "plain lexicon entry is too thin: {}",
                entry.id
            );
            assert_no_stiff_copy(
                &format!("lexicon:{}:professional", entry.id),
                entry.professional_zh,
            );
            assert_no_stiff_copy(&format!("lexicon:{}:plain", entry.id), entry.plain_zh);
            assert!(
                !entry.professional_zh.contains(entry.id) && !entry.plain_zh.contains(entry.id),
                "visible lexicon text should not expose internal id: {}",
                entry.id
            );
        }
    }

    #[test]
    fn timeline_foundation_builds_traceable_signals_without_public_score() {
        let chart = exact_chart();
        let luck = luck_cycles(&chart);
        let annual = Pillar::from_ganzhi("丙午").unwrap();
        let draft = build_timeline_foundation(&chart, &luck, Some(&annual));

        assert_eq!(draft.rule_version.ruleset_id, RULESET_ID);
        assert_eq!(draft.rule_version.version, TIMELINE_RULE_VERSION);
        assert_eq!(draft.audit.status, "passed");
        assert!(
            draft
                .signals
                .iter()
                .any(|signal| signal.category == "ten-god")
        );
        assert!(
            draft
                .signals
                .iter()
                .any(|signal| signal.category == "five-element")
        );
        assert!(
            draft
                .evidence
                .iter()
                .any(|item| item.source == "annual-trigger")
        );
        assert_timeline_copy_quality("timeline_foundation", &draft);

        let text = joined_draft_text(&draft);
        for forbidden in ["score_internal", "0-100", "必然发财", "流月运势"] {
            assert!(
                !text.contains(forbidden),
                "unexpected forbidden text: {forbidden}"
            );
        }
    }

    #[test]
    fn luck_reading_stage_foundation_has_plain_trace_without_annual_default() {
        let chart = exact_chart();
        let luck = luck_cycles(&chart);
        let draft = build_major_luck_stage_foundation(&chart, &luck[0], "current");

        assert_eq!(draft.rule_version.ruleset_id, RULESET_ID);
        assert_eq!(draft.rule_version.version, TIMELINE_RULE_VERSION);
        assert_eq!(draft.audit.status, "passed");
        assert!(!draft.warnings.contains(&"annual_trigger_not_requested"));
        assert!(
            draft
                .signals
                .iter()
                .any(|signal| signal.source == "major-luck-current")
        );
        assert!(
            draft
                .signals
                .iter()
                .any(|signal| signal.category == "ten-god")
        );
        assert!(
            draft
                .signals
                .iter()
                .any(|signal| signal.category == "five-element")
        );
        assert!(
            draft
                .signals
                .iter()
                .any(|signal| signal.category == "hidden-stem")
        );
        assert!(
            draft
                .evidence
                .iter()
                .all(|item| item.source == "major-luck-current")
        );
        assert_timeline_copy_quality("major_luck_stage", &draft);

        let text = joined_draft_text(&draft);
        for forbidden in ["score_internal", "0-100", "必然发财", "流月运势"] {
            assert!(
                !text.contains(forbidden),
                "unexpected forbidden text: {forbidden}"
            );
        }
    }

    #[test]
    fn annual_trigger_requires_explicit_pillar_input() {
        let chart = exact_chart();
        let luck = luck_cycles(&chart);
        let without_annual = build_timeline_foundation(&chart, &luck, None);

        assert!(
            without_annual
                .warnings
                .contains(&"annual_trigger_not_requested")
        );
        assert!(
            !without_annual
                .signals
                .iter()
                .any(|signal| signal.source == "annual-trigger")
        );

        let annual = Pillar::from_ganzhi("丙午").unwrap();
        let with_annual = build_timeline_foundation(&chart, &luck, Some(&annual));
        assert!(
            with_annual
                .signals
                .iter()
                .any(|signal| signal.source == "annual-trigger")
        );
    }

    #[test]
    fn major_luck_and_annual_overlay_is_compositional() {
        let chart = exact_chart();
        let luck = luck_cycles(&chart);
        let annual = Pillar::from_ganzhi("丙午").unwrap();
        let draft = build_timeline_foundation(&chart, &luck, Some(&annual));

        assert!(
            draft
                .signals
                .iter()
                .any(|signal| signal.id == "major-luck-annual-branch-relation")
        );
        assert!(
            draft
                .evidence
                .iter()
                .any(|item| item.source == "major-luck+annual-trigger")
        );
    }

    #[test]
    fn annual_trigger_foundation_uses_explicit_year_and_current_luck_overlay() {
        let chart = exact_chart();
        let luck = luck_cycles(&chart);
        let annual = Pillar::from_ganzhi("丙午").unwrap();
        let draft = build_annual_trigger_foundation(&chart, &luck[0], &annual, 2026);

        assert_eq!(draft.rule_version.ruleset_id, RULESET_ID);
        assert_eq!(draft.rule_version.version, TIMELINE_RULE_VERSION);
        assert_eq!(draft.audit.status, "passed");
        assert!(!draft.warnings.contains(&"annual_trigger_not_requested"));
        assert!(
            draft
                .signals
                .iter()
                .any(|signal| signal.source == "annual-trigger")
        );
        assert!(
            draft
                .signals
                .iter()
                .any(|signal| signal.source == "annual-current-luck")
        );
        assert!(
            draft
                .evidence
                .iter()
                .any(|item| item.source == "major-luck+annual-trigger")
        );
        assert_timeline_copy_quality("annual_trigger", &draft);

        let text = joined_draft_text(&draft);
        assert!(text.contains("2026年年柱丙午"));
        for forbidden in ["score_internal", "0-100", "必然发财", "流月运势"] {
            assert!(
                !text.contains(forbidden),
                "unexpected forbidden text: {forbidden}"
            );
        }
    }

    #[test]
    fn m40_timeline_quality_gate_keeps_compositional_output_bounded() {
        let chart = exact_chart();
        let luck = luck_cycles(&chart);
        let annual_samples = [
            ("baseline_annual", 2024, "甲辰"),
            ("major_clash", 2026, "丙午"),
            ("branch_harmony", 2027, "丁未"),
            ("resource_shift", 2031, "辛亥"),
            ("next_cycle_background", 2035, "乙卯"),
        ];

        assert!(
            timeline_lexicon().len() <= 40,
            "timeline lexicon should stay compositional, not static 60-jiazi text"
        );

        let current_luck = &luck[0];
        assert_m40_bounded_draft(
            "current_major_luck",
            &build_major_luck_stage_foundation(&chart, current_luck, "current"),
        );

        let mut total_signals = 0usize;
        let mut total_readings = 0usize;
        for (label, year, ganzhi) in annual_samples {
            let annual = Pillar::from_ganzhi(ganzhi).unwrap();
            let draft = build_annual_trigger_foundation(&chart, current_luck, &annual, year);
            assert_m40_bounded_draft(label, &draft);
            assert!(
                draft
                    .signals
                    .iter()
                    .any(|signal| signal.source == "annual-trigger")
            );
            assert!(
                draft
                    .signals
                    .iter()
                    .any(|signal| signal.source == "annual-current-luck")
            );
            total_signals += draft.signals.len();
            total_readings += draft.readings.len();
        }

        assert!(
            total_signals <= annual_samples.len() * 24,
            "annual samples should stay bounded"
        );
        assert!(
            total_readings <= annual_samples.len() * 24,
            "annual readings should stay bounded"
        );
    }

    #[test]
    fn unknown_hour_downgrades_timeline_evidence() {
        let chart = unknown_hour_chart();
        let luck = luck_cycles(&chart);
        let annual = Pillar::from_ganzhi("丙午").unwrap();
        let draft = build_timeline_foundation(&chart, &luck, Some(&annual));

        assert!(
            draft
                .warnings
                .contains(&"unknown_hour_timeline_evidence_downgraded")
        );
        assert!(
            !draft
                .evidence
                .iter()
                .any(|item| item.chart_anchor.contains("时支"))
        );
    }

    #[test]
    fn annual_trigger_unknown_hour_downgrades_evidence() {
        let chart = unknown_hour_chart();
        let luck = luck_cycles(&chart);
        let annual = Pillar::from_ganzhi("丙午").unwrap();
        let draft = build_annual_trigger_foundation(&chart, &luck[0], &annual, 2026);

        assert!(
            draft
                .warnings
                .contains(&"unknown_hour_timeline_evidence_downgraded")
        );
        assert!(
            !draft
                .evidence
                .iter()
                .any(|item| item.chart_anchor.contains("时支"))
        );
    }

    #[test]
    fn timeline_audit_rejects_deterministic_claims() {
        let audit = audit_timeline_text("今年必然发财并且必升职");

        assert_eq!(audit.status, "rejected");
        assert!(audit.checked_patterns >= TIMELINE_FORBIDDEN_PATTERNS.len());
    }
}
