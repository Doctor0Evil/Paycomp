use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Residual risk-of-harm in [0.0, 1.0]
pub type RoH = f32;

/// Strongly-typed bound RoH ≤ 0.30 (compile-time marker, run-time checked)
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct RoHBound30(RoH);

impl RoHBound30 {
    pub const MAX: RoH = 0.30;

    pub fn new(value: RoH) -> Option<Self> {
        if value <= Self::MAX && value >= 0.0 {
            Some(Self(value))
        } else {
            None
        }
    }

    pub fn get(self) -> RoH {
        self.0
    }
}

impl fmt::Display for RoHBound30 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.4}", self.0)
    }
}

/// High-level decision kinds across the evolution graph
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum DecisionKind {
    Propose,
    Authorize,
    Approve,
    Reject,
    Defer,
    Escalate,
}

/// Fine-grained upgrade decision outcome
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum UpgradeDecision {
    Approved(RoHBound30),
    Authorized(RoHBound30),
    Rejected,
    Escalated,
    Deferred,
    Denied, // guard-level denial, no state change
}

/// Unique identifiers wiring into existing safety spine
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct HostDid(pub String);      // DID for host (person/agent)
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UpgradeId(pub String);    // upgrade / protocol / model
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EvolutionId(pub String);  // evolution step / graph node id

/// Role identifiers
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DecisionRole {
    HostSelf,
    NeurorightsDecider,
    SafetyDecider,
    GovernanceDecider,
}

/// Core decision record compatible with Cyconetics spine
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecisionRecord {
    pub record_id: Uuid,
    pub host_did: HostDid,
    pub upgrade_id: UpgradeId,
    pub evolution_id: EvolutionId,
    pub role: DecisionRole,
    pub kind: DecisionKind,
    pub timestamp_ms: i64,
    pub roh_before: Option<RoH>,
    pub roh_after: Option<RoH>,
    pub roh_bound: Option<RoHBound30>,
    pub reason_code: String,
    pub notes: String,
}
