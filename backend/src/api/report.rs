// M27: Zero-baseline colloquial Chinese chart report.
// All text is hard-coded templates. No AI/LLM. No deterministic life claims.
// Design: every chapter follows "what → your result → plain language → how to read it"
use crate::api::charts::build_chart_result;
use crate::config::AppConfig;
use crate::domain::analysis::{AnalysisSnapshot, audit_text, DISCLAIMER_ID};
use crate::domain::bazi::{Sex, Pillar, ChartBasis};
use crate::domain::deep_analysis::{assess_strength, classify_pattern, suggest_useful_god};
use crate::domain::luck::compute_luck_cycles;
use crate::error::AppError;
use crate::http::{Request, Response, json};
use crate::api::chart_basis::chart_request_from_query;
use crate::calendar::lunar_data::LunarDataSource;

pub fn generate(config: &AppConfig, request: &Request) -> Result<Response, AppError> {
    let chart = build_chart_result(config, request)?;
    let snapshot = AnalysisSnapshot::build(&chart);

    let chart_request = chart_request_from_query(request)?;
    let basis = ChartBasis::build(chart_request)?;
    let profile = &basis.request.birth_profile;
    let table = LunarDataSource::new(config.lunar_data_path.clone()).load_table()?;
    let date_layer = table
        .lookup(profile.date)
        .ok_or_else(|| AppError::OutOfRange("date out of range".to_string()))?;

    let year_gan = date_layer.gan_zhi_year.chars().next().unwrap_or('甲').to_string();
    let month_pillar = Pillar::from_ganzhi(&date_layer.gan_zhi_month)?;
    let luck_cycles = compute_luck_cycles(&year_gan, &month_pillar, &profile.sex, 0);
    let start_age = if luck_cycles.is_empty() { 0 } else { luck_cycles[0].start_age };

    let strength = assess_strength(&chart);
    let pattern = classify_pattern(&chart, &strength);
    let useful_gods = suggest_useful_god(&strength, &pattern);

    let blocks = build_blocks(&chart, &snapshot, &luck_cycles, start_age, &strength, &pattern, &useful_gods);

    let assembled = blocks.iter()
        .map(|b| format!("【{}】\n{}", b.title, b.body))
        .collect::<Vec<_>>()
        .join("\n\n");

    let disclaimer = build_disclaimer();
    let full_report = format!("{}\n\n---\n\n{}", disclaimer, assembled);
    let audit = audit_text(&full_report);

    let blocks_json = blocks.iter().map(|b| {
        format!(r#"{{"id":{},"title":{},"body":{}}}"#,
            json::string(&b.id), json::string(&b.title), json::string(&b.body))
    }).collect::<Vec<_>>().join(",");

    let body = format!(
        concat!(
            r#"{{"status":"supported","capability":"chart-report","#,
            r#""algo_version":{},"disclaimer_id":{},"disclaimer":{},"#,
            r#""blocks":[{}],"assembled_report":{},"#,
            r#""forbidden_output_audit":{{"status":{},"checked_patterns":{}}}"#,
            r#"}}"#
        ),
        json::string(snapshot.algo_version),
        json::string(DISCLAIMER_ID),
        json::string(&disclaimer),
        blocks_json,
        json::string(&full_report),
        json::string(audit.status),
        audit.checked_patterns,
    );

    Ok(Response::json(body))
}

struct ReportBlock { id: String, title: String, body: String }

fn build_disclaimer() -> String {
    "本报告基于命轨当前排盘与结构分析结果自动生成，仅供传统文化参考。你可以把它当作一份关于性格倾向、做事节奏、关系模式和阶段主题的阅读材料，而不是现实结论。\n\n报告中的「强」「弱」「格局」「用神」「大运」等词，都是传统命理里的分析语言，不代表好坏、高低、成败，也不构成医学、法律、财务、婚恋或人生决策建议。现实中的重要选择，仍应结合实际状况、个人判断和专业意见。".into()
}

fn build_blocks(
    chart: &crate::domain::bazi::ChartResult,
    snapshot: &AnalysisSnapshot,
    luck_cycles: &[crate::domain::luck::LuckCycle],
    start_age: u8,
    strength: &crate::domain::deep_analysis::StrengthAssessment,
    pattern: &crate::domain::deep_analysis::PatternInfo,
    useful_gods: &[crate::domain::deep_analysis::UsefulGodHint],
) -> Vec<ReportBlock> {
    let day_stem = &chart.chart.day.stem;
    let day_el = stem_element_cn(day_stem);
    let yin_yang = if ["甲","丙","戊","庚","壬"].contains(&day_stem.as_str()) { "阳" } else { "阴" };

    vec![
        block_overview(chart),
        block_day_master(day_stem, day_el, yin_yang),
        block_elements(&snapshot.element_metrics),
        block_ten_gods(&snapshot.ten_god_metrics, day_stem),
        block_hidden_stems(chart),
        block_strength(day_stem, day_el, &chart.chart.month.branch, strength),
        block_pattern(pattern),
        block_useful_god(useful_gods),
        block_luck(luck_cycles, start_age),
    ]
}

// ── Chapter 1: 命盘概览 ──
fn block_overview(chart: &crate::domain::bazi::ChartResult) -> ReportBlock {
    let profile = &chart.basis.request.birth_profile;
    let hour_str = chart.chart.hour.as_ref()
        .map(|h| h.ganzhi())
        .unwrap_or_else(|| "未知（时辰未提供）".to_string());

    let unknown_note = if chart.chart.hour.is_none() {
        "\n\n因为当前时辰没有完整提供，时柱部分会显示为未知或候选状态。命轨不会在前端或报告中强行补推时辰，因此这一部分需要按「信息不完整」来阅读。"
    } else { "" };

    ReportBlock {
        id: "chart-overview".into(),
        title: "命盘概览".into(),
        body: format!(
            "这一章先把你的命盘拆成四组时间标签。传统八字里，会把出生时间分成四个位置：年柱、月柱、日柱、时柱。你可以先把它们理解成一张命盘的四个坐标，而不是四个单独下结论的符号。\n\n\
            你的出生时间为公历 {}年{}月{}日 {}时，对应的四柱为：年柱「{}」、月柱「{}」、日柱「{}」、时柱「{}」。\n\n\
            其中，日柱通常会被当作观察「自己」的核心位置。后面提到的日主、十神、强弱和格局，都会围绕日柱展开。也就是说，这一章不是直接判断命好命坏，而是先把命盘的基础坐标摆出来，方便后面逐步解释。{}",
            profile.date.year, profile.date.month, profile.date.day,
            profile.time.as_ref().map(|t| format!("{}:{}", t.hour, t.minute)).unwrap_or_else(|| "未知".to_string()),
            chart.chart.year.ganzhi(), chart.chart.month.ganzhi(), chart.chart.day.ganzhi(), hour_str,
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
            "这一章看的是「日主」。日主可以简单理解为命盘里代表「自己」的那个符号。它不是你性格的全部，也不是单独拿来下结论的标签，但它是后面分析的中心点。\n\n\
            你的日主是「{}」，五行属{}，为{}性。\n\n\
            换成白话说，日主就像命盘里的「主视角」。后面看五行、十神、强弱和格局时，都是在看其他符号如何影响这个「自己」。\n\n\
            {}\n\n\
            需要注意的是，日主只是一个观察起点。它可以帮助你理解自己更容易用哪种方式面对世界，但不能单独决定一个人的性格、能力或人生结果。",
            day_stem, day_el, yin_yang, desc
        ),
    }
}

// ── Chapter 3: 五行分布 ──
fn block_elements(metrics: &[crate::domain::analysis::WeightedMetric]) -> ReportBlock {
    let name: std::collections::HashMap<&str, &str> = [
        ("wood","木"),("fire","火"),("earth","土"),("metal","金"),("water","水"),
    ].into_iter().collect();

    let mut parts = Vec::new();
    let mut plain_parts = Vec::new();
    for m in metrics {
        let cn = name.get(m.id).unwrap_or(&m.id);
        let (level, plain) = element_level_plain(m.weight_x2);
        parts.push(format!("{}行{}", cn, level));
        plain_parts.push(format!("{}：{}", cn, plain));
    }

    let summary = parts.join("、");
    let plain = plain_parts.join("；");

    ReportBlock {
        id: "element-distribution".into(),
        title: "五行分布".into(),
        body: format!(
            "这一章看的是命盘里的「五种能量比例」。你可以先不用把五行想得太玄，它更像是在描述一个人处理事情时常用的几种方式。\n\n\
            木，像成长、规划、主动开拓；\n火，像表达、热情、反应速度和被看见；\n土，像稳定、承接、现实感和耐心；\n金，像规则、判断、边界和执行力；\n水，像思考、学习、流动性和适应变化。\n\n\
            从当前命盘看，五行结果为：{}。\n\n\
            换成白话说，这里不是在判断你「好不好」，而是在看哪类倾向更容易被命盘调动出来。{}\n\n\
            所以，这一章最适合当作「能量结构说明」来看。它不是能力清单，也不是缺点清单。偏弱不代表没有，偏强也不代表一定更好，关键在于不同能量之间能不能形成合适的配合。",
            summary, plain
        ),
    }
}

fn element_level_plain(weight: u16) -> (&'static str, &'static str) {
    match weight {
        0 => ("缺", "这类符号在当前结构里没有直接出现。这里的「缺」不是现实能力缺失，也不是坏事，只是说明它不是命盘里最直接呈现出来的力量"),
        1..=2 => ("偏弱", "偏弱表示这类倾向在命盘里不算特别显眼。它不是说你现实中没有这方面能力，而是说这部分更需要靠后天训练、环境支持或长期习惯来慢慢调动"),
        3..=5 => ("中和", "这类倾向在命盘里相对平衡。它不一定特别突出，但也不算明显不足，通常比较容易在合适场景中自然发挥"),
        _ => ("偏强", "相对偏强，说明和它相关的倾向比较容易被看见。但偏强不等于一定好，也需要学会收放，避免某一种方式过度占据主导"),
    }
}

// ── Chapter 4: 十神关系 ──
fn block_ten_gods(metrics: &[crate::domain::analysis::WeightedMetric], day_stem: &str) -> ReportBlock {
    let name: std::collections::HashMap<&str, &str> = [
        ("peer","比肩"),("rob_wealth","劫财"),("eating_god","食神"),("hurting_officer","伤官"),
        ("direct_wealth","正财"),("indirect_wealth","偏财"),("direct_officer","正官"),
        ("seven_killings","七杀"),("direct_resource","正印"),("indirect_resource","偏印"),
    ].into_iter().collect();

    let active: Vec<String> = metrics.iter()
        .filter(|m| m.weight_x2 > 0)
        .map(|m| {
            let cn = name.get(m.id).unwrap_or(&m.id);
            let desc = ten_god_plain(m.id);
            format!("「{}」{}\n{}", cn, if m.weight_x2 >= 3 { "比较明显" } else { "存在" }, desc)
        }).collect();

    let summary = metrics.iter()
        .filter(|m| m.weight_x2 > 0)
        .map(|m| format!("{}({}分)", name.get(m.id).unwrap_or(&m.id), m.weight_x2))
        .collect::<Vec<_>>()
        .join("、");

    let plain = if active.is_empty() {
        "十神分布较均衡，没有特别突出的模式。".into()
    } else {
        active.join("\n\n")
    };

    ReportBlock {
        id: "ten-god-relations".into(),
        title: "十神关系".into(),
        body: format!(
            "这一章看的是「十神关系」。十神不是神仙，也不是吉凶标签。它是传统命理里用来描述「自己和外界如何发生关系」的一套分类。\n\n\
            你可以先这样理解：\n\
            比肩、劫财：和自我、同辈、竞争、合作有关；\n\
            食神、伤官：和表达、才华、输出、想法有关；\n\
            正财、偏财：和资源、现实事务、经营意识有关；\n\
            正官、七杀：和规则、责任、压力、目标感有关；\n\
            正印、偏印：和学习、理解、支持系统、保护感有关。\n\n\
            以你的日主「{}」为中心，当前命盘中比较明显的十神为：{}。\n\n\
            换成白话说，十神不是在判断你一定会遇到什么人或发生什么事，而是在描述你更容易在哪些关系模式里用力。\n\n\
            {}\n\n\
            所以这一章最适合当作「关系模式说明」来看。它不是职业断语，不是财富断语，也不是人际关系结论，只是帮你看清命盘里哪些互动方式比较容易被触发。",
            day_stem, summary, plain
        ),
    }
}

fn ten_god_plain(id: &str) -> &'static str {
    match id {
        "peer" => "比肩和「自我、同类、同辈、伙伴」有关。它明显时，通常表示一个人更重视自己的立场，也容易在同伴关系、竞争关系或合作关系中找到存在感。",
        "rob_wealth" => "劫财和「竞争、资源分配、行动冲劲」有关。它明显时，可能表示更容易遇到需要争取、协调或重新分配资源的场景。",
        "eating_god" => "食神和「自然表达、技能输出、舒服地发挥」有关。它明显时，通常表示一个人需要通过表达、创造、技能或作品来释放自己。",
        "hurting_officer" => "伤官和「想法突破、表达锋芒、不满足于规则」有关。它明显时，代表表达欲、创造欲或质疑精神更容易被激发，但也要注意表达方式和边界。",
        "direct_wealth" => "正财和「稳定资源、现实安排、可管理的事务」有关。它明显时，通常表示一个人更容易关注具体结果、稳定收益、责任安排和现实秩序。",
        "indirect_wealth" => "偏财和「机会、流动资源、经营意识」有关。它明显时，可能更容易关注变化中的机会、外部资源和灵活经营。",
        "direct_officer" => "正官和「规则、责任、标准、秩序感」有关。它明显时，通常表示责任意识、规则意识或对稳定路径的重视会更强。",
        "seven_killings" => "七杀和「压力、挑战、目标、行动力」有关。它不是坏词，也不是危险判断。它更像一种外部压力或挑战感，能推动人行动，但也需要被合适地管理。",
        "direct_resource" => "正印和「学习、保护、支持、理解力」有关。它明显时，通常表示一个人更需要知识、系统、长辈支持或稳定的安全感。",
        "indirect_resource" => "偏印和「非标准理解、独立思考、特殊兴趣」有关。它明显时，可能表示一个人更容易从不常规的角度理解问题，也可能有较强的独处和思考需求。",
        _ => "",
    }
}

// ── Chapter 5: 地支藏干 ──
fn block_hidden_stems(chart: &crate::domain::bazi::ChartResult) -> ReportBlock {
    let branches = [
        ("年支", &chart.chart.year.branch),
        ("月支", &chart.chart.month.branch),
        ("日支", &chart.chart.day.branch),
    ];
    let details: Vec<String> = branches.iter().map(|(label, branch)| {
        let stems = hidden_stems_for_branch(branch);
        format!("{}「{}」藏{}", label, branch, stems)
    }).collect();

    let hour_detail = chart.chart.hour.as_ref()
        .map(|h| format!("；时支「{}」藏{}", h.branch, hidden_stems_for_branch(&h.branch)))
        .unwrap_or_default();

    ReportBlock {
        id: "hidden-stems".into(),
        title: "地支藏干".into(),
        body: format!(
            "这一章看的是命盘里「不太直接显露出来的部分」。\n\n\
            如果说天干像一个人外在比较容易看到的表现，那么地支藏干就像藏在性格深处、环境里、习惯里的潜在倾向。它不一定每天都表现出来，但在某些情境下，比如压力、机会、关系变化或环境变化时，可能会被触发。\n\n\
            当前地支藏干结果为：{}{}。\n\n\
            换成白话说，你可以把藏干理解成命盘里的「隐藏按钮」。有些倾向平时不明显，但当外部条件合适时，它们会影响你的反应方式、关注点和做事节奏。\n\n\
            这些符号不需要逐个死记。你只要知道：藏干代表的是命盘里更深层、更隐性的倾向，它们不像天干那样直接站在表面，但会在合适条件下参与影响。这部分不适合单独拿来下结论，它更像是补充信息。",
            details.join("；"), hour_detail
        ),
    }
}

fn hidden_stems_for_branch(branch: &str) -> String {
    let stems: &[&str] = match branch {
        "子" => &["癸"], "丑" => &["己","癸","辛"], "寅" => &["甲","丙","戊"],
        "卯" => &["乙"], "辰" => &["戊","乙","癸"], "巳" => &["丙","戊","庚"],
        "午" => &["丁","己"], "未" => &["己","丁","乙"], "申" => &["庚","壬","戊"],
        "酉" => &["辛"], "戌" => &["戊","辛","丁"], "亥" => &["壬","甲"], _ => &[],
    };
    stems.join("、")
}

// ── Chapter 6: 日主强弱 ──
fn block_strength(day_stem: &str, _day_el: &str, month_branch: &str, strength: &crate::domain::deep_analysis::StrengthAssessment) -> ReportBlock {
    let deling_plain = if strength.deling { "是（月令生扶日主，季节环境帮你）" } else { "否（月令不生扶日主，季节环境不太帮得上）" };
    let level_plain = strength_level_plain(strength.level);

    ReportBlock {
        id: "day-master-strength".into(),
        title: "日主强弱".into(),
        body: format!(
            "这一章看的是「日主强弱」。这里的强弱不是现实中的能力强弱，也不是说一个人厉不厉害，更不是好坏判断。它只是看：命盘里代表「自己」的日主，在这张结构中得到多少支持。\n\n\
            你的日主「{}」生于{}月。按照当前算法，月令对日主的支持为：{}；地支中的支持数量为：{}（得地）；天干中的支持数量为：{}（得势）。综合评分为 {}/10，当前判定为「{}」。\n\n\
            得令：出生月份是否支持日主。可以理解为「季节环境是否帮你」。\n\
            得地：地支里是否有支持日主的力量。可以理解为「底层环境是否给你支撑」。\n\
            得势：天干里是否有支持日主的力量。可以理解为「表层力量是否帮你发力」。\n\n\
            换成白话说，这个评分是在看「自己在命盘里是更容易独立发力，还是更需要环境、资源和节奏来支持」。{}\n\n\
            所以，看到「偏弱」不需要紧张，看到「偏强」也不代表一定更好。强弱只是结构状态，不是人生结论。真正重要的是：你适合怎样的节奏，应该怎样借力，怎样避免过度消耗或过度用力。",
            day_stem, month_branch, deling_plain, strength.dedi, strength.deshi, strength.score, strength.level, level_plain
        ),
    }
}

fn strength_level_plain(level: &str) -> &'static str {
    match level {
        "极弱" => "日主极弱，表示命盘里的「自己」得到的支持较少。白话说，这类结构更不适合长期硬扛，适合先建立稳定节奏、找到外部支持，再逐步发力。",
        "偏弱" => "日主偏弱，表示命盘里的「自己」需要更多资源、环境或节奏配合。白话说，不一定是能力不足，而是更适合借力成长、循序渐进，不适合长期把所有压力都压在自己身上。",
        "中和" => "日主中和，表示命盘里的「自己」和周围力量相对平衡。白话说，这类结构通常更容易根据环境调整自己，既能发力，也需要保持节奏。",
        "偏强" => "日主偏强，表示命盘里的「自己」力量比较明显。白话说，主见、推动力或自我意识更容易被看见，但也要注意不要过度用力，学会协作和收放。",
        "极强" => "日主极强，表示命盘里的「自己」力量非常突出。白话说，这类结构往往有较强的推动力和坚持度，但也更需要注意弹性、倾听和转换方式，避免一直用同一种力量推进所有事情。",
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
            "这一章看的是「格局」。格局可以理解为命盘的主要运行方式，不是等级，也不是人生上限。它更像是在回答一个问题：这张命盘最主要的动力、矛盾和用力方向在哪里？\n\n\
            按照当前算法，本命盘的格局初判为「{}」，类型为「{}」，置信度为「{}」。这里的置信度是算法对当前结构匹配程度的描述，不是对人生结果的确定预测。\n\n\
            换成白话说，{}\n\n\
            所以，这一章适合用来理解命盘的「主题」，而不是用来判断高低。格局不是命运标签，它只是帮助你看见：自己更容易在哪类问题上反复用力，哪类能力更值得被整理出来。",
            pattern.pattern_name, pattern.pattern_type, pattern.confidence, desc
        ),
    }
}

fn pattern_plain(name: &str) -> &'static str {
    match name {
        "正官格" => "正官格可以理解为「规则、责任、标准感」比较重要的结构。白话说，这类命盘更容易在秩序、职责、规范、长期稳定的事情里找到方向。但它不是说一定适合某个职业，而是说明规则意识和责任主题比较值得关注。",
        "七杀格" => "七杀格可以理解为「压力、挑战、行动力」比较强的结构。白话说，这类命盘容易被目标、竞争或外部要求推动，但关键是要把压力转化成有秩序的行动，而不是被压力牵着走。",
        "财格" => "财格可以理解为「现实资源、经营意识、落地能力」比较重要的结构。白话说，这类命盘更容易关注实际结果、资源安排和现实交换，但这不是财富承诺，只是说明现实事务是重要主题。",
        "印格" => "印格可以理解为「学习、理解、支持系统」比较重要的结构。白话说，这类命盘更需要知识、经验、稳定支持和内在安全感来帮助自己成长。",
        "食伤格" => "食伤格可以理解为「表达、输出、技能、创造」比较重要的结构。白话说，这类命盘需要把想法、能力或经验表达出来，越能形成稳定输出，越容易找到自己的节奏。",
        "建禄格" => "建禄格可以理解为「自我根基和自主能力」比较明显的结构。白话说，这类命盘更需要建立自己的稳定基础，不太适合长期完全依赖外部安排。",
        "月刃格" => "月刃格可以理解为「自我力量、冲劲和边界感」比较强的结构。白话说，这类命盘更容易有强烈主张，但也更需要学习合作、缓冲和转弯。",
        _ => "当前格局不适合用单一标签概括。白话说，这张命盘可能不是某一种主题特别突出，而是需要结合五行、十神和日主强弱一起看。",
    }
}

// ── Chapter 8: 用神参考 ──
fn block_useful_god(gods: &[crate::domain::deep_analysis::UsefulGodHint]) -> ReportBlock {
    let hints: Vec<String> = gods.iter()
        .map(|h| {
            let el_cn = match h.element {
                "印星(生我)" | "印星(化杀)" => "印星（代表学习、保护、支持）",
                "比劫(同我)" => "比劫（代表自我力量、同伴、行动力）",
                "食伤(泄秀)" => "食伤（代表表达、输出、创造力）",
                "财星(耗身)" => "财星（代表资源、现实、经营意识）",
                "通关用神" => "通关五行（代表调节、平衡、流动）",
                other => other,
            };
            format!("第{}优先：{}——{}", h.priority, el_cn, h.reason)
        }).collect();

    ReportBlock {
        id: "useful-god-hints".into(),
        title: "用神参考".into(),
        body: format!(
            "这一章看的是「用神参考」。用神可以简单理解为：在这张命盘里，哪类能量更像「调节器」。它不是命令，也不是说你现实中必须选择某个行业、颜色、方位、物品或生活方式。它只是传统命理里用来观察平衡方向的工具。\n\n\
            根据当前日主强弱和格局分析，系统给出的用神参考为：\n{}\n\n\
            换成白话说，被列为参考方向的力量，更像是提醒你在做事节奏、学习方式、环境选择和长期习惯上，哪类倾向可能更有帮助。\n\n\
            你可以把这一章当作「调节方向」来看。它更适合用来提醒自己关注某一类力量的发挥，而不是被它绑住手脚。真正重要的选择，仍然需要结合实际条件、个人能力和现实反馈。",
            hints.join("\n")
        ),
    }
}

// ── Chapter 9: 大运走势 ──
// ── Chapter 9: 大运走势 ──
fn block_luck(cycles: &[crate::domain::luck::LuckCycle], start_age: u8) -> ReportBlock {
    let parts: Vec<String> = cycles.iter()
        .map(|c| format!("「{}」运（{}—{}岁）", c.pillar.ganzhi(), c.start_age, c.end_age))
        .collect();

    ReportBlock {
        id: "luck-cycles".into(),
        title: "大运走势".into(),
        body: format!(
            "这一章看的是「大运」。大运可以理解为命盘里的「阶段背景」，不是十年预言。你可以把它想成天气：天气会影响一个人出门时的感受和准备方式，但不会替你决定人生。大运也是类似，它只是在传统命理里描述某个阶段更容易被哪类主题影响。\n\n\
            约 {} 岁起运。当前可计算的大运阶段为：{}。\n\n\
            换成白话说，这些阶段不是在告诉你某十年一定会发生什么，而是在提示：不同年龄段里，命盘关注的主题会有所变化。有的阶段更强调学习和积累，有的阶段更强调责任和压力，有的阶段更强调表达、资源或行动。\n\n\
            所以，大运最适合当作「阶段气候」来看。它可以帮助你理解不同时期的主题变化，但不能替代现实选择，也不能直接推出确定事件。",
            start_age, parts.join("；")
        ),
    }
}

// ── Day stem plain descriptions ──
fn describe_day_stem_full(stem: &str) -> &'static str {
    match stem {
        "甲" => "如果用生活化的比喻，甲木像一棵向上生长的大树。它强调成长、方向感、原则和支撑力。甲木倾向于希望事情有清晰的目标，也比较重视长期积累。\n\n但这不是说你一定强势或固执，只是说明「规划、成长、立起来」这类主题，在观察你命盘时会比较重要。",
        "乙" => "如果用生活化的比喻，乙木像花草藤蔓。它不一定用很强硬的方式推进事情，但往往更重视弹性、适应、审美和细腻的连接。\n\n这不是说你一定柔弱，而是说明你更适合从灵活调整、关系协调和持续生长中找到自己的节奏。",
        "丙" => "如果用生活化的比喻，丙火像太阳。它代表外放、热度、表达、感染力和被看见的需求。\n\n这不是说你一定外向，而是说明在你的命盘解读里，「表达自己、照亮别人、把热度释放出来」会是值得关注的主题。",
        "丁" => "如果用生活化的比喻，丁火像灯火、烛光。它不一定猛烈，但更细腻、更专注，也更容易体现为观察力、感受力和内在明亮感。\n\n这不是说你一定敏感，而是说明你的力量可能不是靠大声表现，而是靠持续照亮某个方向。",
        "戊" => "如果用生活化的比喻，戊土像山、城墙和厚土。它代表稳定、承载、责任感和现实支撑。\n\n这不是说你一定保守，而是说明你的命盘里，「稳住局面、承担事情、建立基础」这类主题会比较重要。",
        "己" => "如果用生活化的比喻，己土像田园之土。它更强调滋养、包容、整理、消化和慢慢培育。\n\n这不是说你一定慢热，而是说明你适合通过耐心、照顾细节和持续经营，把事情一点点养成。",
        "庚" => "如果用生活化的比喻，庚金像矿石、刀剑或未经雕琢的金属。它强调判断、规则、决断、执行和抗压。\n\n这不是说你一定冷硬，也不是说你必须强势。它只是说明，在你的命盘里，「明确边界、做出判断、把事情切开处理」这类能力会是重要主题。",
        "辛" => "如果用生活化的比喻，辛金像首饰、玉器旁的精细金属。它更重视质感、分寸、审美、精确和细节。\n\n这不是说你一定挑剔，而是说明你对品质、边界和精细度可能比较敏感，适合在需要打磨和判断的事情中发挥优势。",
        "壬" => "如果用生活化的比喻，壬水像江河大海。它代表流动、信息、视野、学习和变化中的适应力。\n\n这不是说你一定漂浮不定，而是说明你的命盘里，「看大局、吸收信息、顺势调整」会是值得关注的能力。",
        "癸" => "如果用生活化的比喻，癸水像雨露、雾气和地下水。它更细腻、更内敛，也更强调感受、理解、渗透和长期积累。\n\n这不是说你一定脆弱，而是说明你的力量可能不在表面，而在持续观察、慢慢吸收和细致理解。",
        _ => "日主的特质需要结合具体天干进一步分析。",
    }
}

fn stem_element_cn(stem: &str) -> &'static str {
    match stem {
        "甲"|"乙" => "木", "丙"|"丁" => "火", "戊"|"己" => "土",
        "庚"|"辛" => "金", "壬"|"癸" => "水", _ => "土",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::AppConfig;

    fn sample_config() -> AppConfig {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap_or_else(|| std::path::Path::new("."))
            .join("data").join("raw").join("lunar_data.yaml");
        AppConfig { addr: "127.0.0.1:8787".into(), lunar_data_path: path }
    }

    fn sample_request() -> Request {
        Request::parse(b"GET /api/charts/report?date=2025-01-01&timezone=Asia/Shanghai&time_precision=exact&time=10:30&sex=male HTTP/1.1\r\n\r\n").unwrap()
    }

    #[test]
    fn report_generates_without_error() {
        let result = generate(&sample_config(), &sample_request());
        assert!(result.is_ok());
        let resp = result.unwrap();
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
    }

    #[test]
    fn report_contains_disclaimer() {
        let result = generate(&sample_config(), &sample_request()).unwrap();
        assert!(result.body.contains("仅供传统文化参考"));
        assert!(result.body.contains("不构成医学、法律、财务、婚恋或人生决策建议"));
    }

    #[test]
    fn report_passes_forbidden_output_audit() {
        let result = generate(&sample_config(), &sample_request()).unwrap();
        assert!(result.body.contains("\"status\":\"passed\""));
        assert!(!result.body.contains("diagnosis"));
        assert!(!result.body.contains("guaranteed wealth"));
        assert!(!result.body.contains("death"));
        assert!(!result.body.contains("disease"));
        assert!(!result.body.contains("divorce is certain"));
    }

    #[test]
    fn report_unknown_hour_is_handled() {
        let req = Request::parse(b"GET /api/charts/report?date=2025-01-01&timezone=Asia/Shanghai&time_precision=unknown&sex=female HTTP/1.1\r\n\r\n").unwrap();
        let result = generate(&sample_config(), &req);
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert!(resp.body.contains("未知（时辰未提供）"));
        assert!(resp.body.contains("信息不完整"));
    }
}
