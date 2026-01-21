use crate::types::{RoH, RoHBound30, UpgradeDecision};

/// Host state summary used for RoH prediction
#[derive(Clone, Debug)]
pub struct RoHGuardedHostState {
    pub roh_baseline: RoH,
    pub bio_k_score: f32,
    pub env_risk_score: f32,
    pub upgrade_intensity: f32,
}

impl RoHGuardedHostState {
    pub fn new(
        roh_baseline: RoH,
        bio_k_score: f32,
        env_risk_score: f32,
        upgrade_intensity: f32,
    ) -> Self {
        Self {
            roh_baseline,
            bio_k_score,
            env_risk_score,
            upgrade_intensity,
        }
    }
}

/// Simple composable predictor (replace with your learned model)
pub fn predict_roh(state: &RoHGuardedHostState) -> RoH {
    let mut roh = state.roh_baseline
        + 0.20 * state.upgrade_intensity
        + 0.15 * state.env_risk_score
        - 0.10 * state.bio_k_score;

    if roh < 0.0 {
        roh = 0.0;
    } else if roh > 1.0 {
        roh = 1.0;
    }
    roh
}

/// Core guard: produce an UpgradeDecision and optional RoHBound30
pub fn guarded_decision(predicted: RoH) -> (UpgradeDecision, Option<RoHBound30>) {
    if let Some(bound) = RoHBound30::new(predicted) {
        (UpgradeDecision::Approved(bound), Some(bound))
    } else {
        (UpgradeDecision::Denied, None)
    }
}
