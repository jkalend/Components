//! Restore drills, fixture oracles and backend verification.
//!
//! Extracted from PeerVault's R0 qualification harness. Canonical design:
//! [`P-BAILLIFF`](../../../../project-planning/components/bailiff.md).
//!
//! The crate is transport-neutral: it generates deterministic fixtures, pins and
//! verifies external backend executables, runs bounded process trees, and
//! compares restored trees against an independent expected manifest. It does not
//! own a backup repository format or keys.

pub mod backend;
pub mod fixture;
pub mod media;
pub mod pins;
pub mod process;
