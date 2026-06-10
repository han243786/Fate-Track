use crate::http::{Response, json};

struct Capability {
    id: &'static str,
    status: &'static str,
    route: &'static str,
    source: &'static str,
}

pub fn capabilities() -> Response {
    let items = capability_catalog()
        .iter()
        .map(capability_to_json)
        .collect::<Vec<_>>()
        .join(",");

    Response::json(format!("{{\"capabilities\":[{items}]}}"))
}

fn capability_catalog() -> [Capability; 16] {
    [
        Capability {
            id: "health",
            status: "supported",
            route: "/api/health",
            source: "backend.api",
        },
        Capability {
            id: "lunar-data-meta",
            status: "supported",
            route: "/api/lunar-data/meta",
            source: "data.lunar.raw",
        },
        Capability {
            id: "calendar-date-query",
            status: "supported",
            route: "/api/calendar/query",
            source: "android-date-layer-port",
        },
        Capability {
            id: "calendar-date-query-v1-meta",
            status: "supported",
            route: "/api/calendar/query",
            source: "android-date-layer-port",
        },
        Capability {
            id: "chart-basis-preview",
            status: "restricted",
            route: "/api/charts/basis/preview",
            source: "ft-v1-default-contract",
        },
        Capability {
            id: "chart-create",
            status: "supported",
            route: "/api/charts",
            source: "chart-engine-android-date-layer-v1",
        },
        Capability {
            id: "chart-detail",
            status: "supported",
            route: "/api/charts/detail",
            source: "chart-engine-android-date-layer-v1",
        },
        Capability {
            id: "analysis-snapshot",
            status: "supported",
            route: "/api/analysis/snapshot",
            source: "structured-analysis-v1",
        },
        Capability {
            id: "luck-cycles",
            status: "supported",
            route: "/api/luck/cycles",
            source: "luck-engine-v1-adr-0020",
        },
        Capability {
            id: "case-management",
            status: "restricted",
            route: "/api/cases",
            source: "local-volatile-case-store-v1",
        },
        Capability {
            id: "share-preview",
            status: "restricted",
            route: "/api/share/preview",
            source: "local-volatile-share-store-v1",
        },
        Capability {
            id: "settings",
            status: "restricted",
            route: "/api/settings",
            source: "local-volatile-preferences-v1",
        },
        Capability {
            id: "glossary",
            status: "supported",
            route: "/api/glossary",
            source: "structured-glossary-data-v1",
        },
        Capability {
            id: "case-export",
            status: "restricted",
            route: "/api/cases/export",
            source: "local-volatile-case-store-v1",
        },
        Capability {
            id: "data-derivation",
            status: "restricted",
            route: "/api/data/derive",
            source: "aggregate-derivation-v1",
        },
        Capability {
            id: "astronomy-engine",
            status: "supported",
            route: "data/generated/astronomy/out/*",
            source: "astronomy-engine-v1-adr-0019",
        },
    ]
}

fn capability_to_json(capability: &Capability) -> String {
    format!(
        "{{\"id\":{},\"status\":{},\"route\":{},\"source\":{}}}",
        json::string(capability.id),
        json::string(capability.status),
        json::string(capability.route),
        json::string(capability.source)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exposes_date_layer_metadata_capability() {
        let catalog = capability_catalog();
        let capability = catalog
            .iter()
            .find(|capability| capability.id == "calendar-date-query-v1-meta")
            .expect("date-layer metadata capability should be declared");

        assert_eq!(capability.status, "supported");
        assert_eq!(capability.route, "/api/calendar/query");
        assert_eq!(capability.source, "android-date-layer-port");

        let body = capabilities().body;
        assert!(body.contains("\"id\":\"calendar-date-query-v1-meta\""));
        assert!(body.contains("\"status\":\"supported\""));
    }

    #[test]
    fn exposes_chart_create_as_supported_after_m3_core() {
        let catalog = capability_catalog();
        let capability = catalog
            .iter()
            .find(|capability| capability.id == "chart-create")
            .expect("chart-create capability should be declared");

        assert_eq!(capability.status, "supported");
        assert_eq!(capability.route, "/api/charts");
        assert_eq!(capability.source, "chart-engine-android-date-layer-v1");
    }

    #[test]
    fn exposes_analysis_snapshot_as_supported_after_m4_core() {
        let catalog = capability_catalog();
        let capability = catalog
            .iter()
            .find(|capability| capability.id == "analysis-snapshot")
            .expect("analysis-snapshot capability should be declared");

        assert_eq!(capability.status, "supported");
        assert_eq!(capability.route, "/api/analysis/snapshot");
        assert_eq!(capability.source, "structured-analysis-v1");
    }

    #[test]
    fn exposes_case_management_as_restricted_after_m5_local_storage() {
        let catalog = capability_catalog();
        let capability = catalog
            .iter()
            .find(|capability| capability.id == "case-management")
            .expect("case-management capability should be declared");

        assert_eq!(capability.status, "restricted");
        assert_eq!(capability.route, "/api/cases");
        assert_eq!(capability.source, "local-volatile-case-store-v1");
    }

    #[test]
    fn exposes_settings_as_restricted_after_m5_local_storage() {
        let catalog = capability_catalog();
        let capability = catalog
            .iter()
            .find(|capability| capability.id == "settings")
            .expect("settings capability should be declared");

        assert_eq!(capability.status, "restricted");
        assert_eq!(capability.route, "/api/settings");
        assert_eq!(capability.source, "local-volatile-preferences-v1");
    }

    #[test]
    fn exposes_share_preview_as_restricted_after_m6_privacy_boundary() {
        let catalog = capability_catalog();
        let capability = catalog
            .iter()
            .find(|capability| capability.id == "share-preview")
            .expect("share-preview capability should be declared");

        assert_eq!(capability.status, "restricted");
        assert_eq!(capability.route, "/api/share/preview");
        assert_eq!(capability.source, "local-volatile-share-store-v1");
    }

    #[test]
    fn exposes_astronomy_engine_as_supported_after_m23_promotion() {
        let catalog = capability_catalog();
        let capability = catalog
            .iter()
            .find(|capability| capability.id == "astronomy-engine")
            .expect("astronomy-engine capability should be declared");

        assert_eq!(capability.status, "supported");
        assert_eq!(capability.route, "data/generated/astronomy/out/*");
        assert_eq!(capability.source, "astronomy-engine-v1-adr-0019");

        let body = capabilities().body;
        assert!(body.contains("\"id\":\"astronomy-engine\""));
        assert!(body.contains("\"status\":\"supported\""));
    }
}
