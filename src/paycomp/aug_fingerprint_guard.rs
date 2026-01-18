use std::time::{Duration, SystemTime};

/// Core scalar for K/E/R scoring.
#[derive(Debug, Clone, Copy)]
pub struct KerScore {
    pub knowledge: f32,    // 0.0–1.0
    pub eco_impact: f32,   // 0.0–1.0
    pub risk_of_harm: f32, // 0.0–1.0
}

/// Internal neuro/actuator state snapshot.
#[derive(Debug, Clone, Copy)]
pub struct NeuroState {
    pub s_value: f32,          // normalized internal state S_t
    pub load_value: f32,       // organic_cpu load L_t
    pub s_min: f32,            // lower bound of safe corridor
    pub s_max: f32,            // upper bound of safe corridor
    pub load_max: f32,         // maximum allowed load in safe band
    pub last_update: SystemTime,
}

/// Policy for AI‑consent behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiConsentPolicy {
    Conservative,
    Balanced,
}

/// Control mode for user interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlMode {
    InternalBiophysical,
    ExternalSwitch,
    Voice,
    Mixed,
}

/// Result of a consent evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsentDecision {
    Allow,
    Deny,
    Defer,
}

/// Shard representing a single Aug_Fingerprint wallet for an implanted NFC user.
#[derive(Debug, Clone)]
pub struct AugFingerprintShard {
    pub wallet_did: String,
    pub interface_type: String,     // "implanted_nfc"
    pub control_mode: ControlMode,  // InternalBiophysical for your profile
    pub assist_mode: bool,          // always true for this profile
    pub direct_user_gesture: bool,  // false for internal‑only control
    pub consent_channel: String,    // "AI_inferred_state_only"
    pub quantified_learning: bool,  // true when AI is learning lane patterns

    // Accessibility and neuro/latency profile
    pub speech_reliable: bool,
    pub mobility_profile: String,
    pub oculus_cortex_calibrated: bool,
    pub latency_profile: String,    // e.g., "spiky"
    pub max_cognitive_load: f32,    // normalized 0.0–1.0

    // Spending corridors
    pub max_auto_amount_mills: u64, // sub‑cent unit: 1 = 0.001 USD
    pub max_daily_spend_mills: u64,
    pub max_payments_per_hour: u32,
    pub max_prompts_per_hour: u32,

    // Quantified‑learning consent parameters
    pub learning_mode: String,          // "quantified_lane_switching"
    pub ai_consent_policy: AiConsentPolicy,
    pub min_stability_time: Duration,   // minimum time in safe corridor
    pub consent_suspended: bool,

    // Dynamic state
    pub neuro_state: NeuroState,
    pub stable_since: Option<SystemTime>,
    pub payments_last_hour: u32,
    pub prompts_last_hour: u32,
    pub last_reset_window: SystemTime,

    // Risk/eco scoring
    pub ker_score: KerScore,
    pub r_privacy: f32,
    pub r_fraud: f32,
    pub r_tracking: f32,
    pub e_accessibility: f32,

    // Audit
    pub consent_audit_log_enabled: bool,
}

impl AugFingerprintShard {
    pub fn new(wallet_did: String, now: SystemTime) -> Self {
        Self {
            wallet_did,
            interface_type: "implanted_nfc".to_string(),
            control_mode: ControlMode::InternalBiophysical,
            assist_mode: true,
            direct_user_gesture: false,
            consent_channel: "AI_inferred_state_only".to_string(),
            quantified_learning: true,
            speech_reliable: false,
            mobility_profile: "limited_precision_low_frequency".to_string(),
            oculus_cortex_calibrated: false,
            latency_profile: "spiky".to_string(),
            max_cognitive_load: 0.4,

            max_auto_amount_mills: 50_000,      // $50.000 default cap
            max_daily_spend_mills: 200_000,     // $200.000 per day
            max_payments_per_hour: 6,
            max_prompts_per_hour: 10,

            learning_mode: "quantified_lane_switching".to_string(),
            ai_consent_policy: AiConsentPolicy::Conservative,
            min_stability_time: Duration::from_secs(5),
            consent_suspended: false,

            neuro_state: NeuroState {
                s_value: 0.0,
                load_value: 0.0,
                s_min: 0.4,
                s_max: 0.6,
                load_max: 0.5,
                last_update: now,
            },
            stable_since: None,
            payments_last_hour: 0,
            prompts_last_hour: 0,
            last_reset_window: now,

            ker_score: KerScore {
                knowledge: 0.9,
                eco_impact: 0.9,
                risk_of_harm: 0.15,
            },
            r_privacy: 0.2,
            r_fraud: 0.2,
            r_tracking: 0.2,
            e_accessibility: 0.9,

            consent_audit_log_enabled: true,
        }
    }

    pub fn update_neuro_state(&mut self, state: NeuroState) {
        self.neuro_state = state;
    }

    pub fn reset_counters_if_needed(&mut self, now: SystemTime) {
        if let Ok(delta) = now.duration_since(self.last_reset_window) {
            if delta >= Duration::from_secs(3600) {
                self.payments_last_hour = 0;
                self.prompts_last_hour = 0;
                self.last_reset_window = now;
            }
        }
    }
}

/// Payment request as seen by the guard.
#[derive(Debug, Clone)]
pub struct PaymentRequest {
    pub merchant_id: String,
    pub region_id: String,
    pub amount_mills: u64,
    pub is_essential_service: bool,
    pub now: SystemTime,
}

/// Audit record for consent decisions.
#[derive(Debug, Clone)]
pub struct ConsentAuditRecord {
    pub wallet_did: String,
    pub merchant_id: String,
    pub amount_mills: u64,
    pub decision: ConsentDecision,
    pub timestamp: SystemTime,
    pub s_value: f32,
    pub load_value: f32,
}

/// Core guard that enforces the internal‑state corridor consent model.
pub struct AugFingerprintGuard;

impl AugFingerprintGuard {
    pub fn evaluate_payment(
        shard: &mut AugFingerprintShard,
        request: &PaymentRequest,
    ) -> (ConsentDecision, Option<ConsentAuditRecord>) {
        shard.reset_counters_if_needed(request.now);

        if shard.consent_suspended && !request.is_essential_service {
            return Self::deny_with_audit(shard, request, "consent_suspended");
        }

        if shard.prompts_last_hour >= shard.max_prompts_per_hour && !request.is_essential_service {
            return Self::deny_with_audit(shard, request, "prompt_rate_exceeded");
        }

        if shard.payments_last_hour >= shard.max_payments_per_hour && !request.is_essential_service {
            return Self::deny_with_audit(shard, request, "payment_rate_exceeded");
        }

        if request.amount_mills > shard.max_auto_amount_mills {
            if !request.is_essential_service {
                return Self::deny_with_audit(shard, request, "amount_over_limit");
            }
        }

        let ns = shard.neuro_state;
        let within_s_corridor = ns.s_value >= ns.s_min && ns.s_value <= ns.s_max;
        let within_load_band = ns.load_value <= ns.load_max && ns.load_value <= shard.max_cognitive_load;

        if !within_s_corridor || !within_load_band {
            if !request.is_essential_service {
                shard.consent_suspended = true;
                return Self::deny_with_audit(shard, request, "state_outside_corridor");
            } else {
                return Self::defer_with_audit(shard, request, "essential_state_unstable");
            }
        }

        let now = request.now;
        let stable_since = match shard.stable_since {
            Some(t) => {
                if let Ok(delta) = now.duration_since(t) {
                    if delta >= shard.min_stability_time {
                        t
                    } else {
                        shard.stable_since = Some(now);
                        return Self::defer_with_audit(shard, request, "stability_time_insufficient");
                    }
                } else {
                    shard.stable_since = Some(now);
                    return Self::defer_with_audit(shard, request, "stability_time_invalid");
                }
            }
            None => {
                shard.stable_since = Some(now);
                return Self::defer_with_audit(shard, request, "stability_not_yet_established");
            }
        };

        let _ = stable_since; // kept for clarity, can be used for logging

        if shard.ai_consent_policy == AiConsentPolicy::Conservative {
            if shard.r_fraud > 0.5 || shard.r_privacy > 0.5 || shard.r_tracking > 0.5 {
                return Self::deny_with_audit(shard, request, "risk_scores_too_high");
            }
        }

        shard.payments_last_hour += 1;
        shard.prompts_last_hour += 1;

        let audit = if shard.consent_audit_log_enabled {
            Some(ConsentAuditRecord {
                wallet_did: shard.wallet_did.clone(),
                merchant_id: request.merchant_id.clone(),
                amount_mills: request.amount_mills,
                decision: ConsentDecision::Allow,
                timestamp: request.now,
                s_value: shard.neuro_state.s_value,
                load_value: shard.neuro_state.load_value,
            })
        } else {
            None
        };

        (ConsentDecision::Allow, audit)
    }

    fn deny_with_audit(
        shard: &AugFingerprintShard,
        request: &PaymentRequest,
        _reason: &str,
    ) -> (ConsentDecision, Option<ConsentAuditRecord>) {
        let audit = if shard.consent_audit_log_enabled {
            Some(ConsentAuditRecord {
                wallet_did: shard.wallet_did.clone(),
                merchant_id: request.merchant_id.clone(),
                amount_mills: request.amount_mills,
                decision: ConsentDecision::Deny,
                timestamp: request.now,
                s_value: shard.neuro_state.s_value,
                load_value: shard.neuro_state.load_value,
            })
        } else {
            None
        };
        (ConsentDecision::Deny, audit)
    }

    fn defer_with_audit(
        shard: &AugFingerprintShard,
        request: &PaymentRequest,
        _reason: &str,
    ) -> (ConsentDecision, Option<ConsentAuditRecord>) {
        let audit = if shard.consent_audit_log_enabled {
            Some(ConsentAuditRecord {
                wallet_did: shard.wallet_did.clone(),
                merchant_id: request.merchant_id.clone(),
                amount_mills: request.amount_mills,
                decision: ConsentDecision::Defer,
                timestamp: request.now,
                s_value: shard.neuro_state.s_value,
                load_value: shard.neuro_state.load_value,
            })
        } else {
            None
        };
        (ConsentDecision::Defer, audit)
    }
}
