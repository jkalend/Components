# Sketchbook

Placeholder for workspace package `sketchbook`. Canonical design: [`P-SKETCHBOOK`](../../../project-planning/components/sketchbook.md).

| Fact | Value |
|---|---|
| Status | `exploring` — spec only, no code |
| Languages (design space) | Rust, CUDA |
| Declared ecosystem consumers | Quarry, ModelGrid |
| Declared superplatforms | ComputeFabric, DataSentinel |
| Depends on | C-FOUNDATIONS, P-BUFFERLINE, P-KERNELSMITH |

This directory fixes the component's path in the workspace ([D-0012](../../../project-planning/decisions/0012-component-repositories.md)). Add the manifest, source, and tests when the first slice exists. Do not add CI, release configuration, or a published version before then; conventions are in [AGENTS.md](../../AGENTS.md).
