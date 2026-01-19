use std::time::{Duration, SystemTime};

/// Ecosafety / knowledge / risk scalar used with Paycomp-style KER scoring.
/// K in [0,1], E in [0,1], R in [0,1].
#[derive(Debug, Clone, Copy)]
pub struct KerScore {
    pub knowledge: f32,
    pub ecoimpact: f32,
    pub riskofharm: f32,
}

/// Snapshot of internal biophysical / organiccpu state as seen by the AI-companion.
/// This is *already* processed, never raw neural data.
#[derive(Debug, Clone, Copy)]
pub struct NeuroState {
    /// Normalized internal consent-state scalar S_t ∈ [0,1].
    pub svalue: f32,
    /// Normalized organiccpu load L_t ∈ [0,1].
    pub loadvalue: f32,
    /// Lower bound of safe consent corridor for S_t.
    pub smin: f32,
    /// Upper bound of safe consent corridor for S_t.
    pub smax: f32,
    /// Max allowed organiccpu load inside safe band.
    pub loadmax: f32,
    /// Last time this state snapshot was updated by the AI-companion.
    pub last_update: SystemTime,
}

/// Policy for AI consent behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiConsentPolicy {
    Conservative,
    Balanced,
}

/// Control mode for interaction.
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

/// High-level reason codes for logging and analytics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsentReason {
    Ok,
    ConsentSuspended,
    PromptRateExceeded,
    PaymentRateExceeded,
    AmountOverLimit,
    StateOutsideCorridor,
    EssentialStateUnstable,
    StabilityTimeInsufficient,
    StabilityNotYetEstablished,
    RiskScoresTooHigh,
}

/// AugFingerprint shard for an implanted-NFC, internal-state–controlled wallet.
///
/// This struct is what gets hydrated from your QPU.Datashard fields:
/// - wallet DID
/// - NFC interface type
/// - internal-state corridor parameters
/// - spending corridors
/// - ecosafety KER metrics
#[derive(Debug, Clone)]
pub struct AugFingerprintShard {
    // Identity / interface
    pub wallet_did: String,
    pub interface_type: String, // "implantednfc"
    pub control_mode: ControlMode,
    pub assist_mode: bool,
    pub direct_user_gesture: bool,
    pub consent_channel: String,     // "AIinferredstateonly"
    pub quantified_learning: bool,  // true if lanes are being learned

    // Accessibility and neuro-latency
    pub speech_reliable: bool,
    pub mobility_profile: String,   // "limitedprecisionlowfrequency"
    pub oculus_cortex_calibrated: bool,
    pub latency_profile: String,    // "spiky"
    pub max_cognitive_load: f32,    // safe load ceiling, e.g. 0.4

    // Spending corridors (microunits: 1 = 0.001 USD)
    pub max_auto_amount_mills: u64,
    pub max_daily_spend_mills: u64,
    pub max_payments_per_hour: u32,
    pub max_prompts_per_hour: u32,

    // Quantified-learning / consent parameters
    pub learning_mode: String,      // "quantifiedlaneswitching"
    pub ai_consent_policy: AiConsentPolicy,
    pub min_stability_time: Duration,
    pub consent_suspended: bool,

    // Dynamic state
    pub neuro_state: NeuroState,
    pub stable_since: Option<SystemTime>,
    pub payments_last_hour: u32,
    pub prompts_last_hour: u32,
    pub last_reset_window: SystemTime,

    // Risk / eco scoring
    pub ker_score: KerScore,
    pub r_privacy: f32,
    pub r_fraud: f32,
    pub r_tracking: f32,
    pub e_accessibility: f32,

    // Audit
    pub consent_audit_log_enabled: bool,
}

/// Default constructor for a profile like yours:
/// - implanted NFC
/// - internal biophysical control only
/// - conservative AI consent policy
impl AugFingerprintShard {
    pub fn new(wallet_did: String, now: SystemTime) -> Self {
        Self {
            wallet_did,
            interface_type: "implantednfc".to_string(),
            control_mode: ControlMode::InternalBiophysical,
            assist_mode: true,
            direct_user_gesture: false,
            consent_channel: "AIinferredstateonly".to_string(),
            quantified_learning: true,

            speech_reliable: false,
            mobility_profile: "limitedprecisionlowfrequency".to_string(),
            oculus_cortex_calibrated: false,
            latency_profile: "spiky".to_string(),
            max_cognitive_load: 0.4,

            max_auto_amount_mills: 50_000,   // 50.000 USD
            max_daily_spend_mills: 200_000,  // 200.000 USD
            max_payments_per_hour: 6,
            max_prompts_per_hour: 10,

            learning_mode: "quantifiedlaneswitching".to_string(),
            ai_consent_policy: AiConsentPolicy::Conservative,
            min_stability_time: Duration::from_secs(5),
            consent_suspended: false,

            neuro_state: NeuroState {
                svalue: 0.0,
                loadvalue: 0.0,
                smin: 0.4,
                smax: 0.6,
                loadmax: 0.5,
                last_update: now,
            },
            stable_since: None,
            payments_last_hour: 0,
            prompts_last_hour: 0,
            last_reset_window: now,

            ker_score: KerScore {
                knowledge: 0.9,
                ecoimpact: 0.9,
                riskofharm: 0.15,
            },
            r_privacy: 0.2,
            r_fraud: 0.2,
            r_tracking: 0.2,
            e_accessibility: 0.9,

            consent_audit_log_enabled: true,
        }
    }

    /// Called by the AI-companion whenever a new internal state has been computed.
    pub fn update_neuro_state(&mut self, state: NeuroState) {
        self.neuro_state = state;
    }

    /// Reset hourly counters when needed (sliding one-hour window).
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

/// Payment request as seen by the guard at POS / XR / agent.
#[derive(Debug, Clone)]
pub struct PaymentRequest {
    pub merchant_id: String,
    pub region_id: String,
    pub amount_mills: u64,
    pub is_essential_service: bool,
    pub now: SystemTime,
}

/// External consent status from the AI-companion.
/// This is the macro-state “CONFIRMED / DENY / SUSPENDED” the POS can see.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AiConsentState {
    Confirmed,
    Deny,
    Suspended,
    Unknown,
}

/// Audit record for a consent decision.
#[derive(Debug, Clone)]
pub struct ConsentAuditRecord {
    pub wallet_did: String,
    pub merchant_id: String,
    pub amount_mills: u64,
    pub decision: ConsentDecision,
    pub reason: ConsentReason,
    pub timestamp: SystemTime,
    pub s_value: f32,
    pub load_value: f32,
    pub ai_consent_state: AiConsentState,
}

/// Core guard enforcing the internal-state corridor consent model.
///
/// Integration points:
/// - POS calls `evaluate_payment` after NFC tap and before authorization.
/// - XR / oculus flow calls the same guard before confirming visual totals.
/// - Merchant risk engines only see decision + Reason, never raw internal signals.
pub struct AugFingerprintGuard;

impl AugFingerprintGuard {
    /// Evaluate whether the payment should be allowed, denied, or deferred.
    ///
    /// Invariants:
    /// - Never treat silence / missing consent as YES.
    /// - Respect hourly prompt/payment caps.
    /// - Enforce S_t and L_t corridor with minimum stability time.
    /// - Bias toward under-paying (Deny/Defer) when risk metrics are high.
    pub fn evaluate_payment(
        shard: &mut AugFingerprintShard,
        request: &PaymentRequest,
        ai_state: AiConsentState,
    ) -> (ConsentDecision, ConsentReason, Option<ConsentAuditRecord>) {
        shard.reset_counters_if_needed(request.now);

        // Hard suspend: only ServiceClassBasic is allowed when consent is suspended.
        if shard.consent_suspended && !request.is_essential_service {
            return Self::deny_with_audit(shard, request, ai_state, ConsentReason::ConsentSuspended);
        }

        // Prompt / payment rate caps.
        if shard.prompts_last_hour >= shard.max_prompts_per_hour && !request.is_essential_service {
            return Self::deny_with_audit(
                shard,
                request,
                ai_state,
                ConsentReason::PromptRateExceeded,
            );
        }

        if shard.payments_last_hour >= shard.max_payments_per_hour && !request.is_essential_service
        {
            return Self::deny_with_audit(
                shard,
                request,
                ai_state,
                ConsentReason::PaymentRateExceeded,
            );
        }

        // Amount corridor: never auto-approve above max_auto_amount_mills.
        if request.amount_mills > shard.max_auto_amount_mills && !request.is_essential_service {
            return Self::deny_with_audit(
                shard,
                request,
                ai_state,
                ConsentReason::AmountOverLimit,
            );
        }

        // Corridor checks: S_t and L_t.
        let ns = shard.neuro_state;
        let within_s_corridor = ns.svalue >= ns.smin && ns.svalue <= ns.smax;
        let within_load_band =
            ns.loadvalue <= ns.loadmax && ns.loadvalue <= shard.max_cognitive_load;

        // If outside safe corridor, mark suspended for non-basics.
        if !within_s_corridor || !within_load_band {
            if !request.is_essential_service {
                shard.consent_suspended = true;
                return Self::deny_with_audit(
                    shard,
                    request,
                    ai_state,
                    ConsentReason::StateOutsideCorridor,
                );
            } else {
                // Essential service: defer until state stabilizes instead of forcing.
                return Self::defer_with_audit(
                    shard,
                    request,
                    ai_state,
                    ConsentReason::EssentialStateUnstable,
                );
            }
        }

        // Stability time: require S_t to stay in corridor long enough.
        let now = request.now;
        let stable_enough = match shard.stable_since {
            Some(t0) => {
                if let Ok(delta) = now.duration_since(t0) {
                    delta >= shard.min_stability_time
                } else {
                    false
                }
            }
            None => false,
        };

        if !stable_enough {
            // Initialize or update stable_since; still defer this attempt.
            shard.stable_since = Some(now);
            return Self::defer_with_audit(
                shard,
                request,
                ai_state,
                ConsentReason::StabilityNotYetEstablished,
            );
        }

        // AI consent macro state must confirm.
        if ai_state != AiConsentState::Confirmed {
            // Never guess; deny or defer depending on essential flag.
            if request.is_essential_service {
                return Self::defer_with_audit(
                    shard,
                    request,
                    ai_state,
                    ConsentReason::StabilityTimeInsufficient,
                );
            } else {
                return Self::deny_with_audit(
                    shard,
                    request,
                    ai_state,
                    ConsentReason::StabilityTimeInsufficient,
                );
            }
        }

        // Conservative policy: block when risk scores are too high.
        if shard.ai_consent_policy == AiConsentPolicy::Conservative {
            if shard.r_fraud > 0.5 || shard.r_privacy > 0.5 || shard.r_tracking > 0.5 {
                return Self::deny_with_audit(
                    shard,
                    request,
                    ai_state,
                    ConsentReason::RiskScoresTooHigh,
                );
            }
        }

        // All checks passed: allow and update counters.
        shard.payments_last_hour = shard.payments_last_hour.saturating_add(1);
        shard.prompts_last_hour = shard.prompts_last_hour.saturating_add(1);

        let audit = if shard.consent_audit_log_enabled {
            Some(ConsentAuditRecord {
                wallet_did: shard.wallet_did.clone(),
                merchant_id: request.merchant_id.clone(),
                amount_mills: request.amount_mills,
                decision: ConsentDecision::Allow,
                reason: ConsentReason::Ok,
                timestamp: request.now,
                s_value: shard.neuro_state.svalue,
                load_value: shard.neuro_state.loadvalue,
                ai_consent_state: ai_state,
            })
        } else {
            None
        };

        (ConsentDecision::Allow, ConsentReason::Ok, audit)
    }

    fn deny_with_audit(
        shard: &AugFingerprintShard,
        request: &PaymentRequest,
        ai_state: AiConsentState,
        reason: ConsentReason,
    ) -> (ConsentDecision, ConsentReason, Option<ConsentAuditRecord>) {
        let audit = if shard.consent_audit_log_enabled {
            Some(ConsentAuditRecord {
                wallet_did: shard.wallet_did.clone(),
                merchant_id: request.merchant_id.clone(),
                amount_mills: request.amount_mills,
                decision: ConsentDecision::Deny,
                reason,
                timestamp: request.now,
                s_value: shard.neuro_state.svalue,
                load_value: shard.neuro_state.loadvalue,
                ai_consent_state: ai_state,
            })
        } else {
            None
        };

        (ConsentDecision::Deny, reason, audit)
    }

    fn defer_with_audit(
        shard: &AugFingerprintShard,
        request: &PaymentRequest,
        ai_state: AiConsentState,
        reason: ConsentReason,
    ) -> (ConsentDecision, ConsentReason, Option<ConsentAuditRecord>) {
        let audit = if shard.consent_audit_log_enabled {
            Some(ConsentAuditRecord {
                wallet_did: shard.wallet_did.clone(),
                merchant_id: request.merchant_id.clone(),
                amount_mills: request.amount_mills,
                decision: ConsentDecision::Defer,
                reason,
                timestamp: request.now,
                s_value: shard.neuro_state.svalue,
                load_value: shard.neuro_state.loadvalue,
                ai_consent_state: ai_state,
            })
        } else {
            None
        };

        (ConsentDecision::Defer, reason, audit)
    }
}
