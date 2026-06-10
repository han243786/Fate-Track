use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct CaseDerivedStats {
    pub total_cases: u32,
    pub day_masters: BTreeMap<String, u32>,
    pub elements: BTreeMap<String, u32>,
    pub ten_gods: BTreeMap<String, u32>,
    pub hour_distribution: BTreeMap<String, u32>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CaseStatus {
    Active,
    Archived,
    Deleted,
}

impl CaseStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Archived => "archived",
            Self::Deleted => "deleted",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChartSnapshot {
    pub snapshot_id: String,
    pub chart_algo_version: String,
    pub ruleset_id: String,
    pub day_master: String,
    pub hour_branch: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AnalysisSnapshotRef {
    pub snapshot_id: String,
    pub analysis_algo_version: String,
    pub disclaimer_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaseRecord {
    pub id: String,
    pub title: String,
    pub tags: Vec<String>,
    pub private_note: Option<String>,
    pub status: CaseStatus,
    pub chart_snapshot: ChartSnapshot,
    pub analysis_snapshot: AnalysisSnapshotRef,
    pub element_counts: BTreeMap<String, u32>,
    pub ten_god_counts: BTreeMap<String, u32>,
    pub created_at_unix: u64,
    pub updated_at_unix: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaseSummary {
    pub id: String,
    pub title: String,
    pub tags: Vec<String>,
    pub status: CaseStatus,
    pub chart_snapshot_id: String,
    pub analysis_snapshot_id: String,
}

impl CaseRecord {
    pub fn summary(&self) -> CaseSummary {
        CaseSummary {
            id: self.id.clone(),
            title: self.title.clone(),
            tags: self.tags.clone(),
            status: self.status.clone(),
            chart_snapshot_id: self.chart_snapshot.snapshot_id.clone(),
            analysis_snapshot_id: self.analysis_snapshot.snapshot_id.clone(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SharePreset {
    pub id: String,
    pub label: String,
    pub hide_name: bool,
    pub hide_location: bool,
    pub hide_exact_time: bool,
}

#[derive(Clone, Debug)]
pub struct CaseRepository {
    cases: BTreeMap<String, CaseRecord>,
}

impl CaseRepository {
    pub fn new() -> Self {
        Self {
            cases: BTreeMap::new(),
        }
    }

    pub fn upsert(&mut self, record: CaseRecord) {
        self.cases.insert(record.id.clone(), record);
    }

    pub fn get(&self, id: &str) -> Option<&CaseRecord> {
        self.cases
            .get(id)
            .filter(|case| case.status != CaseStatus::Deleted)
    }

    pub fn list(&self) -> Vec<CaseSummary> {
        self.cases
            .values()
            .filter(|case| case.status != CaseStatus::Deleted)
            .map(CaseRecord::summary)
            .collect()
    }

    pub fn derive_stats(&self) -> CaseDerivedStats {
        let active: Vec<_> = self.cases.values()
            .filter(|c| c.status != CaseStatus::Deleted)
            .collect();
        let total = active.len() as u32;
        let mut day_masters = BTreeMap::new();
        let mut elements = BTreeMap::new();
        let mut ten_gods = BTreeMap::new();
        let mut hour_distribution = BTreeMap::new();
        for c in &active {
            *day_masters.entry(c.chart_snapshot.day_master.clone()).or_default() += 1u32;
            *hour_distribution.entry(c.chart_snapshot.hour_branch.clone()).or_default() += 1u32;
            for (el, count) in &c.element_counts {
                *elements.entry(el.clone()).or_default() += *count;
            }
            for (tg, count) in &c.ten_god_counts {
                *ten_gods.entry(tg.clone()).or_default() += *count;
            }
        }
        CaseDerivedStats { total_cases: total, day_masters, elements, ten_gods, hour_distribution }
    }

    pub fn update_metadata(
        &mut self,
        id: &str,
        title: Option<String>,
        tags: Option<Vec<String>>,
        private_note: Option<Option<String>>,
        updated_at_unix: u64,
    ) -> Option<CaseRecord> {
        let record = self.cases.get_mut(id)?;
        if record.status == CaseStatus::Deleted {
            return None;
        }
        if let Some(title) = title {
            record.title = title;
        }
        if let Some(tags) = tags {
            record.tags = tags;
        }
        if let Some(private_note) = private_note {
            record.private_note = private_note;
        }
        record.updated_at_unix = updated_at_unix;
        Some(record.clone())
    }

    pub fn archive(&mut self, id: &str, updated_at_unix: u64) -> Option<CaseRecord> {
        let record = self.cases.get_mut(id)?;
        if record.status == CaseStatus::Deleted {
            return None;
        }
        record.status = CaseStatus::Archived;
        record.updated_at_unix = updated_at_unix;
        Some(record.clone())
    }

    pub fn delete(&mut self, id: &str, updated_at_unix: u64) -> Option<CaseRecord> {
        let record = self.cases.get_mut(id)?;
        record.status = CaseStatus::Deleted;
        record.updated_at_unix = updated_at_unix;
        Some(record.clone())
    }
}

impl Default for CaseRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(id: &str) -> CaseRecord {
        CaseRecord {
            id: id.to_string(),
            title: "Test".to_string(),
            tags: vec!["sample".to_string()],
            private_note: Some("private".to_string()),
            status: CaseStatus::Active,
            chart_snapshot: ChartSnapshot {
                snapshot_id: format!("{id}:chart:v1"),
                chart_algo_version: "chart-engine-android-date-layer-v1".to_string(),
                ruleset_id: "ft-v1-default".to_string(),
                day_master: "\u{5e9a}".to_string(),
                hour_branch: "\u{5df3}".to_string(),
            },
            analysis_snapshot: AnalysisSnapshotRef {
                snapshot_id: format!("{id}:analysis:v1"),
                analysis_algo_version: "structured-analysis-v1".to_string(),
                disclaimer_id: "traditional-interpretation-not-professional-advice-v1".to_string(),
            },
            element_counts: BTreeMap::new(),
            ten_god_counts: BTreeMap::new(),
            created_at_unix: 1,
            updated_at_unix: 1,
        }
    }

    #[test]
    fn list_omits_private_note_and_deleted_cases() {
        let mut repo = CaseRepository::new();
        repo.upsert(record("case-a"));
        repo.upsert(record("case-b"));
        repo.delete("case-b", 2);

        let list = repo.list();

        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, "case-a");
    }

    #[test]
    fn metadata_update_preserves_snapshot_versions() {
        let mut repo = CaseRepository::new();
        repo.upsert(record("case-a"));
        let before = repo.get("case-a").unwrap().chart_snapshot.clone();

        let updated = repo
            .update_metadata("case-a", Some("Updated".to_string()), None, Some(None), 3)
            .unwrap();

        assert_eq!(updated.chart_snapshot, before);
        assert_eq!(updated.private_note, None);
        assert_eq!(updated.title, "Updated");
    }
}
