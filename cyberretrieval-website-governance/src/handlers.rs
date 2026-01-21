use crate::risk::{ContentPageShard, WebsiteGovernanceConfig};
use crate::roles::WebRole;
use crate::NeurorightsFirewall;

pub struct Router<F: NeurorightsFirewall> {
    pub firewall: F,
    pub cfg: WebsiteGovernanceConfig,
}

impl<F: NeurorightsFirewall> Router<F> {
    pub fn handle_mutation(
        &self,
        current_shard: &ContentPageShard,
        req: &PageMutationRequest,
    ) -> PageMutationResponse {
        self.firewall.validate_page_mutation(&self.cfg, current_shard, req)
    }
}

#[derive(Clone, Debug)]
pub struct PageMutationRequest {
    pub actor_did: String,
    pub actor_role: WebRole,
    pub page_path: String,
    pub new_content_markdown: String,
}

#[derive(Clone, Debug)]
pub struct PageMutationResponse {
    pub accepted: bool,
    pub reason: String,
}

pub trait NeurorightsFirewall {
    fn validate_page_mutation(
        &self,
        cfg: &WebsiteGovernanceConfig,
        shard: &ContentPageShard,
        req: &PageMutationRequest,
    ) -> PageMutationResponse;
}
