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
use serde::{Deserialize, Serialize};

/// Minimum bundle size for meaningful RoH evidence
pub const MIN_EVIDENCE_POINTS: usize = 10;

/// Evidence bundle for RoH estimation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EvidenceBundle {
    pub biokarma_points: Vec<f32>,
    pub device_hour_points: Vec<f32>,
    pub eco_impact_points: Vec<f32>,
}

impl EvidenceBundle {
    pub fn is_sufficient(&self) -> bool {
        self.biokarma_points.len() >= MIN_EVIDENCE_POINTS
            && self.device_hour_points.len() >= MIN_EVIDENCE_POINTS
            && self.eco_impact_points.len() >= MIN_EVIDENCE_POINTS
    }
}

/// Example function subject to 10-element bundle requirement
pub fn roh_from_biokarma(bundle: &EvidenceBundle) -> Option<RoH> {
    if !bundle.is_sufficient() {
        return None;
    }
    let avg_bio = bundle.biokarma_points.iter().copied().sum::<f32>()
        / (bundle.biokarma_points.len() as f32);
    let avg_device = bundle.device_hour_points.iter().copied().sum::<f32>()
        / (bundle.device_hour_points.len() as f32);
    let avg_eco = bundle.eco_impact_points.iter().copied().sum::<f32>()
        / (bundle.eco_impact_points.len() as f32);

    let roh = 0.4 * (1.0 - avg_bio) + 0.3 * avg_device + 0.3 * (1.0 - avg_eco);
    Some(roh.clamp(0.0, 1.0))
}
