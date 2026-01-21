use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatStakeAsset {
    pub asset_id: String, // asset.chat.stake.v1
    pub owner_did: String,
    pub staked_micro_usd: i64,
    pub k_score: f32,
    pub e_score: f32,
    pub r_score: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebsiteGovernanceConfig {
    pub shard_id: String, // governance.chat.website.v1
    pub max_risk_per_page: f32,
    pub neurorights_envelope_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentPageShard {
    pub shard_id: String, // content.website.governance.v1
    pub page_path: String,
    pub hex_stamp: String,
    pub k_score: f32,
    pub e_score: f32,
    pub r_score: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GovernanceTotem {
    pub shard_id: String, // governance.totem.superposition.v1
    pub config_hash: String,
    pub last_audit_ms: i64,
}
