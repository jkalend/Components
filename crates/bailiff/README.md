# bailiff

Restore drills, fixture oracles and backend verification. Canonical design: [`P-BAILLIFF`](../../../project-planning/components/bailiff.md).

| Fact | Value |
|---|---|
| Version | `0.0.1`, `publish = false` |
| Extracted from | PeerVault's R0 qualification harness, 2026-11-01 |
| Crate | [`src/lib.rs`](src/lib.rs); modules `backend`, `fixture`, `media`, `pins`, `process` |
| First consumer | PeerVault (path dependency) |

## What it does

- Generates deterministic fixture corpora and their independent expected manifests (`fixture`).
- Validates pinned external backend descriptors and hashes the executables it runs (`pins`).
- Executes external backends in bounded, cancellable process trees with output caps (`process`).
- Identifies removable media and runs backend bakeoff plus copy/restore drill orchestration (`media`, `backend`).

It does not implement a backup repository format, quiescence, chunking, Merkle roots, retention or scheduling. See the spec's "Not implemented" row before claiming coverage.

## Evidence ownership

The crate records what it ran, but the consuming product owns the evidence inputs: the pin document and the hashes of the product's own build inputs. `backend::EvidenceInputs` is that boundary. PeerVault builds it in `src/pins.rs`, so an `evidence.json` report identifies both the crate version and the product revision.

## Build and test

```sh
cargo build -p bailiff
cargo test -p bailiff
```

Two tests are `#[ignore]`d by design: they are subprocess fixtures invoked by the process-boundary test.

## Fixtures

`tests/fixtures/v1-a.json` and `v1-b.json` are golden manifests for the deterministic generator. Copies also exist in PeerVault because its scripts and documentation reference them directly; both must change together if the generator changes.
