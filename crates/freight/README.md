# Freight

Placeholder for workspace package `freight`. Canonical design: [`P-FREIGHT`](../../../project-planning/components/freight.md).

| Fact | Value |
|---|---|
| Status | `exploring` — spec only, no code |
| Languages (design space) | Go, Rust |
| Declared ecosystem consumers | CloudMesh, Forge |
| Declared superplatforms | PersonalMesh |
| Depends on | P-ANCHOR, C-CAS |

This directory fixes the component's path in the workspace ([D-0012](../../../project-planning/decisions/0012-component-repositories.md)). Add the manifest, source, and tests when the first slice exists. Do not add CI, release configuration, or a published version before then; conventions are in [AGENTS.md](../../AGENTS.md).
