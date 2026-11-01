# Casebox

Placeholder for workspace package `casebox`. Canonical design: [`P-CASEBOX`](../../../project-planning/components/casebox.md).

| Fact | Value |
|---|---|
| Status | `exploring` — spec only, no code |
| Languages (design space) | Go, Rust |
| Declared ecosystem consumers | Sentinel |
| Declared superplatforms | DataSentinel, OpsMesh |
| Depends on | C-CAS, C-PROVENANCE |

This directory fixes the component's path in the workspace ([D-0012](../../../project-planning/decisions/0012-component-repositories.md)). Add the manifest, source, and tests when the first slice exists. Do not add CI, release configuration, or a published version before then; conventions are in [AGENTS.md](../../AGENTS.md).
