use crate::api::chart_basis::{birth_profile_to_json, string_array_to_json};
use crate::api::charts::build_chart_result;
use crate::config::AppConfig;
use crate::domain::analysis::AnalysisSnapshot;
use crate::domain::deep_analysis::{assess_strength, classify_pattern, suggest_useful_god};
use crate::domain::luck::compute_luck_cycle_context;
use crate::domain::topic_report::{
    TopicReport, TopicReportBlock, TopicReportTopic, TopicSignal, TopicTrace, build_career_report,
    build_family_report, build_relationship_report, build_wealth_report, validate_topic_year,
};
use crate::error::AppError;
use crate::http::{Request, Response, json};

pub fn generate(config: &AppConfig, request: &Request) -> Result<Response, AppError> {
    let topic = required_query(request, "topic")?;
    let topic = TopicReportTopic::parse(&topic)?;
    if !topic.is_implemented() {
        return Err(topic.unsupported_error());
    }

    let year = parse_required_year(request)?;
    validate_topic_year(year)?;

    let chart = build_chart_result(config, request)?;
    let snapshot = AnalysisSnapshot::build(&chart);
    let luck_context = compute_luck_cycle_context(
        &chart.chart.year.stem,
        &chart.chart.month,
        &chart.basis.request.birth_profile.sex,
        chart.basis.request.birth_profile.date,
    );
    let luck_cycles = luck_context.cycles;
    let strength = assess_strength(&chart);
    let pattern = classify_pattern(&chart, &strength);
    let useful_gods = suggest_useful_god(&strength, &pattern);

    match topic {
        TopicReportTopic::Relationship => {
            let report = build_relationship_report(
                &chart,
                &snapshot,
                &luck_cycles,
                &strength,
                &pattern,
                &useful_gods,
                year,
            );
            Ok(Response::json(topic_report_to_json(&report)))
        }
        TopicReportTopic::Wealth => {
            let report = build_wealth_report(
                &chart,
                &snapshot,
                &luck_cycles,
                &strength,
                &pattern,
                &useful_gods,
                year,
            );
            Ok(Response::json(topic_report_to_json(&report)))
        }
        TopicReportTopic::Family => {
            let report = build_family_report(
                &chart,
                &snapshot,
                &luck_cycles,
                &strength,
                &pattern,
                &useful_gods,
                year,
            );
            Ok(Response::json(topic_report_to_json(&report)))
        }
        TopicReportTopic::Career => {
            let report = build_career_report(
                &chart,
                &snapshot,
                &luck_cycles,
                &strength,
                &pattern,
                &useful_gods,
                year,
            );
            Ok(Response::json(topic_report_to_json(&report)))
        }
    }
}

fn required_query(request: &Request, key: &str) -> Result<String, AppError> {
    request
        .query_value(key)
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| AppError::BadRequest(format!("missing query parameter: {key}")))
}

fn parse_required_year(request: &Request) -> Result<i32, AppError> {
    let raw = required_query(request, "year")?;
    if raw.len() != 4 || !raw.chars().all(|character| character.is_ascii_digit()) {
        return Err(AppError::BadRequest(
            "year must be a four digit integer".to_string(),
        ));
    }
    raw.parse::<i32>()
        .map_err(|_| AppError::BadRequest("year must be a four digit integer".to_string()))
}

fn topic_report_to_json(report: &TopicReport) -> String {
    let assembled_report =
        crate::domain::topic_report::assemble_report(&report.disclaimer, &report.blocks);

    format!(
        concat!(
            "{{",
            "\"status\":{},",
            "\"capability\":{},",
            "\"topic\":{},",
            "\"topic_label\":{},",
            "\"algo_version\":{},",
            "\"ruleset_id\":{},",
            "\"year\":{},",
            "\"year_source\":{},",
            "\"disclaimer_id\":{},",
            "\"disclaimer\":{},",
            "\"birth_profile\":{},",
            "\"basis\":{},",
            "\"signals\":{},",
            "\"trace\":{},",
            "\"blocks\":{},",
            "\"assembled_report\":{},",
            "\"warnings\":{},",
            "\"forbidden_output_audit\":{{\"status\":{},\"checked_patterns\":{}}}",
            "}}"
        ),
        json::string(report.status),
        json::string(report.capability),
        json::string(report.topic.as_str()),
        json::string(report.topic.label_zh()),
        json::string(report.algo_version),
        json::string(report.ruleset_id),
        report.year,
        json::string(report.year_source),
        json::string(report.disclaimer_id),
        json::string(&report.disclaimer),
        birth_profile_to_json(&report.birth_profile),
        basis_to_json(report),
        signals_to_json(&report.signals),
        trace_to_json(&report.trace),
        blocks_to_json(&report.blocks),
        json::string(&assembled_report),
        string_array_to_json(&report.warnings),
        json::string(report.forbidden_output_audit.status),
        report.forbidden_output_audit.checked_patterns,
    )
}

fn basis_to_json(report: &TopicReport) -> String {
    format!(
        concat!(
            "{{",
            "\"day_master\":{},",
            "\"day_pillar\":{},",
            "\"relationship_palace\":{},",
            "\"sex\":{},",
            "\"time_precision\":{},",
            "\"annual_pillar\":{},",
            "\"strength_level\":{},",
            "\"pattern_name\":{}",
            "}}"
        ),
        json::string(&report.basis.day_master),
        json::string(&report.basis.day_pillar),
        json::string(&report.basis.relationship_palace),
        json::string(report.basis.sex),
        json::string(report.basis.time_precision),
        json::string(&report.basis.annual_pillar),
        json::string(&report.basis.strength_level),
        json::string(&report.basis.pattern_name),
    )
}

fn signals_to_json(signals: &[TopicSignal]) -> String {
    let items = signals
        .iter()
        .map(|signal| {
            format!(
                "{{\"id\":{},\"label\":{},\"qualitative_level\":{},\"summary\":{}}}",
                json::string(signal.id),
                json::string(signal.label),
                json::string(signal.qualitative_level),
                json::string(&signal.summary)
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("[{items}]")
}

fn trace_to_json(trace: &[TopicTrace]) -> String {
    let items = trace
        .iter()
        .map(|item| {
            format!(
                "{{\"id\":{},\"source\":{},\"evidence\":{},\"interpretation\":{}}}",
                json::string(item.id),
                json::string(item.source),
                string_vec_to_json(&item.evidence),
                json::string(&item.interpretation)
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("[{items}]")
}

fn blocks_to_json(blocks: &[TopicReportBlock]) -> String {
    let items = blocks
        .iter()
        .map(|block| {
            format!(
                "{{\"id\":{},\"title\":{},\"body\":{}}}",
                json::string(block.id),
                json::string(block.title),
                json::string(&block.body)
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("[{items}]")
}

fn string_vec_to_json(values: &[String]) -> String {
    let items = values
        .iter()
        .map(|value| json::string(value))
        .collect::<Vec<_>>()
        .join(",");
    format!("[{items}]")
}
