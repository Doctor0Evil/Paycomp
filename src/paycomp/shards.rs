use serde::{Deserialize, Serialize};
use crate::did_types::{Did, DidDocumentRef};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum NodeType {
    CitizenWallet,
    MerchantWallet,
    BankNode,
    MunicipalTreasury,
    PaymentGateway,
    HardwareTerminal,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KerScores {
    /// Knowledge-factor: fraction of critical fields that are equation/data-backed (0–1).
    pub k: f32,
    /// Eco-impact: normalized benefit (e.g., GWP reduction per $1,000 transacted) (0–1).
    pub e: f32,
    /// Risk-of-harm: bounded residual risk (0–1).
    pub r: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CorridorCoordinate {
    /// Name of the corridor (e.g., "debt_to_income", "gwp_per_1k", "outage_exposure").
    pub name: String,
    /// Normalized coordinate r_x in  for in-band operation; outside  is violation.
    pub rx: f32,
    /// Optional human-readable unit / description.
    pub unit: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PaycompShard {
    /// DID of the node this shard describes.
    pub node_did: Did,
    /// DID document reference (keys, metadata).
    pub node_doc: DidDocumentRef,
    /// Node type (citizen, merchant, bank, etc.).
    pub node_type: NodeType,
    /// Region or corridor ID (e.g., "phoenix.district.12").
    pub region_id: String,
    /// Time window this shard summarizes (e.g., day, hour, or block index).
    pub window_start_utc: i64,
    pub window_end_utc: i64,

    /// Total USD-equivalent in-flow and out-flow over this period.
    pub usd_in: f64,
    pub usd_out: f64,

    /// Total number of transactions and sub-cent residuals processed.
    pub tx_count: u64,
    /// Net residual delta due to sub-cent precision (sum of +/− 0.001 deltas over window).
    pub residual_sum_mills: i64,

    /// Computed eco metrics for this node and window.
    /// Example: kg CO2e per $1,000, m^3 water recharged per $1,000, etc.
    pub gwp_kgco2_per_1k: f32,
    pub water_recharge_m3_per_1k: f32,

    /// K/E/R scores for this shard/window.
    pub ker: KerScores,

    /// Corridor coordinates for key variables (leverage, outage risk, surveillance risk, etc.).
    pub corridors: Vec<CorridorCoordinate>,

    /// DID of the author (e.g., city ecosafety office, independent auditor, or auto-guard process).
    pub authored_by: Did,
    /// Unix time of authorship.
    pub authored_at_utc: i64,
    /// DID signature over the serialized shard payload.
    pub signature: String,
}
