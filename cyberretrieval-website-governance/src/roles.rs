use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum WebRole {
    ContentAuthor,
    Curator,
    GovernanceAdmin,
    NeurorightsFirewall,
}
