# Incident evidence graph

Placeholder for workspace package `evidencegraph`. Canonical design: [`P-EVIDENCEGRAPH`](../../../project-planning/components/evidencegraph.md).

| Fact | Value |
|---|---|
| Status | `exploring` — spec only, no code |
| Languages (design space) | Rust, Go |
| Declared ecosystem consumers | — |
| Declared superplatforms | OpsMesh |
| Depends on | C-EVENT, C-COLUMNAR |

This directory fixes the component's path in the workspace ([D-0012](../../../project-planning/decisions/0012-component-repositories.md)). Add the manifest, source, and tests when the first slice exists. Do not add CI, release configuration, or a published version before then; conventions are in [AGENTS.md](../../AGENTS.md).
