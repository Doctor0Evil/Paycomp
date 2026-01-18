use serde::{Deserialize, Serialize};
use crate::did_types::{Did, DidDocumentRef};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpecAnchorShard {
    pub spec_did: Did,
    pub spec_doc: DidDocumentRef,
    pub spec_version: String,
    pub ecosystem_namespace: String,
    /// Indicates this spec passed CI under current ALN grammar.
    pub ci_passed: bool,
    /// K/E/R meta-scores for the spec itself.
    pub ker_meta: crate::shards::KerScores,
    /// DID of governance body that approved this spec (e.g., phoenix.ecosafety.council).
    pub approved_by: Option<Did>,
    pub approved_at_utc: Option<i64>,
    /// DID signature over this shard.
    pub signature: String,
}
