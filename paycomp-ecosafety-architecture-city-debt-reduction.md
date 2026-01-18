---
title: "Paycomp Ecosafety Architecture: Sub-Cent Credit Rails, K/E/R Finance, and DID-Anchored Wallets"
version: 0.1.0
status: "draft-spec"
ecosystem: "EcoNet / Phoenix / Paycomp"
primary_author_did: "did:bostrom:bostrom18sd2ujv24ual9c9pshtxys6j8knh6xaead9ye7"
co_author_dids:
  - "did:bostrom:bostrom1ldgmtf20d6604a24ztr0jxht7xt7az4jhkmsrc"
  - "did:bostrom:zeta12x0up66pzyeretzyku8p4ccuxrjqtqpdc4y4x8"
  - "did:ethr:0x519fC0eB4111323Cac44b70e1aE31c30e405802D"
anchoring_scheme: "DID-linked qpudatashard; no SHA required"
ledger_namespace: "econet.phoenix.paycomp"
grammar_family: "Phoenix-SSG / EcoNet KER / Paycomp"
---

# Paycomp Ecosafety Architecture

Paycomp is an augmented-citizen financial infrastructure for smart-city finance that treats payments and debt as ecosafety variables, not just accounting lines.[file:3] It combines sub-cent USD credit rails, K/E/R-scored qpudatashards, and DID-based wallets enforced by Rust/ALN guard modules to reduce debt leakage and physical cash dependence while preserving data sovereignty.[file:2][file:3]

## 1. DID-Anchored Authorship and Ledger Identity

Every Paycomp node—citizen wallet, merchant, bank, treasury, or hardware device—is identified by a DID, and all authorship and ledger state anchoring is defined in terms of these identifiers, not SHA hashes.[file:3]

```rust
// file: src/paycomp/did_types.rs

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Did {
    /// DID string, e.g. "did:bostrom:..." or "did:ethr:0x..."
    pub id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum DidMethod {
    Bostrom,
    Ethr,
    Other(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DidDocumentRef {
    /// Canonical DID
    pub did: Did,
    /// URI or logical pointer to off-ledger DID document / key material
    pub doc_ref: String,
    /// Optional human-readable label
    pub label: Option<String>,
}
