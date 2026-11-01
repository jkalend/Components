# Differential test engine

Placeholder for workspace package `difftest`. Canonical design: [`P-DIFFTEST`](../../../project-planning/components/difftest.md).

| Fact | Value |
|---|---|
| Status | `exploring` — spec only, no code |
| Languages (design space) | Rust, Go |
| Declared ecosystem consumers | — |
| Declared superplatforms | Verity |
| Depends on | P-HARNESS, C-CAS |

This directory fixes the component's path in the workspace ([D-0012](../../../project-planning/decisions/0012-component-repositories.md)). Add the manifest, source, and tests when the first slice exists. Do not add CI, release configuration, or a published version before then; conventions are in [AGENTS.md](../../AGENTS.md).
