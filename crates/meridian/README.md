# Meridian

Placeholder for workspace package `meridian`. Canonical design: [`P-MERIDIAN`](../../../project-planning/components/meridian.md).

| Fact | Value |
|---|---|
| Status | `exploring` — spec only, no code |
| Languages (design space) | Rust |
| Declared ecosystem consumers | — |
| Declared superplatforms | DataSentinel, OpsMesh |
| Depends on | C-COLUMNAR, P-LENS, P-CASEBOX |

This directory fixes the component's path in the workspace ([D-0012](../../../project-planning/decisions/0012-component-repositories.md)). Add the manifest, source, and tests when the first slice exists. Do not add CI, release configuration, or a published version before then; conventions are in [AGENTS.md](../../AGENTS.md).
