use serde::{Deserialize, Serialize};

/// Amount in thousandths of a USD (mills).
/// 1 USD = 1000 mills.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct UsdMills(pub i64);

impl UsdMills {
    pub fn from_usd(usd: f64) -> Self {
        // Deterministic rounding to nearest mill.
        let mills = (usd * 1000.0).round() as i64;
        UsdMills(mills)
    }

    pub fn to_usd(self) -> f64 {
        (self.0 as f64) / 1000.0
    }
}

/// Core transaction record at mill resolution.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MillTransaction {
    pub from_did: Did,
    pub to_did: Did,
    /// Amount transferred (in mills).
    pub amount_mills: UsdMills,
    /// Rounded display amount in cents, for UI compatibility.
    pub display_amount_cents: i64,
    /// Rounding residual in mills credited back to payer's residual account.
    pub residual_mills_to_payer: i32,
}
