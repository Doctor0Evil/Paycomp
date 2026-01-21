#[macro_export]
macro_rules! decision_roles {
    () => {
        use crate::types::{DecisionKind, DecisionRecord, DecisionRole, EvolutionId, HostDid, RoH, RoHBound30, UpgradeId};
        use crate::roles::{HostSelf, HostSelfDecider, NeurorightsBoard, NeurorightsDecider, SafetyBoard, SafetyDecider};
        use uuid::Uuid;

        pub fn host_self_veto(
            host: &HostDid,
            upgrade: &UpgradeId,
            evo: &EvolutionId,
            roh_before: Option<RoH>,
            reason_code: &str,
            notes: &str,
            now_ms: i64,
        ) -> DecisionRecord {
            let actor = HostSelf;
            actor.self_veto(host, upgrade, evo, roh_before, reason_code, notes, now_ms)
        }

        pub fn neurorights_authorize(
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
            let board = NeurorightsBoard;
            board.decide_neurorights(
                host,
                upgrade,
                evo,
                roh_before,
                roh_after,
                roh_bound,
                reason_code,
                notes,
                now_ms,
            )
        }

        pub fn safety_approve(
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
            let board = SafetyBoard;
            board.decide_safety(
                host,
                upgrade,
                evo,
                roh_before,
                roh_after,
                roh_bound,
                reason_code,
                notes,
                now_ms,
            )
        }

        /// Structural veto: any evolution graph or scheduler must check for host veto
        pub fn has_host_veto(decisions: &[DecisionRecord]) => bool {
            decisions.iter().any(|d| {
                matches!(d.role, DecisionRole::HostSelf)
                    && matches!(d.kind, DecisionKind::Reject | DecisionKind::Escalate)
            })
        }
    };
}
