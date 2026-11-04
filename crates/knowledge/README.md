# knowledge

Markdown retrieval, cited evidence packs and an opt-in grounded-answer path. Canonical design: [`C-KNOWLEDGE`](../../../project-planning/components/knowledge.md).

| Fact | Value |
|---|---|
| Version | `0.1.0`, tagged `knowledge-v0.1.0`; `publish = false` (crates.io `knowledge` is taken) |
| Extracted from | Scribe's `kb` crate, 2026-11-01 |
| Crate | [`src/lib.rs`](src/lib.rs); modules `db`, `ingest`, `retrieve`, `context`, `graph`, `mcp`, plus `full`-gated `answer`, `embed`, `verify` |
| First consumer | Scribe (`kb` CLI), pinned Git tag `knowledge-v0.1.0` |

## Profiles

- **light (default):** SQLite FTS5/BM25, no models, no network. `cargo test -p knowledge` covers this path.
- **full:** embeddings, hybrid retrieval, verified answers. Enable with `cargo test -p knowledge --features full`; the package tests do not call a provider, and consumer integration tests (Scribe) exercise the full retrieval/answer path. It requires configured providers at runtime and is not an answer-quality claim.

## What stays in the product

Scribe keeps its CLI, its configuration documents, the product evaluation harness (`eval.rs`, `answer_eval.rs`) and the fixtures those pin. Those modules hash Scribe's own lock file and fixtures, so they are product evidence, not engine code.

## Build and test

```sh
cargo build -p knowledge
cargo test -p knowledge
cargo test -p knowledge --features full
```

`tests/fixtures/` carries the engine-side fixtures (`canonical/`, `graph-errors/`, `research/`, `audit/`). Scribe keeps its own copies for product tests; they are test data, not duplicated logic.

## Consumption

Consumers pin the Git tag; a path dependency is for co-development only.

```toml
knowledge = { git = "https://github.com/jkalend/Components", tag = "knowledge-v0.1.0" }
```

A path source is not content-pinned by the consumer's `Cargo.lock`, so record the `Components` revision when a build follows a local checkout instead of the tag.
