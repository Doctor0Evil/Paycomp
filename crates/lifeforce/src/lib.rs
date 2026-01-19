use aln_bind::alnbind;

// This tells the codegen to:
// - parse the ALN shard
// - generate typed structs + enums
// - enforce basic range constraints at load time.
alnbind! {
    shard "qpudatashards/au_lifeforce_cybernano_bostrom2026.aln",
    module lifeforce_bostrom2026,
    profile_struct LifeforceProfile,
    policy_struct LifeforcePolicy,
    state_struct LifeforceState
}
