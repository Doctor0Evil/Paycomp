use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub enum ConsentMode {
    BciState,
    XrVisual,
    ExternalDevice,
    CaregiverCoapproval,
    Unknown(String),
}

impl From<String> for ConsentMode {
    fn from(s: String) -> Self {
        match s.as_str() {
            "bcistate" => ConsentMode::BciState,
            "xr_visual" => ConsentMode::XrVisual,
            "external_device" => ConsentMode::ExternalDevice,
            "caregiver_coapproval" => ConsentMode::CaregiverCoapproval,
            other => ConsentMode::Unknown(other.to_owned()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ShardProfile {
    pub au_status: String,
    pub latency_tolerance_ms_min: u64,
    pub latency_tolerance_ms_max: u64,
    pub max_prompts_per_hour: u32,
    pub preferred_consent_mode: ConsentMode,
}

#[derive(Debug, Clone)]
pub struct PromptState {
    pub prompts_issued_this_hour: u32,
    pub hour_window_started_at: Instant,
}

/// Decision returned to the POS flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptDecision {
    Allow,
    DeferSoft,  // e.g., schedule later / ask companion to queue
    DenyHard,   // do not prompt (rights / overload)
}

/// Guard encapsulating AU compatibility logic for POS.
pub struct AugCitizenPosGuard {
    profile: ShardProfile,
    state: PromptState,
}

impl AugCitizenPosGuard {
    pub fn new(profile: ShardProfile, state: PromptState) -> Self {
        Self { profile, state }
    }

    /// Called once per hour by housekeeping to reset counters.
    pub fn reset_hour_window(&mut self) {
        self.state.prompts_issued_this_hour = 0;
        self.state.hour_window_started_at = Instant::now();
    }

    /// Main entry: should the POS present a new payment prompt now?
    /// `expected_latency_ms` is the POS's current best estimate
    /// of round-trip latency for this interaction.
    pub fn should_prompt(&self, expected_latency_ms: u64) -> PromptDecision {
        // 1. If not an organically-integrated augmented citizen,
        //    fall back to default POS behavior.
        if self.profile.au_status != "organically_integrated_augmented_citizen" {
            return PromptDecision::Allow;
        }

        // 2. Enforce max_prompts_per_hour.
        if self.state.prompts_issued_this_hour >= self.profile.max_prompts_per_hour {
            // Soft-defer: ask AI companion to reschedule instead of hard fail.
            return PromptDecision::DeferSoft;
        }

        // 3. Enforce latency tolerance band.
        if expected_latency_ms > self.profile.latency_tolerance_ms_max {
            // Presenting a prompt in this regime risks coercion/overload.
            return PromptDecision::DeferSoft;
        }

        if expected_latency_ms < self.profile.latency_tolerance_ms_min {
            // System is "too fast"; allow but downstream should add
            // pacing via AI companion / UX (not handled here).
            return PromptDecision::Allow;
        }

        // 4. Respect preferred_consent_mode; if POS cannot satisfy it,
        //    deny hard and require alternate interface (e.g., XR or caregiver).
        match self.profile.preferred_consent_mode {
            ConsentMode::BciState => {
                // POS must be wired to the Aug_Fingerprint / BCI consent path.
                if !self.pos_supports_bci_consent() {
                    return PromptDecision::DenyHard;
                }
            }
            ConsentMode::XrVisual => {
                if !self.pos_supports_xr_overlay() {
                    return PromptDecision::DenyHard;
                }
            }
            ConsentMode::ExternalDevice => {
                if !self.pos_supports_external_device() {
                    return PromptDecision::DenyHard;
                }
            }
            ConsentMode::CaregiverCoapproval => {
                if !self.pos_supports_caregiver_flow() {
                    return PromptDecision::DenyHard;
                }
            }
            ConsentMode::Unknown(_) => {
                // Unknown mode: safest is to defer, not to guess.
                return PromptDecision::DeferSoft;
            }
        }

        PromptDecision::Allow
    }

    /// Called by the POS when a prompt is actually shown.
    pub fn record_prompt_issued(&mut self) {
        self.state.prompts_issued_this_hour =
            self.state.prompts_issued_this_hour.saturating_add(1);
    }

    fn pos_supports_bci_consent(&self) -> bool {
        // Stub: wire into actual capability flags for this terminal.
        true
    }

    fn pos_supports_xr_overlay(&self) -> bool {
        // Stub: XR companion / overlay capability.
        true
    }

    fn pos_supports_external_device(&self) -> bool {
        // Stub: phone / companion app consent flows.
        true
    }

    fn pos_supports_caregiver_flow(&self) -> bool {
        // Stub: dual-approval UI (customer + caregiver).
        true
    }
}
