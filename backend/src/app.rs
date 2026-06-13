use crate::api;
use crate::config::AppConfig;
use crate::error::AppError;
use crate::http::{Method, Request, Response};

#[derive(Clone, Debug)]
pub struct App {
    config: AppConfig,
}

impl App {
    pub fn new(config: AppConfig) -> Self {
        Self { config }
    }

    pub fn handle(&self, request: Request) -> Response {
        if request.method == Method::Options {
            return Response::no_content();
        }

        match api::route(&self.config, &request) {
            Ok(response) => response,
            Err(error) => Response::json_error(error),
        }
    }
}

pub fn parse_and_handle(app: &App, bytes: &[u8]) -> Response {
    match Request::parse(bytes) {
        Ok(request) => app.handle(request),
        Err(error) => Response::json_error(error),
    }
}

pub fn unsupported_response(capability: &str, route: &str) -> Response {
    Response::json_error(AppError::Unsupported {
        capability: capability.to_string(),
        route: route.to_string(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::DEFAULT_ADDR;
    use std::path::Path;
    use std::path::PathBuf;

    fn app() -> App {
        app_with_lunar_data_path(PathBuf::from("missing.yaml"))
    }

    fn project_data_app() -> App {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("data")
            .join("raw")
            .join("lunar_data.yaml");
        app_with_lunar_data_path(path)
    }

    fn app_with_lunar_data_path(lunar_data_path: PathBuf) -> App {
        App::new(AppConfig {
            addr: DEFAULT_ADDR.to_string(),
            lunar_data_path,
        })
    }

    #[test]
    fn health_route_returns_ok() {
        let response = parse_and_handle(&app(), b"GET /api/health HTTP/1.1\r\n\r\n");

        assert_eq!(response.status.code(), 200);
        assert!(response.body.contains("\"status\":\"ok\""));
    }

    #[test]
    fn chart_detail_route_returns_supported_snapshot() {
        let response = parse_and_handle(
            &project_data_app(),
            b"GET /api/charts/detail?date=2025-01-01&time=10:30&timezone=Asia%2FShanghai&time_precision=exact HTTP/1.1\r\n\r\n",
        );

        assert_eq!(response.status.code(), 200);
        assert!(response.body.contains("\"capability\":\"chart-detail\""));
        assert!(
            response
                .body
                .contains("\"algo_version\":\"chart-engine-android-date-layer-v1\"")
        );
    }

    #[test]
    fn chart_create_returns_supported_four_pillar_core_for_exact_time() {
        let response = parse_and_handle(
            &project_data_app(),
            b"GET /api/charts?date=2025-01-01&time=10:30&timezone=Asia%2FShanghai&time_precision=exact HTTP/1.1\r\n\r\n",
        );

        assert_eq!(response.status.code(), 200);
        assert!(response.body.contains("\"status\":\"supported\""));
        assert!(response.body.contains("\"capability\":\"chart-create\""));
        assert!(
            response
                .body
                .contains("\"algo_version\":\"chart-engine-android-date-layer-v1\"")
        );
        assert!(response.body.contains("\"year\":{\"stem\":\"\u{7532}\",\"branch\":\"\u{8fb0}\",\"ganzhi\":\"\u{7532}\u{8fb0}\"}"));
        assert!(response.body.contains("\"day\":{\"stem\":\"\u{5e9a}\",\"branch\":\"\u{5348}\",\"ganzhi\":\"\u{5e9a}\u{5348}\"}"));
        assert!(response.body.contains("\"hour\":{\"stem\":\"\u{8f9b}\",\"branch\":\"\u{5df3}\",\"ganzhi\":\"\u{8f9b}\u{5df3}\"}"));
        assert!(response.body.contains("\"true_solar_time\""));
        assert!(response.body.contains("\"iana_timezone_history\""));
    }

    #[test]
    fn chart_create_keeps_unknown_hour_null_with_candidates() {
        let response = parse_and_handle(
            &project_data_app(),
            b"GET /api/charts?date=2025-01-01&timezone=Asia%2FShanghai&time_precision=unknown HTTP/1.1\r\n\r\n",
        );

        assert_eq!(response.status.code(), 200);
        assert!(response.body.contains("\"hour\":null"));
        assert!(
            response
                .body
                .contains("\"ambiguity_flags\":[\"unknown_hour\"]")
        );
        assert!(response.body.contains("\"hour_candidates\":["));
        assert!(response.body.contains("\"ganzhi\":\"\u{4e19}\u{5b50}\""));
        assert!(response.body.contains("\"ganzhi\":\"\u{4e01}\u{4e11}\""));
    }

    #[test]
    fn analysis_snapshot_returns_structured_metrics_cards_and_disclaimer() {
        let response = parse_and_handle(
            &project_data_app(),
            b"GET /api/analysis/snapshot?date=2025-01-01&time=10:30&timezone=Asia%2FShanghai&time_precision=exact HTTP/1.1\r\n\r\n",
        );

        assert_eq!(response.status.code(), 200);
        assert!(
            response
                .body
                .contains("\"capability\":\"analysis-snapshot\"")
        );
        assert!(
            response
                .body
                .contains("\"algo_version\":\"structured-analysis-v1\"")
        );
        assert!(response.body.contains(
            "\"disclaimer_id\":\"traditional-interpretation-not-professional-advice-v1\""
        ));
        assert!(response.body.contains("\"elements\""));
        assert!(response.body.contains("\"ten_gods\""));
        assert!(response.body.contains("\"cards\""));
        assert!(
            response
                .body
                .contains("\"forbidden_output_audit\":{\"status\":\"passed\"")
        );
        assert!(!response.body.contains("guaranteed wealth"));
        assert!(!response.body.contains("diagnosis"));
    }

    #[test]
    fn analysis_snapshot_surfaces_unknown_hour_sensitivity() {
        let response = parse_and_handle(
            &project_data_app(),
            b"GET /api/analysis/snapshot?date=2025-01-01&timezone=Asia%2FShanghai&time_precision=unknown HTTP/1.1\r\n\r\n",
        );

        assert_eq!(response.status.code(), 200);
        assert!(
            response
                .body
                .contains("unknown_hour_affects_hour_pillar_and_hidden_stems")
        );
        assert!(response.body.contains("chart_has_ambiguity_flags"));
    }

    #[test]
    fn case_create_stores_immutable_chart_and_analysis_snapshots() {
        let response = parse_and_handle(
            &project_data_app(),
            b"GET /api/cases?action=create&id=case-loop-015-a&title=Case%20A&tags=alpha,beta&note=private-note&date=2025-01-01&time=10:30&timezone=Asia%2FShanghai&time_precision=exact HTTP/1.1\r\n\r\n",
        );

        assert_eq!(response.status.code(), 200);
        assert!(response.body.contains("\"capability\":\"case-management\""));
        assert!(
            response
                .body
                .contains("\"storage\":\"local-volatile-case-store-v1\"")
        );
        assert!(response.body.contains("\"case_status\":\"active\""));
        assert!(
            response
                .body
                .contains("\"chart_algo_version\":\"chart-engine-android-date-layer-v1\"")
        );
        assert!(
            response
                .body
                .contains("\"analysis_algo_version\":\"structured-analysis-v1\"")
        );
        assert!(response.body.contains("\"private_note\":\"private-note\""));
    }

    #[test]
    fn case_list_omits_private_note_and_deleted_cases() {
        let create = parse_and_handle(
            &project_data_app(),
            b"GET /api/cases?action=create&id=case-loop-015-b&title=Case%20B&tags=keep&note=secret-note&date=2025-01-01&time=10:30&timezone=Asia%2FShanghai&time_precision=exact HTTP/1.1\r\n\r\n",
        );
        assert_eq!(create.status.code(), 200);

        let list = parse_and_handle(
            &project_data_app(),
            b"GET /api/cases?action=list HTTP/1.1\r\n\r\n",
        );
        assert_eq!(list.status.code(), 200);
        assert!(list.body.contains("\"id\":\"case-loop-015-b\""));
        assert!(!list.body.contains("secret-note"));

        let delete = parse_and_handle(
            &project_data_app(),
            b"GET /api/cases?action=delete&id=case-loop-015-b HTTP/1.1\r\n\r\n",
        );
        assert_eq!(delete.status.code(), 200);

        let list_after_delete = parse_and_handle(
            &project_data_app(),
            b"GET /api/cases?action=list HTTP/1.1\r\n\r\n",
        );
        assert_eq!(list_after_delete.status.code(), 200);
        assert!(
            !list_after_delete
                .body
                .contains("\"id\":\"case-loop-015-b\"")
        );
        assert!(!list_after_delete.body.contains("secret-note"));
    }

    #[test]
    fn case_update_metadata_preserves_snapshot_versions() {
        let create = parse_and_handle(
            &project_data_app(),
            b"GET /api/cases?action=create&id=case-loop-015-c&title=Case%20C&tags=old&note=old-note&date=2025-01-01&time=10:30&timezone=Asia%2FShanghai&time_precision=exact HTTP/1.1\r\n\r\n",
        );
        assert_eq!(create.status.code(), 200);

        let update = parse_and_handle(
            &project_data_app(),
            b"GET /api/cases?action=update_metadata&id=case-loop-015-c&title=Updated%20Case&tags=new&note= HTTP/1.1\r\n\r\n",
        );
        assert_eq!(update.status.code(), 200);
        assert!(update.body.contains("\"title\":\"Updated Case\""));
        assert!(update.body.contains("\"tags\":[\"new\"]"));
        assert!(update.body.contains("\"private_note\":null"));
        assert!(update.body.contains(
            "\"snapshot_id\":\"case-loop-015-c:chart:chart-engine-android-date-layer-v1\""
        ));
        assert!(
            update
                .body
                .contains("\"snapshot_id\":\"case-loop-015-c:analysis:structured-analysis-v1\"")
        );
    }

    #[test]
    fn case_archive_marks_case_archived_without_deleting_snapshot() {
        let create = parse_and_handle(
            &project_data_app(),
            b"GET /api/cases?action=create&id=case-loop-015-d&title=Case%20D&date=2025-01-01&time=10:30&timezone=Asia%2FShanghai&time_precision=exact HTTP/1.1\r\n\r\n",
        );
        assert_eq!(create.status.code(), 200);

        let archive = parse_and_handle(
            &project_data_app(),
            b"GET /api/cases?action=archive&id=case-loop-015-d HTTP/1.1\r\n\r\n",
        );
        assert_eq!(archive.status.code(), 200);
        assert!(archive.body.contains("\"case_status\":\"archived\""));
        assert!(
            archive
                .body
                .contains("case-loop-015-d:chart:chart-engine-android-date-layer-v1")
        );
    }

    #[test]
    fn settings_update_returns_local_preferences() {
        let response = parse_and_handle(
            &project_data_app(),
            b"GET /api/settings?action=update&language=en-US&theme=dark&privacy_default=shared_snapshot HTTP/1.1\r\n\r\n",
        );

        assert_eq!(response.status.code(), 200);
        assert!(response.body.contains("\"capability\":\"settings\""));
        assert!(
            response
                .body
                .contains("\"storage\":\"local-volatile-preferences-v1\"")
        );
        assert!(response.body.contains("\"language\":\"en-US\""));
        assert!(response.body.contains("\"theme\":\"dark\""));
        assert!(
            response
                .body
                .contains("\"privacy_default\":\"shared_snapshot\"")
        );
    }

    #[test]
    fn luck_cycles_returns_supported_after_m13() {
        let response = parse_and_handle(
            &project_data_app(),
            b"GET /api/luck/cycles?date=2025-01-01&timezone=Asia%2FShanghai&sex=male HTTP/1.1\r\n\r\n",
        );

        assert_eq!(response.status.code(), 200);
        assert!(response.body.contains("\"capability\":\"luck-cycles\""));
        assert!(response.body.contains("\"direction\""));
        assert!(response.body.contains("\"cycles\""));
        assert!(!response.body.contains("luck-reading"));
        assert!(!response.body.contains("luck_reading"));
        assert!(!response.body.contains("annual-trigger-reading"));
        assert!(!response.body.contains("annual_trigger_reading"));
        assert!(!response.body.contains("topic-timeline-reading"));
        assert!(!response.body.contains("topic-timeline-overlay"));
        assert!(!response.body.contains("topic_timeline"));
        assert!(!response.body.contains("白话说"));
        assert!(!response.body.contains("score_internal"));
    }

    #[test]
    fn chart_report_carries_restricted_luck_reading_after_m36() {
        let response = parse_and_handle(
            &project_data_app(),
            b"GET /api/charts/report?date=2025-01-01&time=10:30&timezone=Asia%2FShanghai&time_precision=exact&sex=male&reading_year=2026 HTTP/1.1\r\n\r\n",
        );

        assert_eq!(response.status.code(), 200);
        assert!(response.body.contains("\"capability\":\"chart-report\""));
        assert!(response.body.contains("\"status\":\"restricted\""));
        assert!(response.body.contains("\"luck_reading\""));
        assert!(response.body.contains("\"capability\":\"luck-reading\""));
        assert!(response.body.contains("\"reference_year\":2026"));
        assert!(response.body.contains("从命理结构看"));
        assert!(response.body.contains("您可以"));
        assert!(!response.body.contains("score_internal"));
        assert!(!response.body.contains("0-100"));
    }

    #[test]
    fn chart_report_carries_restricted_annual_trigger_after_m37() {
        let response = parse_and_handle(
            &project_data_app(),
            b"GET /api/charts/report?date=2025-01-01&time=10:30&timezone=Asia%2FShanghai&time_precision=exact&sex=male&reading_year=2026&year=2026 HTTP/1.1\r\n\r\n",
        );

        assert_eq!(response.status.code(), 200);
        assert!(response.body.contains("\"capability\":\"chart-report\""));
        assert!(response.body.contains("\"annual_trigger_reading\""));
        assert!(
            response
                .body
                .contains("\"capability\":\"annual-trigger-reading\"")
        );
        assert!(response.body.contains("\"year\":2026"));
        assert!(response.body.contains("\"year_source\":\"explicit\""));
        assert!(response.body.contains("\"ganzhi\":\"丙午\""));
        assert!(response.body.contains("major-luck+annual-trigger"));
        assert!(response.body.contains("年度引动"));
        assert!(response.body.contains("2026年年柱丙午"));
        assert!(response.body.contains("从命理结构看"));
        assert!(response.body.contains("您可以"));
        assert!(!response.body.contains("score_internal"));
        assert!(!response.body.contains("0-100"));
        assert!(!response.body.contains("流月运势"));
        assert!(!response.body.contains("每日运势"));
    }

    #[test]
    fn share_preview_returns_redacted_public_dto_without_private_case_state() {
        let create_case = parse_and_handle(
            &project_data_app(),
            b"GET /api/cases?action=create&id=case-loop-016-a&title=Private%20Name&tags=secret-tag&note=secret-note&date=2025-01-01&time=10:30&timezone=Asia%2FShanghai&time_precision=exact HTTP/1.1\r\n\r\n",
        );
        assert_eq!(create_case.status.code(), 200);

        let create_share = parse_and_handle(
            &project_data_app(),
            b"GET /api/share/preview?action=create&case_id=case-loop-016-a&ttl_seconds=3600 HTTP/1.1\r\n\r\n",
        );
        assert_eq!(create_share.status.code(), 200);
        assert!(
            create_share
                .body
                .contains("\"capability\":\"share-preview\"")
        );
        assert!(
            create_share
                .body
                .contains("\"storage\":\"local-volatile-share-store-v1\"")
        );
        assert!(
            create_share
                .body
                .contains("\"token_storage\":\"hash-only\"")
        );
        assert!(create_share.body.contains("\"noindex\":true"));
        assert!(create_share.body.contains("\"editable\":false"));
        assert!(!create_share.body.contains("secret-note"));
        assert!(!create_share.body.contains("secret-tag"));
        assert!(!create_share.body.contains("Private Name"));
        assert!(!create_share.body.contains("case-loop-016-a"));
        assert!(!create_share.body.contains("snapshot_id"));

        let token = extract_json_string(&create_share.body, "token");
        let request =
            format!("GET /api/share/preview?action=public&token={token} HTTP/1.1\r\n\r\n");
        let public = parse_and_handle(&project_data_app(), request.as_bytes());

        assert_eq!(public.status.code(), 200);
        assert!(
            public
                .body
                .contains("\"dto_version\":\"public-share-redacted-v1\"")
        );
        assert!(
            public
                .body
                .contains("\"chart_algo_version\":\"chart-engine-android-date-layer-v1\"")
        );
        assert!(
            public
                .body
                .contains("\"analysis_algo_version\":\"structured-analysis-v1\"")
        );
        assert!(!public.body.contains("secret-note"));
        assert!(!public.body.contains("secret-tag"));
        assert!(!public.body.contains("Private Name"));
        assert!(!public.body.contains("case-loop-016-a"));
        assert!(!public.body.contains("snapshot_id"));
    }

    #[test]
    fn share_revoke_makes_token_unavailable_without_case_existence_leak() {
        let create_case = parse_and_handle(
            &project_data_app(),
            b"GET /api/cases?action=create&id=case-loop-016-b&title=Case%20B&note=private-share-note&date=2025-01-01&time=10:30&timezone=Asia%2FShanghai&time_precision=exact HTTP/1.1\r\n\r\n",
        );
        assert_eq!(create_case.status.code(), 200);

        let create_share = parse_and_handle(
            &project_data_app(),
            b"GET /api/share/preview?action=create&case_id=case-loop-016-b&ttl_seconds=3600 HTTP/1.1\r\n\r\n",
        );
        assert_eq!(create_share.status.code(), 200);
        let token = extract_json_string(&create_share.body, "token");

        let revoke_request =
            format!("GET /api/share/preview?action=revoke&token={token} HTTP/1.1\r\n\r\n");
        let revoke = parse_and_handle(&project_data_app(), revoke_request.as_bytes());
        assert_eq!(revoke.status.code(), 200);
        assert!(revoke.body.contains("\"revoked\":true"));

        let public_request =
            format!("GET /api/share/preview?action=public&token={token} HTTP/1.1\r\n\r\n");
        let public = parse_and_handle(&project_data_app(), public_request.as_bytes());
        assert_error(public, 404, "not_found", "share unavailable");

        let invalid = parse_and_handle(
            &project_data_app(),
            b"GET /api/share/preview?action=public&token=invalid-token HTTP/1.1\r\n\r\n",
        );
        assert_error(invalid, 404, "not_found", "share unavailable");
    }

    #[test]
    fn chart_basis_preview_returns_restricted_contract() {
        let response = parse_and_handle(
            &project_data_app(),
            b"GET /api/charts/basis/preview?date=2025-01-01&time=10:30&timezone=Asia%2FShanghai&time_precision=exact&sex=unspecified&privacy=private HTTP/1.1\r\n\r\n",
        );

        assert_eq!(response.status.code(), 200);
        assert!(response.body.contains("\"status\":\"restricted\""));
        assert!(
            response
                .body
                .contains("\"capability\":\"chart-basis-preview\"")
        );
        assert!(response.body.contains("\"ruleset_id\":\"ft-v1-default\""));
        assert!(
            response
                .body
                .contains("\"day_boundary_rule\":\"local-civil-midnight-00:00\"")
        );
        assert!(response.body.contains("\"hour_pillar\""));
        assert!(!response.body.contains("\"year_pillar\""));
    }

    #[test]
    fn chart_basis_preview_rejects_lunar_input() {
        let response = parse_and_handle(
            &project_data_app(),
            b"GET /api/charts/basis/preview?date=2025-01-01&timezone=Asia%2FShanghai&calendar=lunar HTTP/1.1\r\n\r\n",
        );

        assert_error(response, 501, "unsupported_capability", "lunar-input");
    }

    #[test]
    fn chart_basis_preview_rejects_true_solar_time() {
        let response = parse_and_handle(
            &project_data_app(),
            b"GET /api/charts/basis/preview?date=2025-01-01&timezone=Asia%2FShanghai&true_solar_time=true HTTP/1.1\r\n\r\n",
        );

        assert_error(response, 501, "unsupported_capability", "true-solar-time");
    }

    #[test]
    fn chart_basis_preview_rejects_invalid_exact_time() {
        let response = parse_and_handle(
            &project_data_app(),
            b"GET /api/charts/basis/preview?date=2025-01-01&timezone=Asia%2FShanghai&time_precision=exact&time=25:00 HTTP/1.1\r\n\r\n",
        );

        assert_error(
            response,
            400,
            "bad_request",
            "time must use HH:MM in 24-hour format",
        );
    }

    #[test]
    fn topic_report_relationship_returns_restricted_report_with_explicit_year() {
        let response = parse_and_handle(
            &project_data_app(),
            b"GET /api/charts/topic-report?topic=relationship&date=2025-01-01&time=10:30&timezone=Asia%2FShanghai&time_precision=exact&sex=female&year=2026 HTTP/1.1\r\n\r\n",
        );

        assert_eq!(response.status.code(), 200);
        assert!(response.body.contains("\"status\":\"restricted\""));
        assert!(
            response
                .body
                .contains("\"capability\":\"relationship-report\"")
        );
        assert!(response.body.contains("\"topic\":\"relationship\""));
        assert!(
            response
                .body
                .contains("\"algo_version\":\"topic-report-v1\"")
        );
        assert!(response.body.contains("\"year\":2026"));
        assert!(response.body.contains("\"year_source\":\"explicit\""));
        assert!(
            response
                .body
                .contains("\"forbidden_output_audit\":{\"status\":\"passed\"")
        );
        assert!(response.body.contains("\u{592b}\u{59bb}\u{5bab}"));
        assert!(response.body.contains("\u{914d}\u{5076}\u{661f}"));
        assert!(response.body.contains("\u{5341}\u{795e}"));
        assert!(response.body.contains("\u{5408}\u{51b2}\u{5211}\u{5bb3}"));
        assert!(response.body.contains("\u{5927}\u{8fd0}"));
        assert!(response.body.contains("\u{6d41}\u{5e74}\u{5f15}\u{52a8}"));
        for title in [
            "\"title\":\"总断\"",
            "\"title\":\"伴侣议题\"",
            "\"title\":\"夫妻宫\"",
            "\"title\":\"表达、边界与安全感\"",
            "\"title\":\"年度情感引动\"",
            "\"title\":\"结论\"",
        ] {
            assert!(
                response.body.contains(title),
                "relationship report missing {title}"
            );
        }
        assert!(response.body.contains("2026年"));
        assert!(response.body.contains("稳定回应"));
        assert!(response.body.contains("现实承接"));
        assert_public_timeline_body_is_m40_safe("relationship", &response.body);
        assert!(!response.body.contains("score_internal"));
        assert!(!response.body.contains("\u{5fc5}\u{7136}\u{7ed3}\u{5a5a}"));
        assert!(!response.body.contains("\u{51fa}\u{8f68}"));
        for forbidden in [
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
        ] {
            assert!(
                !response.body.contains(forbidden),
                "relationship report leaked M42 machine wording {forbidden}"
            );
        }
    }

    #[test]
    fn topic_report_relationship_requires_explicit_four_digit_year() {
        let missing = parse_and_handle(
            &project_data_app(),
            b"GET /api/charts/topic-report?topic=relationship&date=2025-01-01&timezone=Asia%2FShanghai HTTP/1.1\r\n\r\n",
        );
        assert_error(missing, 400, "bad_request", "missing query parameter: year");

        let short = parse_and_handle(
            &project_data_app(),
            b"GET /api/charts/topic-report?topic=relationship&date=2025-01-01&timezone=Asia%2FShanghai&year=26 HTTP/1.1\r\n\r\n",
        );
        assert_error(
            short,
            400,
            "bad_request",
            "year must be a four digit integer",
        );
    }

    #[test]
    fn topic_report_all_topics_return_restricted_after_m33() {
        let cases = [
            (
                "wealth",
                "wealth-report",
                &[
                    "正财",
                    "偏财",
                    "食伤生财",
                    "比劫",
                    "承载能力",
                    "引动",
                    "资源入口",
                    "资源不是只指钱",
                    "分享与边界",
                    "资源主题从「有没有」推进到「能不能安稳使用」",
                    "名义年龄约2岁",
                    "不会把金钱专项读成现实收入、投资或独立财务",
                    "资源感怎样形成",
                    "照护资源是否稳定",
                    "您的金钱关键词",
                    "早年阶段真正适合被照看的",
                ][..],
                &[
                    "score_internal",
                    "稳赚",
                    "发财",
                    "破财",
                    "投资建议",
                    "具体金额",
                    "落到日常理解里",
                    "日常读法",
                    "日常看",
                    "放回这张命盘看",
                    "这些牵动提醒您",
                    "财星分正财和偏财",
                    "从十神脉络看",
                    "传统上会把",
                    "产出方面，",
                    "分配方面，",
                    "支持与约束方面",
                    "从「金钱」专项来看",
                    "把2026年放进",
                    "十神与五行这一层",
                    "五行相处的方式提示",
                    "藏干、原局位置和当前大运合到一起时",
                    "本段把它作为阶段背景参考",
                    "这里看的不是单点事件",
                    "年度线索要回到",
                ][..],
            ),
            (
                "family",
                "family-report",
                &[
                    "宫位",
                    "印星",
                    "比劫",
                    "食伤",
                    "财官",
                    "引动",
                    "互动位置",
                    "家庭里的支持，不只是有人帮忙",
                    "家庭越亲近，边界越容易被忽略",
                    "家庭最终会落到很多具体事务上",
                    "名义年龄约2岁",
                    "成长环境",
                    "照护方式",
                    "把回应做稳",
                    "您的家庭关键词",
                    "早年阶段真正适合被照看的",
                ][..],
                &[
                    "score_internal",
                    "亲属会生病",
                    "亲属会死亡",
                    "必然生育",
                    "家庭离散",
                    "落到日常理解里",
                    "日常读法",
                    "日常看",
                    "放回家庭结构里",
                    "这些牵动提醒您",
                    "印星在家庭专项里主要看",
                    "比劫在家庭专项里看",
                    "财官在家庭专项里不解释",
                    "从十神脉络看",
                    "同辈边界：",
                    "表达方式：",
                    "从「家庭」专项来看",
                    "把2026年放进",
                    "十神与五行这一层",
                    "五行相处的方式提示",
                    "藏干、宫位关系和当前大运合到一起时",
                    "本段把它作为阶段背景参考",
                    "这里看的不是单点事件",
                    "年度线索要回到",
                ][..],
            ),
            (
                "career",
                "career-report",
                &[
                    "官杀",
                    "印星",
                    "食伤",
                    "财星",
                    "格局",
                    "用神",
                    "引动",
                    "事业用力方式",
                    "成长场景",
                    "规则和任务",
                    "表达训练",
                    "同伴互动",
                    "名义年龄约2岁",
                    "不会把事业专项读成现实职位、职业选择或工作结果",
                    "学习任务",
                    "规则感",
                    "您的事业关键词",
                    "早年阶段真正适合被照看的",
                    "成长场景",
                ][..],
                &[
                    "score_internal",
                    "必升职",
                    "必然失业",
                    "跳槽必成",
                    "考试必过",
                    "收入必涨",
                    "落到日常理解里",
                    "日常读法",
                    "日常看",
                    "这些牵动提醒您",
                    "官杀代表责任",
                    "食伤代表表达",
                    "比劫代表协作",
                    "技能表达：",
                    "资源落地：",
                    "协作竞争：",
                    "责任方面，",
                    "承接方面，",
                    "从「事业」专项来看",
                    "把2026年放进",
                    "十神与五行这一层",
                    "五行相处的方式提示",
                    "藏干、原局位置和当前大运合到一起时",
                    "本段把它作为阶段背景参考",
                    "这里看的不是单点事件",
                    "年度线索要回到",
                ][..],
            ),
        ];

        for (topic, capability, required_terms, forbidden_terms) in cases {
            let request = format!(
                "GET /api/charts/topic-report?topic={topic}&date=2025-01-01&time=10:30&timezone=Asia%2FShanghai&time_precision=exact&sex=unspecified&year=2026 HTTP/1.1\r\n\r\n"
            );
            let response = parse_and_handle(&project_data_app(), request.as_bytes());

            assert_eq!(response.status.code(), 200);
            assert!(response.body.contains("\"status\":\"restricted\""));
            assert!(
                response
                    .body
                    .contains(&format!("\"capability\":\"{capability}\""))
            );
            assert!(response.body.contains(&format!("\"topic\":\"{topic}\"")));
            assert!(response.body.contains("\"year_source\":\"explicit\""));
            assert!(
                response
                    .body
                    .contains("\"forbidden_output_audit\":{\"status\":\"passed\"")
            );

            for term in required_terms {
                assert!(response.body.contains(term), "{topic} missing {term}");
            }
            for term in forbidden_terms {
                assert!(
                    !response.body.contains(term),
                    "{topic} leaked forbidden term {term}"
                );
            }
            assert_public_timeline_body_is_m40_safe(topic, &response.body);
        }
    }

    #[test]
    fn topic_report_carries_restricted_topic_timeline_overlay_after_m38() {
        for (topic, capability) in [
            ("relationship", "relationship-report"),
            ("wealth", "wealth-report"),
            ("family", "family-report"),
            ("career", "career-report"),
        ] {
            let request = format!(
                "GET /api/charts/topic-report?topic={topic}&date=2025-01-01&time=10:30&timezone=Asia%2FShanghai&time_precision=exact&sex=unspecified&year=2026 HTTP/1.1\r\n\r\n"
            );
            let response = parse_and_handle(&project_data_app(), request.as_bytes());

            assert_eq!(response.status.code(), 200);
            assert!(response.body.contains("\"status\":\"restricted\""));
            assert!(
                response
                    .body
                    .contains(&format!("\"capability\":\"{capability}\""))
            );
            assert!(response.body.contains("\"id\":\"topic-timeline-overlay\""));
            assert!(response.body.contains("\"source\":\"timeline-core-v1\""));
            assert!(!response.body.contains("年龄段约为1至10岁"));
            assert!(!response.body.contains("约在 1 至 10 岁"));
            assert!(response.body.contains("annual-trigger"));
            if topic == "relationship" {
                assert!(response.body.contains("如果被\\\"冲\\\"牵动"));
                assert!(response.body.contains("\"title\":\"年度情感引动\""));
                assert!(response.body.contains("同场"));
                assert!(response.body.contains("当前大运"));
                assert!(
                    !response.body.contains(
                        "\"id\":\"topic-timeline-overlay\",\"title\":\"本专题的大运流年\""
                    )
                );
            } else {
                assert!(response.body.contains("相遇"));
                assert!(response.body.contains("现实事件"));
            }
            assert!(!response.body.contains("专业解释"));
            assert!(!response.body.contains("白话解释"));
            assert!(
                response
                    .body
                    .contains("\"forbidden_output_audit\":{\"status\":\"passed\"")
            );
            assert!(!response.body.contains("score_internal"));
            assert!(!response.body.contains("0-100"));
            assert!(!response.body.contains("流月运势"));
            assert!(!response.body.contains("每日运势"));
        }
    }

    #[test]
    fn m40_timeline_public_quality_gate_covers_golden_samples() {
        let chart_samples = [
            (
                "baseline_chart",
                "GET /api/charts/report?date=2025-01-01&time=10:30&timezone=Asia%2FShanghai&time_precision=exact&sex=male&reading_year=2026&year=2026 HTTP/1.1\r\n\r\n",
                &[
                    "\"luck_reading\"",
                    "\"annual_trigger_reading\"",
                    "\"reference_year\":2026",
                    "\"year_source\":\"explicit\"",
                    "从命理结构看",
                    "您可以",
                    "不是现实事件预告",
                ][..],
            ),
            (
                "unknown_hour_chart",
                "GET /api/charts/report?date=2025-01-01&timezone=Asia%2FShanghai&time_precision=unknown&sex=female&reading_year=2026&year=2026 HTTP/1.1\r\n\r\n",
                &[
                    "\"luck_reading\"",
                    "\"annual_trigger_reading\"",
                    "unknown_hour_timeline_evidence_downgraded",
                    "信息不完整",
                ][..],
            ),
            (
                "annual_year_only",
                "GET /api/charts/report?date=2025-01-01&time=10:30&timezone=Asia%2FShanghai&time_precision=exact&sex=male&year=2026 HTTP/1.1\r\n\r\n",
                &[
                    "\"annual_trigger_reading\"",
                    "\"reference_year\":null",
                    "reading_year_not_requested",
                    "\"year_source\":\"explicit\"",
                ][..],
            ),
        ];

        for (label, request, required_terms) in chart_samples {
            let response = parse_and_handle(&project_data_app(), request.as_bytes());
            assert_eq!(response.status.code(), 200, "{label} should return 200");
            assert!(
                response
                    .body
                    .contains("\"forbidden_output_audit\":{\"status\":\"passed\""),
                "{label} should pass forbidden-output audit"
            );
            assert_public_timeline_body_is_m40_safe(label, &response.body);
            assert!(
                response.body.len() <= 180_000,
                "{label} response is unexpectedly large"
            );
            for term in required_terms {
                assert!(response.body.contains(term), "{label} missing {term}");
            }
        }

        for (topic, capability) in [
            ("relationship", "relationship-report"),
            ("wealth", "wealth-report"),
            ("family", "family-report"),
            ("career", "career-report"),
        ] {
            let request = format!(
                "GET /api/charts/topic-report?topic={topic}&date=2025-01-01&time=10:30&timezone=Asia%2FShanghai&time_precision=exact&sex=unspecified&year=2026 HTTP/1.1\r\n\r\n"
            );
            let response = parse_and_handle(&project_data_app(), request.as_bytes());
            assert_eq!(response.status.code(), 200, "{topic} should return 200");
            assert!(
                response
                    .body
                    .contains(&format!("\"capability\":\"{capability}\""))
            );
            assert!(response.body.contains("\"id\":\"topic-timeline-overlay\""));
            assert!(response.body.contains("\"source\":\"timeline-core-v1\""));
            assert!(response.body.contains("\"year_source\":\"explicit\""));
            assert!(!response.body.contains("年龄段约为1至10岁"));
            assert!(!response.body.contains("约在 1 至 10 岁"));
            assert!(
                response
                    .body
                    .contains("\"forbidden_output_audit\":{\"status\":\"passed\"")
            );
            assert_public_timeline_body_is_m40_safe(topic, &response.body);
            assert!(
                response.body.len() <= 180_000,
                "{topic} response is unexpectedly large"
            );
        }

        let raw_luck = parse_and_handle(
            &project_data_app(),
            b"GET /api/luck/cycles?date=2025-01-01&timezone=Asia%2FShanghai&sex=male HTTP/1.1\r\n\r\n",
        );
        assert_eq!(raw_luck.status.code(), 200);
        for forbidden in [
            "luck_reading",
            "annual_trigger_reading",
            "topic-timeline-overlay",
            "专业说法",
            "白话说",
            "专业解释",
            "白话解释",
            "score_internal",
            "0-100",
        ] {
            assert!(
                !raw_luck.body.contains(forbidden),
                "raw luck route leaked {forbidden}"
            );
        }

        let capabilities = parse_and_handle(
            &project_data_app(),
            b"GET /api/capabilities HTTP/1.1\r\n\r\n",
        );
        assert_eq!(capabilities.status.code(), 200);
        for id in [
            "luck-reading",
            "annual-trigger-reading",
            "topic-timeline-reading",
        ] {
            assert!(
                capabilities
                    .body
                    .contains(&format!("\"id\":\"{id}\",\"status\":\"restricted\"")),
                "{id} should remain restricted after M40"
            );
        }
        assert!(
            !capabilities
                .body
                .contains("\"id\":\"topic-timeline-reading\",\"status\":\"supported\"")
        );
    }

    fn assert_public_timeline_body_is_m40_safe(label: &str, body: &str) {
        for forbidden in [
            "score_internal",
            "0-100",
            "保证发财",
            "必然发财",
            "必然结婚",
            "一定结婚",
            "必然离婚",
            "出轨",
            "亲属会生病",
            "亲属会死亡",
            "必升职",
            "一定升职",
            "必然失业",
            "跳槽必成",
            "结果保证",
            "确定发生",
            "流月运势",
            "流日运势",
            "每日运势",
            "白话",
            "专业说法",
            "专业解释",
            "指定年份",
            "这一年",
            "某一年",
            "用户",
            "帮助你",
            "帮助用户",
            "你的",
            "你可以",
            "帮你",
            "换成日常语言",
            "后端返回",
            "前端追加",
            "读盘时",
            "主题里",
            "事业主题里",
            "您在事业中",
            "当前提取结果",
            "观察年度",
            "共享证据",
            "规则版本",
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
            "当前算法",
            "系统给出",
            "综合评分",
            "相关信号共",
            "未见明显显性信号",
            "这份盘面目前没有触发",
            "降级参考",
            "共找到",
            "今年最值得留意",
            "当前报告没有收到",
            "系统再解释",
            "当前可计算",
            "置信度是算法",
            "分)",
            "盘中可用的时间线索",
            "重点看的牵动",
            "读这一段时",
            "在盘中有",
            "关键牵动是",
            "盘中能看到的关键牵动",
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
            "不作主线",
            "有一处落点",
            "有两处落点",
            "有三处落点",
            "落到这张盘上",
            "参与这组结构",
            "这组结构说明",
            "主要牵动如下",
            "盘面上先看这几股牵动",
            "这些牵动只说明",
            "这些牵动提醒您",
            "日常读法",
            "日常看",
            "放回这张命盘看",
            "放回家庭结构里",
            "这份报告适合当作",
            "这一章看的是",
            "基本脉络如下",
            "第一优先",
            "第二优先",
            "原局引动主要看",
            "先看这几层关系",
            "不能只看流年",
            "不必急着找事件结论",
            "偏弱表示这类倾向",
            "哪里需要放慢",
            "哪里需要承接",
            "读2026年这一层",
            "这一章会把",
            "牵动会先落在这些位置",
            "2026年的时间气候",
            "先从这些层次落下去看",
            "大运首段",
            "天干处先露出十神主题",
            "月支这一处",
            "日支这一处",
            "在这份金钱专项里",
            "在这份家庭专项里",
            "在这份事业专项里",
            "表达与安全感则落在日常相处里",
            "以目前资料来看，这份情感专项可以把重点放在",
            "在同一张桌上慢慢理清",
            "先看天干",
            "再看五行关系",
            "这一章只说明",
            "时间气候可以按这个顺序读",
            "当前资料可以按完整四柱合参",
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
            "· 先看天干",
        ] {
            assert!(
                !body.contains(forbidden),
                "{label} leaked M40 forbidden term {forbidden}"
            );
        }
    }

    #[test]
    fn topic_report_requires_supported_topic_value() {
        let missing_topic = parse_and_handle(
            &project_data_app(),
            b"GET /api/charts/topic-report?date=2025-01-01&timezone=Asia%2FShanghai&year=2026 HTTP/1.1\r\n\r\n",
        );
        assert_error(
            missing_topic,
            400,
            "bad_request",
            "missing query parameter: topic",
        );

        let unknown_topic = parse_and_handle(
            &project_data_app(),
            b"GET /api/charts/topic-report?topic=health&date=2025-01-01&timezone=Asia%2FShanghai&year=2026 HTTP/1.1\r\n\r\n",
        );
        assert_error(
            unknown_topic,
            400,
            "bad_request",
            "unsupported topic value: health",
        );
    }

    #[test]
    fn calendar_query_rejects_missing_date() {
        let response = parse_and_handle(
            &project_data_app(),
            b"GET /api/calendar/query HTTP/1.1\r\n\r\n",
        );

        assert_error(
            response,
            400,
            "bad_request",
            "missing query parameter: date",
        );
    }

    #[test]
    fn calendar_query_rejects_invalid_date() {
        let response = parse_and_handle(
            &project_data_app(),
            b"GET /api/calendar/query?date=2025-02-29 HTTP/1.1\r\n\r\n",
        );

        assert_error(response, 400, "bad_request", "date must use YYYY-MM-DD");
    }

    #[test]
    fn calendar_query_reports_out_of_range_dates() {
        for request in [
            b"GET /api/calendar/query?date=1901-01-01 HTTP/1.1\r\n\r\n".as_slice(),
            b"GET /api/calendar/query?date=2101-01-01 HTTP/1.1\r\n\r\n".as_slice(),
        ] {
            let response = parse_and_handle(&project_data_app(), request);

            assert_error(
                response,
                404,
                "out_of_range",
                "calendar date out of supported range",
            );
        }
    }

    #[test]
    fn calendar_query_accepts_supported_boundary_dates() {
        for (request, expected_date) in [
            (
                b"GET /api/calendar/query?date=1901-02-19 HTTP/1.1\r\n\r\n".as_slice(),
                "\"gregorian\":{\"year\":1901,\"month\":2,\"day\":19}",
            ),
            (
                b"GET /api/calendar/query?date=2100-12-31 HTTP/1.1\r\n\r\n".as_slice(),
                "\"gregorian\":{\"year\":2100,\"month\":12,\"day\":31}",
            ),
        ] {
            let response = parse_and_handle(&project_data_app(), request);

            assert_eq!(response.status.code(), 200);
            assert!(response.body.contains(expected_date));
            assert!(response.body.contains("\"meta\":"));
        }
    }

    #[test]
    fn calendar_query_reports_missing_data_source() {
        let response = parse_and_handle(
            &app(),
            b"GET /api/calendar/query?date=2025-01-01 HTTP/1.1\r\n\r\n",
        );

        assert_error(response, 500, "io_error", "read lunar data");
    }

    fn assert_error(
        response: Response,
        expected_status: u16,
        expected_code: &str,
        expected_message: &str,
    ) {
        assert_eq!(response.status.code(), expected_status);
        assert!(
            response
                .body
                .contains(&format!("\"error\":\"{expected_code}\""))
        );
        assert!(
            response.body.contains(expected_message),
            "body did not contain {expected_message:?}: {}",
            response.body
        );
    }

    #[test]
    fn glossary_returns_entries() {
        let response = parse_and_handle(&project_data_app(), b"GET /api/glossary HTTP/1.1\r\n\r\n");
        assert_eq!(response.status.code(), 200);
        assert!(response.body.contains("\"glossary\":["));
        assert!(response.body.contains("\"count\":"));
        // Should contain stem and branch entries
        assert!(response.body.contains("\"甲\""));
        assert!(response.body.contains("\"子\""));
    }

    #[test]
    fn glossary_filters_by_term() {
        let response = parse_and_handle(
            &project_data_app(),
            b"GET /api/glossary?term=%E6%AF%94%E8%82%A9 HTTP/1.1\r\n\r\n", // term=比肩
        );
        assert_eq!(response.status.code(), 200);
        assert!(response.body.contains("比肩"));
        // Should return fewer entries than unfiltered
        let unfiltered =
            parse_and_handle(&project_data_app(), b"GET /api/glossary HTTP/1.1\r\n\r\n");
        assert!(response.body.len() < unfiltered.body.len());
    }

    #[test]
    fn case_export_requires_id() {
        let response = parse_and_handle(
            &project_data_app(),
            b"GET /api/cases/export HTTP/1.1\r\n\r\n",
        );
        assert_eq!(response.status.code(), 400);
        assert!(response.body.contains("bad_request"));
    }

    #[test]
    fn case_export_returns_404_for_missing_id() {
        let response = parse_and_handle(
            &project_data_app(),
            b"GET /api/cases/export?id=nonexistent HTTP/1.1\r\n\r\n",
        );
        assert_eq!(response.status.code(), 404);
    }

    fn extract_json_string(body: &str, key: &str) -> String {
        let needle = format!("\"{key}\":\"");
        let start = body.find(&needle).expect("json string key should exist") + needle.len();
        let rest = &body[start..];
        let end = rest.find('"').expect("json string should terminate");
        rest[..end].to_string()
    }
}
