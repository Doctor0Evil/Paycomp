use std::time::Instant;
use crate::augfingerprint_corridor::AugFingerprintCorridor;

#[derive(Debug, Clone)]
pub struct PromptState {
    pub prompts_issued_this_hour: u32,
    pub hour_window_started_at: Instant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptDecision {
    Allow,
    DeferSoft, // ask AI companion to queue / delay
    DenyHard,  // do not prompt (rights or overload)
}

pub struct AugCitizenPosGuard<'a> {
    pub corridor: &'a AugFingerprintCorridor,
    pub state: PromptState,
}

impl<'a> AugCitizenPosGuard<'a> {
    pub fn new(corridor: &'a AugFingerprintCorridor, state: PromptState) -> Self {
        Self { corridor, state }
    }

    pub fn reset_hour_window(&mut self) {
        self.state.prompts_issued_this_hour = 0;
        self.state.hour_window_started_at = Instant::now();
    }

    /// Should the POS present a new payment prompt now, given expected latency?
    pub fn should_prompt(&self, expected_latency_ms: u64) -> PromptDecision {
        // 1. If not an organically-integrated augmented citizen, fallback.
        if self.corridor.austatus != "organicallyintegratedaugmentedcitizen" {
            return PromptDecision::Allow;
        }

        // 2. Enforce max prompts per hour (already merchant-tightened).
        if self.state.prompts_issued_this_hour >= self.corridor.max_prompts_per_hour {
            return PromptDecision::DeferSoft;
        }

        // 3. Enforce latency comfort band to avoid coercive timeouts.
        if expected_latency_ms > self.corridor.latency_ms_max {
            return PromptDecision::DeferSoft;
        }

        // Below min: allow, but AI companion should pace UX.
        if expected_latency_ms < self.corridor.latency_ms_min {
            return PromptDecision::Allow;
        }

        PromptDecision::Allow
    }
}
