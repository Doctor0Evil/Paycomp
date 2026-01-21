use crate::macros::evolutiongraph;
use crate::types::DecisionKind::{self, *};

// Cyconetics NeuroAssist v2 evolution graph
evolutiongraph!(NeuroAssistV2Graph {
    // Baseline -> propose v2; small modeling/unknowns bump
    "BASELINE" => {
        "PROPOSE_V2" => (Propose, 0.05)
    },

    // Proposal -> Neurorights review (can also escalate early)
    "PROPOSE_V2" => {
        "NEUROCHECK_OK"   => (Authorize, 0.08), // total 0.13
        "ESCALATED_BOARD" => (Escalate, 0.25)   // sink; exempt from ceiling
    },

    // Neurorights OK -> Safety check or host rejection
    "NEUROCHECK_OK" => {
        "SAFETYCHECK_OK" => (Approve, 0.10),    // total 0.23
        "REJECTED_HOST"  => (Reject, 0.20)      // sink; exempt from ceiling
    },

    // Safety check OK -> Apply or escalate
    "SAFETYCHECK_OK" => {
        "APPLIED_V2"      => (Approve, 0.05),   // total 0.28
        "ESCALATED_BOARD" => (Escalate, 0.10)   // sink; exempt from ceiling
    },

    // Sinks (must not fan out further)
    "APPLIED_V2" => {
        // No forward edges; terminal applied state
    },

    "ESCALATED_BOARD" => {
        // Handled by governance outside this local graph
    },

    "REJECTED_HOST" => {
        // HostSelf veto path; must be respected by scheduler
    }
});
