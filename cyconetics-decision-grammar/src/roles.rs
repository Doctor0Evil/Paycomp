use uuid::Uuid;

use crate::types::{
    DecisionKind, DecisionRecord, DecisionRole, EvolutionId, HostDid, RoH, RoHBound30, UpgradeId,
};

pub trait NeurorightsDecider {
    fn decide_neurorights(
        &self,
        host: &HostDid,
        upgrade: &UpgradeId,
        evo: &EvolutionId,
        roh_before: Option<RoH>,
        roh_after: Option<RoH>,
        roh_bound: Option<RoHBound30>,
        reason_code: &str,
        notes: &str,
        now_ms: i64,
    ) -> DecisionRecord;
}

pub trait SafetyDecider {
    fn decide_safety(
        &self,
        host: &HostDid,
        upgrade: &UpgradeId,
        evo: &EvolutionId,
        roh_before: Option<RoH>,
        roh_after: Option<RoH>,
        roh_bound: Option<RoHBound30>,
        reason_code: &str,
        notes: &str,
        now_ms: i64,
    ) -> DecisionRecord;
}

pub trait HostSelfDecider {
    fn self_veto(
        &self,
        host: &HostDid,
        upgrade: &UpgradeId,
        evo: &EvolutionId,
        roh_before: Option<RoH>,
        reason_code: &str,
        notes: &str,
        now_ms: i64,
    ) -> DecisionRecord;
}

pub struct NeurorightsBoard;
pub struct SafetyBoard;
pub struct HostSelf;

impl NeurorightsDecider for NeurorightsBoard {
    fn decide_neurorights(
        &self,
        host: &HostDid,
        upgrade: &UpgradeId,
        evo: &EvolutionId,
        roh_before: Option<RoH>,
        roh_after: Option<RoH>,
        roh_bound: Option<RoHBound30>,
        reason_code: &str,
        notes: &str,
        now_ms: i64,
    ) -> DecisionRecord {
        DecisionRecord {
            record_id: Uuid::new_v4(),
            host_did: host.clone(),
            upgrade_id: upgrade.clone(),
            evolution_id: evo.clone(),
            role: DecisionRole::NeurorightsDecider,
            kind: DecisionKind::Authorize,
            timestamp_ms: now_ms,
            roh_before,
            roh_after,
            roh_bound,
            reason_code: reason_code.to_string(),
            notes: notes.to_string(),
        }
    }
}

impl SafetyDecider for SafetyBoard {
    fn decide_safety(
        &self,
        host: &HostDid,
        upgrade: &UpgradeId,
        evo: &EvolutionId,
        roh_before: Option<RoH>,
        roh_after: Option<RoH>,
        roh_bound: Option<RoHBound30>,
        reason_code: &str,
        notes: &str,
        now_ms: i64,
    ) -> DecisionRecord {
        DecisionRecord {
            record_id: Uuid::new_v4(),
            host_did: host.clone(),
            upgrade_id: upgrade.clone(),
            evolution_id: evo.clone(),
            role: DecisionRole::SafetyDecider,
            kind: DecisionKind::Approve,
            timestamp_ms: now_ms,
            roh_before,
            roh_after,
            roh_bound,
            reason_code: reason_code.to_string(),
            notes: notes.to_string(),
        }
    }
}

impl HostSelfDecider for HostSelf {
    fn self_veto(
        &self,
        host: &HostDid,
        upgrade: &UpgradeId,
        evo: &EvolutionId,
        roh_before: Option<RoH>,
        reason_code: &str,
        notes: &str,
        now_ms: i64,
    ) -> DecisionRecord {
        DecisionRecord {
            record_id: Uuid::new_v4(),
            host_did: host.clone(),
            upgrade_id: upgrade.clone(),
            evolution_id: evo.clone(),
            role: DecisionRole::HostSelf,
            kind: DecisionKind::Reject,
            timestamp_ms: now_ms,
            roh_before,
            roh_after: roh_before,
            roh_bound: None,
            reason_code: reason_code.to_string(),
            notes: notes.to_string(),
        }
    }
}
