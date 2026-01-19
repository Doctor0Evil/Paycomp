use std::time::Instant;
use crate::lifeforce_bostrom2026::{LifeforceProfile, LifeforceState, LifeforcePolicy};
use crate::cybernano_payment_guard::{LifeforceBand, NanoDebitClass, NanoDebitRequest, NanoDebitDecision};
use crate::cybernano_payment_guard::CyberNanoPaymentGuard;

impl CyberNanoPaymentGuard {
    pub fn eval_with_policy(
        profile: &LifeforceProfile,
        state: &LifeforceState,
        policy: &LifeforcePolicy,
        req: &NanoDebitRequest,
    ) -> NanoDebitDecision {
        let now = Instant::now();

        // 1. Enforce ALN-derived policy invariants (already validated at load).
        if req.class == NanoDebitClass::MerchantPayment {
            return NanoDebitDecision {
                approved: false,
                reason: "Nanoswarm→merchant payment is forbidden by lifeforce policy.".into(),
                new_lifeforce_level: profile.lifeforce_level,
                new_blood_tokens: state.bloodtoken_balance_microusd3,
                hextrace: "0xCNGUARD_ALN_MERCHANT_FORBIDDEN".into(),
                decided_at: now,
            };
        }
        if !req.local_only && policy.nano_local_only_required {
            return NanoDebitDecision {
                approved: false,
                reason: "Non-local nanoswarm debit violates nano_local_only_required.".into(),
                new_lifeforce_level: profile.lifeforce_level,
                new_blood_tokens: state.bloodtoken_balance_microusd3,
                hextrace: "0xCNGUARD_ALN_LOCAL_ONLY".into(),
                decided_at: now,
            };
        }

        // 2. Check whether this NanoDebitClass is allowed.
        match req.class {
            NanoDebitClass::HostMaintenance if !policy.nanodebit_hostmaintenance_allowed => {
                return NanoDebitDecision {
                    approved: false,
                    reason: "HostMaintenance nanoswarm debits disabled by policy.".into(),
                    new_lifeforce_level: profile.lifeforce_level,
                    new_blood_tokens: state.bloodtoken_balance_microusd3,
                    hextrace: "0xCNGUARD_CLASS_FORBIDDEN_HOST".into(),
                    decided_at: now,
                };
            }
            NanoDebitClass::LocalEcoAction if !policy.nanodebit_localeco_allowed => {
                return NanoDebitDecision {
                    approved: false,
                    reason: "LocalEcoAction nanoswarm debits disabled by policy.".into(),
                    new_lifeforce_level: profile.lifeforce_level,
                    new_blood_tokens: state.bloodtoken_balance_microusd3,
                    hextrace: "0xCNGUARD_CLASS_FORBIDDEN_ECO".into(),
                    decided_at: now,
                };
            }
            _ => {}
        }

        // 3. Lifeforce delta caps from policy, band-specific.
        let max_delta = match profile.lifeforce_band {
            LifeforceBand::Stable => policy.max_lifeforce_delta_stable,
            LifeforceBand::Fragile => policy.max_lifeforce_delta_fragile,
            LifeforceBand::Recovering => policy.max_lifeforce_delta_recovering,
            LifeforceBand::Other(_) => policy.max_lifeforce_delta_fragile,
        };

        if req.estimated_lifeforce_delta > max_delta {
            return NanoDebitDecision {
                approved: false,
                reason: "Requested nanoswarm operation exceeds band-specific lifeforce delta cap from ALN policy.".into(),
                new_lifeforce_level: profile.lifeforce_level,
                new_blood_tokens: state.bloodtoken_balance_microusd3,
                hextrace: "0xCNGUARD_DELTA_CAP".into(),
                decided_at: now,
            };
        }

        // 4. Floor + margin (no snapping) check.
        let projected = profile.lifeforce_level - req.estimated_lifeforce_delta;
        let min_allowed = profile.lifeforce_floor + profile.lifeforce_curve_margin;

        if projected < min_allowed {
            return NanoDebitDecision {
                approved: false,
                reason: "Projected lifeforce falls below floor+margin; nanoswarm debit denied.".into(),
                new_lifeforce_level: profile.lifeforce_level,
                new_blood_tokens: state.bloodtoken_balance_microusd3,
                hextrace: "0xCNGUARD_FLOOR_MARGIN".into(),
                decided_at: now,
            };
        }

        // 5. Balance and approval logic (same as before, not repeated here)...
        // (You can reuse the balance checks from the earlier CyberNanoPaymentGuard.)

        // ...
        unimplemented!()
    }
}
