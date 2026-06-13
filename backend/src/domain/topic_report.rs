use crate::domain::analysis::{AnalysisSnapshot, ForbiddenOutputAudit, audit_text};
use crate::domain::bazi::{
    BRANCHES, BirthProfile, ChartResult, Pillar, RULESET_ID, STEMS, Sex, VALIDATED_RANGE_END_YEAR,
    VALIDATED_RANGE_START_YEAR,
};
use crate::domain::deep_analysis::{PatternInfo, StrengthAssessment, UsefulGodHint};
use crate::domain::luck::LuckCycle;
use crate::domain::timeline::{
    TimelineEvidence, TimelineReadingDraft, TimelineSignal, build_annual_trigger_foundation,
};
use crate::error::AppError;
use std::collections::BTreeMap;

pub const TOPIC_REPORT_ALGO_VERSION: &str = "topic-report-v1";
pub const TOPIC_REPORT_DISCLAIMER_ID: &str = "topic-report-traditional-reference-not-advice-v1";
const TOPIC_REPORT_ROUTE: &str = "/api/charts/topic-report";

const TOPIC_FORBIDDEN_PATTERNS: &[&str] = &[
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
    "必然失业",
    "跳槽必成",
    "行业选择是",
    "考试必过",
    "收入必涨",
    "结果保证",
    "确定发生",
    "当前算法",
    "系统给出",
    "综合评分",
    "相关信号共",
    "未见明显显性信号",
    "这份盘面目前没有触发",
    "降级参考",
    "共找到",
    "今年最值得留意",
    "盘中可用的时间线索",
    "重点看的牵动",
    "读这一段时",
    "在盘中有",
    "关键牵动是",
    "这张命盘里的",
    "食伤出现",
    "比劫出现",
    "印星出现",
    "出现 0 处",
    "出现 1 处",
    "出现 2 处",
    "出现 3 处",
    "出现 4 处",
    "正财 0 处",
    "偏财 1 处",
    "结构上被点亮",
    "从现有四柱看",
    "的重心更偏向",
    "不能只看流年",
    "不必急着找事件结论",
    "哪里需要放慢",
    "哪里需要承接",
    "读2026年这一层",
    "这一章只说明",
    "时间气候可以按这个顺序读",
    "时间气候，先从这些层次",
    "先从这些层次落下去看",
    "先看天干",
    "再看五行关系",
    "当前资料可以按完整四柱合参",
    "在这份金钱专项里",
    "在这份家庭专项里",
    "在这份事业专项里",
    "表达与安全感则落在日常相处里",
    "以目前资料来看，这份情感专项可以把重点放在",
    "在同一张桌上慢慢理清",
    "形成六冲",
    "形成六合",
    "形成三刑",
    "形成六害",
    "形成自刑",
    "年度本身先露出的",
    "流年天干把十神主题推到台前",
    "五行关系继续说明力量怎样靠近",
    "再往下看，藏干、原局位置和大运同场",
    "推到台前",
    "走到台前",
    "拿到台前",
    "从「金钱」专项来看",
    "从「家庭」专项来看",
    "从「事业」专项来看",
    "把2026年放进",
    "十神与五行这一层",
    "五行相处的方式提示",
    "藏干、原局位置和当前大运合到一起时",
    "藏干、宫位关系和当前大运合到一起时",
    "本段把它作为阶段背景参考",
    "这里看的不是单点事件",
    "年度线索要回到",
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TopicReportTopic {
    Relationship,
    Wealth,
    Family,
    Career,
}

impl TopicReportTopic {
    pub fn parse(value: &str) -> Result<Self, AppError> {
        match value {
            "relationship" => Ok(Self::Relationship),
            "wealth" => Ok(Self::Wealth),
            "family" => Ok(Self::Family),
            "career" => Ok(Self::Career),
            other => Err(AppError::BadRequest(format!(
                "unsupported topic value: {other}"
            ))),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Relationship => "relationship",
            Self::Wealth => "wealth",
            Self::Family => "family",
            Self::Career => "career",
        }
    }

    pub fn label_zh(&self) -> &'static str {
        match self {
            Self::Relationship => "情感",
            Self::Wealth => "金钱",
            Self::Family => "家庭",
            Self::Career => "事业",
        }
    }

    pub fn capability_id(&self) -> &'static str {
        match self {
            Self::Relationship => "relationship-report",
            Self::Wealth => "wealth-report",
            Self::Family => "family-report",
            Self::Career => "career-report",
        }
    }

    pub fn is_implemented(&self) -> bool {
        true
    }

    pub fn unsupported_error(&self) -> AppError {
        AppError::Unsupported {
            capability: self.capability_id().to_string(),
            route: TOPIC_REPORT_ROUTE.to_string(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopicSignal {
    pub id: &'static str,
    pub label: &'static str,
    pub qualitative_level: &'static str,
    pub summary: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopicTrace {
    pub id: &'static str,
    pub source: &'static str,
    pub evidence: Vec<String>,
    pub interpretation: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopicReportBlock {
    pub id: &'static str,
    pub title: &'static str,
    pub body: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopicReportBasis {
    pub day_master: String,
    pub day_pillar: String,
    pub relationship_palace: String,
    pub sex: &'static str,
    pub time_precision: &'static str,
    pub annual_pillar: String,
    pub strength_level: String,
    pub pattern_name: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopicReport {
    pub status: &'static str,
    pub capability: &'static str,
    pub topic: TopicReportTopic,
    pub algo_version: &'static str,
    pub ruleset_id: &'static str,
    pub year: i32,
    pub year_source: &'static str,
    pub disclaimer_id: &'static str,
    pub disclaimer: String,
    pub birth_profile: BirthProfile,
    pub basis: TopicReportBasis,
    pub signals: Vec<TopicSignal>,
    pub trace: Vec<TopicTrace>,
    pub blocks: Vec<TopicReportBlock>,
    pub warnings: Vec<&'static str>,
    pub forbidden_output_audit: ForbiddenOutputAudit,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopicTimelineOverlay {
    pub topic: TopicReportTopic,
    pub year: i32,
    pub annual_pillar: String,
    pub current_luck_pillar: String,
    pub rule_version: &'static str,
    pub shared_signal_count: usize,
    pub shared_evidence_count: usize,
    pub selected_signal_summaries: Vec<String>,
    pub selected_evidence: Vec<String>,
    pub professional: String,
    pub plain: String,
    pub boundary: &'static str,
    age_context: TopicAgeContext,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TopicAgeContext {
    reference_age: i32,
    early_stage: bool,
}

pub fn validate_topic_year(year: i32) -> Result<(), AppError> {
    if !(VALIDATED_RANGE_START_YEAR..=VALIDATED_RANGE_END_YEAR).contains(&year) {
        return Err(AppError::OutOfRange(format!(
            "topic report year out of supported range: {year}"
        )));
    }
    Ok(())
}

pub fn annual_pillar(year: i32) -> Pillar {
    let stem_index = (year - 4).rem_euclid(STEMS.len() as i32) as usize;
    let branch_index = (year - 4).rem_euclid(BRANCHES.len() as i32) as usize;
    Pillar {
        stem: STEMS[stem_index].to_string(),
        branch: BRANCHES[branch_index].to_string(),
    }
}

fn topic_age_context(chart: &ChartResult, year: i32) -> TopicAgeContext {
    let birth_year = chart.basis.request.birth_profile.date.year;
    let reference_age = (year - birth_year + 1).max(1);
    TopicAgeContext {
        reference_age,
        early_stage: reference_age < 16,
    }
}

fn early_stage_context_text(topic: TopicReportTopic, context: TopicAgeContext) -> Option<String> {
    if !context.early_stage {
        return None;
    }
    let text = match topic {
        TopicReportTopic::Relationship => format!(
            "由于名义年龄约{}岁，仍属于早年阶段，本报告不会把情感专项读成现实恋爱状态，而是观察情绪回应、依恋安全感和边界感怎样慢慢成形。",
            context.reference_age
        ),
        TopicReportTopic::Wealth => format!(
            "由于名义年龄约{}岁，仍属于早年阶段，本报告不会把金钱专项读成现实收入、投资或独立财务，而是观察资源意识、学习产出和分配边界的早期雏形。",
            context.reference_age
        ),
        TopicReportTopic::Family => format!(
            "由于名义年龄约{}岁，仍属于早年阶段，家庭专项会更偏向成长环境、照护方式、情绪表达和边界建立，不替亲属或家庭事件下结论。",
            context.reference_age
        ),
        TopicReportTopic::Career => format!(
            "由于名义年龄约{}岁，仍属于早年阶段，本报告不会把事业专项读成现实职位、职业选择或工作结果，而是观察规则感、学习承接和表达能力的早期雏形。",
            context.reference_age
        ),
    };
    Some(text)
}

fn early_stage_context_paragraph(topic: TopicReportTopic, context: TopicAgeContext) -> String {
    early_stage_context_text(topic, context)
        .map(|text| format!("\n\n{text}"))
        .unwrap_or_default()
}

pub fn build_relationship_report(
    chart: &ChartResult,
    snapshot: &AnalysisSnapshot,
    luck_cycles: &[LuckCycle],
    strength: &StrengthAssessment,
    pattern: &PatternInfo,
    useful_gods: &[UsefulGodHint],
    year: i32,
) -> TopicReport {
    let topic = TopicReportTopic::Relationship;
    let annual = annual_pillar(year);
    let age_context = topic_age_context(chart, year);
    let ten_god_counts = ten_god_counts(chart);
    let spouse_summary = relationship_spouse_star_summary(chart, &ten_god_counts);
    let current_luck = current_luck_cycle_for_year(chart, luck_cycles, year);
    let palace_relations = relationship_palace_relations(chart, &annual, current_luck);
    let expression_summary = relationship_expression_summary(&ten_god_counts);
    let support_summary = relationship_support_summary(&ten_god_counts, useful_gods);
    let mut warnings = relationship_warnings(chart);

    let mut signals = vec![
        TopicSignal {
            id: "relationship-palace",
            label: "夫妻宫",
            qualitative_level: relation_level(&palace_relations),
            summary: format!(
                "日支「{}」作为夫妻宫，{}",
                chart.chart.day.branch,
                palace_relation_plain(&palace_relations)
            ),
        },
        TopicSignal {
            id: "spouse-star",
            label: "配偶星",
            qualitative_level: spouse_summary.level,
            summary: spouse_summary.summary.clone(),
        },
        TopicSignal {
            id: "relationship-expression",
            label: "表达方式",
            qualitative_level: expression_summary.level,
            summary: expression_summary.summary.clone(),
        },
        TopicSignal {
            id: "annual-trigger",
            label: "年度引动",
            qualitative_level: annual_trigger_level(&palace_relations),
            summary: format!(
                "{}年柱为「{}」，{}",
                year,
                annual.ganzhi(),
                annual_trigger_plain(&palace_relations)
            ),
        },
    ];

    let mut trace = Vec::new();
    trace.push(TopicTrace {
        id: "day-branch-palace",
        source: "chart.day.branch",
        evidence: vec![format!("日支={}", chart.chart.day.branch)],
        interpretation: "日支在本项目情感专项中作为夫妻宫，也就是亲密互动的主要落点。".to_string(),
    });
    trace.push(TopicTrace {
        id: "spouse-star-ten-gods",
        source: "analysis.ten_gods",
        evidence: spouse_summary.evidence.clone(),
        interpretation: spouse_summary.summary.clone(),
    });
    trace.push(TopicTrace {
        id: "branch-relations",
        source: "chart/luck/year branches",
        evidence: palace_relations
            .iter()
            .map(|relation| relation.evidence.clone())
            .collect(),
        interpretation: palace_relation_plain(&palace_relations).to_string(),
    });

    let (timeline_overlay, timeline_warnings) = apply_topic_timeline_overlay_to_evidence(
        topic,
        chart,
        luck_cycles,
        &annual,
        year,
        &mut signals,
        &mut trace,
    );
    warnings.extend(timeline_warnings);

    let blocks = vec![
        relationship_overview_block(
            chart,
            strength,
            pattern,
            &spouse_summary,
            &palace_relations,
            age_context,
        ),
        relationship_spouse_star_block(chart, &spouse_summary),
        relationship_palace_block(chart, &palace_relations),
        relationship_expression_block(&expression_summary, &support_summary),
        relationship_trigger_block(
            year,
            &annual,
            current_luck,
            &palace_relations,
            timeline_overlay.as_ref(),
            age_context,
        ),
        relationship_plain_block(
            chart,
            snapshot,
            &warnings,
            &spouse_summary,
            &expression_summary,
            &support_summary,
            &palace_relations,
            timeline_overlay.as_ref(),
            age_context,
        ),
    ];

    let disclaimer = topic_disclaimer(topic);
    let assembled_report = assemble_report(&disclaimer, &blocks);
    let audit = audit_topic_text(&assembled_report);

    TopicReport {
        status: "restricted",
        capability: topic.capability_id(),
        topic,
        algo_version: TOPIC_REPORT_ALGO_VERSION,
        ruleset_id: RULESET_ID,
        year,
        year_source: "explicit",
        disclaimer_id: TOPIC_REPORT_DISCLAIMER_ID,
        disclaimer,
        birth_profile: chart.basis.request.birth_profile.clone(),
        basis: TopicReportBasis {
            day_master: chart.chart.day.stem.clone(),
            day_pillar: chart.chart.day.ganzhi(),
            relationship_palace: chart.chart.day.branch.clone(),
            sex: chart.basis.request.birth_profile.sex.as_str(),
            time_precision: chart.basis.request.birth_profile.time_precision.as_str(),
            annual_pillar: annual.ganzhi(),
            strength_level: strength.level.to_string(),
            pattern_name: pattern.pattern_name.clone(),
        },
        signals,
        trace,
        blocks,
        warnings,
        forbidden_output_audit: audit,
    }
}

pub fn build_wealth_report(
    chart: &ChartResult,
    snapshot: &AnalysisSnapshot,
    luck_cycles: &[LuckCycle],
    strength: &StrengthAssessment,
    pattern: &PatternInfo,
    useful_gods: &[UsefulGodHint],
    year: i32,
) -> TopicReport {
    let topic = TopicReportTopic::Wealth;
    let annual = annual_pillar(year);
    let age_context = topic_age_context(chart, year);
    let counts = ten_god_counts(chart);
    let wealth = ten_god_group_summary(
        &counts,
        &["direct_wealth", "indirect_wealth"],
        "财星",
        "正财偏向稳定资源和可管理事务；偏财偏向机会、流动资源和外部交换。",
    );
    let output = ten_god_group_summary(
        &counts,
        &["eating_god", "hurting_officer"],
        "食伤",
        "食伤这条线要看能力、表达和产出能不能变成持续资源。",
    );
    let peer = ten_god_group_summary(
        &counts,
        &["peer", "rob_wealth"],
        "比劫",
        "比劫这条线要看合作、竞争和资源分配边界能不能说清。",
    );
    let resource = ten_god_group_summary(
        &counts,
        &["direct_resource", "indirect_resource"],
        "印星",
        "印星这条线要看学习、支持和保护系统能不能托住资源压力。",
    );
    let officer = ten_god_group_summary(
        &counts,
        &["direct_officer", "seven_killings"],
        "官杀",
        "官杀这条线要看规则、责任和现实秩序能不能把资源稳住。",
    );
    let trigger = cycle_trigger_summary(
        chart,
        &annual,
        current_luck_cycle_for_year(chart, luck_cycles, year),
        &[
            "direct_wealth",
            "indirect_wealth",
            "eating_god",
            "hurting_officer",
            "peer",
            "rob_wealth",
        ],
        "资源主题",
    );

    let mut signals = vec![
        TopicSignal {
            id: "wealth-star",
            label: "财星",
            qualitative_level: wealth.level,
            summary: wealth.summary.clone(),
        },
        TopicSignal {
            id: "output-to-wealth",
            label: "食伤生财",
            qualitative_level: output.level,
            summary: output.summary.clone(),
        },
        TopicSignal {
            id: "peer-resource-boundary",
            label: "比劫分配",
            qualitative_level: peer.level,
            summary: peer.summary.clone(),
        },
        TopicSignal {
            id: "annual-trigger",
            label: "年度引动",
            qualitative_level: trigger.level,
            summary: format!("{}年柱为「{}」，{}", year, annual.ganzhi(), trigger.summary),
        },
    ];

    let mut trace = vec![
        TopicTrace {
            id: "wealth-ten-gods",
            source: "analysis.ten_gods",
            evidence: merge_evidence(&[&wealth, &output, &peer, &resource, &officer]),
            interpretation: "金钱专项先看财星，再看食伤能否形成产出转化，比劫是否牵动资源分配，印星和官杀是否提供承接与约束。".to_string(),
        },
        TopicTrace {
            id: "wealth-strength",
            source: "deep_analysis.strength",
            evidence: vec![
                format!("日主强弱={}", strength.level),
                format!("格局={}", pattern.pattern_name),
            ],
            interpretation: "日主强弱在这里解释为结构承载能力，不等于现实赚钱能力，也不等于财富多少。".to_string(),
        },
        TopicTrace {
            id: "wealth-cycle-trigger",
            source: "luck/year ten-gods",
            evidence: trigger.evidence.clone(),
            interpretation: trigger.summary.clone(),
        },
    ];

    let mut blocks = vec![
        wealth_overview_block(chart, strength, pattern, age_context),
        wealth_daily_reading_block(age_context),
        wealth_star_block(&wealth, age_context),
        wealth_flow_block(&output, &peer, &resource, &officer, age_context),
        wealth_capacity_block(strength, pattern, useful_gods, age_context),
        wealth_trigger_block(
            year,
            &annual,
            current_luck_cycle_for_year(chart, luck_cycles, year),
            &trigger,
            age_context,
        ),
        wealth_plain_block(snapshot, age_context),
    ];

    let mut warnings = wealth_warnings(chart);
    warnings.extend(apply_topic_timeline_overlay(
        topic,
        chart,
        luck_cycles,
        &annual,
        year,
        &mut signals,
        &mut trace,
        &mut blocks,
    ));

    finish_topic_report(
        topic, chart, strength, pattern, year, annual, signals, trace, blocks, warnings,
    )
}

pub fn build_family_report(
    chart: &ChartResult,
    snapshot: &AnalysisSnapshot,
    luck_cycles: &[LuckCycle],
    strength: &StrengthAssessment,
    pattern: &PatternInfo,
    useful_gods: &[UsefulGodHint],
    year: i32,
) -> TopicReport {
    let topic = TopicReportTopic::Family;
    let annual = annual_pillar(year);
    let age_context = topic_age_context(chart, year);
    let counts = ten_god_counts(chart);
    let resource = ten_god_group_summary(
        &counts,
        &["direct_resource", "indirect_resource"],
        "印星",
        "印星这条线要看支持、理解、接纳和安全感怎样进入家庭互动。",
    );
    let peer = ten_god_group_summary(
        &counts,
        &["peer", "rob_wealth"],
        "比劫",
        "比劫这条线要看同辈协作、立场边界和资源分配感。",
    );
    let output = ten_god_group_summary(
        &counts,
        &["eating_god", "hurting_officer"],
        "食伤",
        "食伤这条线要看话怎样说出口，需求怎样被听见。",
    );
    let duty = ten_god_group_summary(
        &counts,
        &[
            "direct_wealth",
            "indirect_wealth",
            "direct_officer",
            "seven_killings",
        ],
        "财官",
        "财官这条线要看家庭事务、角色安排和现实责任怎样落位。",
    );
    let current_luck = current_luck_cycle_for_year(chart, luck_cycles, year);
    let palace_relations =
        branch_relations_for_anchors(family_anchors(chart), &annual, current_luck);
    let trigger = family_trigger_summary(year, &annual, &palace_relations);

    let mut signals = vec![
        TopicSignal {
            id: "family-palaces",
            label: "宫位",
            qualitative_level: relation_level(&palace_relations),
            summary: family_palace_summary(chart),
        },
        TopicSignal {
            id: "family-resource",
            label: "印星",
            qualitative_level: resource.level,
            summary: resource.summary.clone(),
        },
        TopicSignal {
            id: "family-peer",
            label: "比劫",
            qualitative_level: peer.level,
            summary: peer.summary.clone(),
        },
        TopicSignal {
            id: "annual-trigger",
            label: "年度引动",
            qualitative_level: trigger.level,
            summary: trigger.summary.clone(),
        },
    ];

    let mut trace = vec![
        TopicTrace {
            id: "family-palace-map",
            source: "chart.pillars",
            evidence: family_anchor_evidence(chart),
            interpretation: "家庭专项把年柱、月柱、日支和时柱作为不同层次的家庭参考位置；时辰未知时，时柱相关内容必须降级。".to_string(),
        },
        TopicTrace {
            id: "family-ten-gods",
            source: "analysis.ten_gods",
            evidence: merge_evidence(&[&resource, &peer, &output, &duty]),
            interpretation: "家庭专项用印星看支持系统，比劫看同辈边界，食伤看表达和晚辈主题，财官看现实责任和秩序。".to_string(),
        },
        TopicTrace {
            id: "family-cycle-trigger",
            source: "chart/luck/year branches",
            evidence: palace_relations.iter().map(|hit| hit.evidence.clone()).collect(),
            interpretation: trigger.summary.clone(),
        },
    ];

    let mut blocks = vec![
        family_overview_block(chart, strength, pattern, age_context),
        family_daily_reading_block(),
        family_support_block(&resource, useful_gods),
        family_peer_output_block(&peer, &output),
        family_duty_block(&duty),
        family_trigger_block(year, &annual, current_luck, &palace_relations, age_context),
        family_plain_block(chart, snapshot, age_context),
    ];

    let mut warnings = family_warnings(chart);
    warnings.extend(apply_topic_timeline_overlay(
        topic,
        chart,
        luck_cycles,
        &annual,
        year,
        &mut signals,
        &mut trace,
        &mut blocks,
    ));

    finish_topic_report(
        topic, chart, strength, pattern, year, annual, signals, trace, blocks, warnings,
    )
}

pub fn build_career_report(
    chart: &ChartResult,
    snapshot: &AnalysisSnapshot,
    luck_cycles: &[LuckCycle],
    strength: &StrengthAssessment,
    pattern: &PatternInfo,
    useful_gods: &[UsefulGodHint],
    year: i32,
) -> TopicReport {
    let topic = TopicReportTopic::Career;
    let annual = annual_pillar(year);
    let age_context = topic_age_context(chart, year);
    let counts = ten_god_counts(chart);
    let officer = ten_god_group_summary(
        &counts,
        &["direct_officer", "seven_killings"],
        "官杀",
        "官杀这条线要看责任、规则、压力和外部要求怎样靠近。",
    );
    let resource = ten_god_group_summary(
        &counts,
        &["direct_resource", "indirect_resource"],
        "印星",
        "印星这条线要看学习、资质、支持系统和缓冲方式是否跟得上。",
    );
    let output = ten_god_group_summary(
        &counts,
        &["eating_god", "hurting_officer"],
        "食伤",
        "食伤这条线要看表达、技术、产出和解决方案能不能被交付。",
    );
    let wealth = ten_god_group_summary(
        &counts,
        &["direct_wealth", "indirect_wealth"],
        "财星",
        "财星这条线要看资源、现实落地和结果意识能不能接住技能。",
    );
    let peer = ten_god_group_summary(
        &counts,
        &["peer", "rob_wealth"],
        "比劫",
        "比劫这条线要看协作、竞争、自主性和团队边界是否清楚。",
    );
    let current_luck = current_luck_cycle_for_year(chart, luck_cycles, year);
    let palace_relations =
        branch_relations_for_anchors(career_anchors(chart), &annual, current_luck);
    let trigger = cycle_trigger_summary(
        chart,
        &annual,
        current_luck,
        &[
            "direct_officer",
            "seven_killings",
            "direct_resource",
            "indirect_resource",
            "eating_god",
            "hurting_officer",
            "direct_wealth",
            "indirect_wealth",
        ],
        "事业主题",
    );

    let mut signals = vec![
        TopicSignal {
            id: "career-officer",
            label: "官杀",
            qualitative_level: officer.level,
            summary: officer.summary.clone(),
        },
        TopicSignal {
            id: "career-output",
            label: "食伤",
            qualitative_level: output.level,
            summary: output.summary.clone(),
        },
        TopicSignal {
            id: "career-resource",
            label: "印星",
            qualitative_level: resource.level,
            summary: resource.summary.clone(),
        },
        TopicSignal {
            id: "annual-trigger",
            label: "年度引动",
            qualitative_level: trigger.level,
            summary: format!("{}年柱为「{}」，{}", year, annual.ganzhi(), trigger.summary),
        },
    ];

    let mut trace = vec![
        TopicTrace {
            id: "career-ten-gods",
            source: "analysis.ten_gods",
            evidence: merge_evidence(&[&officer, &resource, &output, &wealth, &peer]),
            interpretation: "事业专项以官杀看责任压力，以印星看承接支持，以食伤看技能产出，以财星看资源落地，以比劫看协作竞争。".to_string(),
        },
        TopicTrace {
            id: "career-pattern",
            source: "deep_analysis.pattern",
            evidence: vec![
                format!("日主强弱={}", strength.level),
                format!("格局={}", pattern.pattern_name),
                format!("用神={}", useful_god_text(useful_gods)),
            ],
            interpretation: "格局与用神只说明事业主题的组织方式，不评价现实能力高低或社会价值。".to_string(),
        },
        TopicTrace {
            id: "career-cycle-trigger",
            source: "luck/year ten-gods and branches",
            evidence: merge_strings(
                trigger.evidence.clone(),
                palace_relations.iter().map(|hit| hit.evidence.clone()).collect(),
            ),
            interpretation: trigger.summary.clone(),
        },
    ];

    let mut blocks = vec![
        career_overview_block(chart, strength, pattern, age_context),
        career_daily_reading_block(age_context),
        career_responsibility_block(&officer, &resource, age_context),
        career_skill_resource_block(&output, &wealth, age_context),
        career_collaboration_block(&peer, useful_gods, age_context),
        career_trigger_block(
            year,
            &annual,
            current_luck,
            &trigger,
            &palace_relations,
            age_context,
        ),
        career_plain_block(snapshot, age_context),
    ];

    let mut warnings = career_warnings(chart);
    warnings.extend(apply_topic_timeline_overlay(
        topic,
        chart,
        luck_cycles,
        &annual,
        year,
        &mut signals,
        &mut trace,
        &mut blocks,
    ));

    finish_topic_report(
        topic, chart, strength, pattern, year, annual, signals, trace, blocks, warnings,
    )
}

fn finish_topic_report(
    topic: TopicReportTopic,
    chart: &ChartResult,
    strength: &StrengthAssessment,
    pattern: &PatternInfo,
    year: i32,
    annual: Pillar,
    signals: Vec<TopicSignal>,
    trace: Vec<TopicTrace>,
    blocks: Vec<TopicReportBlock>,
    warnings: Vec<&'static str>,
) -> TopicReport {
    let disclaimer = topic_disclaimer(topic);
    let assembled_report = assemble_report(&disclaimer, &blocks);
    let audit = audit_topic_text(&assembled_report);

    TopicReport {
        status: "restricted",
        capability: topic.capability_id(),
        topic,
        algo_version: TOPIC_REPORT_ALGO_VERSION,
        ruleset_id: RULESET_ID,
        year,
        year_source: "explicit",
        disclaimer_id: TOPIC_REPORT_DISCLAIMER_ID,
        disclaimer,
        birth_profile: chart.basis.request.birth_profile.clone(),
        basis: TopicReportBasis {
            day_master: chart.chart.day.stem.clone(),
            day_pillar: chart.chart.day.ganzhi(),
            relationship_palace: chart.chart.day.branch.clone(),
            sex: chart.basis.request.birth_profile.sex.as_str(),
            time_precision: chart.basis.request.birth_profile.time_precision.as_str(),
            annual_pillar: annual.ganzhi(),
            strength_level: strength.level.to_string(),
            pattern_name: pattern.pattern_name.clone(),
        },
        signals,
        trace,
        blocks,
        warnings,
        forbidden_output_audit: audit,
    }
}

fn apply_topic_timeline_overlay(
    topic: TopicReportTopic,
    chart: &ChartResult,
    luck_cycles: &[LuckCycle],
    annual: &Pillar,
    year: i32,
    signals: &mut Vec<TopicSignal>,
    trace: &mut Vec<TopicTrace>,
    blocks: &mut Vec<TopicReportBlock>,
) -> Vec<&'static str> {
    let (overlay, warnings) = apply_topic_timeline_overlay_to_evidence(
        topic,
        chart,
        luck_cycles,
        annual,
        year,
        signals,
        trace,
    );
    if let Some(overlay) = overlay {
        let block = topic_timeline_overlay_block(&overlay);
        if let Some(index) = blocks.iter().position(|item| item.title == "结论") {
            blocks.insert(index, block);
        } else {
            blocks.push(block);
        }
    }
    warnings
}

fn apply_topic_timeline_overlay_to_evidence(
    topic: TopicReportTopic,
    chart: &ChartResult,
    luck_cycles: &[LuckCycle],
    annual: &Pillar,
    year: i32,
    signals: &mut Vec<TopicSignal>,
    trace: &mut Vec<TopicTrace>,
) -> (Option<TopicTimelineOverlay>, Vec<&'static str>) {
    let (current_luck, mut warnings) =
        select_current_luck_for_topic_timeline(chart, luck_cycles, year);
    let Some(current_luck) = current_luck else {
        return (None, warnings);
    };

    let draft = build_annual_trigger_foundation(chart, current_luck, annual, year);
    warnings.extend(draft.warnings.iter().copied());
    if draft.audit.status != "passed" {
        warnings.push("topic_timeline_shared_audit_not_passed");
    }

    let overlay =
        topic_timeline_overlay_from_draft(topic, chart, year, annual, current_luck, &draft);
    signals.push(topic_timeline_signal(&overlay));
    trace.push(topic_timeline_trace(&overlay));
    (Some(overlay), warnings)
}

fn select_current_luck_for_topic_timeline<'a>(
    chart: &ChartResult,
    luck_cycles: &'a [LuckCycle],
    year: i32,
) -> (Option<&'a LuckCycle>, Vec<&'static str>) {
    let mut warnings = Vec::new();
    if luck_cycles.is_empty() {
        warnings.push("topic_timeline_luck_cycle_missing");
        return (None, warnings);
    }

    let birth_year = chart.basis.request.birth_profile.date.year;
    let raw_age = year - birth_year + 1;
    if raw_age < 1 {
        warnings.push("topic_timeline_reference_age_before_birth");
    }
    if raw_age > u8::MAX as i32 {
        warnings.push("topic_timeline_reference_age_outside_supported_range");
    }
    let age = raw_age.clamp(1, u8::MAX as i32) as u8;

    let mut index = luck_cycles
        .iter()
        .position(|cycle| age >= cycle.start_age && age <= cycle.end_age);
    if index.is_none() {
        if age < luck_cycles[0].start_age {
            warnings.push("topic_timeline_reference_age_before_first_luck");
            index = Some(0);
        } else {
            warnings.push("topic_timeline_reference_age_after_last_luck");
            index = Some(luck_cycles.len() - 1);
        }
    }

    (index.map(|value| &luck_cycles[value]), warnings)
}

fn current_luck_cycle_for_year<'a>(
    chart: &ChartResult,
    luck_cycles: &'a [LuckCycle],
    year: i32,
) -> Option<&'a LuckCycle> {
    select_current_luck_for_topic_timeline(chart, luck_cycles, year).0
}

fn topic_timeline_overlay_from_draft(
    topic: TopicReportTopic,
    chart: &ChartResult,
    year: i32,
    annual: &Pillar,
    current_luck: &LuckCycle,
    draft: &TimelineReadingDraft,
) -> TopicTimelineOverlay {
    let age_context = topic_age_context(chart, year);
    let selected_signals = select_topic_timeline_signals(topic, &draft.signals);
    let selected_ids = selected_signals
        .iter()
        .map(|signal| signal.id.clone())
        .collect::<Vec<_>>();
    let mut selected_evidence = draft
        .evidence
        .iter()
        .filter(|item| selected_ids.iter().any(|id| id == &item.signal_id))
        .take(6)
        .map(topic_timeline_evidence_text)
        .collect::<Vec<_>>();
    if selected_evidence.is_empty() {
        selected_evidence = draft
            .evidence
            .iter()
            .take(4)
            .map(topic_timeline_evidence_text)
            .collect();
    }

    let selected_signal_summaries = selected_signals
        .iter()
        .take(5)
        .map(|signal| topic_timeline_signal_summary(signal, year))
        .collect::<Vec<_>>();

    let professional = topic_timeline_professional(
        topic,
        year,
        draft,
        selected_signal_summaries.len(),
        age_context,
    );
    let plain = topic_timeline_plain(topic, year, annual, current_luck, age_context);

    TopicTimelineOverlay {
        topic,
        year,
        annual_pillar: annual.ganzhi(),
        current_luck_pillar: current_luck.pillar.ganzhi(),
        rule_version: draft.rule_version.version,
        shared_signal_count: draft.signals.len(),
        shared_evidence_count: draft.evidence.len(),
        selected_signal_summaries,
        selected_evidence,
        professional,
        plain,
        boundary: "边界提醒：这里不推断现实事件，也不提供婚恋、财务、家庭、职业决策建议；它只说明流年和当前大运如何共同触发命盘结构。",
        age_context,
    }
}

fn select_topic_timeline_signals<'a>(
    topic: TopicReportTopic,
    signals: &'a [TimelineSignal],
) -> Vec<&'a TimelineSignal> {
    let mut selected = signals
        .iter()
        .filter(|signal| {
            signal.applies_to_topics.contains(&topic.as_str())
                || signal.source == "annual-trigger"
                || signal.source == "annual-current-luck"
                || signal.source == "major-luck+annual-trigger"
        })
        .collect::<Vec<_>>();
    if selected.is_empty() {
        selected = signals.iter().take(3).collect();
    }
    selected
}

fn topic_timeline_evidence_text(item: &TimelineEvidence) -> String {
    let source = match item.source {
        "annual-trigger" => "流年信号",
        "annual-current-luck" => "当前大运信号",
        "major-luck+annual-trigger" => "大运流年叠加",
        "major-luck" => "大运信号",
        _ => "结构信号",
    };
    format!(
        "{}：{}受「{}」牵动，{}",
        source,
        item.chart_anchor,
        readable_evidence_text(&item.trigger),
        readable_evidence_text(&item.detail)
    )
}

fn readable_evidence_text(value: &str) -> String {
    let text = value
        .replace(" = ", "形成")
        .replace('=', "为")
        .replace("。。", "。")
        .replace(" 与 ", "与")
        .replace('(', "（")
        .replace(')', "）");
    quote_relation_names(&replace_luck_ordinals(&replace_age_range_hyphen(&text)))
}

fn topic_timeline_professional(
    topic: TopicReportTopic,
    year: i32,
    _draft: &TimelineReadingDraft,
    _selected_signal_count: usize,
    age_context: TopicAgeContext,
) -> String {
    if age_context.early_stage {
        return match topic {
            TopicReportTopic::Relationship => format!(
                "{}年放到情感上，因为仍是早年阶段，不按现实恋爱状态来读，而是看情绪回应、依恋安全感和边界意识怎样被环境慢慢塑形。流年、当前大运和原局同场时，重点仍是关系模式的雏形。",
                year
            ),
            TopicReportTopic::Wealth => format!(
                "{}年放到金钱上，因为仍是早年阶段，不按现实收入或投资来读，而是看资源意识、学习产出和分享边界怎样开始成形。流年、当前大运和原局同场时，重点要落回资源感如何被照护与规则托住。",
                year
            ),
            TopicReportTopic::Family => format!(
                "{}年放到家庭上，因为仍是早年阶段，重点更偏向成长环境、照护方式、情绪表达和边界建立。流年、当前大运和原局同场时，家庭里的支持与规则会更需要安顿。",
                year
            ),
            TopicReportTopic::Career => format!(
                "{}年放到事业上，因为仍是早年阶段，不按现实职位或职业结果来读，而是看规则感、学习承接和表达能力怎样萌芽。流年、当前大运和原局同场时，重点要落回早期任务感能不能被温和接住。",
                year
            ),
        };
    }
    match topic {
        TopicReportTopic::Relationship => format!(
            "{}年落到情感上，要把流年、当前大运和原局一起看，重点仍是亲密互动、表达边界和稳定回应怎样靠近。",
            year
        ),
        TopicReportTopic::Wealth => format!(
            "{}年放到金钱上，先看资源有没有节奏感：机会会不会推近，产出能不能连续，合作和预算有没有边界。流年、当前大运和原局同场时，重点要落回钱如何进、如何留、如何稳。",
            year
        ),
        TopicReportTopic::Family => format!(
            "{}年放到家庭上，重点不在替家人定结果，而在看支持从哪里来、话怎样说清、责任怎样放回合适的位置。流年、当前大运和原局同场时，家庭里的情绪、边界和现实事务会更需要安顿。",
            year
        ),
        TopicReportTopic::Career => format!(
            "{}年放到事业上，先看任务压力怎样靠近，也看技能、资源和协作能不能接住它。流年、当前大运和原局同场时，重点要落回工作节奏能不能从紧绷走向可持续。",
            year
        ),
    }
}

fn topic_timeline_plain(
    topic: TopicReportTopic,
    year: i32,
    annual: &Pillar,
    current_luck: &LuckCycle,
    age_context: TopicAgeContext,
) -> String {
    let opening = format!(
        "{}年年柱「{}」遇到当前大运「{}」时，{}会更容易被看见。",
        year,
        annual.ganzhi(),
        current_luck.pillar.ganzhi(),
        topic_timeline_plain_focus(topic)
    );
    let guidance = match topic {
        TopicReportTopic::Relationship if age_context.early_stage => format!(
            "落到{}年，更适合看安全感、回应方式和边界感怎样被照护出来；不把它读成现实伴侣关系。",
            year
        ),
        TopicReportTopic::Relationship => format!(
            "读{}年这一层，先看亲密互动能不能从一时牵动落到稳定回应：话是否说得早，边界是否被尊重，承诺能不能经得起日常相处。",
            year
        ),
        TopicReportTopic::Wealth if age_context.early_stage => format!(
            "落到{}年，更适合看照护资源是否稳定、兴趣和能力是否被鼓励、分享与边界是否被温和建立；不把它读成赚钱或投资。",
            year
        ),
        TopicReportTopic::Wealth => format!(
            "落到{}年，预算意识要走在机会前面，技能产出要能连续，合作分配也要提前说清；资源能不能扩大，关键在节奏和规则能否托住。",
            year
        ),
        TopicReportTopic::Family if age_context.early_stage => format!(
            "落到{}年，家里更需要稳定照护、清楚回应和温和规则；情绪能不能被接住，比责任如何分配更先一步。",
            year
        ),
        TopicReportTopic::Family => format!(
            "落到{}年，家里需要被说明白的话、需要分清的责任、需要安放的情绪都会更容易浮上来；把关系位置放稳，比急着定性更重要。",
            year
        ),
        TopicReportTopic::Career if age_context.early_stage => format!(
            "落到{}年，更适合看学习任务、规则感、表达训练和支持系统怎样配合；不把它读成岗位、升迁或职业成败。",
            year
        ),
        TopicReportTopic::Career => format!(
            "落到{}年，任务标准、技能输出、资源承接和协作边界会一起压到工作节奏里；压力要化成可持续行动，不能只停在紧绷感里。",
            year
        ),
    };
    format!("{opening}{guidance}")
}

fn topic_timeline_focus(topic: TopicReportTopic) -> &'static str {
    match topic {
        TopicReportTopic::Relationship => "夫妻宫、配偶星、表达边界与支持方式",
        TopicReportTopic::Wealth => "资源模式、产出转化、分配边界与承载节奏",
        TopicReportTopic::Family => "支持系统、边界协作、表达方式与现实责任",
        TopicReportTopic::Career => "责任结构、技能表达、资源落地与协作边界",
    }
}

fn topic_timeline_signal_summary(signal: &TimelineSignal, year: i32) -> String {
    let label = if signal.label.contains("十神触发") {
        "流年天干所成十神"
    } else if signal.label.contains("五行流向") {
        "流年天干与日主的五行关系"
    } else if signal.label.contains("藏干背景") {
        "流年地支的内在底色"
    } else if signal.label.contains("月支关系") {
        "月支被流年触动"
    } else if signal.label.contains("日支关系") {
        "日支被流年触动"
    } else if signal.label.contains("大运与流年") {
        "大运与流年的同场关系"
    } else {
        signal.label.as_str()
    };
    let summary = readable_signal_summary(&signal.summary, year);
    format!("{label}：{summary}")
}

fn readable_signal_summary(value: &str, year: i32) -> String {
    let mut text = value.replace(&format!("{year}年年柱"), "流年");
    text = text.replace("结构。", "。");
    quote_relation_names(&replace_luck_ordinals(&replace_age_range_hyphen(&text)))
}

fn topic_timeline_plain_focus(topic: TopicReportTopic) -> &'static str {
    match topic {
        TopicReportTopic::Relationship => "亲密互动、表达、边界和支持",
        TopicReportTopic::Wealth => "资源、产出、分配和承载",
        TopicReportTopic::Family => "支持、照顾、边界和责任",
        TopicReportTopic::Career => "责任、技能、资源和协作",
    }
}

fn topic_timeline_signal(overlay: &TopicTimelineOverlay) -> TopicSignal {
    TopicSignal {
        id: "topic-timeline-overlay",
        label: "大运流年叠加",
        qualitative_level: "结构叠加",
        summary: format!(
            "{}年流年「{}」与当前大运「{}」相遇，专题阅读以{}为主线。",
            overlay.year,
            overlay.annual_pillar,
            overlay.current_luck_pillar,
            topic_timeline_focus(overlay.topic)
        ),
    }
}

fn topic_timeline_trace(overlay: &TopicTimelineOverlay) -> TopicTrace {
    TopicTrace {
        id: "topic-timeline-overlay",
        source: overlay.rule_version,
        evidence: overlay.selected_evidence.clone(),
        interpretation: "本追踪只把大运与流年的结构线索对应到当前专题，不额外生成现实事件判断。"
            .to_string(),
    }
}

fn topic_timeline_overlay_block(overlay: &TopicTimelineOverlay) -> TopicReportBlock {
    let signal_text = topic_timeline_signal_story(overlay);

    TopicReportBlock {
        id: "topic-timeline-overlay",
        title: "本专题的大运流年",
        body: format!(
            "{}\n\n{}\n\n{}\n\n{}",
            overlay.professional,
            overlay.plain,
            signal_text,
            topic_timeline_year_guidance(overlay.topic, overlay.year, overlay.age_context)
        ),
    }
}

fn topic_timeline_signal_story(overlay: &TopicTimelineOverlay) -> String {
    if overlay.selected_signal_summaries.is_empty() {
        return "暂未看到额外专题信号，先保留年度引动作为背景。".to_string();
    }

    let mut primary = Vec::new();
    let mut background = Vec::new();
    for summary in &overlay.selected_signal_summaries {
        let sentence = topic_timeline_signal_sentence(summary, overlay.year);
        let (label, _) = summary.split_once('：').unwrap_or(("时间线索", summary));
        if matches!(label, "流年天干所成十神" | "流年天干与日主的五行关系") {
            primary.push(sentence);
        } else {
            background.push(sentence);
        }
    }
    let primary_text = if primary.is_empty() {
        "年度天干和五行没有单独形成强烈线索，先把它们作为背景处理。".to_string()
    } else {
        primary.join("")
    };
    let background_text = if background.is_empty() {
        "地支关系、藏干和大运同场暂时没有补出额外重点。".to_string()
    } else {
        background.join("")
    };
    topic_timeline_signal_story_for_topic(
        overlay.topic,
        overlay.year,
        overlay.age_context,
        &primary_text,
        &background_text,
    )
}

fn topic_timeline_signal_sentence(summary: &str, year: i32) -> String {
    let (label, detail) = summary.split_once('：').unwrap_or(("时间线索", summary));
    let detail = ensure_sentence(detail.trim());
    match label {
        "流年天干所成十神" => format!("{year}年的天干先把十神信号点亮，{detail}"),
        "流年天干与日主的五行关系" => {
            format!("这股力量靠近日主时，{detail}")
        }
        "流年地支的内在底色" => {
            format!("地支深处还带着背景，{detail}")
        }
        "月支被流年触动" => {
            format!("月支被{year}年牵动后，{detail}")
        }
        "日支被流年触动" => {
            format!("日支被{year}年牵动后，{detail}")
        }
        "大运与流年的同场关系" => {
            format!("当前大运与{year}年相遇时，{detail}")
        }
        _ => format!("{label}上，{detail}"),
    }
}

fn topic_timeline_signal_story_for_topic(
    topic: TopicReportTopic,
    year: i32,
    age_context: TopicAgeContext,
    primary_text: &str,
    background_text: &str,
) -> String {
    if age_context.early_stage {
        return match topic {
            TopicReportTopic::Relationship => format!(
                "{}年的情感读法，会先落在安全感、回应方式和边界感的早期塑形上。年度里靠近的压力和支持，会影响日后亲密互动的底色：{}\n\n再看地支、藏干和当前大运，重点不是现实伴侣状态，而是关系模式的雏形能不能被稳定照护接住：{}",
                year, primary_text, background_text
            ),
            TopicReportTopic::Wealth => format!(
                "{}年的金钱读法，会先落在资源感怎样形成：照护是否稳定，兴趣能不能被鼓励，分享和边界是否有温和规则。年度里靠近的压力和机会，会影响资源意识的早期底色：{}\n\n再看地支、藏干和当前大运，重点不是收入或投资，而是资源感、产出欲和边界意识能不能被稳稳托住：{}",
                year, primary_text, background_text
            ),
            TopicReportTopic::Family => format!(
                "{}年的家庭读法，会落在成长环境里最具体的几件事：被怎样回应，情绪怎样被接住，规则怎样被建立。年度里靠近的压力和支持，会影响家庭互动的早期底色：{}\n\n再看地支、藏干和当前大运，真正要稳的是照护、表达和边界，而不是替家庭事件下结论：{}",
                year, primary_text, background_text
            ),
            TopicReportTopic::Career => format!(
                "{}年的事业读法，会先落在规则感、学习任务和表达训练上。年度里靠近的压力和行动要求，会影响早期面对任务的方式：{}\n\n再看地支、藏干和当前大运，重点不是现实职业结果，而是学习承接、方法感和支持系统能不能跟上：{}",
                year, primary_text, background_text
            ),
        };
    }
    match topic {
        TopicReportTopic::Relationship => format!(
            "把{}年放进「情感」专项来看，年度线索要回到亲密互动、表达边界和稳定回应上。十神与五行这一层，会让关系里的吸引、压力和承接感变得更明显：{}\n\n藏干、夫妻宫和当前大运合到一起时，重点就落在靠近之后能不能稳住节奏：{}",
            year, primary_text, background_text
        ),
        TopicReportTopic::Wealth => format!(
            "{}年的金钱读法，可以先落到两件事：机会怎样靠近，资源靠近以后能不能稳住。天干与五行带出的压力和机会会先影响判断节奏：{}\n\n再看地支、藏干和当前大运，重点就不只是有没有资源，而是预算、规则和承载节奏能不能接住资源流动：{}",
            year, primary_text, background_text
        ),
        TopicReportTopic::Family => format!(
            "{}年的家庭读法，会落在三件事上：话怎么说，责任怎么分，情绪怎么安放。年度里靠近的压力和支持，会影响家里谁需要照顾、谁需要边界：{}\n\n再看地支、藏干和当前大运，家庭位置会被进一步拉出来，真正要稳的是支持、表达和责任的摆放：{}",
            year, primary_text, background_text
        ),
        TopicReportTopic::Career => format!(
            "{}年的事业读法，会落在任务标准、技能交付和协作边界上。年度里靠近的压力和行动要求，会先影响工作节奏：{}\n\n再看地支、藏干和当前大运，事业压力能不能转成可持续行动，关键在方法、资源和边界能不能跟上：{}",
            year, primary_text, background_text
        ),
    }
}

fn ensure_sentence(value: &str) -> String {
    if value.ends_with('。') || value.ends_with('！') || value.ends_with('？') {
        value.to_string()
    } else {
        format!("{value}。")
    }
}

fn topic_timeline_year_guidance(
    topic: TopicReportTopic,
    year: i32,
    age_context: TopicAgeContext,
) -> String {
    if age_context.early_stage {
        return match topic {
            TopicReportTopic::Relationship => format!(
                "{}年落到情感上，重点不是现实关系结果，而是让回应更稳定、情绪更能被理解，边界感慢慢有安全的位置。",
                year
            ),
            TopicReportTopic::Wealth => format!(
                "{}年落到金钱上，重点不是收益多少，而是照护资源是否稳定，分享规则是否温和，兴趣和能力能不能被稳稳托住。",
                year
            ),
            TopicReportTopic::Family => format!(
                "{}年落到家庭上，重点不是替亲属下结论，而是看照护、回应、边界和规则如何让早年环境更安稳。",
                year
            ),
            TopicReportTopic::Career => format!(
                "{}年落到事业上，重点不是岗位成败，而是学习任务、规则感、表达训练和支持系统能不能形成更稳的早期节奏。",
                year
            ),
        };
    }
    match topic {
        TopicReportTopic::Relationship => format!(
            "{}年落到情感上，重点不是急着确认结果，而是看亲密互动能不能承接稳定回应、清楚边界和持续沟通。",
            year
        ),
        TopicReportTopic::Wealth => format!(
            "{}年落到金钱上，重点不是判断收益多少，而是把预算、产出、合作分配和规则承接放到明处，先稳住资源节奏。",
            year
        ),
        TopicReportTopic::Family => format!(
            "{}年落到家庭上，重点不是替亲属下结论，而是看支持、边界、表达和责任如何重新安排，尽量让话说清、事分明。",
            year
        ),
        TopicReportTopic::Career => format!(
            "{}年落到事业上，重点不是断岗位成败，而是看责任压力、技能表达、资源落地和协作边界能不能形成更稳定的工作节奏。",
            year
        ),
    }
}

pub fn assemble_report(disclaimer: &str, blocks: &[TopicReportBlock]) -> String {
    let assembled = blocks
        .iter()
        .map(|block| format!("【{}】\n{}", block.title, block.body))
        .collect::<Vec<_>>()
        .join("\n\n");
    format!("{disclaimer}\n\n---\n\n{assembled}")
}

fn topic_disclaimer(topic: TopicReportTopic) -> String {
    match topic {
        TopicReportTopic::Relationship => "本报告是「情感专项」的传统命理结构解读，适合用于文化阅读、自我观察和关系节奏参考。它会使用夫妻宫、配偶星、十神、合冲刑害、大运和流年引动等术语；文中的建议只围绕相处节奏、沟通方式和边界意识展开，现实中的亲密关系仍应以沟通、尊重和实际处境为准。".to_string(),
        TopicReportTopic::Wealth => "本报告是「金钱专项」的传统命理结构解读，只用于文化阅读和自我观察。它会使用财星、正财、偏财、食伤生财、比劫、印星、官杀、大运和流年引动等术语，但这些术语只描述命盘中的资源模式、产出方式和承载结构，不构成金融、借贷、资产配置、税务、法律或商业决策意见。现实财务事务仍应以事实、预算、风控和专业意见为准。".to_string(),
        TopicReportTopic::Family => "本报告是「家庭专项」的传统命理结构解读，只用于文化阅读和自我观察。它会使用宫位、年柱、月柱、日支、时柱、印星、比劫、食伤、财官、大运和流年引动等术语，但这些术语只描述这张命盘中的家庭互动结构，不预测亲属健康、婚育、离合或家庭事件。现实家庭关系仍应以沟通、尊重、照护边界和实际处境为准。".to_string(),
        TopicReportTopic::Career => "本报告是「事业专项」的传统命理结构解读，只用于文化阅读和自我观察。它会使用官杀、印星、食伤、财星、比劫、格局、用神、大运和流年引动等术语，但这些术语只描述责任结构、技能表达、资源调度和协作方式，不构成职业规划结论、岗位承诺或现实成果判断。现实职业选择仍应结合能力、机会、市场、合同和专业建议。".to_string(),
    }
}

fn audit_topic_text(value: &str) -> ForbiddenOutputAudit {
    let base = audit_text(value);
    let rejected = base.status == "rejected"
        || TOPIC_FORBIDDEN_PATTERNS
            .iter()
            .any(|pattern| value.contains(pattern));
    ForbiddenOutputAudit {
        status: if rejected { "rejected" } else { "passed" },
        checked_patterns: base.checked_patterns + TOPIC_FORBIDDEN_PATTERNS.len(),
    }
}

#[derive(Clone, Debug)]
struct StarSummary {
    level: &'static str,
    summary: String,
    evidence: Vec<String>,
}

#[derive(Clone, Debug)]
struct RelationHit {
    relation: &'static str,
    target: &'static str,
    target_branch: String,
    evidence: String,
}

fn relationship_overview_block(
    chart: &ChartResult,
    strength: &StrengthAssessment,
    pattern: &PatternInfo,
    spouse_summary: &StarSummary,
    relations: &[RelationHit],
    age_context: TopicAgeContext,
) -> TopicReportBlock {
    let relation_tone = palace_relation_plain(relations);
    let opening = relationship_overview_opening(relations, spouse_summary);
    let spouse_tone = if spouse_summary.level == "不显" {
        "伴侣议题不靠单一配偶星撑起，更需要结合夫妻宫、表达方式和安全感一起看。"
    } else {
        "命局里能看见配偶星的牵动，关系靠近时往往会同时带出责任、吸引、边界和稳定感的考验。"
    };
    TopicReportBlock {
        id: "relationship-overview",
        title: "总断",
        body: format!(
            "{}{}\n\n从命理结构看，您的日主为「{}」，日柱为「{}」，夫妻宫落在日支「{}」。当前日主强弱为「{}」，格局参考为「{}」。这说明情感不能只看一个符号，要把日主的用力方式、夫妻宫的落点、配偶星的牵动和表达支持系统放在一起读。\n\n{}\n\n{}",
            opening,
            early_stage_context_paragraph(TopicReportTopic::Relationship, age_context),
            chart.chart.day.stem,
            chart.chart.day.ganzhi(),
            chart.chart.day.branch,
            strength.level,
            pattern.pattern_name,
            spouse_tone,
            relation_tone
        ),
    }
}

fn relationship_overview_opening(
    relations: &[RelationHit],
    spouse_summary: &StarSummary,
) -> &'static str {
    if relations.iter().any(|hit| hit.relation == "六冲") {
        "您的情感结构带着明显的节奏感：关系靠近时，吸引和拉扯常常会同时出现。真正要看的不是有没有情绪波动，而是双方能不能在靠近之后把节奏、边界和回应方式稳定下来。"
    } else if relations.iter().any(|hit| hit.relation == "六合") {
        "您的情感结构更重视连接感。关系一旦有机会靠近，重点不在短暂热度，而在彼此能否自然汇合、持续回应，并把好感落成可信的相处。"
    } else if relations.is_empty() && spouse_summary.level == "不显" {
        "您的情感主题不靠单一强信号推动，更像是慢慢看清、慢慢建立的类型。关系里的关键不是突然发生什么，而是谁能让您感到被理解、被尊重，并愿意一起把日子安顿下来。"
    } else {
        "您的情感结构并不适合只看表面的热烈。关系靠近后，真正重要的是责任感、表达方式和安全感能不能接得住，而不是一开始有多强烈。"
    }
}

fn relationship_spouse_star_block(
    chart: &ChartResult,
    spouse_summary: &StarSummary,
) -> TopicReportBlock {
    let sex_note = match chart.basis.request.birth_profile.sex {
        Sex::Male => {
            "按传统取象，男性命盘会更偏重财星来观察伴侣议题；这里仍会同时参考官杀、食伤、比劫和印星，避免把关系压成单一符号。"
        }
        Sex::Female => {
            "按传统取象，女性命盘会更偏重官杀来观察伴侣议题；这里仍会同时参考财星、食伤、比劫和印星，避免把关系压成单一符号。"
        }
        Sex::Unspecified => {
            "性别信息未参与取象时，本段采用中性读法：财星和官杀都只作为关系议题符号参考，不套用单一性别断语。"
        }
    };
    TopicReportBlock {
        id: "spouse-star",
        title: "伴侣议题",
        body: format!(
            "{}\n\n从十神脉络看，{}\n\n放到亲密关系里看，您很难只被表面的热度打动。您会看对方是否可靠，是否有承担感，是否能把吸引力落成稳定相处。若关系一开始很强烈，更要慢慢看它能否沉下来；若关系推进较慢，也未必是坏事，关键在于回应是否持续、边界是否被尊重。",
            sex_note, spouse_summary.summary
        ),
    }
}

fn relationship_palace_block(chart: &ChartResult, relations: &[RelationHit]) -> TopicReportBlock {
    let relation_text = if relations.is_empty() {
        "当前夫妻宫没有被月支、时支、大运或年度地支形成明显合冲刑害。".to_string()
    } else {
        relations
            .iter()
            .map(|hit| {
                format!(
                    "{}与{}「{}」形成\"{}\"",
                    chart.chart.day.branch, hit.target, hit.target_branch, hit.relation
                )
            })
            .collect::<Vec<_>>()
            .join("；")
    };

    TopicReportBlock {
        id: "relationship-palace",
        title: "夫妻宫",
        body: format!(
            "夫妻宫看的是日支「{}」，它是本报告观察亲密关系落点的位置。放回这张命盘来看，{}。\n\n传统所说的合冲刑害，在这里不是拿来制造结论，而是看关系靠近后的运动方式。如果夫妻宫被\"合\"牵动，关系主题容易出现靠近、连接和议题汇合；如果被\"冲\"牵动，感情里更容易出现节奏差、立场差、靠近与拉开的反复；如果被\"刑害\"牵动，则更要留意细节磨合、沟通成本和边界感。\n\n对您来说，夫妻宫最值得看的不是有没有吸引，而是吸引之后能不能协调节奏。合适的关系既要有热度，也要有秩序；既能靠近，也能给彼此留出稳定的位置。",
            chart.chart.day.branch, relation_text
        ),
    }
}

fn relationship_expression_block(
    expression_summary: &StarSummary,
    support_summary: &StarSummary,
) -> TopicReportBlock {
    TopicReportBlock {
        id: "relationship-expression",
        title: "表达、边界与安全感",
        body: format!(
            "情感关系不只看配偶星，也要看您怎样表达、怎样设边界、怎样获得安全感。食伤对应表达和情绪输出，比劫对应自我边界和关系中的立场感，印星对应被理解、被接纳和被支持的需要。\n\n{}\n\n{}\n\n这表示您并不是没有感受，而是感受未必会第一时间柔软地说出来。关系越重要，越需要更早、更温和地讲清楚：我需要什么，我不接受什么，我希望关系怎样慢慢稳定下来。",
            expression_summary.summary, support_summary.summary
        ),
    }
}

fn relationship_trigger_block(
    year: i32,
    annual: &Pillar,
    current_luck: Option<&LuckCycle>,
    relations: &[RelationHit],
    overlay: Option<&TopicTimelineOverlay>,
    age_context: TopicAgeContext,
) -> TopicReportBlock {
    let luck_text = current_luck_stage_text(year, current_luck);

    let annual_text = relations
        .iter()
        .find(|hit| hit.target == "年度地支")
        .map(|hit| format!("{year}年年度地支与夫妻宫形成\"{}\"，情感议题会被牵动，尤其要看靠近之后的节奏、边界和安全感。", hit.relation))
        .unwrap_or_else(|| format!("{year}年年度地支没有与夫妻宫形成明显合冲刑害，因此只作为普通年度背景记录。"));

    let overlay_text = overlay
        .map(|item| {
            let signal_summary = if item.selected_signal_summaries.is_empty() {
                "大运与流年的同场作用没有再带出额外情感重点，仍以夫妻宫和配偶星作为主线。".to_string()
            } else {
                "继续看大运与流年的同场作用，几条情感重点会一起出现，核心仍落在亲密互动、表达、边界和支持。".to_string()
            };
            format!(
                "{}年年柱「{}」与当前大运「{}」同场时，情感主题会更容易浮到眼前。{}",
                year, item.annual_pillar, item.current_luck_pillar, signal_summary
            )
        })
        .unwrap_or_else(|| format!("{year}年暂未形成可用的大运叠加文本，本段先保留年度引动。"));
    let guidance = if age_context.early_stage {
        format!(
            "若把{}年作为早年阶段来看，本段不讨论现实恋爱状态，而是观察情绪回应、依恋安全感和边界感怎样被照护出来。关系模式越早得到稳定回应，日后面对亲近关系时越容易知道什么是安全、什么是尊重。",
            year
        )
    } else {
        format!(
            "如果目前单身，{}年适合慢慢分辨：谁只是带来短暂波动，谁能给出稳定回应并长期兑现。若已有关系，{}年更适合把沟通节奏、边界、承诺和稳定感摆到明处，不急着争输赢，先看关系能否变得更清楚、更安定。",
            year, year
        )
    };

    TopicReportBlock {
        id: "relationship-trigger",
        title: "年度情感引动",
        body: format!(
            "{}\n\n{}年的年柱为「{}」。{}\n\n{}\n\n{}",
            luck_text,
            year,
            annual.ganzhi(),
            annual_text,
            overlay_text,
            guidance
        ),
    }
}

fn relationship_plain_block(
    chart: &ChartResult,
    snapshot: &AnalysisSnapshot,
    warnings: &[&'static str],
    _spouse_summary: &StarSummary,
    _expression_summary: &StarSummary,
    _support_summary: &StarSummary,
    relations: &[RelationHit],
    overlay: Option<&TopicTimelineOverlay>,
    age_context: TopicAgeContext,
) -> TopicReportBlock {
    let reading_condition = relationship_reading_condition(warnings, snapshot);
    let relation_text = palace_relation_plain(relations);
    let overlay_text = overlay
        .map(|item| {
            format!(
                "{}年与当前大运「{}」叠加后，情感主题会更集中落在{}。",
                item.year,
                item.current_luck_pillar,
                topic_timeline_plain_focus(TopicReportTopic::Relationship)
            )
        })
        .unwrap_or_else(|| "年度叠加信息不足时，仍以原局情感结构作为主要参考。".to_string());
    let closing = if age_context.early_stage {
        "所以，早年阶段真正值得照看的，是稳定回应、温和边界和被理解的安全感。它不会直接指向现实关系结果，却会慢慢塑造一个人以后怎样靠近、怎样表达、怎样相信关系。"
    } else {
        "所以，真正适合您的关系，会让您越来越安定，而不是让您长期处在猜测和紧绷里。吸引力可以作为开始，稳定回应、清楚边界和现实承接，才是关系能不能走长的关键。"
    };

    TopicReportBlock {
        id: "relationship-plain-summary",
        title: "结论",
        body: format!(
            "综合来看，您的情感关键词是：慢热、强牵动、重边界、要稳定。夫妻宫「{}」提示亲密关系的相处位置，{}配偶星把吸引、承担和可靠感带进来；表达与安全感则提醒您，越在意的关系，越需要把话说早、说稳。\n\n{}\n\n{}。\n\n{}",
            chart.chart.day.branch, relation_text, overlay_text, reading_condition, closing
        ),
    }
}

fn relationship_reading_condition(
    warnings: &[&'static str],
    snapshot: &AnalysisSnapshot,
) -> String {
    let mut notes = Vec::new();
    if warnings
        .iter()
        .any(|warning| warning.contains("时辰未知") || warning.contains("unknown_hour"))
        || snapshot
            .sensitivity_flags
            .iter()
            .any(|flag| flag.contains("unknown_hour"))
    {
        notes
            .push("因为出生时辰信息不完整，时柱所代表的长期互动、后段节奏和细节承接会读得更保守。");
    }
    if warnings
        .iter()
        .any(|warning| warning.contains("性别未指定"))
    {
        notes.push(
            "因为本次没有指定性别，伴侣议题会同时参考财星与官杀，不把关系判断压到单一取象上。",
        );
    }
    if snapshot
        .sensitivity_flags
        .iter()
        .any(|flag| flag.contains("ambiguity"))
    {
        notes.push(
            "命盘基础信息存在可读分歧时，本报告会优先采用稳健解释，避免把细小信号说成确定结论。",
        );
    }
    if warnings
        .iter()
        .any(|warning| warning.starts_with("topic_timeline_"))
    {
        notes.push("时间线资料不足的部分已经降级处理，因此年度引动只取最清楚的结构线索。");
    }

    if notes.is_empty() {
        "这份情感专项的收束，可以落在夫妻宫、伴侣议题、表达方式和年度引动的合参上".to_string()
    } else {
        notes.join("")
    }
}

fn wealth_overview_block(
    chart: &ChartResult,
    strength: &StrengthAssessment,
    pattern: &PatternInfo,
    age_context: TopicAgeContext,
) -> TopicReportBlock {
    let opening = if age_context.early_stage {
        "您的金钱结构在早年阶段不按现实财富多少来读，而是看资源感、分享边界、兴趣产出和规则意识怎样被照护出来。真正要看的不是钱有多少，而是资源靠近时，孩子能不能慢慢形成稳定感、分寸感和持续投入的能力。"
    } else {
        "您的金钱结构不能只看财星有没有出现，更要看资源从哪里来、靠什么产出、由谁分配，又用什么规则承接。真正要读的不是现实财富多少，而是资源一靠近，您更容易先被机会牵动，还是先被责任、边界和节奏牵动。"
    };
    TopicReportBlock {
        id: "wealth-overview",
        title: "总断",
        body: format!(
            "{}{}\n\n从命理结构看，您的日主为「{}」，日柱为「{}」，当前日主强弱为「{}」，格局参考为「{}」。这说明金钱议题要把日主承载力、财星、食伤产出、比劫分配、印星支持和官杀规则放在一起读。\n\n财星在这里更像资源意识和交换模式。日主偏弱时，钱的议题更怕节奏被推得太急，需要支持、边界和规则托住；日主偏强时，则更适合谈资源调度、主动经营和产出转化。",
            opening,
            early_stage_context_paragraph(TopicReportTopic::Wealth, age_context),
            chart.chart.day.stem,
            chart.chart.day.ganzhi(),
            strength.level,
            pattern.pattern_name
        ),
    }
}

fn wealth_daily_reading_block(age_context: TopicAgeContext) -> TopicReportBlock {
    let body = if age_context.early_stage {
        "早年阶段的金钱专项，更适合看资源感从哪里开始：照护是否稳定，兴趣材料是否充足，学习和表达有没有被鼓励，分享时有没有温和边界，想要与等待之间能不能慢慢形成分寸。\n\n对孩子来说，资源不是只指钱，也包括时间、关注、玩具、学习材料和被允许探索的空间。这里最值得看的，是资源靠近之后，孩子能不能感到安心，能不能学会珍惜，也能不能慢慢知道什么可以分享、什么需要等待。"
    } else {
        "金钱议题通常会从几道门进入生活：稳定事务、外部机会、技能产出、合作分配、规则约束。正财偏向稳定可管理的事务，偏财偏向外部流动和机会交换，食伤把能力做成输出，比劫让合作、竞争和分配边界更容易被看见。\n\n对您来说，这一章最值得看的，是资源靠近之后能不能变成稳定节奏：产出是否持续，合作边界是否清楚，机会出现时有没有规则承接。能把这三件事稳住，金钱议题才不容易只剩短暂波动。"
    };
    TopicReportBlock {
        id: "wealth-daily-reading",
        title: "资源入口",
        body: body.to_string(),
    }
}

fn wealth_star_block(wealth: &StarSummary, age_context: TopicAgeContext) -> TopicReportBlock {
    let body = if age_context.early_stage {
        format!(
            "财星放到早年阶段，看的不是收入和资产，而是孩子怎样感知资源：稳定照护能不能让他安心，外部刺激会不会让兴趣被点亮，资源变化时有没有人帮他建立规则。\n\n这张盘里的财星线索可以这样读：{}\n\n如果财星明显，孩子会更容易注意到资源变化、被给予的东西和外界带来的新鲜感；如果财星不明显，也不是没有资源主题，而是更需要靠稳定照护、表达鼓励和清楚规则，把资源感慢慢养出来。",
            wealth.summary
        )
    } else {
        format!(
            "财星看的是资源怎样靠近您，也看资源靠近之后能不能留下来。正财偏稳定、可管理和长期经营；偏财偏机会、流动和外部交换。对您来说，重点不是只问有没有机会，而是机会来了以后，能不能被节奏、规则和承载能力接住。\n\n这张盘里的财星线索可以这样读：{}\n\n如果财星明显，您容易对资源变化、机会窗口和现实回报更敏感；如果财星不明显，也不是没有金钱主题，而是更需要靠食伤产出、比劫边界、印星支持和官杀规则把资源慢慢做实。钱能不能稳，往往取决于资源靠近之后有没有清楚的承接方式。",
            wealth.summary
        )
    };
    TopicReportBlock {
        id: "wealth-star",
        title: "正财、偏财与资源意识",
        body,
    }
}

fn wealth_flow_block(
    output: &StarSummary,
    peer: &StarSummary,
    resource: &StarSummary,
    officer: &StarSummary,
    age_context: TopicAgeContext,
) -> TopicReportBlock {
    let body = if age_context.early_stage {
        format!(
            "早年资源感要成形，不能只靠被给予。第一层看表达和兴趣：{} 第二层看分享与边界：{} 第三层看照护和规则：{}{}\n\n这几层合在一起，会把资源主题从「有没有」推进到「能不能安稳使用」。兴趣如果常被鼓励，孩子会更愿意尝试；分享如果有温和边界，就不容易变成争抢；规则如果清楚，资源带来的刺激才更容易沉成安全感。",
            output.summary, peer.summary, resource.summary, officer.summary
        )
    } else {
        format!(
            "金钱要成节奏，不能只靠机会本身。第一层看产出：{} 第二层看分配：{} 第三层看承接：{}{}\n\n这几层合在一起，会把金钱议题从「来不来」推进到「留不留得住」。能力如果不能持续输出，资源容易短促；合作如果边界不清，分配容易消耗；支持和规则如果跟不上，机会再近也容易变成压力。真正值得您重视的，是把产出、分配、规则放进同一个节奏里。",
            output.summary, peer.summary, resource.summary, officer.summary
        )
    };
    TopicReportBlock {
        id: "wealth-flow",
        title: "食伤生财、比劫分配与约束",
        body,
    }
}

fn wealth_capacity_block(
    strength: &StrengthAssessment,
    pattern: &PatternInfo,
    useful_gods: &[UsefulGodHint],
    age_context: TopicAgeContext,
) -> TopicReportBlock {
    let closing = if age_context.early_stage {
        "早年阶段如果结构偏弱，资源主题更需要稳定照护和规则托住；如果结构偏强，则更适合用分享、等待和表达训练来调节。无论哪一种，重点都在资源感怎样被温和养成。"
    } else {
        "如果结构偏弱，财星主题出现时更适合强调支持、节奏和边界；如果结构偏强，资源主题更适合结合产出、规则和调度能力来读。无论哪一种，重点都在资源节奏怎样被稳住。"
    };
    TopicReportBlock {
        id: "wealth-capacity",
        title: "承载能力与用神提示",
        body: format!(
            "金钱专项不能只看财星数量，还要看日主是否能承载财星。这里的承载能力指的是命盘结构能否接住资源压力。当前日主强弱为「{}」，格局参考为「{}」。\n\n用神提示：{}\n\n{}",
            strength.level,
            pattern.pattern_name,
            useful_god_text(useful_gods),
            closing
        ),
    }
}

fn wealth_trigger_block(
    year: i32,
    annual: &Pillar,
    current_luck: Option<&LuckCycle>,
    trigger: &StarSummary,
    age_context: TopicAgeContext,
) -> TopicReportBlock {
    let guidance = if age_context.early_stage {
        format!(
            "对早年阶段来说，{}年更适合看照护资源是否稳定、兴趣和能力能不能被鼓励、分享与边界有没有温和规则；不把它读成赚钱、投资或独立财务。",
            year
        )
    } else {
        format!(
            "对您来说，{}年更适合把预算意识、资源边界、技能产出和现实规则放到明处。机会可以看，但先看能不能承接；资源可以动，但先看节奏和边界是否稳。",
            year
        )
    };
    TopicReportBlock {
        id: "wealth-trigger",
        title: "大运与流年引动",
        body: format!(
            "大运和流年放到金钱专项里，重点看资源主题在什么阶段更容易被推到眼前。\n\n{}\n\n{}年的年柱为「{}」。{}\n\n{}",
            current_luck_stage_text(year, current_luck),
            year,
            annual.ganzhi(),
            trigger.summary,
            guidance
        ),
    }
}

fn wealth_plain_block(
    snapshot: &AnalysisSnapshot,
    age_context: TopicAgeContext,
) -> TopicReportBlock {
    let sensitivity = sensitivity_text(snapshot, TopicReportTopic::Wealth);
    let closing = if age_context.early_stage {
        "所以，早年阶段真正适合被照看的，不是钱能不能扩大，而是资源感能不能稳定、兴趣能不能被支持、分享和边界能不能慢慢建立。"
    } else {
        "所以，真正适合您的资源节奏，不是被一时机会推着走，而是先把预算边界立稳，把产出节奏做实，再让合作和规则接住资源流动。钱的议题越靠近，越要先稳住承载，再谈扩大。"
    };
    TopicReportBlock {
        id: "wealth-plain-summary",
        title: "结论",
        body: format!(
            "综合来看，您的金钱关键词是：重承接、看边界、靠产出、要规则。财星让您看见资源从哪里来，食伤决定能力能不能变成持续产出，比劫提醒合作与分配要说清，印星和官杀则负责把支持、秩序和现实约束立住。\n\n{}\n\n{}",
            sensitivity, closing
        ),
    }
}

fn family_overview_block(
    chart: &ChartResult,
    strength: &StrengthAssessment,
    pattern: &PatternInfo,
    age_context: TopicAgeContext,
) -> TopicReportBlock {
    TopicReportBlock {
        id: "family-overview",
        title: "总断",
        body: format!(
            "您的家庭结构要看的不是亲属会发生什么，而是您在家庭关系里怎样感受支持、怎样表达需求、怎样维护边界，又怎样承接现实责任。家庭越靠近生活细节，越不能只看一个宫位，要把宫位、印星、比劫、食伤和财官合在一起读。{}\n\n从命理结构看，年柱「{}」可作为家族背景和早年环境参考，月柱「{}」可作为成长环境与长辈关系参考，日支「{}」是自我与亲密生活的落点，时柱则用于晚辈、长期安排和后段主题的谨慎参考。\n\n当前日主强弱为「{}」，格局参考为「{}」。这说明家庭议题的重点不在替家人下结论，而在看支持、边界、表达和责任能不能各归其位。",
            early_stage_context_paragraph(TopicReportTopic::Family, age_context),
            chart.chart.year.ganzhi(),
            chart.chart.month.ganzhi(),
            chart.chart.day.branch,
            strength.level,
            pattern.pattern_name
        ),
    }
}

fn family_daily_reading_block() -> TopicReportBlock {
    TopicReportBlock {
        id: "family-daily-reading",
        title: "互动位置",
        body: "家庭关系里，一个人常常会在几个位置之间切换：有时需要被支持，有时负责协调，有时必须把话说清，有时要守住边界，有时又不得不承接现实事务。年柱、月柱、日支、时柱对应这些互动层次。\n\n印星看您怎样感到被照顾，比劫看您怎样和同辈或身边人划边界，食伤看您怎样开口表达，财官看现实事务怎样安排。真正有用的读法，是看自己在家里最常用哪一种方式，哪一种方式又最需要温和调整。".to_string(),
    }
}

fn family_support_block(resource: &StarSummary, useful_gods: &[UsefulGodHint]) -> TopicReportBlock {
    TopicReportBlock {
        id: "family-support",
        title: "印星与支持系统",
        body: format!(
            "家庭里的支持，不只是有人帮忙，也包括情绪能不能被理解、需求能不能被接住、规则里有没有保护感。印星落到这里，看的就是您在家庭中怎样获得安定，也怎样把安定感给出去。\n\n这张盘里的支持线索可以这样读：{}\n\n用神提示：{}\n\n如果印星明显，家庭互动更需要稳定的照顾、理解和保护感；如果印星不明显，也不等于没有支持，而是支持常常要靠清楚表达、边界协商和现实责任来补足。对您来说，真正要辨认的是：家里哪些地方能接住您，哪些地方需要您把需求说得更明白。",
            resource.summary,
            useful_god_text(useful_gods)
        ),
    }
}

fn family_peer_output_block(peer: &StarSummary, output: &StarSummary) -> TopicReportBlock {
    TopicReportBlock {
        id: "family-peer-output",
        title: "比劫边界与食伤表达",
        body: format!(
            "家庭越亲近，边界越容易被忽略；话越重要，越容易因为顾虑而说晚。比劫和食伤放在一起，是看您怎样守住自己的位置，也看您怎样把需求说到别人能听懂。\n\n边界这一层：{}\n\n表达这一层：{}\n\n如果边界先动，家里的议题容易落在分工、立场和资源分配上；如果表达先动，重点就会转到怎么开口、怎么解释、怎么避免情绪累积。对您来说，比较稳的方式不是忍到最后才说，也不是一开口就定性，而是先把位置放稳，再把话说清。",
            peer.summary, output.summary
        ),
    }
}

fn family_duty_block(duty: &StarSummary) -> TopicReportBlock {
    TopicReportBlock {
        id: "family-duty",
        title: "财官与现实责任",
        body: format!(
            "家庭最终会落到很多具体事务上：谁来安排，谁来承担，规则怎么定，事情出了变化谁来接。财官这一层看的正是这些现实责任，而不是替亲属下结论。\n\n这张盘里的责任线索可以这样读：{}\n\n财官明显时，家庭主题更容易落在事务安排、角色分工和规则稳定上。它提醒您，很多家庭压力不是因为没有感情，而是责任没有说清、位置没有放稳。把事情的边界讲明白，关系反而更容易安定。",
            duty.summary
        ),
    }
}

fn family_trigger_block(
    year: i32,
    annual: &Pillar,
    current_luck: Option<&LuckCycle>,
    relations: &[RelationHit],
    age_context: TopicAgeContext,
) -> TopicReportBlock {
    let relation_text = relation_hits_text(
        relations,
        "家庭宫位暂未被年度地支或当前大运形成明显合冲刑害。",
    );
    let guidance = if age_context.early_stage {
        format!(
            "对早年阶段来说，{}年更适合看照护是否稳定、情绪能不能被接住、规则能不能温和建立。把回应做稳，比急着分责任更重要。",
            year
        )
    } else {
        format!(
            "对您来说，{}年更适合把家里的话说清、事分明：谁需要支持，谁需要边界，哪些责任要提前讲明。把关系里的位置安顿得更稳，家庭互动就不容易被一时情绪带着走。",
            year
        )
    };
    TopicReportBlock {
        id: "family-trigger",
        title: "大运与流年引动",
        body: format!(
            "大运和流年放到家庭专项里，重点看支持、边界、表达和责任在什么时候更容易被触动。\n\n{}\n\n{}年的年柱为「{}」。{}\n\n{}",
            current_luck_stage_text(year, current_luck),
            year,
            annual.ganzhi(),
            relation_text,
            guidance
        ),
    }
}

fn family_plain_block(
    chart: &ChartResult,
    snapshot: &AnalysisSnapshot,
    age_context: TopicAgeContext,
) -> TopicReportBlock {
    let hour_note = if chart.chart.hour.is_some() {
        "时柱已提供，因此晚辈、长期安排和后段主题可以作为参考，但仍不作事件断语。"
    } else {
        "时辰未知，因此时柱相关的晚辈、长期安排和后段主题已降级，不展开具体判断。"
    };
    let closing = if age_context.early_stage {
        "所以，早年阶段真正适合被照看的，是稳定照护、清楚回应和温和边界。家庭节奏稳下来，孩子才更容易把情绪、需求和规则慢慢放到合适的位置。"
    } else {
        "所以，真正适合您的家庭节奏，不是所有事都靠忍耐维持，也不是一有情绪就急着定性。先把需求说清，再把责任分明，最后把边界放稳，家庭关系才更容易从情绪拉扯回到可以承接的位置。"
    };
    TopicReportBlock {
        id: "family-plain-summary",
        title: "结论",
        body: format!(
            "综合来看，您的家庭关键词是：要支持、重边界、需表达、能承接。宫位提示家庭互动的层次，印星看照护和理解，比劫看同辈边界，食伤看开口方式，财官看现实责任怎样落位。\n\n{}\n\n{}\n\n{}",
            hour_note,
            sensitivity_text(snapshot, TopicReportTopic::Family),
            closing
        ),
    }
}

fn career_overview_block(
    chart: &ChartResult,
    strength: &StrengthAssessment,
    pattern: &PatternInfo,
    age_context: TopicAgeContext,
) -> TopicReportBlock {
    let opening = if age_context.early_stage {
        "您的事业结构在早年阶段不按职位高低来读，而是看规则感、学习承接、表达训练和支持系统能不能形成稳定节奏。压力不一定是坏事，关键在于它有没有被照护、方法和边界接住。"
    } else {
        "您的事业结构带着明显的责任感和压力感：真正要看的不是现实职位高低，而是任务、规则、技能、资源和协作能不能形成稳定节奏。压力不一定是坏事，关键在于它有没有被学习、方法和边界接住。"
    };
    TopicReportBlock {
        id: "career-overview",
        title: "总断",
        body: format!(
            "{}{}\n\n从命理结构看，月柱「{}」是外部环境、秩序和阶段任务的重要参考，日主「{}」说明用力主体，时柱可作为长期安排和后段发力参考。当前日主强弱为「{}」，格局参考为「{}」。这说明事业不能只看官杀，也要合看印星承接、食伤表达、财星落地和比劫协作。",
            opening,
            early_stage_context_paragraph(TopicReportTopic::Career, age_context),
            chart.chart.month.ganzhi(),
            chart.chart.day.stem,
            strength.level,
            pattern.pattern_name
        ),
    }
}

fn career_daily_reading_block(age_context: TopicAgeContext) -> TopicReportBlock {
    let body = if age_context.early_stage {
        "早年阶段的事业专项，更适合看学习和成长中的用力方式：规则感怎样建立，任务感怎样被接住，表达训练有没有出口，兴趣成果能不能被鼓励，同伴边界是否清楚。\n\n放进成长场景里，官杀明显时，要留意规则和任务会不会给孩子带来压力；印星明显时，要看照护、理解和学习支持是否足够；食伤明显时，要看表达、动手和解决问题的兴趣；财星明显时，看资源材料能不能支持探索；比劫明显时，则看同伴互动和自我边界。"
    } else {
        "事业议题通常会从几种方式进入生活：规则和责任先到，学习和支撑先到，方案和表达先到，资源落地先到，或者协作边界先到。官杀、印星、食伤、财星、比劫这些词，分别对应责任、承接、技能表达、资源落地和协作竞争。\n\n放进工作场景里，官杀明显时，要留意任务标准和压力承接；印星明显时，要看学习、资质和支持系统；食伤明显时，要看表达、技术和解决方案；财星明显时，要看资源怎样落地；比劫明显时，要看团队边界和协作节奏。这些线索合起来，就是事业上的用力方式。"
    };
    TopicReportBlock {
        id: "career-daily-reading",
        title: "事业用力方式",
        body: body.to_string(),
    }
}

fn career_responsibility_block(
    officer: &StarSummary,
    resource: &StarSummary,
    age_context: TopicAgeContext,
) -> TopicReportBlock {
    let body = if age_context.early_stage {
        format!(
            "早年阶段的责任压力，更多体现为规则、要求、任务感和环境期待。官杀把秩序和标准推近；印星则看孩子有没有足够的理解、照护、学习支持和情绪缓冲来承接它。\n\n责任这一层：{}\n\n承接这一层：{}\n\n这说明早年面对规则时，真正不能只靠催促。任务越明显，越要把规则说清，把步骤拆小，把支持给足。这样压力才可能变成秩序感；如果缺少承接，孩子感到的就容易只是紧张。",
            officer.summary, resource.summary
        )
    } else {
        format!(
            "事业里的压力不是只看大小，而是看它有没有被方法、学习和支持系统接住。官杀把任务、标准和外部要求推近；印星则看您有没有足够的理解、资质、缓冲和支撑来承接它。\n\n责任这一层：{}\n\n承接这一层：{}\n\n这说明您面对事业压力时，真正不能只靠硬扛。压力越明显，越要把标准拆清楚，把学习路径补上，把支持系统用起来。能做到这一点，压力才可能变成行动力；做不到时，压力就容易只剩紧绷感。",
            officer.summary, resource.summary
        )
    };
    TopicReportBlock {
        id: "career-responsibility",
        title: "官杀责任与印星承接",
        body,
    }
}

fn career_skill_resource_block(
    output: &StarSummary,
    wealth: &StarSummary,
    age_context: TopicAgeContext,
) -> TopicReportBlock {
    let body = if age_context.early_stage {
        format!(
            "早年的能力不急着看结果，更适合看兴趣、表达和动手能力能不能被看见、被鼓励、被慢慢训练。食伤看表达和解决问题的方式，财星看材料、资源和外部反馈能不能支持这些尝试。\n\n表达训练这一层：{}\n\n资源支持这一层：{}\n\n如果食伤更醒目，孩子需要把想法说出来、做出来，并得到温和回应；如果财星更醒目，就要看资源和材料是否足够稳定。这里不替孩子预设职业，而是在看能力萌芽时，环境能不能给出合适的土壤。",
            output.summary, wealth.summary
        )
    } else {
        format!(
            "事业不能只停在想法和能力上，还要看它能不能被交付、被看见、被现实资源接住。食伤看技能和表达怎样出来，财星看这些输出能不能落成结果、资源和可持续事务。\n\n技能这一层：{}\n\n落地这一层：{}\n\n如果食伤更醒目，您需要把想法、技术或方案做成别人能理解、能使用的成果；如果财星更醒目，就要看资源配置、现实目标和结果承接。这里不替您指定行业，而是在提醒：能力只有落到可交付的形式，事业节奏才不会停在消耗里。",
            output.summary, wealth.summary
        )
    };
    TopicReportBlock {
        id: "career-skill-resource",
        title: "食伤技能与财星落地",
        body,
    }
}

fn career_collaboration_block(
    peer: &StarSummary,
    useful_gods: &[UsefulGodHint],
    age_context: TopicAgeContext,
) -> TopicReportBlock {
    let body = if age_context.early_stage {
        let peer_summary = peer
            .summary
            .replace("团队边界", "同伴边界")
            .replace("协作竞争", "同伴磨合");
        format!(
            "早年阶段的协作，主要落在同伴互动、分享边界、自我立场和规则感上。比劫这一层不是说一定会争抢，而是提醒：孩子需要知道自己的位置，也需要慢慢学会怎样和别人一起玩、一起学、一起完成事情。\n\n同伴边界这一层：{}\n\n用神提示：{}\n\n如果比劫明显，成长里更容易同时出现自主表达和同伴磨合的需求；如果格局压力也明显，就更需要用清楚规则、稳定照护和温和提醒来接住。稳的成长节奏，不是压住孩子的立场，而是帮他把边界、规则和合作感慢慢养出来。",
            peer_summary,
            useful_god_text(useful_gods)
        )
    } else {
        format!(
            "事业越往现实推进，越绕不开协作、竞争和边界。比劫这一层不是说一定会有冲突，而是提醒您：自己要站稳，合作要说清，资源和责任不要混在一起。\n\n协作这一层：{}\n\n用神提示：{}\n\n如果比劫明显，工作里容易同时出现自主推进和团队协调的需求；如果格局压力也明显，就更需要用学习、规则和清晰边界来接住。对您来说，稳的事业节奏不是一味单打独斗，也不是把边界交给别人决定，而是把自己的职责、资源和合作方式讲清楚。",
            peer.summary,
            useful_god_text(useful_gods)
        )
    };
    TopicReportBlock {
        id: "career-collaboration",
        title: "比劫协作与格局用神",
        body,
    }
}

fn career_trigger_block(
    year: i32,
    annual: &Pillar,
    current_luck: Option<&LuckCycle>,
    trigger: &StarSummary,
    relations: &[RelationHit],
    age_context: TopicAgeContext,
) -> TopicReportBlock {
    let relation_text = relation_hits_text(
        relations,
        "事业参考宫位暂未被年度地支或当前大运形成明显合冲刑害。",
    );
    let guidance = if age_context.early_stage {
        format!(
            "对早年阶段来说，{}年更适合看学习任务、规则感、表达训练和支持系统是否配合；不把它读成岗位、升迁或职业成败。",
            year
        )
    } else {
        format!(
            "对您来说，{}年更适合把任务标准、表达方式、资源承接和协作边界放到明处。压力可以成为行动力，但前提是节奏清楚、责任清楚、边界也清楚。",
            year
        )
    };
    TopicReportBlock {
        id: "career-trigger",
        title: "大运与流年引动",
        body: format!(
            "大运和流年放到事业专项里，重点看阶段任务、技能表达、资源落地和协作边界在什么时候更容易被推到眼前。\n\n{}\n\n{}年的年柱为「{}」。{}\n\n宫位牵动：{}\n\n{}",
            current_luck_stage_text(year, current_luck),
            year,
            annual.ganzhi(),
            trigger.summary,
            relation_text,
            guidance
        ),
    }
}

fn career_plain_block(
    snapshot: &AnalysisSnapshot,
    age_context: TopicAgeContext,
) -> TopicReportBlock {
    let closing = if age_context.early_stage {
        "所以，早年阶段真正适合被照看的，不是事业结果，而是规则感、学习承接、表达能力和支持系统能不能慢慢稳定。"
    } else {
        "所以，真正适合您的事业节奏，不是只靠硬撑，也不是只等机会。先把责任边界分清，把技能磨到能交付，再让资源和协作跟上，事业主题才更容易从压力变成可持续行动。"
    };
    TopicReportBlock {
        id: "career-plain-summary",
        title: "结论",
        body: format!(
            "综合来看，您的事业关键词是：有压力、要承接、靠技能、重边界。官杀带来责任和要求，印星负责承接和学习，食伤让技能与表达变得更明显，财星看资源落地，比劫则提醒协作和竞争不能混在一起。\n\n{}\n\n{}",
            sensitivity_text(snapshot, TopicReportTopic::Career),
            closing
        ),
    }
}

fn relationship_warnings(chart: &ChartResult) -> Vec<&'static str> {
    let mut warnings = Vec::new();
    if chart.chart.hour.is_none() {
        warnings.push("时辰未知：时柱相关的长期互动和后段安排只做降级阅读");
    }
    if matches!(chart.basis.request.birth_profile.sex, Sex::Unspecified) {
        warnings.push("性别未指定：配偶星采用中性取象");
    }
    warnings
}

fn wealth_warnings(chart: &ChartResult) -> Vec<&'static str> {
    let mut warnings = Vec::new();
    if chart.chart.hour.is_none() {
        warnings.push("时辰未知：时柱相关的长期资源安排只做降级阅读");
    }
    warnings
}

fn family_warnings(chart: &ChartResult) -> Vec<&'static str> {
    let mut warnings = Vec::new();
    if chart.chart.hour.is_none() {
        warnings.push("时辰未知：时柱晚辈、长期安排和后段主题已降级");
    }
    warnings
}

fn career_warnings(chart: &ChartResult) -> Vec<&'static str> {
    let mut warnings = Vec::new();
    if chart.chart.hour.is_none() {
        warnings.push("时辰未知：时柱长期安排和后段发力主题已降级");
    }
    warnings
}

fn ten_god_group_summary(
    counts: &BTreeMap<&'static str, u16>,
    keys: &[&'static str],
    label: &'static str,
    meaning: &'static str,
) -> StarSummary {
    let total = keys
        .iter()
        .map(|key| counts.get(key).copied().unwrap_or(0))
        .sum::<u16>();
    let evidence = keys
        .iter()
        .map(|key| {
            let count = counts.get(key).copied().unwrap_or(0);
            if count == 0 {
                format!("{}不是最先被看的线索。", ten_god_label(key))
            } else {
                format!("{}。", ten_god_signal_sentence(key, count))
            }
        })
        .collect::<Vec<_>>();
    let summary = if total == 0 {
        format!("{label}不是最先被看的线索，读法要更多回到其他结构合看。{meaning}")
    } else {
        let active = keys
            .iter()
            .filter(|key| counts.get(*key).copied().unwrap_or(0) > 0)
            .map(|key| ten_god_signal_sentence(key, counts.get(*key).copied().unwrap_or(0)))
            .collect::<Vec<_>>();
        format!("{}。{meaning}", active.join("；"))
    };
    StarSummary {
        level: qualitative_level(total),
        summary,
        evidence,
    }
}

fn ten_god_signal_sentence(key: &str, count: u16) -> String {
    let label = ten_god_label(key);
    let quality = ten_god_quality_phrase(key);
    match count {
        0 => format!("{label}不是最先被看的线索"),
        1 => format!("{label}带出{quality}"),
        2 => format!("{label}的分量更重，{quality}会更容易被看见"),
        _ => format!("{label}反复出现，{quality}会成为需要持续观察的方向"),
    }
}

fn ten_god_quality_phrase(key: &str) -> &'static str {
    match key {
        "direct_wealth" => "稳定事务、可管理资源和持续承接",
        "indirect_wealth" => "机会资源、外部流动和交换意识",
        "eating_god" => "温和表达、技能沉淀和持续产出",
        "hurting_officer" => "想法出口、技术表达和解决问题的锋芒",
        "peer" => "自我立场、同伴协作和边界意识",
        "rob_wealth" => "资源分配、竞争感和协作边界",
        "direct_resource" => "稳定支持、接纳保护和规则内的承接",
        "indirect_resource" => "独立理解、内在消化和不标准的支持方式",
        "direct_officer" => "规则、责任、秩序和可靠承诺",
        "seven_killings" => "压力、挑战、边界考验和行动驱动",
        _ => "需要合并其他信号谨慎判断的提醒",
    }
}

fn cycle_trigger_summary(
    chart: &ChartResult,
    annual: &Pillar,
    current_luck: Option<&LuckCycle>,
    keys: &[&'static str],
    label: &'static str,
) -> StarSummary {
    let evidence = cycle_ten_god_hits(chart, annual, current_luck, keys);
    let summary = if evidence.is_empty() {
        format!("{label}相关十神在年度和当前大运里没有明显被点亮，本段先把它作为阶段背景看。")
    } else {
        format!(
            "{}相关十神被年度和当前大运带出来：{}。",
            label,
            evidence.join("；")
        )
    };
    StarSummary {
        level: qualitative_level(evidence.len() as u16),
        summary,
        evidence,
    }
}

fn cycle_ten_god_hits(
    chart: &ChartResult,
    annual: &Pillar,
    current_luck: Option<&LuckCycle>,
    keys: &[&'static str],
) -> Vec<String> {
    let mut stems = vec![("年度天干", annual.stem.clone())];
    for stem in hidden_stems(&annual.branch) {
        stems.push(("年度藏干", stem.to_string()));
    }
    if let Some(current_luck) = current_luck {
        stems.push(("当前大运天干", current_luck.pillar.stem.clone()));
        for stem in hidden_stems(&current_luck.pillar.branch) {
            stems.push(("当前大运藏干", stem.to_string()));
        }
    }

    stems
        .into_iter()
        .filter_map(|(source, stem)| {
            let god = ten_god_id(&chart.chart.day.stem, &stem);
            if keys.contains(&god) {
                Some(format!("{source}「{stem}」形成{}", ten_god_label(god)))
            } else {
                None
            }
        })
        .collect()
}

fn family_anchors(chart: &ChartResult) -> Vec<(&'static str, String)> {
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

fn career_anchors(chart: &ChartResult) -> Vec<(&'static str, String)> {
    let mut anchors = vec![("月支", chart.chart.month.branch.clone())];
    if let Some(hour) = &chart.chart.hour {
        anchors.push(("时支", hour.branch.clone()));
    }
    anchors
}

fn branch_relations_for_anchors(
    anchors: Vec<(&'static str, String)>,
    annual: &Pillar,
    current_luck: Option<&LuckCycle>,
) -> Vec<RelationHit> {
    let mut targets = vec![("年度地支", annual.branch.clone())];
    if let Some(current_luck) = current_luck {
        targets.push(("当前大运", current_luck.pillar.branch.clone()));
    }

    let mut hits = Vec::new();
    for (anchor_name, anchor_branch) in anchors {
        for (target_name, target_branch) in &targets {
            if let Some(relation) = branch_relation(&anchor_branch, target_branch) {
                hits.push(RelationHit {
                    relation,
                    target: *target_name,
                    target_branch: target_branch.clone(),
                    evidence: format!("{anchor_name}「{anchor_branch}」与{target_name}「{target_branch}」形成\"{relation}\""),
                });
            }
        }
    }
    hits
}

fn family_trigger_summary(year: i32, annual: &Pillar, relations: &[RelationHit]) -> StarSummary {
    let evidence = relations
        .iter()
        .map(|hit| hit.evidence.clone())
        .collect::<Vec<_>>();
    let summary = if evidence.is_empty() {
        format!(
            "{year}年柱「{}」暂未与家庭参考宫位形成明显合冲刑害，只作为年度背景记录。",
            annual.ganzhi()
        )
    } else {
        format!(
            "{year}年柱「{}」与家庭参考宫位存在牵动：{}。",
            annual.ganzhi(),
            evidence.join("；")
        )
    };
    StarSummary {
        level: qualitative_level(evidence.len() as u16),
        summary,
        evidence,
    }
}

fn family_palace_summary(chart: &ChartResult) -> String {
    let hour = chart
        .chart
        .hour
        .as_ref()
        .map(|pillar| format!("时柱「{}」可作长期安排参考", pillar.ganzhi()))
        .unwrap_or_else(|| "时辰未知，时柱相关主题降级".to_string());
    format!(
        "年柱「{}」看早年与家族背景，月柱「{}」看成长环境与长辈关系，日支「{}」看亲密生活落点，{}。",
        chart.chart.year.ganzhi(),
        chart.chart.month.ganzhi(),
        chart.chart.day.branch,
        hour
    )
}

fn family_anchor_evidence(chart: &ChartResult) -> Vec<String> {
    let mut evidence = vec![
        format!("年柱为「{}」", chart.chart.year.ganzhi()),
        format!("月柱为「{}」", chart.chart.month.ganzhi()),
        format!("日支为「{}」", chart.chart.day.branch),
    ];
    if let Some(hour) = &chart.chart.hour {
        evidence.push(format!("时柱为「{}」", hour.ganzhi()));
    } else {
        evidence.push("时柱信息未知".to_string());
    }
    evidence
}

fn relation_hits_text(relations: &[RelationHit], fallback: &str) -> String {
    if relations.is_empty() {
        fallback.to_string()
    } else {
        relations
            .iter()
            .map(|hit| readable_evidence_text(&hit.evidence))
            .collect::<Vec<_>>()
            .join("；")
    }
}

fn current_luck_stage_text(year: i32, current_luck: Option<&LuckCycle>) -> String {
    current_luck
        .map(|cycle| {
            format!(
                "{}年落在当前大运「{}」（{}至{}岁），这段大运是理解阶段主题的背景。",
                year,
                cycle.pillar.ganzhi(),
                cycle.start_age,
                cycle.end_age
            )
        })
        .unwrap_or_else(|| format!("大运序列当前不可用，因此本段只回看原局与{year}年。"))
}

fn useful_god_text(useful_gods: &[UsefulGodHint]) -> String {
    if useful_gods.is_empty() {
        "当前无明显提示".to_string()
    } else {
        useful_gods
            .iter()
            .map(|hint| format!("{}：{}", useful_god_label(hint.element), hint.reason))
            .collect::<Vec<_>>()
            .join("；")
    }
}

fn useful_god_label(element: &str) -> String {
    match element {
        "印星(生我)" | "印星(化杀)" => "印星（学习、保护、支持）".to_string(),
        "比劫(同我)" => "比劫（自我力量、同伴、行动力）".to_string(),
        "食伤(泄秀)" => "食伤（表达、输出、创造力）".to_string(),
        "财星(耗身)" => "财星（资源、现实、经营意识）".to_string(),
        "通关用神" => "通关五行（调节、平衡、流动）".to_string(),
        other => other.replace('(', "（").replace(')', "）"),
    }
}

fn replace_age_range_hyphen(value: &str) -> String {
    let mut text = value.to_string();
    for start in 0..=120 {
        for end in start..=120 {
            let needle = format!("{start}-{end}岁");
            if text.contains(&needle) {
                text = text.replace(&needle, &format!("{start}至{end}岁"));
            }
        }
    }
    text
}

fn replace_luck_ordinals(value: &str) -> String {
    let mut text = value.to_string();
    for index in 1..=10 {
        text = text.replace(
            &format!("第{index}运"),
            &format!("{}运", ordinal_text(index)),
        );
    }
    text
}

fn quote_relation_names(value: &str) -> String {
    value
        .replace("形成六冲", "形成\"六冲\"")
        .replace("形成六合", "形成\"六合\"")
        .replace("形成三刑", "形成\"三刑\"")
        .replace("形成六害", "形成\"六害\"")
        .replace("形成自刑", "形成\"自刑\"")
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

fn sensitivity_text(snapshot: &AnalysisSnapshot, topic: TopicReportTopic) -> String {
    if snapshot.sensitivity_flags.is_empty() {
        match topic {
            TopicReportTopic::Relationship => {
                "以目前资料来看，情感专项可以把夫妻宫、伴侣议题、表达方式和年度引动合在一起看。".to_string()
            }
            TopicReportTopic::Wealth => {
                "金钱线索的合参重点，落在资源入口、产出方式、合作分配和规则承接能否形成同一个节奏。".to_string()
            }
            TopicReportTopic::Family => {
                "家庭线索的合参重点，落在支持从哪里来、话怎样说清、边界怎样放稳、责任怎样分明。".to_string()
            }
            TopicReportTopic::Career => {
                "事业线索的合参重点，落在压力如何承接、技能如何交付、资源如何落地、协作边界如何说清。".to_string()
            }
        }
    } else {
        let notes = snapshot
            .sensitivity_flags
            .iter()
            .map(|flag| {
                if flag.contains("unknown_hour") {
                    "时辰信息不完整，时柱相关的长期节奏会读得更保守。"
                } else if flag.contains("ambiguity") {
                    "命盘基础信息存在可读分歧，本报告会优先采用稳健解释。"
                } else {
                    "资料存在需要保守处理的部分，本报告会把结论放在较稳的结构线索上。"
                }
            })
            .collect::<Vec<_>>();
        notes.join("")
    }
}

fn merge_evidence(groups: &[&StarSummary]) -> Vec<String> {
    groups
        .iter()
        .flat_map(|group| group.evidence.iter().cloned())
        .collect()
}

fn merge_strings(mut left: Vec<String>, right: Vec<String>) -> Vec<String> {
    left.extend(right);
    left
}

fn relationship_spouse_star_summary(
    chart: &ChartResult,
    counts: &BTreeMap<&'static str, u16>,
) -> StarSummary {
    let direct_wealth = counts.get("direct_wealth").copied().unwrap_or(0);
    let indirect_wealth = counts.get("indirect_wealth").copied().unwrap_or(0);
    let direct_officer = counts.get("direct_officer").copied().unwrap_or(0);
    let seven_killings = counts.get("seven_killings").copied().unwrap_or(0);
    let level_count = match chart.basis.request.birth_profile.sex {
        Sex::Male => direct_wealth + indirect_wealth,
        Sex::Female => direct_officer + seven_killings,
        Sex::Unspecified => direct_wealth + indirect_wealth + direct_officer + seven_killings,
    };
    let sex_context = match chart.basis.request.birth_profile.sex {
        Sex::Male => {
            "财星会被优先放到伴侣议题里看，同时官杀带来的秩序感、压力感和承诺议题也需要合参"
        }
        Sex::Female => {
            "官杀会被优先放到伴侣议题里看，同时财星带来的吸引力、现实承接和相处资源也需要合参"
        }
        Sex::Unspecified => {
            "财星与官杀会一起参与伴侣议题的观察，用来区分吸引、承诺、压力和现实承接的不同层次"
        }
    };
    let mut evidence = Vec::new();
    if direct_wealth > 0 {
        evidence.push("正财让关系更重视稳定、照顾和现实承接".to_string());
    }
    if indirect_wealth > 0 {
        evidence.push("偏财让关系里带着吸引、机会感和外部牵动".to_string());
    }
    if direct_officer > 0 {
        evidence.push("正官把承诺、秩序和可靠感带进伴侣议题".to_string());
    }
    if seven_killings > 0 {
        evidence.push("七杀会让压力、边界和安全感考验一起变得更明显".to_string());
    }
    if evidence.is_empty() {
        evidence.push(
            "财星和官杀都不算显眼，伴侣议题更需要回到夫妻宫、表达方式和支持系统合看".to_string(),
        );
    }
    let summary = format!(
        "{}。{}。这表示伴侣议题已经进入命盘视野，但只能解释为关系主题被看见，不能直接推成现实结果。",
        sex_context,
        evidence.join("；")
    );
    StarSummary {
        level: qualitative_level(level_count),
        summary,
        evidence: evidence.iter().map(|item| format!("{item}。")).collect(),
    }
}

fn relationship_expression_summary(counts: &BTreeMap<&'static str, u16>) -> StarSummary {
    let eating_god = counts.get("eating_god").copied().unwrap_or(0);
    let hurting_officer = counts.get("hurting_officer").copied().unwrap_or(0);
    let peer = counts.get("peer").copied().unwrap_or(0);
    let rob_wealth = counts.get("rob_wealth").copied().unwrap_or(0);
    let output = eating_god + hurting_officer;
    let boundary = peer + rob_wealth;
    let mut evidence = Vec::new();
    if eating_god > 0 && hurting_officer > 0 {
        evidence.push("食神和伤官同时参与，说明感受既需要被照顾，也需要有出口".to_string());
    } else if eating_god > 0 {
        evidence.push("食神让表达更重视照顾、缓和和彼此舒服".to_string());
    } else if hurting_officer > 0 {
        evidence.push("伤官让感受和想法需要出口，压久了容易变成尖锐表达".to_string());
    }
    if peer > 0 && rob_wealth > 0 {
        evidence
            .push("比肩和劫财同时参与，关系越近，自我立场、边界和同辈影响越容易浮上来".to_string());
    } else if peer > 0 || rob_wealth > 0 {
        evidence.push("比劫让自我立场更清楚，也让关系中的边界感更需要被尊重".to_string());
    }
    if evidence.is_empty() {
        evidence.push("表达与边界信号不算锋利，关系更适合在稳定相处里慢慢把话说清楚".to_string());
    }
    let summary = format!(
        "表达侧，{}。关系越靠近，越需要把话说早、说稳，而不是等情绪累积后再一次性爆发。",
        evidence.join("；")
    );
    StarSummary {
        level: qualitative_level(output + boundary),
        summary,
        evidence: evidence.iter().map(|item| format!("{item}。")).collect(),
    }
}

fn relationship_support_summary(
    counts: &BTreeMap<&'static str, u16>,
    useful_gods: &[UsefulGodHint],
) -> StarSummary {
    let direct_resource = counts.get("direct_resource").copied().unwrap_or(0);
    let indirect_resource = counts.get("indirect_resource").copied().unwrap_or(0);
    let resource = direct_resource + indirect_resource;
    let useful = useful_gods
        .iter()
        .map(|hint| {
            let element = hint.element.replace('(', "（").replace(')', "）");
            format!("{}，{}", element, hint.reason)
        })
        .collect::<Vec<_>>()
        .join("；");
    let mut evidence = Vec::new();
    if direct_resource > 0 {
        evidence.push("正印让关系更需要被接纳、被保护，也更看重稳定支持".to_string());
    }
    if indirect_resource > 0 {
        evidence.push("偏印让安全感带着独立消化的色彩，很多感受需要先在心里整理清楚".to_string());
    }
    if evidence.is_empty() {
        evidence.push("印星不算显眼，安全感更需要从清楚边界、稳定回应和现实承接里建立".to_string());
    }
    let useful_text = if useful.is_empty() {
        "当前用神没有给出额外补充方向。".to_string()
    } else {
        format!("用神给出的补充方向是：{}。", useful)
    };
    let summary = format!(
        "支持侧，{}。印星在这里看的是被理解、被接纳和被支持的需要，{}",
        evidence.join("；"),
        useful_text
    );
    StarSummary {
        level: qualitative_level(resource),
        summary,
        evidence: evidence.iter().map(|item| format!("{item}。")).collect(),
    }
}

fn relationship_palace_relations(
    chart: &ChartResult,
    annual: &Pillar,
    current_luck: Option<&LuckCycle>,
) -> Vec<RelationHit> {
    let day_branch = &chart.chart.day.branch;
    let mut targets = vec![
        ("月支", chart.chart.month.branch.clone()),
        ("年度地支", annual.branch.clone()),
    ];
    if let Some(hour) = &chart.chart.hour {
        targets.push(("时支", hour.branch.clone()));
    }
    if let Some(current_luck) = current_luck {
        targets.push(("当前大运", current_luck.pillar.branch.clone()));
    }

    targets
        .into_iter()
        .filter_map(|(target, branch)| {
            branch_relation(day_branch, &branch).map(|relation| RelationHit {
                relation,
                target,
                target_branch: branch.clone(),
                evidence: format!("夫妻宫「{day_branch}」与{target}「{branch}」形成\"{relation}\""),
            })
        })
        .collect()
}

fn palace_relation_plain(relations: &[RelationHit]) -> &'static str {
    if relations.is_empty() {
        "夫妻宫目前没有明显合冲刑害，关系主题以原局结构和十神分布为主。"
    } else if relations.iter().any(|hit| hit.relation == "六冲") {
        "夫妻宫被\"六冲\"触发，适合重点观察互动节奏、立场差异和沟通方式。"
    } else if relations.iter().any(|hit| hit.relation == "六合") {
        "夫妻宫被\"六合\"触发，关系主题更容易呈现连接、靠近或议题汇合。"
    } else {
        "夫妻宫被\"刑害\"类关系触发，适合观察细节磨合、边界感和表达成本。"
    }
}

fn relation_level(relations: &[RelationHit]) -> &'static str {
    if relations.is_empty() {
        "平稳"
    } else if relations.iter().any(|hit| hit.relation == "六冲") {
        "明显"
    } else {
        "存在"
    }
}

fn annual_trigger_level(relations: &[RelationHit]) -> &'static str {
    if relations.iter().any(|hit| hit.target == "年度地支") {
        "已引动"
    } else {
        "未明显引动"
    }
}

fn annual_trigger_plain(relations: &[RelationHit]) -> &'static str {
    if relations.iter().any(|hit| hit.target == "年度地支") {
        "与夫妻宫存在合冲刑害关系，说明当年的情感议题更容易被带到眼前。"
    } else {
        "未与夫妻宫形成明显合冲刑害，只作为年度背景记录。"
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

fn ten_god_counts(chart: &ChartResult) -> BTreeMap<&'static str, u16> {
    let mut counts = BTreeMap::new();
    for stem in visible_and_primary_hidden_stems(chart) {
        let god = ten_god_id(&chart.chart.day.stem, &stem);
        *counts.entry(god).or_insert(0) += 1;
    }
    counts
}

fn visible_and_primary_hidden_stems(chart: &ChartResult) -> Vec<String> {
    let mut stems = vec![
        chart.chart.year.stem.clone(),
        chart.chart.month.stem.clone(),
        chart.chart.day.stem.clone(),
    ];
    if let Some(hour) = &chart.chart.hour {
        stems.push(hour.stem.clone());
    }
    for branch in [
        &chart.chart.year.branch,
        &chart.chart.month.branch,
        &chart.chart.day.branch,
    ] {
        if let Some(primary) = hidden_stems(branch).first() {
            stems.push((*primary).to_string());
        }
    }
    if let Some(hour) = &chart.chart.hour {
        if let Some(primary) = hidden_stems(&hour.branch).first() {
            stems.push((*primary).to_string());
        }
    }
    stems
}

fn hidden_stems(branch: &str) -> Vec<&'static str> {
    match branch {
        "子" => vec!["癸"],
        "丑" => vec!["己", "癸", "辛"],
        "寅" => vec!["甲", "丙", "戊"],
        "卯" => vec!["乙"],
        "辰" => vec!["戊", "乙", "癸"],
        "巳" => vec!["丙", "戊", "庚"],
        "午" => vec!["丁", "己"],
        "未" => vec!["己", "丁", "乙"],
        "申" => vec!["庚", "壬", "戊"],
        "酉" => vec!["辛"],
        "戌" => vec!["戊", "辛", "丁"],
        "亥" => vec!["壬", "甲"],
        _ => Vec::new(),
    }
}

fn ten_god_id(day_stem: &str, other_stem: &str) -> &'static str {
    let day_element = stem_element(day_stem);
    let other_element = stem_element(other_stem);
    let same_polarity = is_yang(day_stem) == is_yang(other_stem);

    if day_element == other_element {
        if same_polarity { "peer" } else { "rob_wealth" }
    } else if generates(day_element, other_element) {
        if same_polarity {
            "eating_god"
        } else {
            "hurting_officer"
        }
    } else if controls(day_element, other_element) {
        if same_polarity {
            "indirect_wealth"
        } else {
            "direct_wealth"
        }
    } else if controls(other_element, day_element) {
        if same_polarity {
            "seven_killings"
        } else {
            "direct_officer"
        }
    } else if generates(other_element, day_element) {
        if same_polarity {
            "indirect_resource"
        } else {
            "direct_resource"
        }
    } else {
        "peer"
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

fn qualitative_level(value: u16) -> &'static str {
    match value {
        0 => "不突出",
        1 => "存在",
        2..=3 => "较明显",
        _ => "明显",
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::calendar::civil::CivilDate;
    use crate::domain::bazi::{
        BirthProfile, BirthTime, CalendarKind, ChartBasis, ChartRequest, DateLayerPillars,
        PrivacyLevel, TimePrecision,
    };
    use crate::domain::deep_analysis::{assess_strength, classify_pattern, suggest_useful_god};
    use crate::domain::luck::compute_luck_cycles;

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
        ChartResult::build(
            ChartBasis::build(request).unwrap(),
            DateLayerPillars {
                year: "甲辰".to_string(),
                month: "丙子".to_string(),
                day: "庚午".to_string(),
            },
        )
        .unwrap()
    }

    #[test]
    fn annual_pillar_uses_standard_sexagenary_year_cycle() {
        assert_eq!(annual_pillar(2024).ganzhi(), "甲辰");
        assert_eq!(annual_pillar(2025).ganzhi(), "乙巳");
        assert_eq!(annual_pillar(2026).ganzhi(), "丙午");
    }

    #[test]
    fn relationship_report_has_trace_without_public_score() {
        let chart = chart();
        let snapshot = AnalysisSnapshot::build(&chart);
        let luck = compute_luck_cycles(
            &chart.chart.year.stem,
            &chart.chart.month,
            &chart.basis.request.birth_profile.sex,
            0,
        );
        let strength = assess_strength(&chart);
        let pattern = classify_pattern(&chart, &strength);
        let useful = suggest_useful_god(&strength, &pattern);
        let report =
            build_relationship_report(&chart, &snapshot, &luck, &strength, &pattern, &useful, 2026);

        assert_eq!(report.status, "restricted");
        assert_eq!(report.capability, "relationship-report");
        assert_eq!(report.year_source, "explicit");
        assert_eq!(report.blocks.len(), 6);
        assert_eq!(
            report
                .blocks
                .iter()
                .map(|block| block.title)
                .collect::<Vec<_>>(),
            vec![
                "总断",
                "伴侣议题",
                "夫妻宫",
                "表达、边界与安全感",
                "年度情感引动",
                "结论",
            ]
        );
        assert!(
            report
                .blocks
                .iter()
                .any(|block| block.body.contains("夫妻宫"))
        );
        assert!(
            report
                .blocks
                .iter()
                .any(|block| block.body.contains("配偶星"))
        );
        assert!(
            report
                .blocks
                .iter()
                .any(|block| block.body.contains("合冲刑害"))
        );
        assert!(report.trace.iter().any(
            |trace| trace.id == "topic-timeline-overlay" && trace.source == "timeline-core-v1"
        ));
        assert!(
            !report
                .blocks
                .iter()
                .any(|block| block.id == "topic-timeline-overlay")
        );
        assert_eq!(report.forbidden_output_audit.status, "passed");
        let assembled = assemble_report(&report.disclaimer, &report.blocks);
        assert!(assembled.contains("如果被\"冲\"牵动"));
        for golden_term in [
            "关系靠近",
            "稳定回应",
            "名义年龄约2岁",
            "不会把情感专项读成现实恋爱状态",
            "不讨论现实恋爱状态",
            "早年阶段真正值得照看的",
            "表达侧",
            "支持侧",
        ] {
            assert!(
                assembled.contains(golden_term),
                "relationship golden sample lost user-facing term {golden_term}"
            );
        }
        assert!(!assembled.contains("score_internal"));
        assert!(!assembled.contains("必然结婚"));
        assert!(!assembled.contains("出轨"));
        for forbidden in [
            "不作主线",
            "有一处落点",
            "有两处落点",
            "有三处落点",
            "您的情感线并不是没有缘分",
            "标记为已引动",
            "共享时间线",
            "筛出",
            "当前提取结果",
            "当前关系：",
            "基础阅读",
            "blended",
            "正官=",
            "七杀=",
            "食伤=",
            "比劫=",
            "印星=",
            "表达与安全感则落在日常相处里",
            "以目前资料来看，这份情感专项可以把重点放在",
            "如果目前单身",
            "若已有关系",
            "真正适合您的关系",
        ] {
            assert!(
                !assembled.contains(forbidden),
                "relationship report leaked M42 machine wording {forbidden}"
            );
        }
    }
}

#[cfg(test)]
mod topic_timeline_overlay_tests {
    use super::*;
    use crate::calendar::civil::CivilDate;
    use crate::domain::analysis::AnalysisSnapshot;
    use crate::domain::bazi::{
        BirthProfile, BirthTime, CalendarKind, ChartBasis, ChartRequest, DateLayerPillars,
        PrivacyLevel, Sex, TimePrecision,
    };
    use crate::domain::deep_analysis::{assess_strength, classify_pattern, suggest_useful_god};
    use crate::domain::luck::compute_luck_cycles;

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
        ChartResult::build(
            ChartBasis::build(request).unwrap(),
            DateLayerPillars {
                year: "甲辰".to_string(),
                month: "丙子".to_string(),
                day: "庚午".to_string(),
            },
        )
        .unwrap()
    }

    #[test]
    fn topic_timeline_overlay_reuses_shared_engine_for_all_topics() {
        let chart = chart();
        let snapshot = AnalysisSnapshot::build(&chart);
        let luck = compute_luck_cycles(
            &chart.chart.year.stem,
            &chart.chart.month,
            &chart.basis.request.birth_profile.sex,
            0,
        );
        let strength = assess_strength(&chart);
        let pattern = classify_pattern(&chart, &strength);
        let useful = suggest_useful_god(&strength, &pattern);
        let reports = [
            build_relationship_report(&chart, &snapshot, &luck, &strength, &pattern, &useful, 2026),
            build_wealth_report(&chart, &snapshot, &luck, &strength, &pattern, &useful, 2026),
            build_family_report(&chart, &snapshot, &luck, &strength, &pattern, &useful, 2026),
            build_career_report(&chart, &snapshot, &luck, &strength, &pattern, &useful, 2026),
        ];

        for report in reports {
            assert!(
                report
                    .signals
                    .iter()
                    .any(|signal| signal.id == "topic-timeline-overlay"
                        && signal.summary.contains("专题阅读以")
                        && !signal.summary.contains("timeline-core-v1"))
            );
            assert!(report.trace.iter().any(|trace| {
                trace.id == "topic-timeline-overlay"
                    && trace.source == "timeline-core-v1"
                    && trace
                        .evidence
                        .iter()
                        .any(|item| item.contains("流年信号") || item.contains("大运流年叠加"))
            }));

            if report.topic == TopicReportTopic::Relationship {
                assert!(
                    !report
                        .blocks
                        .iter()
                        .any(|block| block.id == "topic-timeline-overlay")
                );
                let block = report
                    .blocks
                    .iter()
                    .find(|block| block.id == "relationship-trigger")
                    .expect("relationship trigger block should be present");
                for required in ["2026年", "同场", "当前大运", "稳定回应"] {
                    assert!(
                        block.body.contains(required),
                        "{} report missing {required}",
                        report.topic.as_str()
                    );
                }
            } else {
                let block = report
                    .blocks
                    .iter()
                    .find(|block| block.id == "topic-timeline-overlay")
                    .expect("topic timeline overlay block should be present");
                assert_eq!(block.title, "本专题的大运流年");
                let timeline_index = report
                    .blocks
                    .iter()
                    .position(|block| block.id == "topic-timeline-overlay")
                    .expect("topic timeline block index");
                let conclusion_index = report
                    .blocks
                    .iter()
                    .position(|block| block.title == "结论")
                    .expect("conclusion block index");
                assert!(
                    timeline_index < conclusion_index,
                    "{} report should close with conclusion after timeline",
                    report.topic.as_str()
                );
                for required in [
                    "遇到",
                    "2026年放到",
                    "再看地支、藏干和当前大运",
                    "当前大运",
                    "流年、当前大运和原局",
                ] {
                    assert!(
                        block.body.contains(required),
                        "{} report missing {required}",
                        report.topic.as_str()
                    );
                }
                for forbidden in [
                    "年度本身先露出的",
                    "流年天干把十神主题推到台前",
                    "五行关系继续说明力量怎样靠近",
                    "再往下看，藏干、原局位置和大运同场",
                    "从「金钱」专项来看",
                    "从「家庭」专项来看",
                    "从「事业」专项来看",
                    "从「",
                    "把2026年放进",
                    "十神与五行这一层",
                    "五行相处的方式提示",
                    "藏干、原局位置和当前大运合到一起时",
                    "藏干、宫位关系和当前大运合到一起时",
                    "本段把它作为阶段背景参考",
                    "这里看的不是单点事件",
                    "年度线索要回到",
                    "不是罗列符号",
                    "先把预算意识",
                    "先看家里哪些话",
                    "先看任务标准",
                ] {
                    assert!(
                        !block.body.contains(forbidden),
                        "{} report kept stale M57 timeline wording {forbidden}",
                        report.topic.as_str()
                    );
                }
                let topic_required = match report.topic {
                    TopicReportTopic::Wealth => [
                        "资源感怎样形成",
                        "照护是否稳定",
                        "重点不是收入或投资",
                        "分享和边界",
                    ],
                    TopicReportTopic::Family => {
                        ["成长环境", "照护方式", "情绪表达", "规则会更需要安顿"]
                    }
                    TopicReportTopic::Career => {
                        ["学习任务", "规则感", "表达训练", "重点不是岗位成败"]
                    }
                    TopicReportTopic::Relationship => unreachable!(),
                };
                for required in topic_required {
                    assert!(
                        block.body.contains(required),
                        "{} report missing M53 topic-specific timeline wording {required}",
                        report.topic.as_str()
                    );
                }
                for forbidden in [
                    "专业解释",
                    "白话解释",
                    "timeline-core-v1",
                    "shared timeline engine",
                    "后端返回",
                    "前端追加",
                    "帮助用户",
                    "不替用户",
                    "指定年份",
                    "这条信号",
                    "当前提取结果",
                    "观察年度",
                    "共享证据",
                    "结构依据",
                    "落到您的命盘",
                    "规则版本",
                    "annual-trigger",
                    "annual-current-luck",
                    "盘中可用的时间线索",
                    "重点看的牵动",
                    "读这一段时",
                    "在盘中有",
                    "关键牵动是",
                    "不作主线",
                    "有一处落点",
                    "有两处落点",
                    "有三处落点",
                    "落到这张盘上",
                    "参与这组结构",
                    "这组结构说明",
                    "主要牵动如下",
                    "这些牵动只说明",
                    "不能只看流年",
                    "不必急着找事件结论",
                    "哪里需要放慢",
                    "哪里需要承接",
                    "读2026年这一层",
                    "这一章只说明",
                    "时间气候可以按这个顺序读",
                    "2026年的时间气候",
                    "从「金钱」专项来看",
                    "从「家庭」专项来看",
                    "从「事业」专项来看",
                    "从「",
                    "把2026年放进",
                    "十神与五行这一层",
                    "五行相处的方式提示",
                    "藏干、原局位置和当前大运合到一起时",
                    "藏干、宫位关系和当前大运合到一起时",
                    "本段把它作为阶段背景参考",
                    "这里看的不是单点事件",
                    "年度线索要回到",
                    "先从这些层次落下去看",
                    "大运首段",
                    "天干处先露出十神主题",
                    "月支这一处",
                    "日支这一处",
                    "在这份金钱专项里",
                    "在这份家庭专项里",
                    "在这份事业专项里",
                    "在同一张桌上慢慢理清",
                    "先看天干",
                    "再看五行关系",
                    "当前资料可以按完整四柱合参",
                ] {
                    assert!(
                        !block.body.contains(forbidden),
                        "{} report leaked stiff wording {forbidden}",
                        report.topic.as_str()
                    );
                }
            }

            let assembled = assemble_report(&report.disclaimer, &report.blocks);
            assert_eq!(report.forbidden_output_audit.status, "passed");
            if report.topic == TopicReportTopic::Relationship {
                assert!(assembled.contains("如果被\"冲\"牵动"));
            } else {
                let required: &[&str] = match report.topic {
                    TopicReportTopic::Wealth => &[
                        "资源入口",
                        "资源不是只指钱",
                        "分享与边界",
                        "资源主题从「有没有」推进到「能不能安稳使用」",
                        "您的金钱关键词",
                        "早年阶段真正适合被照看的",
                    ],
                    TopicReportTopic::Family => &[
                        "互动位置",
                        "家庭里的支持，不只是有人帮忙",
                        "家庭越亲近，边界越容易被忽略",
                        "家庭最终会落到很多具体事务上",
                        "您的家庭关键词",
                        "早年阶段真正适合被照看的",
                    ],
                    TopicReportTopic::Career => &[
                        "事业用力方式",
                        "成长场景",
                        "规则和任务",
                        "表达训练",
                        "同伴互动",
                        "您的事业关键词",
                        "早年阶段真正适合被照看的",
                    ],
                    TopicReportTopic::Relationship => unreachable!(),
                };
                for term in required {
                    assert!(
                        assembled.contains(term),
                        "{} report missing M50 narrative term {term}",
                        report.topic.as_str()
                    );
                }
                let forbidden: &[&str] = match report.topic {
                    TopicReportTopic::Wealth => &[
                        "财星分正财和偏财",
                        "从十神脉络看",
                        "传统上会把",
                        "产出方面，",
                        "分配方面，",
                        "支持与约束方面",
                    ],
                    TopicReportTopic::Family => &[
                        "印星在家庭专项里主要看",
                        "比劫在家庭专项里看",
                        "财官在家庭专项里不解释",
                        "从十神脉络看",
                        "同辈边界：",
                        "表达方式：",
                    ],
                    TopicReportTopic::Career => &[
                        "官杀代表责任",
                        "食伤代表表达",
                        "比劫代表协作",
                        "技能表达：",
                        "资源落地：",
                        "协作竞争：",
                        "责任方面，",
                        "承接方面，",
                    ],
                    TopicReportTopic::Relationship => unreachable!(),
                };
                for term in forbidden {
                    assert!(
                        !assembled.contains(term),
                        "{} report leaked M59 textbook wording {term}",
                        report.topic.as_str()
                    );
                }
            }
            for forbidden in [
                "score_internal",
                "0-100",
                "必然发财",
                "流月运势",
                "每日运势",
                "当前提取结果",
                "观察年度",
                "共享证据",
                "规则版本",
                "annual-trigger",
                "annual-current-luck",
                "正财=",
                "偏财=",
                "正官=",
                "七杀=",
                "食神=",
                "伤官=",
                "比肩=",
                "劫财=",
                "正印=",
                "偏印=",
                "您的情感线并不是没有缘分",
                "被合牵动",
                "被冲牵动",
                "被刑害牵动",
                "形成六冲",
                "形成六合",
                "形成三刑",
                "形成六害",
                "形成自刑",
                "本次输入",
                "本报告只做",
                "结构敏感性：",
                "目前可追溯证据如下",
                "盘中可用的时间线索",
                "重点看的牵动",
                "读这一段时",
                "在盘中有",
                "关键牵动是",
                "不作主线",
                "有一处落点",
                "有两处落点",
                "有三处落点",
                "落到这张盘上",
                "参与这组结构",
                "这组结构说明",
                "主要牵动如下",
                "这些牵动只说明",
                "不能只看流年",
                "不必急着找事件结论",
                "这一章只说明",
                "时间气候可以按这个顺序读",
                "当前资料可以按完整四柱合参",
                "这些牵动提醒您",
                "日常读法",
                "日常看",
                "放回这张命盘看",
                "放回家庭结构里",
                "这份报告适合当作",
                "这一章看的是",
                "在这份金钱专项里",
                "在这份家庭专项里",
                "在这份事业专项里",
                "表达与安全感则落在日常相处里",
                "以目前资料来看，这份情感专项可以把重点放在",
                "在同一张桌上慢慢理清",
                " = ",
            ] {
                assert!(
                    !assembled.contains(forbidden),
                    "{} report leaked forbidden term {forbidden}",
                    report.topic.as_str()
                );
            }
        }
    }
}
