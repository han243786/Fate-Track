// M24: Colloquial Chinese chart report.
// All text is hard-coded templates. No AI/LLM. No deterministic life claims.
use crate::api::charts::build_chart_result;
use crate::config::AppConfig;
use crate::domain::analysis::{AnalysisSnapshot, audit_text, DISCLAIMER_ID};
use crate::domain::bazi::{Sex, Pillar};
use crate::domain::deep_analysis::{assess_strength, classify_pattern, suggest_useful_god};
use crate::domain::luck::compute_luck_cycles;
use crate::error::AppError;
use crate::http::{Request, Response, json};
use crate::api::chart_basis::chart_request_from_query;
use crate::domain::bazi::ChartBasis;
use crate::calendar::lunar_data::LunarDataSource;

pub fn generate(config: &AppConfig, request: &Request) -> Result<Response, AppError> {
    // 1. Build chart
    let chart = build_chart_result(config, request)?;

    // 2. Build analysis
    let snapshot = AnalysisSnapshot::build(&chart);

    // 3. Build luck cycles (simplified: re-parse chart params)
    let chart_request = chart_request_from_query(request)?;
    let basis = ChartBasis::build(chart_request)?;
    let profile = &basis.request.birth_profile;
    let table = LunarDataSource::new(config.lunar_data_path.clone()).load_table()?;
    let date_layer = table
        .lookup(profile.date)
        .ok_or_else(|| AppError::OutOfRange("date out of range".to_string()))?;

    let year_gan = date_layer.gan_zhi_year.chars().next().unwrap_or('甲').to_string();
    let month_pillar = Pillar::from_ganzhi(&date_layer.gan_zhi_month)?;

    // Compute days_to_jie using astronomy engine
    use crate::astronomy::terms::solar_terms_for_year;
    let terms = solar_terms_for_year(profile.date.year);
    let doy = profile.date.day_of_year() as u16;

    let is_forward = {
        let yang = ["甲", "丙", "戊", "庚", "壬"].contains(&year_gan.as_str());
        let male = matches!(profile.sex, Sex::Male);
        (yang && male) || (!yang && !male)
    };

    let days_to_jie = if is_forward {
        let jie_indices = [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22];
        let mut next_doy = 366u16;
        for &ji in &jie_indices {
            if let Some(term) = terms.get(ji) {
                let td = (term.jd_tt
                    - crate::astronomy::time::gregorian_to_jd(profile.date.year, 1, 1, 0.0).whole()) as u16 + 1;
                if td > doy && td < next_doy { next_doy = td; }
            }
        }
        if next_doy == 366 {
            let next_terms = solar_terms_for_year(profile.date.year + 1);
            if let Some(t) = next_terms.first() {
                next_doy = (t.jd_tt
                    - crate::astronomy::time::gregorian_to_jd(profile.date.year + 1, 1, 1, 0.0).whole()) as u16 + 365;
            }
        }
        next_doy.saturating_sub(doy)
    } else {
        let jie_indices = [0, 2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22];
        let mut prev_doy = 0u16;
        for &ji in &jie_indices {
            if let Some(term) = terms.get(ji) {
                let td = (term.jd_tt
                    - crate::astronomy::time::gregorian_to_jd(profile.date.year, 1, 1, 0.0).whole()) as u16 + 1;
                if td <= doy && td > prev_doy { prev_doy = td; }
            }
        }
        if prev_doy == 0 {
            let prev_terms = solar_terms_for_year(profile.date.year - 1);
            if let Some(t) = prev_terms.get(11) {
                let td = (t.jd_tt
                    - crate::astronomy::time::gregorian_to_jd(profile.date.year - 1, 1, 1, 0.0).whole()) as u16 + 1;
                prev_doy = td;
            }
            doy + 365 - prev_doy
        } else {
            doy - prev_doy
        }
    };

    let luck_cycles = compute_luck_cycles(&year_gan, &month_pillar, &profile.sex, days_to_jie);
    let start_age = if luck_cycles.is_empty() { 0 } else { luck_cycles[0].start_age };

    // 4. Deep analysis
    let strength = assess_strength(&chart);
    let pattern = classify_pattern(&chart, &strength);
    let useful_gods = suggest_useful_god(&strength, &pattern);

    // 5. Build text blocks
    let disclaimer = format!(
        "本报告基于传统命理学算法（{}）自动生成，仅供文化参考，不构成任何专业建议。命理推断有其局限性，请理性看待。",
        snapshot.algo_version
    );

    let blocks = build_blocks(
        &chart,
        &snapshot,
        &luck_cycles,
        start_age,
        is_forward,
        &strength,
        &pattern,
        &useful_gods,
    );

    // 6. Audit assembled report
    let assembled = blocks.iter()
        .map(|b| format!("【{}】\n{}", b.title, b.body))
        .collect::<Vec<_>>()
        .join("\n\n");
    let full_report = format!("{}\n\n---\n\n{}", disclaimer, assembled);
    let audit = audit_text(&full_report);

    let blocks_json = blocks.iter().map(|b| {
        format!(
            r#"{{"id":{},"title":{},"body":{}}}"#,
            json::string(&b.id),
            json::string(&b.title),
            json::string(&b.body),
        )
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

struct ReportBlock {
    id: String,
    title: String,
    body: String,
}

fn build_blocks(
    chart: &crate::domain::bazi::ChartResult,
    snapshot: &AnalysisSnapshot,
    luck_cycles: &[crate::domain::luck::LuckCycle],
    start_age: u8,
    is_forward: bool,
    strength: &crate::domain::deep_analysis::StrengthAssessment,
    pattern: &crate::domain::deep_analysis::PatternInfo,
    useful_gods: &[crate::domain::deep_analysis::UsefulGodHint],
) -> Vec<ReportBlock> {
    let day_stem = &chart.chart.day.stem;
    let day_el = stem_element_name(day_stem);
    let yin_yang = if ["甲","丙","戊","庚","壬"].contains(&day_stem.as_str()) { "阳" } else { "阴" };

    let mut blocks = Vec::new();

    // Block 1: 命盘概览 — chart pillars
    {
        let hour_str = chart.chart.hour.as_ref()
            .map(|h| h.ganzhi())
            .unwrap_or_else(|| "未知（时辰未提供）".to_string());
        blocks.push(ReportBlock {
            id: "chart-overview".into(),
            title: "命盘概览".into(),
            body: format!(
                "你的出生时间为公历 {}年{}月{}日 {}时，对应的八字四柱为：年柱「{}」、月柱「{}」、日柱「{}」、时柱「{}」。",
                chart.basis.request.birth_profile.date.year,
                chart.basis.request.birth_profile.date.month,
                chart.basis.request.birth_profile.date.day,
                chart.basis.request.birth_profile.time.as_ref()
                    .map(|t| format!("{}:{}", t.hour, t.minute))
                    .unwrap_or_else(|| "未知".to_string()),
                chart.chart.year.ganzhi(),
                chart.chart.month.ganzhi(),
                chart.chart.day.ganzhi(),
                hour_str,
            ),
        });
    }

    // Block 2: 日主介绍
    {
        let day_desc = describe_day_stem(day_stem);
        blocks.push(ReportBlock {
            id: "day-master-intro".into(),
            title: "日主介绍".into(),
            body: format!(
                "你的日主是「{}」，五行属{}，为{}性。{}。日主代表你自己，是整个命盘的核心，所有十神关系和五行分析都围绕日主展开。",
                day_stem, day_el, yin_yang, day_desc
            ),
        });
    }

    // Block 3: 五行分布
    {
        let elements = describe_elements(&snapshot.element_metrics);
        blocks.push(ReportBlock {
            id: "element-distribution".into(),
            title: "五行分布".into(),
            body: format!(
                "命盘中五行分布如下：{}。五行是传统命理的基础框架，反映了天性与环境的互动倾向。",
                elements
            ),
        });
    }

    // Block 4: 十神关系
    {
        let ten_gods = describe_ten_gods(&snapshot.ten_god_metrics, day_stem);
        blocks.push(ReportBlock {
            id: "ten-god-relations".into(),
            title: "十神关系".into(),
            body: format!(
                "以日主「{}」为中心，各天干与日主的关系（即十神）如下：{}。十神反映了人际关系、事业、财富等方面的倾向，是传统命理分析的重要维度。",
                day_stem, ten_gods
            ),
        });
    }

    // Block 5: 藏干透出
    {
        let hidden = describe_hidden_stems(&snapshot.hidden_stem_metrics, &chart.chart);
        blocks.push(ReportBlock {
            id: "hidden-stems".into(),
            title: "地支藏干".into(),
            body: format!(
                "地支中隐藏的天干称为藏干，代表了潜在的气质与能量。{}。藏干在特定条件下会显现出来，影响命局的细微变化。",
                hidden
            ),
        });
    }

    // Block 6: 日主强弱
    {
        let el_cn = stem_element_cn(day_stem);
        blocks.push(ReportBlock {
            id: "day-master-strength".into(),
            title: "日主强弱".into(),
            body: format!(
                "日主「{}」属{}，生于{}月（{}季），得令{}，四柱地支中得地{}支，天干中得势{}干。综合评分{}/10，判定为「{}」。{}",
                day_stem, el_cn,
                chart.chart.month.branch,
                month_season_cn(&chart.chart.month.branch),
                if strength.deling { "是（月令生扶日主）" } else { "否（月令不生扶日主）" },
                strength.dedi,
                strength.deshi,
                strength.score,
                strength.level,
                strength_advice(strength.level, day_stem, el_cn)
            ),
        });
    }

    // Block 7: 格局初判
    {
        blocks.push(ReportBlock {
            id: "pattern-classification".into(),
            title: "格局初判".into(),
            body: format!(
                "以月令为主、日主为核心的格局推演结果为「{}」（{}，置信度：{}）。{}",
                pattern.pattern_name,
                pattern.pattern_type,
                pattern.confidence,
                pattern_description(&pattern.pattern_name)
            ),
        });
    }

    // Block 8: 用神参考
    {
        let hints: Vec<String> = useful_gods.iter()
            .map(|h| format!("第{}优先：{}——{}", h.priority, h.element, h.reason))
            .collect();
        blocks.push(ReportBlock {
            id: "useful-god-hints".into(),
            title: "用神参考".into(),
            body: format!(
                "基于日主强弱和格局分析，用神建议如下：{}。需要说明的是，用神为传统命理概念，反映五行的调候和平衡思路，并非对现实决策的指令。命理分析有其文化和哲学价值，但生活中的重要选择应结合实际情况和个人判断。",
                hints.join("；")
            ),
        });
    }

    // Block 9: 大运走势
    {
        let luck_desc = describe_luck_cycles(luck_cycles, start_age, is_forward);
        blocks.push(ReportBlock {
            id: "luck-cycles".into(),
            title: "大运走势".into(),
            body: format!(
                "大运十年一换，反映人生不同阶段的运势基调。本命盘大运{}行，约{}岁起运。{}",
                if is_forward { "顺" } else { "逆" },
                start_age,
                luck_desc
            ),
        });
    }

    blocks
}

fn stem_element_name(stem: &str) -> &'static str {
    match stem {
        "甲"|"乙" => "木", "丙"|"丁" => "火", "戊"|"己" => "土",
        "庚"|"辛" => "金", "壬"|"癸" => "水", _ => "土",
    }
}

fn stem_element_cn(stem: &str) -> &'static str {
    match stem {
        "甲"|"乙" => "木", "丙"|"丁" => "火", "戊"|"己" => "土",
        "庚"|"辛" => "金", "壬"|"癸" => "水", _ => "土",
    }
}

fn month_season_cn(branch: &str) -> &'static str {
    match branch {
        "寅"|"卯"|"辰" => "春", "巳"|"午"|"未" => "夏",
        "申"|"酉"|"戌" => "秋", "亥"|"子"|"丑" => "冬",
        _ => "未知",
    }
}

fn describe_day_stem(stem: &str) -> String {
    match stem {
        "甲" => "甲木为参天大树，正直挺拔，有领导力，但有时过于刚直".into(),
        "乙" => "乙木为花草藤萝，柔韧灵活，善于适应，有艺术气质".into(),
        "丙" => "丙火为太阳之火，热情奔放，光明磊落，有感染力".into(),
        "丁" => "丁火为灯烛之火，温和细腻，内心明亮，有洞察力".into(),
        "戊" => "戊土为城墙之土，厚重沉稳，信实可靠，有承载力".into(),
        "己" => "己土为田园之土，温和包容，善于滋养，有耐心".into(),
        "庚" => "庚金为刀剑之金，刚毅果断，有决断力，不畏困难".into(),
        "辛" => "辛金为首饰之金，精致细腻，追求完美，有审美力".into(),
        "壬" => "壬水为江河之水，奔流不息，智慧通达，有大局观".into(),
        "癸" => "癸水为雨露之水，细腻渗透，内敛深沉，有悟性".into(),
        _ => "特质需进一步分析".into(),
    }
}

fn describe_elements(metrics: &[crate::domain::analysis::WeightedMetric]) -> String {
    let name: std::collections::HashMap<&str, &str> = [
        ("wood", "木"), ("fire", "火"), ("earth", "土"),
        ("metal", "金"), ("water", "水"),
    ].into_iter().collect();

    let parts: Vec<String> = metrics.iter().map(|m| {
        let cn = name.get(m.id).unwrap_or(&m.id);
        let level = match m.weight_x2 {
            0 => "缺",
            1..=2 => "偏弱",
            3..=5 => "中和",
            _ => "偏强",
        };
        format!("{}行{}（权重{}）", cn, level, m.weight_x2)
    }).collect();
    parts.join("，")
}

fn describe_ten_gods(metrics: &[crate::domain::analysis::WeightedMetric], _day_stem: &str) -> String {
    let name: std::collections::HashMap<&str, &str> = [
        ("peer", "比肩"), ("rob_wealth", "劫财"),
        ("eating_god", "食神"), ("hurting_officer", "伤官"),
        ("direct_wealth", "正财"), ("indirect_wealth", "偏财"),
        ("direct_officer", "正官"), ("seven_killings", "七杀"),
        ("direct_resource", "正印"), ("indirect_resource", "偏印"),
    ].into_iter().collect();

    let active: Vec<String> = metrics.iter()
        .filter(|m| m.weight_x2 > 0)
        .map(|m| {
            let cn = name.get(m.id).unwrap_or(&m.id);
            format!("{}（{}分）", cn, m.weight_x2)
        }).collect();

    if active.is_empty() {
        "十神分布较均衡".into()
    } else {
        active.join("、")
    }
}

fn describe_hidden_stems(
    _metrics: &[crate::domain::analysis::WeightedMetric],
    chart: &crate::domain::bazi::BaziChart,
) -> String {
    let label: std::collections::HashMap<&str, &str> = [
        ("year_branch_hidden", "年支"), ("month_branch_hidden", "月支"),
        ("day_branch_hidden", "日支"), ("hour_branch_hidden", "时支"),
    ].into_iter().collect();

    let branches = [
        ("year_branch_hidden", &chart.year.branch),
        ("month_branch_hidden", &chart.month.branch),
        ("day_branch_hidden", &chart.day.branch),
    ];

    let parts: Vec<String> = branches.iter().map(|(key, branch)| {
        let hidden = hidden_stems_for_branch(branch);
        format!("{}「{}」藏{}", label.get(key).unwrap_or(&""), branch, hidden)
    }).collect();
    parts.join("；")
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

fn strength_advice(level: &str, _day_stem: &str, _el_cn: &str) -> &'static str {
    match level {
        "极弱" => "日主极弱，如同细流难御洪流，人生中需注意培养自信和定力，适合寻求外部支持和指导。",
        "偏弱" => "日主偏弱，如同幼苗需要呵护，宜循序渐进，借力成长，不宜独挑大梁。",
        "中和" => "日主中和，刚柔并济，适应力强，能够在不同环境中找到自己的节奏。",
        "偏强" => "日主偏强，如同大树根深叶茂，有自己的主张和能力，宜适当放手施展。",
        "极强" => "日主极强，如同山岳巍然，意志坚定但需注意柔和与倾听，刚过易折。",
        _ => "日主强弱需结合具体命局进一步判断。",
    }
}

fn pattern_description(name: &str) -> &'static str {
    match name {
        "正官格" => "正官格为人正直，重视规则与责任，适合体制内或管理类工作。",
        "七杀格" => "七杀格魄力十足，敢于突破，但也需注意冲动和控制情绪，宜以印星化解杀性。",
        "财格" => "财格注重实际，有经营头脑，对物质和资源有较强的感知力。",
        "印格" => "印格好学深思，重视精神世界和知识积累，有包容心和智慧。",
        "食伤格" => "食伤格才华横溢，富有创造力和表达能力，适合艺术、技术类方向。",
        "建禄格" => "建禄格根基扎实，自力更生，有稳定的发展能力。",
        "月刃格" => "月刃格意志坚定但可能过于自我，需注意合作和变通。",
        _ => "此格局需结合更多信息深入分析。",
    }
}

fn describe_luck_cycles(cycles: &[crate::domain::luck::LuckCycle], _start_age: u8, _is_forward: bool) -> String {
    if cycles.is_empty() {
        return "大运数据暂不可用。".into();
    }
    let parts: Vec<String> = cycles.iter().map(|c| {
        format!("第{}运「{}」（{}—{}岁）", c.label, c.pillar.ganzhi(), c.start_age, c.end_age)
    }).collect();
    format!("具体为：{}。每步大运十年一换，天干主导前五年，地支主导后五年。", parts.join("；"))
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
        Request::parse(
            b"GET /api/charts/report?date=2025-01-01&timezone=Asia/Shanghai&time_precision=exact&time=10:30&sex=male HTTP/1.1\r\n\r\n"
        ).unwrap()
    }

    #[test]
    fn report_generates_without_error() {
        let result = generate(&sample_config(), &sample_request());
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert!(resp.body.contains("chart-report"));
        assert!(resp.body.contains("本报告基于传统命理学算法"));
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
        assert!(result.body.contains("仅供文化参考"));
        assert!(result.body.contains("不构成任何专业建议"));
    }

    #[test]
    fn report_passes_forbidden_output_audit() {
        let result = generate(&sample_config(), &sample_request()).unwrap();
        assert!(result.body.contains("\"status\":\"passed\""));
        // Must NOT contain any forbidden patterns
        assert!(!result.body.contains("diagnosis"));
        assert!(!result.body.contains("guaranteed wealth"));
        assert!(!result.body.contains("death"));
        assert!(!result.body.contains("disease"));
        assert!(!result.body.contains("divorce is certain"));
    }

    #[test]
    fn report_unknown_hour_is_handled() {
        let req = Request::parse(
            b"GET /api/charts/report?date=2025-01-01&timezone=Asia/Shanghai&time_precision=unknown&sex=female HTTP/1.1\r\n\r\n"
        ).unwrap();
        let result = generate(&sample_config(), &req);
        assert!(result.is_ok());
        let resp = result.unwrap();
        assert!(resp.body.contains("未知（时辰未提供）"));
    }
}
