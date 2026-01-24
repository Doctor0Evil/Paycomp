#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BiophysicalNetworkMode {
    InternalBioOnly,
    InternalBioPlusNfc,
    Offline,
    Other(&'static str),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NeurostateHealthBand {
    Stable,
    Fragile,
    Recovering,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct ActiveBiophysicalNetwork {
    pub active: bool,
    pub mode: BiophysicalNetworkMode,
    pub health_band: NeurostateHealthBand,
}

/// Minimal view that a POS / BFC plugin needs to know about the host.
#[derive(Debug, Clone)]
pub struct HostBiophysicalContext {
    pub wallet_did: String,
    pub austatus: String, // e.g. "organicallyintegratedaugmentedcitizen"
    pub network: ActiveBiophysicalNetwork,
    pub no_exclusion_basic_services: bool,
    pub no_score_from_inner_state: bool,
}

impl HostBiophysicalContext {
    /// Merchant-safe check: is there a live, neurorights-safe biophysical network behind this wallet?
    pub fn is_active_biophysical_network(&self) -> bool {
        self.network.active
            && matches!(self.network.mode, BiophysicalNetworkMode::InternalBioOnly
                                        | BiophysicalNetworkMode::InternalBioPlusNfc)
            && self.no_exclusion_basic_services
            && self.no_score_from_inner_state
    }
}
