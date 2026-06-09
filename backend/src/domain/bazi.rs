use crate::calendar::civil::CivilDate;
use crate::error::AppError;

pub const RULESET_ID: &str = "ft-v1-default";
pub const CHART_BASIS_ALGO_VERSION: &str = "chart-basis-contract-v1";
pub const CHART_ENGINE_ALGO_VERSION: &str = "chart-engine-android-date-layer-v1";
pub const DATE_LAYER_RULESET_ID: &str = "ft-date-layer-android-v1";
pub const VALIDATED_RANGE_START_YEAR: i32 = 1901;
pub const VALIDATED_RANGE_END_YEAR: i32 = 2100;

pub const STEMS: [&str; 10] = [
    "\u{7532}", "\u{4e59}", "\u{4e19}", "\u{4e01}", "\u{620a}", "\u{5df1}", "\u{5e9a}", "\u{8f9b}",
    "\u{58ec}", "\u{7678}",
];
pub const BRANCHES: [&str; 12] = [
    "\u{5b50}", "\u{4e11}", "\u{5bc5}", "\u{536f}", "\u{8fb0}", "\u{5df3}", "\u{5348}", "\u{672a}",
    "\u{7533}", "\u{9149}", "\u{620c}", "\u{4ea5}",
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CalendarKind {
    Gregorian,
    Lunar,
}

impl CalendarKind {
    pub fn parse(value: Option<&str>) -> Result<Self, AppError> {
        match value.unwrap_or("gregorian") {
            "gregorian" => Ok(Self::Gregorian),
            "lunar" => Ok(Self::Lunar),
            other => Err(AppError::BadRequest(format!(
                "unsupported calendar value: {other}"
            ))),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Gregorian => "gregorian",
            Self::Lunar => "lunar",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Sex {
    Female,
    Male,
    Unspecified,
}

impl Sex {
    pub fn parse(value: Option<&str>) -> Result<Self, AppError> {
        match value.unwrap_or("unspecified") {
            "female" => Ok(Self::Female),
            "male" => Ok(Self::Male),
            "unspecified" => Ok(Self::Unspecified),
            other => Err(AppError::BadRequest(format!(
                "unsupported sex value: {other}"
            ))),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Female => "female",
            Self::Male => "male",
            Self::Unspecified => "unspecified",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PrivacyLevel {
    Private,
    SharedSnapshot,
}

impl PrivacyLevel {
    pub fn parse(value: Option<&str>) -> Result<Self, AppError> {
        match value.unwrap_or("private") {
            "private" => Ok(Self::Private),
            "shared_snapshot" => Ok(Self::SharedSnapshot),
            other => Err(AppError::BadRequest(format!(
                "unsupported privacy value: {other}"
            ))),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Private => "private",
            Self::SharedSnapshot => "shared_snapshot",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TimePrecision {
    Exact,
    Unknown,
}

impl TimePrecision {
    pub fn parse(value: Option<&str>) -> Result<Self, AppError> {
        match value.unwrap_or("unknown") {
            "exact" => Ok(Self::Exact),
            "unknown" => Ok(Self::Unknown),
            other => Err(AppError::BadRequest(format!(
                "unsupported time_precision value: {other}"
            ))),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Unknown => "unknown",
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BirthTime {
    pub hour: u8,
    pub minute: u8,
}

impl BirthTime {
    pub fn parse(value: &str) -> Option<Self> {
        let mut parts = value.split(':');
        let hour = parts.next()?.parse::<u8>().ok()?;
        let minute = parts.next()?.parse::<u8>().ok()?;
        if parts.next().is_some() || hour > 23 || minute > 59 {
            return None;
        }

        Some(Self { hour, minute })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BirthProfile {
    pub display_name: Option<String>,
    pub sex: Sex,
    pub privacy_level: PrivacyLevel,
    pub calendar_kind: CalendarKind,
    pub date: CivilDate,
    pub time: Option<BirthTime>,
    pub time_precision: TimePrecision,
    pub timezone: String,
    pub is_leap_month: bool,
    pub use_true_solar_time: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChartRequest {
    pub birth_profile: BirthProfile,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CalculationMetadata {
    pub ruleset_id: &'static str,
    pub algo_version: &'static str,
    pub date_layer_ruleset_id: &'static str,
    pub year_boundary_rule: &'static str,
    pub month_boundary_rule: &'static str,
    pub day_boundary_rule: &'static str,
    pub hour_policy: &'static str,
    pub timezone_policy: &'static str,
    pub true_solar_time_policy: &'static str,
    pub lunar_input_policy: &'static str,
}

impl CalculationMetadata {
    pub fn v1_default() -> Self {
        Self {
            ruleset_id: RULESET_ID,
            algo_version: CHART_BASIS_ALGO_VERSION,
            date_layer_ruleset_id: DATE_LAYER_RULESET_ID,
            year_boundary_rule: "lichun-target-not-calculated-in-m2",
            month_boundary_rule: "solar-term-jie-target-not-calculated-in-m2",
            day_boundary_rule: "local-civil-midnight-00:00",
            hour_policy: "hour-pillar-not-calculated-in-m2",
            timezone_policy: "timezone-identifier-recorded-not-historically-resolved-in-m2",
            true_solar_time_policy: "unsupported-in-m2",
            lunar_input_policy: "unsupported-while-dg-004-open",
        }
    }

    pub fn chart_engine_v1() -> Self {
        Self {
            ruleset_id: RULESET_ID,
            algo_version: CHART_ENGINE_ALGO_VERSION,
            date_layer_ruleset_id: DATE_LAYER_RULESET_ID,
            year_boundary_rule: "android-date-layer-year-ganzhi",
            month_boundary_rule: "solar-term-data-driven-month-starts-adr-0018",
            day_boundary_rule: "local-civil-midnight-00:00",
            hour_policy: "civil-two-hour-branches-five-rat-stem-rule",
            timezone_policy: "timezone-identifier-recorded-not-historically-resolved-in-m3",
            true_solar_time_policy: "unsupported-in-m3",
            lunar_input_policy: "unsupported-while-dg-004-closed-no-direct-lunar-input",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChartBasis {
    pub request: ChartRequest,
    pub metadata: CalculationMetadata,
    pub validated_range_start_year: i32,
    pub validated_range_end_year: i32,
    pub supported_outputs: Vec<&'static str>,
    pub unsupported_outputs: Vec<&'static str>,
}

impl ChartBasis {
    pub fn build(request: ChartRequest) -> Result<Self, AppError> {
        validate_birth_profile(&request.birth_profile)?;

        Ok(Self {
            request,
            metadata: CalculationMetadata::v1_default(),
            validated_range_start_year: VALIDATED_RANGE_START_YEAR,
            validated_range_end_year: VALIDATED_RANGE_END_YEAR,
            supported_outputs: vec![
                "ruleset_metadata",
                "birth_profile_contract",
                "chart_request_contract",
            ],
            unsupported_outputs: vec![
                "complete_four_pillars",
                "hour_pillar",
                "iana_timezone_history",
                "true_solar_time",
                "lunar_input",
                "persisted_chart",
            ],
        })
    }
}

fn validate_birth_profile(profile: &BirthProfile) -> Result<(), AppError> {
    if matches!(profile.calendar_kind, CalendarKind::Lunar) {
        return Err(AppError::Unsupported {
            capability: "lunar-input".to_string(),
            route: "/api/charts/basis/preview".to_string(),
        });
    }

    if profile.use_true_solar_time {
        return Err(AppError::Unsupported {
            capability: "true-solar-time".to_string(),
            route: "/api/charts/basis/preview".to_string(),
        });
    }

    if profile.date.year < VALIDATED_RANGE_START_YEAR
        || profile.date.year > VALIDATED_RANGE_END_YEAR
    {
        return Err(AppError::OutOfRange(
            "birth date out of V1 official validated range".to_string(),
        ));
    }

    if profile.timezone.trim().is_empty() {
        return Err(AppError::BadRequest("timezone is required".to_string()));
    }

    if matches!(profile.time_precision, TimePrecision::Exact) && profile.time.is_none() {
        return Err(AppError::BadRequest(
            "time is required when time_precision=exact".to_string(),
        ));
    }

    Ok(())
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Pillar {
    pub stem: String,
    pub branch: String,
}

impl Pillar {
    pub fn from_ganzhi(value: &str) -> Result<Self, AppError> {
        let mut chars = value.chars();
        let stem = chars
            .next()
            .ok_or_else(|| AppError::BadRequest("missing stem".to_string()))?
            .to_string();
        let branch = chars
            .next()
            .ok_or_else(|| AppError::BadRequest("missing branch".to_string()))?
            .to_string();

        Ok(Self { stem, branch })
    }

    pub fn from_indexes(stem_index: usize, branch_index: usize) -> Self {
        Self {
            stem: STEMS[stem_index % STEMS.len()].to_string(),
            branch: BRANCHES[branch_index % BRANCHES.len()].to_string(),
        }
    }

    pub fn ganzhi(&self) -> String {
        format!("{}{}", self.stem, self.branch)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DateLayerPillars {
    pub year: String,
    pub month: String,
    pub day: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BaziChart {
    pub year: Pillar,
    pub month: Pillar,
    pub day: Pillar,
    pub hour: Option<Pillar>,
    pub hour_candidates: Vec<Pillar>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChartResult {
    pub basis: ChartBasis,
    pub chart: BaziChart,
    pub metadata: CalculationMetadata,
    pub warnings: Vec<&'static str>,
    pub ambiguity_flags: Vec<&'static str>,
    pub unsupported_outputs: Vec<&'static str>,
}

impl ChartResult {
    pub fn build(basis: ChartBasis, date_layer: DateLayerPillars) -> Result<Self, AppError> {
        let year = Pillar::from_ganzhi(&date_layer.year)?;
        let month = Pillar::from_ganzhi(&date_layer.month)?;
        let day = Pillar::from_ganzhi(&date_layer.day)?;
        let profile = &basis.request.birth_profile;

        let (hour, hour_candidates, warnings, ambiguity_flags) = match profile.time_precision {
            TimePrecision::Exact => {
                let time = profile.time.ok_or_else(|| {
                    AppError::BadRequest("time is required when time_precision=exact".to_string())
                })?;
                (
                    Some(hour_pillar(&day, time)?),
                    Vec::new(),
                    vec![
                        "timezone identifier is recorded but historical offset is not resolved in M3",
                    ],
                    Vec::new(),
                )
            }
            TimePrecision::Unknown => (
                None,
                hour_candidates(&day)?,
                vec!["unknown hour: hour pillar is null and candidates are informational"],
                vec!["unknown_hour"],
            ),
        };

        Ok(Self {
            basis,
            chart: BaziChart {
                year,
                month,
                day,
                hour,
                hour_candidates,
            },
            metadata: CalculationMetadata::chart_engine_v1(),
            warnings,
            ambiguity_flags,
            unsupported_outputs: vec![
                "iana_timezone_history",
                "true_solar_time",
                "lunar_input",
                "persisted_chart",
                "analysis_snapshot",
                "luck_cycles",
            ],
        })
    }
}

fn hour_pillar(day: &Pillar, time: BirthTime) -> Result<Pillar, AppError> {
    let branch_index = hour_branch_index(time.hour);
    let day_stem_index = stem_index(&day.stem)?;
    let stem_index = ((day_stem_index % 5) * 2 + branch_index) % STEMS.len();
    Ok(Pillar::from_indexes(stem_index, branch_index))
}

fn hour_candidates(day: &Pillar) -> Result<Vec<Pillar>, AppError> {
    (0..BRANCHES.len())
        .map(|branch_index| {
            let day_stem_index = stem_index(&day.stem)?;
            let stem_index = ((day_stem_index % 5) * 2 + branch_index) % STEMS.len();
            Ok(Pillar::from_indexes(stem_index, branch_index))
        })
        .collect()
}

fn hour_branch_index(hour: u8) -> usize {
    if hour == 23 || hour == 0 {
        0
    } else {
        ((hour + 1) / 2) as usize
    }
}

fn stem_index(stem: &str) -> Result<usize, AppError> {
    STEMS
        .iter()
        .position(|candidate| *candidate == stem)
        .ok_or_else(|| AppError::BadRequest(format!("unknown day stem: {stem}")))
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnalysisSnapshot {
    pub five_element_summary: Vec<ElementCount>,
    pub relation_summary: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ElementCount {
    pub element: String,
    pub count: u8,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LuckCycle {
    pub label: String,
    pub start_age: u8,
    pub end_age: u8,
    pub pillar: Pillar,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request() -> ChartRequest {
        ChartRequest {
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
        }
    }

    #[test]
    fn builds_restricted_chart_basis_contract() {
        let basis = ChartBasis::build(request()).unwrap();

        assert_eq!(basis.metadata.ruleset_id, RULESET_ID);
        assert!(basis.supported_outputs.contains(&"ruleset_metadata"));
        assert!(basis.unsupported_outputs.contains(&"hour_pillar"));
    }

    #[test]
    fn rejects_lunar_input_while_decision_gate_is_open() {
        let mut request = request();
        request.birth_profile.calendar_kind = CalendarKind::Lunar;

        let error = ChartBasis::build(request).unwrap_err();

        assert_eq!(error.code(), "unsupported_capability");
        assert!(error.message().contains("lunar-input"));
    }

    #[test]
    fn rejects_true_solar_time_in_m2() {
        let mut request = request();
        request.birth_profile.use_true_solar_time = true;

        let error = ChartBasis::build(request).unwrap_err();

        assert_eq!(error.code(), "unsupported_capability");
        assert!(error.message().contains("true-solar-time"));
    }

    #[test]
    fn builds_chart_with_exact_hour_pillar() {
        let basis = ChartBasis::build(request()).unwrap();
        let result = ChartResult::build(
            basis,
            DateLayerPillars {
                year: "\u{7532}\u{8fb0}".to_string(),
                month: "\u{4e59}\u{4e11}".to_string(),
                day: "\u{5e9a}\u{5348}".to_string(),
            },
        )
        .unwrap();

        assert_eq!(result.metadata.algo_version, CHART_ENGINE_ALGO_VERSION);
        assert_eq!(result.chart.day.ganzhi(), "\u{5e9a}\u{5348}");
        assert_eq!(result.chart.hour.unwrap().ganzhi(), "\u{8f9b}\u{5df3}");
        assert!(result.chart.hour_candidates.is_empty());
    }

    #[test]
    fn unknown_hour_returns_candidates_without_fabricated_hour() {
        let mut request = request();
        request.birth_profile.time = None;
        request.birth_profile.time_precision = TimePrecision::Unknown;
        let basis = ChartBasis::build(request).unwrap();
        let result = ChartResult::build(
            basis,
            DateLayerPillars {
                year: "\u{7532}\u{8fb0}".to_string(),
                month: "\u{4e59}\u{4e11}".to_string(),
                day: "\u{5e9a}\u{5348}".to_string(),
            },
        )
        .unwrap();

        assert!(result.chart.hour.is_none());
        assert_eq!(result.chart.hour_candidates.len(), 12);
        assert!(result.ambiguity_flags.contains(&"unknown_hour"));
    }
}
