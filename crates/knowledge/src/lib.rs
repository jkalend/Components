//! Markdown retrieval, knowledge index and MCP server.
//!
//! Extracted from Scribe's `kb` crate. Canonical design: the knowledge-index
//! foundation in [`C-FOUNDATIONS`](../../../../project-planning/components/shared-foundations.md)
//! and the [`P-KNOWLEDGE`](../../../../project-planning/projects/knowledge-base.md) product.
//!
//! `light` is the default build: SQLite FTS5/BM25, no models, no network. The
//! `full` feature adds embeddings, hybrid retrieval and grounded answers.

#[cfg(feature = "full")]
pub mod answer;
pub mod config;
pub mod context;
pub mod db;
#[cfg(feature = "full")]
pub mod embed;
pub mod error;
pub mod full_config;
pub mod graph;
pub mod ingest;
pub mod mcp;
pub mod model;
pub mod retrieve;
#[cfg(feature = "full")]
mod vector_index;
#[cfg(feature = "full")]
pub mod verify;
