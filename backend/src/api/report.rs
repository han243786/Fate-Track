// M27: Zero-baseline colloquial Chinese chart report.
// All text is hard-coded templates. No AI/LLM. No deterministic life claims.
// Design: every chapter follows "what → your result → plain language → how to read it"
use crate::api::chart_basis::chart_request_from_query;
use crate::api::charts::build_chart_result;
use crate::calendar::lunar_data::LunarDataSource;
use crate::config::AppConfig;
use crate::domain::analysis::{AnalysisSnapshot, DISCLAIMER_ID};
use crate::domain::bazi::{ChartBasis, ChartResult, Pillar};
use crate::domain::deep_analysis::{assess_strength, classify_pattern, suggest_useful_god};
use crate::domain::luck::{LuckCycle, LuckCycleContext, compute_luck_cycle_context};
use crate::domain::timeline::{
    PlainReading, TimelineEvidence, TimelineReadingDraft, TimelineSignal, audit_timeline_text,
    build_annual_trigger_foundation, build_major_luck_stage_foundation,
};
use crate::domain::topic_report::{annual_pillar, validate_topic_year};
use crate::error::AppError;
use crate::http::{Request, Response, json};

pub fn generate(config: &AppConfig, request: &Request) -> Result<Response, AppError> {
    let chart = build_chart_result(config, request)?;
    let snapshot = AnalysisSnapshot::build(&chart);
    let reading_year = parse_optional_reading_year(request)?;
    let annual_year = parse_optional_annual_year(request)?;

    let chart_request = chart_request_from_query(request)?;
    let basis = ChartBasis::build(chart_request)?;
    let profile = &basis.request.birth_profile;
    let table = LunarDataSource::new(config.lunar_data_path.clone()).load_table()?;
    let date_layer = table
        .lookup(profile.date)
        .ok_or_else(|| AppError::OutOfRange("date out of range".to_string()))?;

    let year_gan = date_layer
        .gan_zhi_year
        .chars()
        .next()
        .unwrap_or('甲')
        .to_string();
    let month_pillar = Pillar::from_ganzhi(&date_layer.gan_zhi_month)?;
    let luck_context =
        compute_luck_cycle_context(&year_gan, &month_pillar, &profile.sex, profile.date);
    let luck_reading = build_luck_reading_report(
        &chart,
        &luck_context.cycles,
        profile.date.year,
        reading_year,
    )?;
    let annual_trigger_reading = build_annual_trigger_reading_report(
        &chart,
        &luck_context.cycles,
        profile.date.year,
        annual_year,
    )?;

    let strength = assess_strength(&chart);
    let pattern = classify_pattern(&chart, &strength);
    let useful_gods = suggest_useful_god(&strength, &pattern);

    let blocks = build_blocks(MainReportContext {
        chart: &chart,
        snapshot: &snapshot,
        luck_context: &luck_context,
        luck_reading: &luck_reading,
        annual_trigger_reading: &annual_trigger_reading,
        strength: &strength,
        pattern: &pattern,
        useful_gods: &useful_gods,
    });

    let assembled = blocks
        .iter()
        .map(|b| format!("【{}】\n{}", b.title, b.body))
        .collect::<Vec<_>>()
        .join("\n\n");

    let disclaimer = build_disclaimer();
    let full_report = format!("{}\n\n---\n\n{}", disclaimer, assembled);
    let audit = audit_timeline_text(&full_report);

    let blocks_json = blocks
        .iter()
        .map(|b| {
            format!(
                r#"{{"id":{},"title":{},"body":{}}}"#,
                json::string(&b.id),
                json::string(&b.title),
                json::string(&b.body)
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    let body = format!(
        concat!(
            r#"{{"status":"restricted","capability":"chart-report","#,
            r#""algo_version":{},"disclaimer_id":{},"disclaimer":{},"#,
            r#""blocks":[{}],"luck_reading":{},"annual_trigger_reading":{},"assembled_report":{},"#,
            r#""forbidden_output_audit":{{"status":{},"checked_patterns":{}}}"#,
            r#"}}"#
        ),
        json::string(snapshot.algo_version),
        json::string(DISCLAIMER_ID),
        json::string(&disclaimer),
        blocks_json,
        luck_reading_to_json(&luck_reading),
        annual_trigger_reading_to_json(&annual_trigger_reading),
        json::string(&full_report),
        json::string(audit.status),
        audit.checked_patterns,
    );

    Ok(Response::json(body))
}

struct ReportBlock {
    id: String,
    title: String,
    body: String,
}

struct LuckReadingReport {
    reference_year: Option<i32>,
    reference_age: Option<u8>,
    age_policy: &'static str,
    current_index: Option<usize>,
    previous_cycle: Option<LuckCycle>,
    current_cycle: Option<LuckCycle>,
    next_cycle: Option<LuckCycle>,
    draft: Option<TimelineReadingDraft>,
    warnings: Vec<&'static str>,
}

struct AnnualTriggerReadingReport {
    year: Option<i32>,
    year_source: Option<&'static str>,
    annual_pillar: Option<Pillar>,
    current_luck_cycle: Option<LuckCycle>,
    draft: Option<TimelineReadingDraft>,
    warnings: Vec<&'static str>,
}

fn parse_optional_reading_year(request: &Request) -> Result<Option<i32>, AppError> {
    match request.query_value("reading_year") {
        None => Ok(None),
        Some(value) if value.trim().is_empty() => Ok(None),
        Some(value)
            if value.len() == 4 && value.chars().all(|character| character.is_ascii_digit()) =>
        {
            value.parse::<i32>().map(Some).map_err(|_| {
                AppError::BadRequest("reading_year must be a four digit integer".to_string())
            })
        }
        Some(_) => Err(AppError::BadRequest(
            "reading_year must be a four digit integer".to_string(),
        )),
    }
}

fn parse_optional_annual_year(request: &Request) -> Result<Option<i32>, AppError> {
    match request.query_value("year") {
        None => Ok(None),
        Some(value) if value.trim().is_empty() => Ok(None),
        Some(value)
            if value.len() == 4 && value.chars().all(|character| character.is_ascii_digit()) =>
        {
            let year = value.parse::<i32>().map_err(|_| {
                AppError::BadRequest("year must be a four digit integer".to_string())
            })?;
            validate_topic_year(year)?;
            Ok(Some(year))
        }
        Some(_) => Err(AppError::BadRequest(
            "year must be a four digit integer".to_string(),
        )),
    }
}

fn build_luck_reading_report(
    chart: &ChartResult,
    cycles: &[LuckCycle],
    birth_year: i32,
    reference_year: Option<i32>,
) -> Result<LuckReadingReport, AppError> {
    let mut warnings = Vec::new();
    let Some(year) = reference_year else {
        warnings.push("reading_year_not_requested");
        return Ok(LuckReadingReport {
            reference_year: None,
            reference_age: None,
            age_policy: "nominal_year_difference_plus_one",
            current_index: None,
            previous_cycle: None,
            current_cycle: None,
            next_cycle: None,
            draft: None,
            warnings,
        });
    };

    let raw_age = year - birth_year + 1;
    if raw_age <= 0 {
        return Err(AppError::BadRequest(
            "reading_year must not be earlier than birth year".to_string(),
        ));
    }
    if raw_age > u8::MAX as i32 {
        warnings.push("reference_age_outside_supported_cycle_range");
    }
    let age = raw_age.clamp(1, u8::MAX as i32) as u8;

    if cycles.is_empty() {
        warnings.push("major_luck_cycle_missing");
        return Ok(LuckReadingReport {
            reference_year: Some(year),
            reference_age: Some(age),
            age_policy: "nominal_year_difference_plus_one",
            current_index: None,
            previous_cycle: None,
            current_cycle: None,
            next_cycle: None,
            draft: None,
            warnings,
        });
    }

    let mut current_index = cycles
        .iter()
        .position(|cycle| age >= cycle.start_age && age <= cycle.end_age);
    if current_index.is_none() {
        if age < cycles[0].start_age {
            warnings.push("reference_age_before_first_luck");
            current_index = Some(0);
        } else {
            warnings.push("reference_age_after_last_luck");
            current_index = Some(cycles.len() - 1);
        }
    }

    let index = current_index.expect("index is set when cycles are not empty");
    let current_cycle = cycles[index].clone();
    let draft = build_major_luck_stage_foundation(chart, &current_cycle, "current");
    warnings.extend(draft.warnings.iter().copied());

    Ok(LuckReadingReport {
        reference_year: Some(year),
        reference_age: Some(age),
        age_policy: "nominal_year_difference_plus_one",
        current_index: Some(index),
        previous_cycle: index
            .checked_sub(1)
            .and_then(|idx| cycles.get(idx).cloned()),
        current_cycle: Some(current_cycle),
        next_cycle: cycles.get(index + 1).cloned(),
        draft: Some(draft),
        warnings,
    })
}

fn build_annual_trigger_reading_report(
    chart: &ChartResult,
    cycles: &[LuckCycle],
    birth_year: i32,
    annual_year: Option<i32>,
) -> Result<AnnualTriggerReadingReport, AppError> {
    let mut warnings = Vec::new();
    let Some(year) = annual_year else {
        warnings.push("annual_year_not_requested");
        return Ok(AnnualTriggerReadingReport {
            year: None,
            year_source: None,
            annual_pillar: None,
            current_luck_cycle: None,
            draft: None,
            warnings,
        });
    };

    let annual = annual_pillar(year);
    let raw_age = year - birth_year + 1;
    if raw_age <= 0 {
        return Err(AppError::BadRequest(
            "year must not be earlier than birth year".to_string(),
        ));
    }
    if raw_age > u8::MAX as i32 {
        warnings.push("reference_age_outside_supported_cycle_range");
    }
    let age = raw_age.clamp(1, u8::MAX as i32) as u8;

    if cycles.is_empty() {
        warnings.push("major_luck_cycle_missing");
        return Ok(AnnualTriggerReadingReport {
            year: Some(year),
            year_source: Some("explicit"),
            annual_pillar: Some(annual),
            current_luck_cycle: None,
            draft: None,
            warnings,
        });
    }

    let mut current_index = cycles
        .iter()
        .position(|cycle| age >= cycle.start_age && age <= cycle.end_age);
    if current_index.is_none() {
        if age < cycles[0].start_age {
            warnings.push("reference_age_before_first_luck");
            current_index = Some(0);
        } else {
            warnings.push("reference_age_after_last_luck");
            current_index = Some(cycles.len() - 1);
        }
    }

    let current_luck =
        cycles[current_index.expect("index is set when cycles are not empty")].clone();

    let draft = build_annual_trigger_foundation(chart, &current_luck, &annual, year);
    warnings.extend(draft.warnings.iter().copied());

    Ok(AnnualTriggerReadingReport {
        year: Some(year),
        year_source: Some("explicit"),
        annual_pillar: Some(annual),
        current_luck_cycle: Some(current_luck),
        draft: Some(draft),
        warnings,
    })
}

fn build_disclaimer() -> String {
    "本报告基于命轨当前排盘与结构分析结果自动生成，仅供传统文化参考。您可以把它当作一份关于性格倾向、做事节奏、关系模式和阶段主题的阅读材料，而不是现实结论。\n\n报告中的「强」「弱」「格局」「用神」「大运」等词，都是传统命理里的分析语言，不代表好坏、高低、成败，也不构成医学、法律、财务、婚恋或人生决策建议。现实中的重要选择，仍应结合实际状况、个人判断和专业意见。".into()
}

struct MainReportContext<'a> {
    chart: &'a crate::domain::bazi::ChartResult,
    snapshot: &'a AnalysisSnapshot,
    luck_context: &'a LuckCycleContext,
    luck_reading: &'a LuckReadingReport,
    annual_trigger_reading: &'a AnnualTriggerReadingReport,
    strength: &'a crate::domain::deep_analysis::StrengthAssessment,
    pattern: &'a crate::domain::deep_analysis::PatternInfo,
    useful_gods: &'a [crate::domain::deep_analysis::UsefulGodHint],
}

fn build_blocks(context: MainReportContext<'_>) -> Vec<ReportBlock> {
    let day_stem = &context.chart.chart.day.stem;
    let day_el = stem_element_cn(day_stem);
    let yin_yang = if ["甲", "丙", "戊", "庚", "壬"].contains(&day_stem.as_str()) {
        "阳"
    } else {
        "阴"
    };

    vec![
        block_overview(context.chart),
        block_day_master(day_stem, day_el, yin_yang),
        block_elements(&context.snapshot.element_metrics),
        block_ten_gods(&context.snapshot.ten_god_metrics, day_stem),
        block_hidden_stems(context.chart),
        block_strength(
            day_stem,
            day_el,
            &context.chart.chart.month.branch,
            context.strength,
        ),
        block_pattern(context.pattern),
        block_useful_god(context.useful_gods),
        block_luck(context.luck_context, context.luck_reading),
        block_annual_trigger(context.annual_trigger_reading),
    ]
}

// ── Chapter 1: 命盘概览 ──
fn block_overview(chart: &crate::domain::bazi::ChartResult) -> ReportBlock {
    let profile = &chart.basis.request.birth_profile;
    let hour_str = chart
        .chart
        .hour
        .as_ref()
        .map(|h| h.ganzhi())
        .unwrap_or_else(|| "未知（时辰未提供）".to_string());

    let unknown_note = if chart.chart.hour.is_none() {
        "\n\n因为当前时辰没有完整提供，时柱部分会显示为未知或候选状态。命轨不会在界面或报告中强行补推时辰，因此这一部分需要按「信息不完整」来阅读。"
    } else {
        ""
    };

    ReportBlock {
        id: "chart-overview".into(),
        title: "命盘概览".into(),
        body: format!(
            "主盘的第一层，是年柱、月柱、日柱、时柱这四组时间坐标。它们不是四个孤立的结论，而是这张命盘展开解读的骨架。\n\n\
            您的出生时间为公历 {}年{}月{}日 {}时，对应的四柱为：年柱「{}」、月柱「{}」、日柱「{}」、时柱「{}」。\n\n\
            其中，日柱是观察「自己」的核心位置。后面提到的日主、十神、强弱和格局，都会围绕日柱展开。先把四柱坐稳，后面的判断才不会飘到单个符号上。{}",
            profile.date.year,
            profile.date.month,
            profile.date.day,
            profile
                .time
                .as_ref()
                .map(|t| format!("{}:{}", t.hour, t.minute))
                .unwrap_or_else(|| "未知".to_string()),
            chart.chart.year.ganzhi(),
            chart.chart.month.ganzhi(),
            chart.chart.day.ganzhi(),
            hour_str,
            unknown_note
        ),
    }
}

// ── Chapter 2: 日主介绍 ──
fn block_day_master(day_stem: &str, day_el: &str, yin_yang: &str) -> ReportBlock {
    let desc = describe_day_stem_full(day_stem);
    ReportBlock {
        id: "day-master-intro".into(),
        title: "日主介绍".into(),
        body: format!(
            "日主是整张命盘的主视角，代表这套结构里最需要被照看的「自己」。它不是性格的全部，也不是单独下结论的标签，但后面的五行、十神、强弱和格局都会围绕它展开。\n\n\
            您的日主是「{}」，五行属{}，为{}性。\n\n\
            对您来说，日主「{}」会让命盘更重视自己怎样判断、怎样发力、怎样与外界互动。\n\n\
            {}\n\n\
            所以，日主给出的不是人生定论，而是读盘的起点：先看您惯用什么方式面对世界，再看其他力量如何支持、牵动或修正这个方式。",
            day_stem, day_el, yin_yang, day_stem, desc
        ),
    }
}

// ── Chapter 3: 五行分布 ──
fn block_elements(metrics: &[crate::domain::analysis::WeightedMetric]) -> ReportBlock {
    let name: std::collections::HashMap<&str, &str> = [
        ("wood", "木"),
        ("fire", "火"),
        ("earth", "土"),
        ("metal", "金"),
        ("water", "水"),
    ]
    .into_iter()
    .collect();

    let mut parts = Vec::new();
    for m in metrics {
        let cn = name.get(m.id).unwrap_or(&m.id);
        let level = element_level_label(m.weight_x2);
        parts.push(format!("{}行{}", cn, level));
    }

    let summary = parts.join("、");
    let plain = element_distribution_plain(metrics, &name);

    ReportBlock {
        id: "element-distribution".into(),
        title: "五行分布".into(),
        body: format!(
            "五行分布呈现的是这张命盘的基本用力比例。木、火、土、金、水可以看作五种处理世界的方式。\n\n\
            木，像成长、规划、主动开拓；\n火，像表达、热情、反应速度和被看见；\n土，像稳定、承接、现实感和耐心；\n金，像规则、判断、边界和执行力；\n水，像思考、学习、流动性和适应变化。\n\n\
            从当前命盘看，五行结果为：{}。\n\n\
            这不是在判断您「好不好」，而是在看哪类倾向更容易被命盘调动出来。{}\n\n\
            五行真正要看的，是不同力量能不能配合起来。偏弱不代表没有，偏强也不代表一定更好；关键在于该发力时能发力，该收住时能收住。",
            summary, plain
        ),
    }
}

fn element_level_label(weight: u16) -> &'static str {
    match weight {
        0 => "缺",
        1..=2 => "偏弱",
        3..=5 => "中和",
        _ => "偏强",
    }
}

fn element_distribution_plain(
    metrics: &[crate::domain::analysis::WeightedMetric],
    name: &std::collections::HashMap<&str, &str>,
) -> String {
    let mut absent = Vec::new();
    let mut weak = Vec::new();
    let mut balanced = Vec::new();
    let mut strong = Vec::new();

    for m in metrics {
        let cn = name.get(m.id).unwrap_or(&m.id).to_string();
        match m.weight_x2 {
            0 => absent.push(cn),
            1..=2 => weak.push(cn),
            3..=5 => balanced.push(cn),
            _ => strong.push(cn),
        }
    }

    let mut sentences = Vec::new();
    if !balanced.is_empty() {
        sentences.push(format!(
            "{}相对平衡，说明这些方式在合适场景里比较容易自然调动。",
            balanced.join("、")
        ));
    }
    if !weak.is_empty() {
        sentences.push(format!(
            "{}偏弱，不是现实能力缺失，而是这几类力量更需要靠训练、环境支持和长期习惯慢慢养出来。",
            weak.join("、")
        ));
    }
    if !strong.is_empty() {
        sentences.push(format!(
            "{}偏强，说明相关倾向更容易先被看见，也更需要学会收放。",
            strong.join("、")
        ));
    }
    if !absent.is_empty() {
        sentences.push(format!(
            "{}没有直接出现，只说明它们不是这张盘最表层的力量，不代表现实中不能培养。",
            absent.join("、")
        ));
    }

    sentences.join("")
}

// ── Chapter 4: 十神关系 ──
fn block_ten_gods(
    metrics: &[crate::domain::analysis::WeightedMetric],
    day_stem: &str,
) -> ReportBlock {
    let summary = metrics
        .iter()
        .filter(|m| m.weight_x2 > 0)
        .map(|m| ten_god_summary_phrase(m.id).to_string())
        .collect::<Vec<_>>()
        .join("、");
    let summary = if summary.is_empty() {
        "暂时没有特别集中的十神线索".to_string()
    } else {
        summary
    };

    let grouped = ten_god_grouped_reading(metrics);
    ReportBlock {
        id: "ten-god-relations".into(),
        title: "十神关系".into(),
        body: format!(
            "十神关系看的是日主与外界怎样发生连接。十神不是神仙，也不是吉凶标签，而是一套观察自我、表达、资源、责任和支持的分类。\n\n\
            以您的日主「{}」为中心，当前命盘中比较明显的十神线索为：{}。若收成几组力量，可以这样读：{}。\n\n\
            这些线索不是在断您一定会遇到什么人或发生什么事，而是在说明您更容易在哪些关系模式里用力。综合来看，十神让这张盘的互动方式更清楚：哪里需要表达，哪里需要规则，哪里需要资源承接，哪里又要留意边界和支持。",
            day_stem, summary, grouped
        ),
    }
}

fn ten_god_grouped_reading(metrics: &[crate::domain::analysis::WeightedMetric]) -> String {
    let has = |ids: &[&str]| {
        metrics
            .iter()
            .any(|metric| metric.weight_x2 > 0 && ids.contains(&metric.id))
    };
    let mut parts = Vec::new();
    if has(&["peer", "rob_wealth"]) {
        parts.push("比劫让自我立场、同辈协作和资源分配更需要说清");
    }
    if has(&["eating_god", "hurting_officer"]) {
        parts.push("食伤把表达、技术、想法出口和产出方式带到明处");
    }
    if has(&["direct_wealth", "indirect_wealth"]) {
        parts.push("财星提示资源、现实事务和外部交换会成为重要议题");
    }
    if has(&["direct_officer", "seven_killings"]) {
        parts.push("官杀让规则、责任、压力和目标感更容易靠近");
    }
    if has(&["direct_resource", "indirect_resource"]) {
        parts.push("印星提醒您需要学习、理解、支持和保护系统");
    }
    if parts.is_empty() {
        "十神分布较均衡，没有特别突出的模式".to_string()
    } else {
        parts.join("；")
    }
}

fn ten_god_summary_phrase(id: &str) -> &'static str {
    match id {
        "peer" => "比肩带来自我立场和同辈协作",
        "rob_wealth" => "劫财牵动竞争感和资源分配",
        "eating_god" => "食神强调自然表达和稳定输出",
        "hurting_officer" => "伤官让想法锋芒和表达出口更明显",
        "direct_wealth" => "正财指向稳定资源和现实安排",
        "indirect_wealth" => "偏财带出机会资源和外部交换",
        "direct_officer" => "正官让规则、责任和秩序更容易被看见",
        "seven_killings" => "七杀带来压力挑战和行动张力",
        "direct_resource" => "正印提示学习、保护和支持系统",
        "indirect_resource" => "偏印提示独立理解和内在消化",
        _ => "十神线索需要合并五行和宫位继续观察",
    }
}

fn count_text(value: u16) -> &'static str {
    match value {
        0 => "暂无",
        1 => "一处",
        2 => "两处",
        3 => "三处",
        4 => "四处",
        5 => "五处",
        _ => "多处",
    }
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

// ── Chapter 5: 地支藏干 ──
fn block_hidden_stems(chart: &crate::domain::bazi::ChartResult) -> ReportBlock {
    let branches = [
        ("年支", &chart.chart.year.branch),
        ("月支", &chart.chart.month.branch),
        ("日支", &chart.chart.day.branch),
    ];
    let details: Vec<String> = branches
        .iter()
        .map(|(label, branch)| {
            let stems = hidden_stems_for_branch(branch);
            format!("{}「{}」藏{}", label, branch, stems)
        })
        .collect();

    let hour_detail = chart
        .chart
        .hour
        .as_ref()
        .map(|h| {
            format!(
                "；时支「{}」藏{}",
                h.branch,
                hidden_stems_for_branch(&h.branch)
            )
        })
        .unwrap_or_default();

    ReportBlock {
        id: "hidden-stems".into(),
        title: "地支藏干".into(),
        body: format!(
            "地支藏干代表命盘里不太直接显露、但会在特定情境中参与发力的部分。\n\n\
            如果说天干像一个人外在比较容易看到的表现，那么地支藏干就像藏在性格深处、环境里、习惯里的潜在倾向。它不一定每天都表现出来，但在某些情境下，比如压力、机会、关系变化或环境变化时，可能会被触发。\n\n\
            当前地支藏干结果为：{}{}。\n\n\
            藏干像命盘里的「隐藏按钮」。有些倾向平时不明显，但当外部条件合适时，它们会影响您的反应方式、关注点和做事节奏。\n\n\
            这些符号不需要逐个死记。您只要知道：藏干代表的是更深层、更隐性的倾向，它们不像天干那样直接站在表面，却会在合适条件下参与影响。这部分适合作为辅证，不适合单独拿来下结论。",
            details.join("；"),
            hour_detail
        ),
    }
}

fn hidden_stems_for_branch(branch: &str) -> String {
    let stems: &[&str] = match branch {
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
    };
    stems.join("、")
}

// ── Chapter 6: 日主强弱 ──
fn block_strength(
    day_stem: &str,
    _day_el: &str,
    month_branch: &str,
    strength: &crate::domain::deep_analysis::StrengthAssessment,
) -> ReportBlock {
    let deling_plain = if strength.deling {
        "是（月令生扶日主，季节环境有承托）"
    } else {
        "否（月令不生扶日主，季节环境承托较少）"
    };
    let level_plain = strength_level_plain(strength.level);

    ReportBlock {
        id: "day-master-strength".into(),
        title: "日主强弱".into(),
        body: format!(
            "日主强弱看的不是现实能力高低，而是命盘中的「自己」得到多少承托、适合怎样发力。\n\n\
            您的日主「{}」生于{}月。这张命盘里，月令对日主的支持为：{}；地支有{}形成支撑（得地）；天干有{}形成支撑（得势）。综合这些承托，当前读为「{}」。\n\n\
            得令：出生月份是否支持日主。可以理解为「季节环境是否形成承托」。\n\
            得地：地支里是否有支持日主的力量。可以理解为「底层环境是否给您支撑」。\n\
            得势：天干里是否有支持日主的力量。可以理解为「表层力量是否利于发力」。\n\n\
            这一层要读的是：您在命盘里更适合独立硬推，还是更需要环境、资源和节奏来支持。{}\n\n\
            所以，看到「偏弱」不需要紧张，看到「偏强」也不代表一定更好。强弱只是结构状态，不是人生结论。真正重要的是：您适合怎样的节奏，应该怎样借力，怎样避免过度消耗或过度用力。",
            day_stem,
            month_branch,
            deling_plain,
            count_text(u16::from(strength.dedi)),
            count_text(u16::from(strength.deshi)),
            strength.level,
            level_plain
        ),
    }
}

fn strength_level_plain(level: &str) -> &'static str {
    match level {
        "极弱" => {
            "日主极弱，表示命盘里的「自己」得到的支持较少。这类结构更不适合长期硬扛，适合先建立稳定节奏、找到外部支持，再逐步发力。"
        }
        "偏弱" => {
            "日主偏弱，表示命盘里的「自己」需要更多资源、环境或节奏配合。不一定是能力不足，而是更适合借力成长、循序渐进，不适合长期把所有压力都压在自己身上。"
        }
        "中和" => {
            "日主中和，表示命盘里的「自己」和周围力量相对平衡。这类结构通常更容易根据环境调整自己，既能发力，也需要保持节奏。"
        }
        "偏强" => {
            "日主偏强，表示命盘里的「自己」力量比较明显。主见、推动力或自我意识更容易被看见，但也要注意不要过度用力，学会协作和收放。"
        }
        "极强" => {
            "日主极强，表示命盘里的「自己」力量非常突出。这类结构往往有较强的推动力和坚持度，但也更需要注意弹性、倾听和转换方式，避免一直用同一种力量推进所有事情。"
        }
        _ => "",
    }
}

// ── Chapter 7: 格局初判 ──
fn block_pattern(pattern: &crate::domain::deep_analysis::PatternInfo) -> ReportBlock {
    let desc = pattern_plain(&pattern.pattern_name);
    ReportBlock {
        id: "pattern-classification".into(),
        title: "格局初判".into(),
        body: format!(
            "格局呈现的是命盘的主要运行方式，不是等级，也不是人生上限。它回答的是：这张盘最主要的动力、矛盾和用力方向在哪里。\n\n\
            按命盘结构来看，本命盘的格局初判为「{}」，类型为「{}」，置信度为「{}」。这里的「置信度」只说明格局线索是否集中，不是对人生结果的确定预测。\n\n\
            {}\n\n\
            所以，格局更像命盘的主题线索，而不是高低判断。它让您看见：自己更容易在哪类问题上反复用力，哪类能力更值得被整理出来。",
            pattern.pattern_name, pattern.pattern_type, pattern.confidence, desc
        ),
    }
}

fn pattern_plain(name: &str) -> &'static str {
    match name {
        "正官格" => {
            "正官格可以理解为「规则、责任、标准感」比较重要的结构。这类命盘更容易在秩序、职责、规范、长期稳定的事情里找到方向。但它不是说一定适合某个职业，而是说明规则意识和责任主题比较值得关注。"
        }
        "七杀格" => {
            "七杀格可以理解为「压力、挑战、行动力」比较强的结构。这类命盘容易被目标、竞争或外部要求推动，但关键是要把压力转化成有秩序的行动，而不是被压力牵着走。"
        }
        "财格" => {
            "财格可以理解为「现实资源、经营意识、落地能力」比较重要的结构。这类命盘更容易关注实际结果、资源安排和现实交换，但这不是财富承诺，只是说明现实事务是重要主题。"
        }
        "印格" => {
            "印格可以理解为「学习、理解、支持系统」比较重要的结构。这类命盘更需要知识、经验、稳定支持和内在安全感来帮助自己成长。"
        }
        "食伤格" => {
            "食伤格可以理解为「表达、输出、技能、创造」比较重要的结构。这类命盘需要把想法、能力或经验表达出来，越能形成稳定输出，越容易找到自己的节奏。"
        }
        "建禄格" => {
            "建禄格可以理解为「自我根基和自主能力」比较明显的结构。这类命盘更需要建立自己的稳定基础，不太适合长期完全依赖外部安排。"
        }
        "月刃格" => {
            "月刃格可以理解为「自我力量、冲劲和边界感」比较强的结构。这类命盘更容易有强烈主张，但也更需要学习合作、缓冲和转弯。"
        }
        _ => {
            "当前格局不适合用单一标签概括。这张命盘可能不是某一种主题特别突出，而是需要结合五行、十神和日主强弱一起看。"
        }
    }
}

// ── Chapter 8: 用神参考 ──
fn block_useful_god(gods: &[crate::domain::deep_analysis::UsefulGodHint]) -> ReportBlock {
    let hints: Vec<String> = gods
        .iter()
        .map(|h| {
            let el_cn = match h.element {
                "印星(生我)" | "印星(化杀)" => "印星，代表学习、保护和支持",
                "比劫(同我)" => "比劫，代表自我力量、同伴和行动力",
                "食伤(泄秀)" => "食伤，代表表达、输出和创造力",
                "财星(耗身)" => "财星，代表资源、现实和经营意识",
                "通关用神" => "通关五行，代表调节、平衡和流动",
                other => other,
            };
            let lead = if h.priority == 1 { "先看" } else { "再看" };
            format!("{}{}：{}。", lead, el_cn, h.reason)
        })
        .collect();

    ReportBlock {
        id: "useful-god-hints".into(),
        title: "用神参考".into(),
        body: format!(
            "用神参考看的是这张命盘里哪类力量更适合充当「调节器」。它不是命令，也不是要求您现实中必须选择某个行业、颜色、方位、物品或生活方式，而是传统命理里观察平衡方向的工具。\n\n\
            结合当前日主强弱和格局，较适合作为调节方向的力量可以这样看：{}\n\n\
            被列为参考方向的力量，更像是在提醒您：做事节奏、学习方式、环境选择和长期习惯上，哪类倾向更值得留意。\n\n\
            用神的价值在于调节，不在于把人绑住。真正重要的选择，仍然要结合实际条件、个人能力和现实反馈。",
            hints.join("")
        ),
    }
}

// ── Chapter 9: 大运走势 ──
fn block_luck(luck_context: &LuckCycleContext, luck_reading: &LuckReadingReport) -> ReportBlock {
    let parts: Vec<String> = luck_context
        .cycles
        .iter()
        .map(|c| {
            format!(
                "「{}」运（{}—{}岁）",
                c.pillar.ganzhi(),
                c.start_age,
                c.end_age
            )
        })
        .collect();
    let direction = if luck_context.direction == "forward" {
        "顺行"
    } else {
        "逆行"
    };
    let stage_text = if let (Some(year), Some(age), Some(current)) = (
        luck_reading.reference_year,
        luck_reading.reference_age,
        luck_reading.current_cycle.as_ref(),
    ) {
        let index_text = luck_reading
            .current_index
            .map(|index| format!("{}个十年阶段", ordinal_text(index + 1)))
            .unwrap_or_else(|| "当前可定位阶段".to_string());
        let current_label = luck_reading
            .current_index
            .map(|index| format!("{}运", ordinal_text(index + 1)))
            .unwrap_or_else(|| current.label.clone());
        let previous = luck_reading
            .previous_cycle
            .as_ref()
            .map(|cycle| format!("前一阶段为「{}」", cycle.pillar.ganzhi()))
            .unwrap_or_else(|| "前一阶段暂无可计算项".to_string());
        let next = luck_reading
            .next_cycle
            .as_ref()
            .map(|cycle| format!("后一阶段为「{}」", cycle.pillar.ganzhi()))
            .unwrap_or_else(|| "后一阶段暂无可计算项".to_string());

        format!(
            "您选定的观察年份为 {}年，按「出生年差 + 1」的名义年龄口径约为 {} 岁，落在{}：{}「{}」（{}至{}岁）。{}，{}。",
            year,
            age,
            index_text,
            current_label,
            current.pillar.ganzhi(),
            current.start_age,
            current.end_age,
            previous,
            next
        )
    } else {
        "如果还没有明确选择观察年份，本章先只展示大运列表，不把年份静默定位到具体十年阶段。"
            .to_string()
    };

    let reading_text = luck_reading
        .draft
        .as_ref()
        .map(|draft| timeline_plain_report_summary(&draft.readings, 3))
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| {
            "大运列表先给出阶段坐标；等您选定观察年份后，再展开对应年份所在阶段的结构主题。"
                .to_string()
        });

    ReportBlock {
        id: "luck-cycles".into(),
        title: "大运走势".into(),
        body: format!(
            "大运是命盘里的阶段背景，不是十年预言。它像一段时间的气候，会影响主题、节奏和准备方式，但不会替您决定人生。\n\n\
            当前起运方向为{}，约 {} 岁起运。目前可排出的大运阶段为：{}。\n\n\
            {}\n\n\
            当前阶段重点看大运怎样给一段时间定调：哪些压力靠近，哪些支持能接住，哪些节奏需要放慢。这样读，是为了看阶段主题怎样靠近您，而不是把它说成现实事件。\n\n\
            {}\n\n\
            大运真正提示的是阶段重心：有的阶段更强调学习和积累，有的阶段更强调责任和压力，有的阶段更强调表达、资源或行动。看清阶段气候，您就更容易知道哪里该顺势、哪里要留余力。",
            direction,
            luck_context.starting_age,
            parts.join("；"),
            stage_text,
            reading_text
        ),
    }
}

// ── Chapter 10: 年度引动 ──
fn block_annual_trigger(report: &AnnualTriggerReadingReport) -> ReportBlock {
    let body = if let (Some(year), Some(annual), Some(current_luck), Some(draft)) = (
        report.year,
        report.annual_pillar.as_ref(),
        report.current_luck_cycle.as_ref(),
        report.draft.as_ref(),
    ) {
        let reading_text = timeline_plain_report_summary(&draft.readings, 3);
        let evidence_text = annual_evidence_story(draft, year);

        format!(
            "年度引动把您选定的{}年放回命盘里，看{}年的年柱怎样与原局、当前大运发生关系。它不展开逐月、逐日或事件预测，只看{}年哪些主题更容易浮到眼前。\n\n\
            您选定的年度为{}年，年度干支为「{}」。同时参考当前大运「{}」（{}至{}岁）。\n\n\
            年度读法会把年柱天干、五行流向、地支藏干、合冲刑害和当前大运放在同一处看。天干看主题怎样被唤起，五行看这股力量怎样贴近日主；藏干补出底层背景，地支关系则看原局哪些位置被牵动。\n\n\
            {}年靠近命盘时，读盘脉络会分成两个方向慢慢浮出来：{}\n\n\
            {}\n\n\
            综合起来，年度引动回答的是：{}年哪类命盘结构更容易被您看见和整理。它不会告诉您 {}年一定发生什么；真正要做决定时，仍然要把现实条件、个人判断和专业意见放在前面。",
            year,
            year,
            year,
            year,
            annual.ganzhi(),
            current_luck.pillar.ganzhi(),
            current_luck.start_age,
            current_luck.end_age,
            year,
            evidence_text,
            if reading_text.is_empty() {
                "当前年度引动没有形成足够明确的可读信号，因此只保留年度坐标和边界提醒。".to_string()
            } else {
                reading_text
            },
            year,
            year
        )
    } else {
        "年度引动需要您明确选择年份。尚未选择年份时，本章先不生成年度解释，也不会从当前日期静默推断。\n\n这个边界会让读盘更稳：年度推演必须在您明确选择年份后才展开。".to_string()
    };

    ReportBlock {
        id: "annual-trigger".into(),
        title: "年度引动".into(),
        body,
    }
}

fn annual_evidence_story(draft: &TimelineReadingDraft, year: i32) -> String {
    let mut primary = Vec::new();
    let mut background = Vec::new();
    for item in draft.evidence.iter().take(6) {
        let sentence = annual_evidence_sentence(item, year);
        if item.signal_id.contains("stem-ten-god")
            || item.signal_id.contains("stem-element-relation")
        {
            primary.push(sentence);
        } else {
            background.push(sentence);
        }
    }

    if primary.is_empty() && background.is_empty() {
        "暂无足够明确的年度牵动，先保留年度坐标和边界提醒。".to_string()
    } else {
        let primary_text = if primary.is_empty() {
            "年度天干和五行没有形成特别集中的单独线索，更多作为年度背景参与。".to_string()
        } else {
            primary.join("")
        };
        let background_text = if background.is_empty() {
            "地支关系和当前大运暂时没有补出额外重点。".to_string()
        } else {
            background.join("")
        };
        format!(
            "\n\n年度线索会落在日主怎样承接压力、资源和行动节奏上：{}\n\n地支、藏干和当前大运合到一起后，会补出哪些位置被带动、哪些背景需要慢慢安顿：{}",
            primary_text, background_text
        )
    }
}

fn annual_evidence_sentence(item: &TimelineEvidence, year: i32) -> String {
    let detail = clean_annual_detail(&item.detail, year);
    if item.signal_id.contains("stem-ten-god") {
        if item.source == "annual-current-luck" {
            format!("当前大运也在场，{detail}")
        } else {
            let signal_name =
                ten_god_signal_name(&detail).unwrap_or_else(|| "十神信号".to_string());
            format!("{year}年的天干让{signal_name}更醒目，{detail}")
        }
    } else if item.signal_id.contains("stem-element-relation") {
        format!("五行关系会把这股力量贴近日主的方式说清，{detail}")
    } else if item.signal_id.contains("hidden-stems") {
        format!("地支里还藏着一层底色，{detail}")
    } else if item.signal_id.contains("branch-relation") {
        let anchor = public_report_text(&item.chart_anchor);
        format!("{anchor}被{year}年牵动后，{detail}")
    } else {
        let anchor = public_report_text(&item.chart_anchor);
        let trigger = public_report_text(&item.trigger);
        format!("{anchor}与{trigger}相接，{detail}")
    }
}

fn timeline_plain_report_summary(readings: &[PlainReading], limit: usize) -> String {
    let points = readings
        .iter()
        .take(limit)
        .map(|reading| public_report_text(&reading.plain))
        .filter(|value| !value.trim().is_empty())
        .collect::<Vec<_>>();
    if points.is_empty() {
        String::new()
    } else {
        format!("把这些线索收回到实际读法，重点是：{}", points.join(" "))
    }
}

fn clean_annual_detail(value: &str, year: i32) -> String {
    let mut text = public_report_text(value);
    text = text.replace("以日主为中心，", "");
    text = text.replace(&format!("{year}年年柱"), "流年");
    text = text.replace(" = ", "形成");
    text = text.replace('=', "形成");
    text = text.trim().to_string();
    if text.ends_with('。') {
        text
    } else {
        format!("{text}。")
    }
}

fn ten_god_signal_name(detail: &str) -> Option<String> {
    let marker = "形成「";
    let start = detail.find(marker)? + marker.len();
    let rest = &detail[start..];
    let end = rest.find('」')?;
    let name = rest[..end].trim();
    if name.is_empty() {
        None
    } else {
        Some(format!("{name}信号"))
    }
}

fn public_report_text(value: &str) -> String {
    let text = value
        .replace(" = ", "形成")
        .replace('=', "为")
        .replace("。。", "。")
        .replace(" 与 ", "与")
        .replace('(', "（")
        .replace(')', "）");
    quote_relation_names(&replace_luck_ordinals(&replace_age_range_hyphen(&text)))
}

fn quote_relation_names(value: &str) -> String {
    value
        .replace("形成六冲", "形成\"六冲\"")
        .replace("形成六合", "形成\"六合\"")
        .replace("形成三刑", "形成\"三刑\"")
        .replace("形成六害", "形成\"六害\"")
        .replace("形成自刑", "形成\"自刑\"")
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

// ── Day stem plain descriptions ──
fn describe_day_stem_full(stem: &str) -> &'static str {
    match stem {
        "甲" => {
            "如果用生活化的比喻，甲木像一棵向上生长的大树。它强调成长、方向感、原则和支撑力。甲木倾向于希望事情有清晰的目标，也比较重视长期积累。\n\n但这不是说您一定强势或固执，只是说明「规划、成长、立起来」这类主题，在观察您命盘时会比较重要。"
        }
        "乙" => {
            "如果用生活化的比喻，乙木像花草藤蔓。它不一定用很强硬的方式推进事情，但往往更重视弹性、适应、审美和细腻的连接。\n\n这不是说您一定柔弱，而是说明您更适合从灵活调整、关系协调和持续生长中找到自己的节奏。"
        }
        "丙" => {
            "如果用生活化的比喻，丙火像太阳。它代表外放、热度、表达、感染力和被看见的需求。\n\n这不是说您一定外向，而是说明在您的命盘解读里，「表达自己、照亮别人、把热度释放出来」会是值得关注的主题。"
        }
        "丁" => {
            "如果用生活化的比喻，丁火像灯火、烛光。它不一定猛烈，但更细腻、更专注，也更容易体现为观察力、感受力和内在明亮感。\n\n这不是说您一定敏感，而是说明您的力量可能不是靠大声表现，而是靠持续照亮某个方向。"
        }
        "戊" => {
            "如果用生活化的比喻，戊土像山、城墙和厚土。它代表稳定、承载、责任感和现实支撑。\n\n这不是说您一定保守，而是说明您的命盘里，「稳住局面、承担事情、建立基础」这类主题会比较重要。"
        }
        "己" => {
            "如果用生活化的比喻，己土像田园之土。它更强调滋养、包容、整理、消化和慢慢培育。\n\n这不是说您一定慢热，而是说明您适合通过耐心、照顾细节和持续经营，把事情一点点养成。"
        }
        "庚" => {
            "如果用生活化的比喻，庚金像矿石、刀剑或未经雕琢的金属。它强调判断、规则、决断、执行和抗压。\n\n这不是说您一定冷硬，也不是说您必须强势。它只是说明，在您的命盘里，「明确边界、做出判断、把事情切开处理」这类能力会是重要主题。"
        }
        "辛" => {
            "如果用生活化的比喻，辛金像首饰、玉器旁的精细金属。它更重视质感、分寸、审美、精确和细节。\n\n这不是说您一定挑剔，而是说明您对品质、边界和精细度可能比较敏感，适合在需要打磨和判断的事情中发挥优势。"
        }
        "壬" => {
            "如果用生活化的比喻，壬水像江河大海。它代表流动、信息、视野、学习和变化中的适应力。\n\n这不是说您一定漂浮不定，而是说明您的命盘里，「看大局、吸收信息、顺势调整」会是值得关注的能力。"
        }
        "癸" => {
            "如果用生活化的比喻，癸水像雨露、雾气和地下水。它更细腻、更内敛，也更强调感受、理解、渗透和长期积累。\n\n这不是说您一定脆弱，而是说明您的力量可能不在表面，而在持续观察、慢慢吸收和细致理解。"
        }
        _ => "日主的特质需要结合具体天干进一步分析。",
    }
}

fn stem_element_cn(stem: &str) -> &'static str {
    match stem {
        "甲" | "乙" => "木",
        "丙" | "丁" => "火",
        "戊" | "己" => "土",
        "庚" | "辛" => "金",
        "壬" | "癸" => "水",
        _ => "土",
    }
}

fn luck_reading_to_json(report: &LuckReadingReport) -> String {
    let current_index = report
        .current_index
        .map(|index| (index + 1).to_string())
        .unwrap_or_else(|| "null".to_string());
    let reference_year = report
        .reference_year
        .map(|year| year.to_string())
        .unwrap_or_else(|| "null".to_string());
    let reference_age = report
        .reference_age
        .map(|age| age.to_string())
        .unwrap_or_else(|| "null".to_string());
    let signals = report
        .draft
        .as_ref()
        .map(|draft| {
            draft
                .signals
                .iter()
                .map(timeline_signal_to_json)
                .collect::<Vec<_>>()
                .join(",")
        })
        .unwrap_or_default();
    let evidence = report
        .draft
        .as_ref()
        .map(|draft| {
            draft
                .evidence
                .iter()
                .map(timeline_evidence_to_json)
                .collect::<Vec<_>>()
                .join(",")
        })
        .unwrap_or_default();
    let readings = report
        .draft
        .as_ref()
        .map(|draft| {
            draft
                .readings
                .iter()
                .map(plain_reading_to_json)
                .collect::<Vec<_>>()
                .join(",")
        })
        .unwrap_or_default();
    let audit = report
        .draft
        .as_ref()
        .map(|draft| draft.audit.clone())
        .unwrap_or_else(|| audit_timeline_text(""));
    let rule_version = report
        .draft
        .as_ref()
        .map(rule_version_to_json)
        .unwrap_or_else(|| "null".to_string());

    format!(
        concat!(
            "{{",
            "\"status\":\"restricted\",",
            "\"capability\":\"luck-reading\",",
            "\"route_carrier\":\"/api/charts/report\",",
            "\"reference_year\":{},",
            "\"reference_age\":{},",
            "\"age_policy\":{},",
            "\"current_index\":{},",
            "\"previous_cycle\":{},",
            "\"current_cycle\":{},",
            "\"next_cycle\":{},",
            "\"rule_version\":{},",
            "\"signals\":[{}],",
            "\"evidence\":[{}],",
            "\"readings\":[{}],",
            "\"warnings\":{},",
            "\"forbidden_output_audit\":{{\"status\":{},\"checked_patterns\":{}}}",
            "}}"
        ),
        reference_year,
        reference_age,
        json::string(report.age_policy),
        current_index,
        cycle_to_json(report.previous_cycle.as_ref()),
        cycle_to_json(report.current_cycle.as_ref()),
        cycle_to_json(report.next_cycle.as_ref()),
        rule_version,
        signals,
        evidence,
        readings,
        str_array_to_json(&report.warnings),
        json::string(audit.status),
        audit.checked_patterns,
    )
}

fn annual_trigger_reading_to_json(report: &AnnualTriggerReadingReport) -> String {
    let year = report
        .year
        .map(|year| year.to_string())
        .unwrap_or_else(|| "null".to_string());
    let year_source = report
        .year_source
        .map(json::string)
        .unwrap_or_else(|| "null".to_string());
    let signals = report
        .draft
        .as_ref()
        .map(|draft| {
            draft
                .signals
                .iter()
                .map(timeline_signal_to_json)
                .collect::<Vec<_>>()
                .join(",")
        })
        .unwrap_or_default();
    let evidence = report
        .draft
        .as_ref()
        .map(|draft| {
            draft
                .evidence
                .iter()
                .map(timeline_evidence_to_json)
                .collect::<Vec<_>>()
                .join(",")
        })
        .unwrap_or_default();
    let readings = report
        .draft
        .as_ref()
        .map(|draft| {
            draft
                .readings
                .iter()
                .map(plain_reading_to_json)
                .collect::<Vec<_>>()
                .join(",")
        })
        .unwrap_or_default();
    let audit = report
        .draft
        .as_ref()
        .map(|draft| draft.audit.clone())
        .unwrap_or_else(|| audit_timeline_text(""));
    let rule_version = report
        .draft
        .as_ref()
        .map(rule_version_to_json)
        .unwrap_or_else(|| "null".to_string());
    let status = if report.year.is_some() && report.draft.is_some() {
        "restricted"
    } else {
        "not_requested"
    };

    format!(
        concat!(
            "{{",
            "\"status\":{},",
            "\"capability\":\"annual-trigger-reading\",",
            "\"route_carrier\":\"/api/charts/report?year=YYYY\",",
            "\"year\":{},",
            "\"year_source\":{},",
            "\"annual_pillar\":{},",
            "\"current_luck_cycle\":{},",
            "\"rule_version\":{},",
            "\"signals\":[{}],",
            "\"evidence\":[{}],",
            "\"readings\":[{}],",
            "\"warnings\":{},",
            "\"forbidden_output_audit\":{{\"status\":{},\"checked_patterns\":{}}}",
            "}}"
        ),
        json::string(status),
        year,
        year_source,
        pillar_to_json(report.annual_pillar.as_ref()),
        cycle_to_json(report.current_luck_cycle.as_ref()),
        rule_version,
        signals,
        evidence,
        readings,
        str_array_to_json(&report.warnings),
        json::string(audit.status),
        audit.checked_patterns,
    )
}

fn rule_version_to_json(draft: &TimelineReadingDraft) -> String {
    format!(
        concat!(
            "{{",
            "\"ruleset_id\":{},",
            "\"version\":{},",
            "\"disclaimer_id\":{}",
            "}}"
        ),
        json::string(draft.rule_version.ruleset_id),
        json::string(draft.rule_version.version),
        json::string(draft.rule_version.disclaimer_id),
    )
}

fn pillar_to_json(pillar: Option<&Pillar>) -> String {
    pillar
        .map(|pillar| {
            format!(
                concat!(
                    "{{",
                    "\"stem\":{},",
                    "\"branch\":{},",
                    "\"ganzhi\":{}",
                    "}}"
                ),
                json::string(&pillar.stem),
                json::string(&pillar.branch),
                json::string(&pillar.ganzhi()),
            )
        })
        .unwrap_or_else(|| "null".to_string())
}

fn cycle_to_json(cycle: Option<&LuckCycle>) -> String {
    cycle
        .map(|cycle| {
            format!(
                concat!(
                    "{{",
                    "\"label\":{},",
                    "\"start_age\":{},",
                    "\"end_age\":{},",
                    "\"stem\":{},",
                    "\"branch\":{},",
                    "\"ganzhi\":{}",
                    "}}"
                ),
                json::string(&cycle.label),
                cycle.start_age,
                cycle.end_age,
                json::string(&cycle.pillar.stem),
                json::string(&cycle.pillar.branch),
                json::string(&cycle.pillar.ganzhi()),
            )
        })
        .unwrap_or_else(|| "null".to_string())
}

fn timeline_signal_to_json(signal: &TimelineSignal) -> String {
    format!(
        concat!(
            "{{",
            "\"id\":{},",
            "\"label\":{},",
            "\"category\":{},",
            "\"qualitative_level\":{},",
            "\"source\":{},",
            "\"applies_to_topics\":{},",
            "\"risk_tags\":{},",
            "\"summary\":{}",
            "}}"
        ),
        json::string(&signal.id),
        json::string(&signal.label),
        json::string(signal.category),
        json::string(signal.qualitative_level),
        json::string(signal.source),
        str_array_to_json(&signal.applies_to_topics),
        str_array_to_json(&signal.risk_tags),
        json::string(&signal.summary),
    )
}

fn timeline_evidence_to_json(item: &TimelineEvidence) -> String {
    format!(
        concat!(
            "{{",
            "\"id\":{},",
            "\"signal_id\":{},",
            "\"source\":{},",
            "\"relation\":{},",
            "\"chart_anchor\":{},",
            "\"trigger\":{},",
            "\"detail\":{}",
            "}}"
        ),
        json::string(&item.id),
        json::string(&item.signal_id),
        json::string(item.source),
        json::string(&quote_relation_value(&item.relation)),
        json::string(&item.chart_anchor),
        json::string(&item.trigger),
        json::string(&quote_relation_names(&item.detail)),
    )
}

fn plain_reading_to_json(reading: &PlainReading) -> String {
    format!(
        concat!(
            "{{",
            "\"id\":{},",
            "\"signal_id\":{},",
            "\"professional\":{},",
            "\"plain\":{},",
            "\"boundary\":{}",
            "}}"
        ),
        json::string(&reading.id),
        json::string(&reading.signal_id),
        json::string(&reading.professional),
        json::string(&reading.plain),
        json::string(reading.boundary),
    )
}

fn quote_relation_value(value: &str) -> String {
    match value {
        "六冲" | "六合" | "三刑" | "六害" | "自刑" => format!("\"{value}\""),
        _ => value.to_string(),
    }
}

fn str_array_to_json(values: &[&str]) -> String {
    let items = values
        .iter()
        .map(|value| json::string(value))
        .collect::<Vec<_>>()
        .join(",");
    format!("[{items}]")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;

    fn sample_config() -> AppConfig {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap_or_else(|| std::path::Path::new("."))
            .join("data")
            .join("raw")
            .join("lunar_data.yaml");
        AppConfig {
            addr: "127.0.0.1:8787".into(),
            lunar_data_path: path,
        }
    }

    fn sample_request() -> Request {
        Request::parse(b"GET /api/charts/report?date=2025-01-01&timezone=Asia/Shanghai&time_precision=exact&time=10:30&sex=male HTTP/1.1\r\n\r\n").unwrap()
    }

    fn sample_luck_reading_request() -> Request {
        Request::parse(b"GET /api/charts/report?date=2025-01-01&timezone=Asia/Shanghai&time_precision=exact&time=10:30&sex=male&reading_year=2026 HTTP/1.1\r\n\r\n").unwrap()
    }

    fn sample_annual_trigger_request() -> Request {
        Request::parse(b"GET /api/charts/report?date=2025-01-01&timezone=Asia/Shanghai&time_precision=exact&time=10:30&sex=male&reading_year=2026&year=2026 HTTP/1.1\r\n\r\n").unwrap()
    }

    fn sample_annual_trigger_year_only_request() -> Request {
        Request::parse(b"GET /api/charts/report?date=2025-01-01&timezone=Asia/Shanghai&time_precision=exact&time=10:30&sex=male&year=2026 HTTP/1.1\r\n\r\n").unwrap()
    }

    #[test]
    fn report_generates_without_error() {
        let result = generate(&sample_config(), &sample_request());
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert!(resp.body.contains("\"status\":\"restricted\""));
        assert!(resp.body.contains("chart-report"));
        assert!(resp.body.contains("仅供传统文化参考"));
        assert!(resp.body.contains("命盘概览"));
        assert!(resp.body.contains("日主介绍"));
        assert!(resp.body.contains("五行分布"));
        assert!(resp.body.contains("十神关系"));
        assert!(resp.body.contains("地支藏干"));
        assert!(resp.body.contains("日主强弱"));
        assert!(resp.body.contains("格局初判"));
        assert!(resp.body.contains("用神参考"));
        assert!(resp.body.contains("大运走势"));
        assert!(resp.body.contains("年度引动"));
    }

    #[test]
    fn main_report_uses_reading_tone_after_m51() {
        let result = generate(&sample_config(), &sample_annual_trigger_request()).unwrap();

        assert!(result.body.contains("主盘的第一层"));
        assert!(result.body.contains("十神线索为"));
        assert!(result.body.contains("若收成几组力量"));
        assert!(result.body.contains("2026年靠近命盘时"));
        assert!(result.body.contains("把这些线索收回到实际读法"));
        assert!(result.body.contains("当前阶段重点看大运怎样给一段时间定调"));
        assert!(result.body.contains("大运真正提示的是阶段重心"));
        assert!(result.body.contains("木、土、金、水偏弱"));
        assert!(
            result
                .body
                .contains("更需要靠训练、环境支持和长期习惯慢慢养出来")
        );
        for forbidden in [
            "基本脉络如下",
            "第一优先",
            "第二优先",
            "原局引动主要看",
            "先看这几层关系",
            "这一年",
            "偏弱表示这类倾向",
            "这一章会把",
            "牵动会先落在这些位置",
            "先看天干",
            "再看五行",
            "这一章看的是",
            "这一章先把",
            "放到日常理解里",
            "最适合当作",
            "可以先这样理解",
            "比较明显的十神为：比肩一处",
            "比较明显的十神为：",
            "读这一章时",
            "这条线已经进入命盘视野",
            "命理结构上，当前阶段大运",
            "这条十神线索",
        ] {
            assert!(
                !result.body.contains(forbidden),
                "main report leaked M51 teaching/count wording {forbidden}"
            );
        }
    }

    #[test]
    fn luck_reading_report_is_restricted_traceable_and_scoreless() {
        let result = generate(&sample_config(), &sample_luck_reading_request()).unwrap();

        assert!(result.body.contains("\"luck_reading\""));
        assert!(result.body.contains("\"capability\":\"luck-reading\""));
        assert!(result.body.contains("\"status\":\"restricted\""));
        assert!(result.body.contains("\"reference_year\":2026"));
        assert!(
            result
                .body
                .contains("\"age_policy\":\"nominal_year_difference_plus_one\"")
        );
        assert!(result.body.contains("\"current_cycle\""));
        assert!(result.body.contains("\"signals\""));
        assert!(result.body.contains("\"evidence\""));
        assert!(result.body.contains("从命理结构看"));
        assert!(result.body.contains("您可以"));
        assert!(result.body.contains("选定的观察年份为 2026年"));
        assert!(!result.body.contains("运运"));
        assert!(!result.body.contains("score_internal"));
        assert!(!result.body.contains("0-100"));
        assert!(!result.body.contains("必然发财"));
        assert!(!result.body.contains("流月运势"));
    }

    #[test]
    fn annual_trigger_report_requires_explicit_year_and_is_scoreless() {
        let result = generate(&sample_config(), &sample_annual_trigger_request()).unwrap();

        assert!(result.body.contains("\"annual_trigger_reading\""));
        assert!(
            result
                .body
                .contains("\"capability\":\"annual-trigger-reading\"")
        );
        assert!(result.body.contains("\"status\":\"restricted\""));
        assert!(result.body.contains("\"year\":2026"));
        assert!(result.body.contains("\"year_source\":\"explicit\""));
        assert!(result.body.contains("\"annual_pillar\""));
        assert!(result.body.contains("\"ganzhi\":\"丙午\""));
        assert!(result.body.contains("\"current_luck_cycle\""));
        assert!(result.body.contains("annual-trigger"));
        assert!(result.body.contains("annual-current-luck"));
        assert!(result.body.contains("major-luck+annual-trigger"));
        assert!(result.body.contains("2026年年柱丙午"));
        assert!(result.body.contains("年度引动"));
        assert!(result.body.contains("同时参考当前大运"));
        assert!(result.body.contains("2026年靠近命盘时"));
        assert!(result.body.contains("年度线索会落在日主怎样承接"));
        assert!(result.body.contains("2026年的天干让七杀信号更醒目"));
        assert!(result.body.contains("地支、藏干和当前大运合到一起后"));
        assert!(
            result
                .body
                .contains("五行关系会把这股力量贴近日主的方式说清")
        );
        assert!(result.body.contains("从命理结构看"));
        assert!(result.body.contains("您可以"));
        assert!(!result.body.contains("运运"));
        assert!(!result.body.contains("盘面上先看这几股牵动"));
        assert!(!result.body.contains("主要牵动如下"));
        assert!(!result.body.contains("流年天干把十神主题推到台前"));
        assert!(!result.body.contains("天干把十神主题推到台前"));
        assert!(!result.body.contains("主题怎样被推到台前"));
        assert!(!result.body.contains("拿到台前观察"));
        assert!(!result.body.contains("五行说明这股力量怎样靠近日主"));
        assert!(!result.body.contains("年度本身先露出的"));
        assert!(!result.body.contains("再往下看，地支关系、藏干和当前大运"));
        assert!(!result.body.contains("五行关系继续说明力量怎样靠近"));
        assert!(!result.body.contains("· 先看天干"));
        assert!(!result.body.contains("这一章会把"));
        assert!(!result.body.contains("牵动会先落在这些位置"));
        assert!(!result.body.contains("天干处先露出十神主题"));
        assert!(!result.body.contains("规则版本"));
        assert!(!result.body.contains("score_internal"));
        assert!(!result.body.contains("0-100"));
        assert!(!result.body.contains("必然发财"));
        assert!(!result.body.contains("流月运势"));
        assert!(!result.body.contains("每日运势"));
    }

    #[test]
    fn annual_trigger_uses_year_without_requiring_reading_year() {
        let result =
            generate(&sample_config(), &sample_annual_trigger_year_only_request()).unwrap();

        assert!(result.body.contains("\"annual_trigger_reading\""));
        assert!(
            result
                .body
                .contains("\"capability\":\"annual-trigger-reading\"")
        );
        assert!(result.body.contains("\"status\":\"restricted\""));
        assert!(result.body.contains("\"year\":2026"));
        assert!(result.body.contains("\"current_luck_cycle\""));
        assert!(result.body.contains("annual-trigger"));
        assert!(result.body.contains("major-luck+annual-trigger"));
        assert!(result.body.contains("\"luck_reading\""));
        assert!(result.body.contains("reading_year_not_requested"));
        assert!(!result.body.contains("score_internal"));
    }

    #[test]
    fn annual_trigger_is_not_inferred_without_explicit_year() {
        let result = generate(&sample_config(), &sample_luck_reading_request()).unwrap();

        assert!(result.body.contains("\"annual_trigger_reading\""));
        assert!(
            result
                .body
                .contains("\"capability\":\"annual-trigger-reading\"")
        );
        assert!(result.body.contains("\"status\":\"not_requested\""));
        assert!(result.body.contains("annual_year_not_requested"));
        assert!(result.body.contains("尚未选择年份"));
        assert!(!result.body.contains("2026年年柱丙午"));
        assert!(!result.body.contains("score_internal"));
    }

    #[test]
    fn report_contains_disclaimer() {
        let result = generate(&sample_config(), &sample_request()).unwrap();
        assert!(result.body.contains("仅供传统文化参考"));
        assert!(
            result
                .body
                .contains("不构成医学、法律、财务、婚恋或人生决策建议")
        );
    }

    #[test]
    fn report_passes_forbidden_output_audit() {
        let result = generate(&sample_config(), &sample_request()).unwrap();
        assert!(result.body.contains("\"status\":\"passed\""));
        for forbidden in [
            "diagnosis",
            "guaranteed wealth",
            "death",
            "disease",
            "divorce is certain",
            "前端",
            "后端",
            "DTO",
            "score_internal",
            "error.message",
        ] {
            assert!(
                !result.body.contains(forbidden),
                "main report leaked forbidden public wording: {forbidden}"
            );
        }
    }

    #[test]
    fn report_unknown_hour_is_handled() {
        let req = Request::parse(b"GET /api/charts/report?date=2025-01-01&timezone=Asia/Shanghai&time_precision=unknown&sex=female HTTP/1.1\r\n\r\n").unwrap();
        let result = generate(&sample_config(), &req);
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert!(resp.body.contains("未知（时辰未提供）"));
        assert!(resp.body.contains("信息不完整"));
        assert!(!resp.body.contains("前端"));
        assert!(!resp.body.contains("后端"));
    }
}
