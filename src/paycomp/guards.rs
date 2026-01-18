use crate::shards::{PaycompShard, CorridorCoordinate};
use crate::subcent::MillTransaction;
use crate::did_types::Did;

/// Static corridor thresholds (would typically come from ALN grammar / config shards).
pub struct CorridorThresholds {
    pub max_risk_r: f32,
    pub min_knowledge_k: f32,
    pub min_eco_e: f32,
}

pub struct GuardContext {
    pub thresholds: CorridorThresholds,
    /// Latest shards for the from/to DIDs.
    pub from_shard: PaycompShard,
    pub to_shard: PaycompShard,
}

fn corridor_value(shard: &PaycompShard, name: &str) -> Option<f32> {
    shard
        .corridors
        .iter()
        .find(|c: &&CorridorCoordinate| c.name == name)
        .map(|c| c.rx)
}

/// Returns true if transaction is admissible under K/E/R and corridor invariants.
pub fn admit_transaction(ctx: &GuardContext, tx: &MillTransaction) -> bool {
    // 1. Basic K/E/R thresholds for both parties.
    if ctx.from_shard.ker.k < ctx.thresholds.min_knowledge_k
        || ctx.to_shard.ker.k < ctx.thresholds.min_knowledge_k
    {
        return false;
    }
    if ctx.from_shard.ker.e < ctx.thresholds.min_eco_e
        || ctx.to_shard.ker.e < ctx.thresholds.min_eco_e
    {
        return false;
    }
    if ctx.from_shard.ker.r > ctx.thresholds.max_risk_r
        || ctx.to_shard.ker.r > ctx.thresholds.max_risk_r
    {
        return false;
    }

    // 2. Example corridor: leverage / debt-to-income must stay in-band.
    if let Some(rx_leverage) = corridor_value(&ctx.from_shard, "debt_to_income") {
        if rx_leverage > 1.0 {
            // Out-of-band leverage: no new credit extension.
            return false;
        }
    }

    // 3. Example corridor: outage exposure or surveillance risk cannot increase beyond 1.0.
    if let Some(rx_surv) = corridor_value(&ctx.from_shard, "surveillance_risk") {
        if rx_surv > 1.0 {
            return false;
        }
    }

    true
}
