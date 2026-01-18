use crate::did_types::Did;
use crate::subcent::UsdMills;
use crate::shards::PaycompShard;

#[derive(Clone, Debug)]
pub struct Wallet {
    pub owner_did: Did,
    /// Current on-ledger balance (mills).
    pub balance_mills: UsdMills,
    /// Residual mill balance (accumulated rounding credits).
    pub residual_credit_mills: UsdMills,
    /// Last committed shard snapshot.
    pub last_shard: Option<PaycompShard>,
}

impl Wallet {
    pub fn can_spend(&self, amount: UsdMills) -> bool {
        self.balance_mills.0 >= amount.0
    }
}
