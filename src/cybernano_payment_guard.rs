use std::time::Instant;

/// Abstracted lifeforce envelope for a cybernetic host.
/// This is NOT energy metering; it encodes sustainable-integrity bands.
#[derive(Debug, Clone)]
pub struct LifeforceEnvelope {
    /// 0.0–1.0 normalized overall lifeforce level (cy/zen/chi aggregate).
    pub lifeforce_level: f32,
    /// Minimum safe lifeforce reserve that must never be crossed.
    pub lifeforce_floor: f32,
    /// Additional safety margin to avoid band snapping.
    pub lifeforce_curve_margin: f32,
    /// Current lifeforce-chi band (e.g., "stable", "fragile", "recovering").
    pub lifeforce_band: LifeforceBand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifeforceBand {
    Stable,
    Fragile,
    Recovering,
}

/// Local-only blood-token balance controlled by nanoswarm.
#[derive(Debug, Clone)]
pub struct BloodTokenLedger {
    pub microusd3_units: u64, // 1 unit = 0.001 USD
}

/// Classification of a requested debit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NanoDebitClass {
    HostMaintenance, // e.g., local nanoswarm housekeeping, sensor upkeep
    LocalEcoAction,  // e.g., local detox/regeneration nano-work
    MerchantPayment, // direct merchant settlement (forbidden)
}

/// Request for nanoswarm-funded debit.
#[derive(Debug, Clone)]
pub struct NanoDebitRequest {
    pub class: NanoDebitClass,
    pub amount_microusd3: u64,
    /// Estimated lifeforce impact in normalized units [0.0–1.0].
    pub estimated_lifeforce_delta: f32,
    /// Local-only flag; must be true for nanoswarm spend.
    pub local_only: bool,
}

/// Result of applying the guard.
#[derive(Debug, Clone)]
pub struct NanoDebitDecision {
    pub approved: bool,
    pub reason: String,
    pub new_lifeforce_level: f32,
    pub new_blood_tokens: u64,
    pub hextrace: String,
    pub decided_at: Instant,
}

/// Guard that enforces:
/// - nanoswarm never talks directly to merchant processors,
/// - lifeforce bands are never crossed or snapped,
/// - blood-tokens are local-only and host-controlled.
pub struct CyberNanoPaymentGuard;

impl CyberNanoPaymentGuard {
    pub fn eval(
        lifeforce: &LifeforceEnvelope,
        blood_ledger: &BloodTokenLedger,
        req: &NanoDebitRequest,
    ) -> NanoDebitDecision {
        let now = Instant::now();

        // 1. Hard block any attempt to use nanoswarm for direct merchant settlement.
        if req.class == NanoDebitClass::MerchantPayment {
            return NanoDebitDecision {
                approved: false,
                reason: "Direct merchant settlement via nanoswarm is forbidden; route through host wallet and standard Paycomp/BioPay rails.".to_string(),
                new_lifeforce_level: lifeforce.lifeforce_level,
                new_blood_tokens: blood_ledger.microusd3_units,
                hextrace: "0xCNGUARD_NO_MERCHANT_MIX".to_string(),
                decided_at: now,
            };
        }

        // 2. Enforce local-only constraint for all nanoswarm-funded operations.
        if !req.local_only {
            return NanoDebitDecision {
                approved: false,
                reason: "Nanoswarm blood-token spend must be local-only; remote or networked targets are not allowed.".to_string(),
                new_lifeforce_level: lifeforce.lifeforce_level,
                new_blood_tokens: blood_ledger.microusd3_units,
                hextrace: "0xCNGUARD_LOCAL_ONLY_REQUIRED".to_string(),
                decided_at: now,
            };
        }

        // 3. Ensure sufficient blood-token balance.
        if req.amount_microusd3 > blood_ledger.microusd3_units {
            return NanoDebitDecision {
                approved: false,
                reason: "Insufficient blood-token microusd3 balance for nanoswarm operation.".to_string(),
                new_lifeforce_level: lifeforce.lifeforce_level,
                new_blood_tokens: blood_ledger.microusd3_units,
                hextrace: "0xCNGUARD_BALANCE_INSUFFICIENT".to_string(),
                decided_at: now,
            };
        }

        // 4. Lifeforce protection: projected level must stay above floor + margin.
        let projected = lifeforce.lifeforce_level - req.estimated_lifeforce_delta;
        let min_allowed = lifeforce.lifeforce_floor + lifeforce.lifeforce_curve_margin;

        if projected < min_allowed {
            return NanoDebitDecision {
                approved: false,
                reason: "Requested nanoswarm operation would deplete lifeforce below safe envelope (floor + margin).".to_string(),
                new_lifeforce_level: lifeforce.lifeforce_level,
                new_blood_tokens: blood_ledger.microusd3_units,
                hextrace: "0xCNGUARD_LIFEFORCE_PROTECT".to_string(),
                decided_at: now,
            };
        }

        // 5. Additional prudence when band is Fragile or Recovering.
        if matches!(lifeforce.lifeforce_band, LifeforceBand::Fragile | LifeforceBand::Recovering)
            && req.estimated_lifeforce_delta > 0.05
        {
            return NanoDebitDecision {
                approved: false,
                reason: "In Fragile/Recovering lifeforce band, nanoswarm operations with high lifeforce delta are forbidden.".to_string(),
                new_lifeforce_level: lifeforce.lifeforce_level,
                new_blood_tokens: blood_ledger.microusd3_units,
                hextrace: "0xCNGUARD_FRAGILE_BAND".to_string(),
                decided_at: now,
            };
        }

        // 6. Approve safe, local, non-merchant nanoswarm debit.
        let new_level = projected;
        let new_balance = blood_ledger.microusd3_units - req.amount_microusd3;

        NanoDebitDecision {
            approved: true,
            reason: "Nanoswarm local-only debit approved within lifeforce envelope; merchant settlement must still go through host wallet.".to_string(),
            new_lifeforce_level: new_level,
            new_blood_tokens: new_balance,
            hextrace: "0xCNGUARD_APPROVED_LOCAL_SAFE".to_string(),
            decided_at: now,
        }
    }
}
