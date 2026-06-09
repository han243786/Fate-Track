use crate::domain::cases::{AnalysisSnapshotRef, CaseRecord, ChartSnapshot};
use std::collections::BTreeMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub const SHARE_STORE_VERSION: &str = "local-volatile-share-store-v1";
pub const PUBLIC_SHARE_DTO_VERSION: &str = "public-share-redacted-v1";
pub const UNAVAILABLE_MESSAGE: &str = "share unavailable";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShareRecord {
    pub share_id: String,
    pub case_id: String,
    pub token_hash: String,
    pub created_at_unix: u64,
    pub expires_at_unix: u64,
    pub revoked_at_unix: Option<u64>,
    pub snapshot: RedactedShareSnapshot,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RedactedShareSnapshot {
    pub label: String,
    pub chart_snapshot: ChartSnapshot,
    pub analysis_snapshot: AnalysisSnapshotRef,
    pub noindex: bool,
    pub editable: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShareCreateResult {
    pub record: ShareRecord,
    pub token: String,
}

#[derive(Clone, Debug)]
pub struct ShareRepository {
    shares: BTreeMap<String, ShareRecord>,
    next_id: u64,
}

impl ShareRepository {
    pub fn new() -> Self {
        Self {
            shares: BTreeMap::new(),
            next_id: 1,
        }
    }

    pub fn create(
        &mut self,
        case: &CaseRecord,
        ttl_seconds: u64,
        now_unix: u64,
    ) -> ShareCreateResult {
        let share_id = format!("share-{}", self.next_id);
        self.next_id += 1;
        let token = token_for(&share_id, now_unix);
        let record = ShareRecord {
            share_id: share_id.clone(),
            case_id: case.id.clone(),
            token_hash: hash_token(&token),
            created_at_unix: now_unix,
            expires_at_unix: now_unix.saturating_add(ttl_seconds.max(1)),
            revoked_at_unix: None,
            snapshot: RedactedShareSnapshot {
                label: "Shared chart".to_string(),
                chart_snapshot: case.chart_snapshot.clone(),
                analysis_snapshot: case.analysis_snapshot.clone(),
                noindex: true,
                editable: false,
            },
        };

        self.shares.insert(share_id, record.clone());

        ShareCreateResult { record, token }
    }

    pub fn resolve_public(&self, token: &str, now_unix: u64) -> Option<RedactedShareSnapshot> {
        let token_hash = hash_token(token);
        self.shares
            .values()
            .find(|record| {
                record.token_hash == token_hash
                    && record.revoked_at_unix.is_none()
                    && record.expires_at_unix > now_unix
            })
            .map(|record| record.snapshot.clone())
    }

    pub fn revoke(&mut self, token: &str, now_unix: u64) -> bool {
        let token_hash = hash_token(token);
        let Some(record) = self
            .shares
            .values_mut()
            .find(|record| record.token_hash == token_hash)
        else {
            return false;
        };
        if record.expires_at_unix <= now_unix {
            return false;
        }

        record.revoked_at_unix = Some(now_unix);
        true
    }
}

impl Default for ShareRepository {
    fn default() -> Self {
        Self::new()
    }
}

pub fn hash_token(token: &str) -> String {
    let mut hasher = DefaultHasher::new();
    token.hash(&mut hasher);
    format!("{:016x}", hasher.finish())
}

fn token_for(share_id: &str, now_unix: u64) -> String {
    format!("{share_id}.{now_unix}.local")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn case_record() -> CaseRecord {
        CaseRecord {
            id: "case-private".to_string(),
            title: "Private Name".to_string(),
            tags: vec!["secret-tag".to_string()],
            private_note: Some("secret-note".to_string()),
            status: crate::domain::cases::CaseStatus::Active,
            chart_snapshot: ChartSnapshot {
                snapshot_id: "case-private:chart:v1".to_string(),
                chart_algo_version: "chart-engine-android-date-layer-v1".to_string(),
                ruleset_id: "ft-v1-default".to_string(),
                day_master: "\u{5e9a}".to_string(),
            },
            analysis_snapshot: AnalysisSnapshotRef {
                snapshot_id: "case-private:analysis:v1".to_string(),
                analysis_algo_version: "structured-analysis-v1".to_string(),
                disclaimer_id: "traditional-interpretation-not-professional-advice-v1".to_string(),
            },
            created_at_unix: 1,
            updated_at_unix: 1,
        }
    }

    #[test]
    fn stores_only_token_hash_and_builds_redacted_snapshot() {
        let mut repo = ShareRepository::new();
        let created = repo.create(&case_record(), 60, 100);

        assert_ne!(created.record.token_hash, created.token);
        assert_eq!(created.record.snapshot.label, "Shared chart");
        assert!(created.record.snapshot.noindex);
        assert!(!created.record.snapshot.editable);
        assert_eq!(
            created.record.snapshot.chart_snapshot.chart_algo_version,
            "chart-engine-android-date-layer-v1"
        );
    }

    #[test]
    fn revoked_or_expired_share_does_not_resolve() {
        let mut repo = ShareRepository::new();
        let created = repo.create(&case_record(), 1, 100);

        assert!(repo.resolve_public(&created.token, 100).is_some());
        assert!(repo.resolve_public(&created.token, 101).is_none());

        let created = repo.create(&case_record(), 60, 200);
        assert!(repo.revoke(&created.token, 201));
        assert!(repo.resolve_public(&created.token, 202).is_none());
    }
}
