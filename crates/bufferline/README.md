# Bufferline

Placeholder for workspace package `bufferline`. Canonical design: [`P-BUFFERLINE`](../../../project-planning/components/bufferline.md).

| Fact | Value |
|---|---|
| Status | `exploring` — spec only, no code |
| Languages (design space) | Rust, C, CUDA |
| Declared ecosystem consumers | Quarry, ModelGrid |
| Declared superplatforms | ComputeFabric |
| Depends on | C-FOUNDATIONS |

This directory fixes the component's path in the workspace ([D-0012](../../../project-planning/decisions/0012-component-repositories.md)). Add the manifest, source, and tests when the first slice exists. Do not add CI, release configuration, or a published version before then; conventions are in [AGENTS.md](../../AGENTS.md).
