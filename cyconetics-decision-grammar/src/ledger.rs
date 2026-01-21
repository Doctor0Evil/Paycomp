use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::{
    DecisionKind, DecisionRecord, DecisionRole, EvolutionId, HostDid, RoH, RoHBound30, UpgradeId,
};

/// On-ledger entry keyed by (host_did, upgrade_id, evolution_id)
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecisionLedgerEntry {
    pub entry_id: Uuid,
    pub host_did: HostDid,
    pub upgrade_id: UpgradeId,
    pub evolution_id: EvolutionId,

    pub role: DecisionRole,
    pub kind: DecisionKind,

    pub roh_before: Option<RoH>,
    pub roh_after: Option<RoH>,
    pub roh_bound: Option<RoHBound30>,

    pub reason_code: String,
    pub notes: String,

    /// Physiological / context telemetry hashes
    pub biophysical_envelope_hash: String,
    pub env_context_hash: String,

    /// Governance / audit fields
    pub governance_envelope_hash: String,
    pub decision_record_hash: String,
    pub did_signature_hex: String,
    pub timestamp_ms: i64,
}
