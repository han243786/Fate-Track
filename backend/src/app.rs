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

    fn extract_json_string(body: &str, key: &str) -> String {
        let needle = format!("\"{key}\":\"");
        let start = body.find(&needle).expect("json string key should exist") + needle.len();
        let rest = &body[start..];
        let end = rest.find('"').expect("json string should terminate");
        rest[..end].to_string()
    }
}
