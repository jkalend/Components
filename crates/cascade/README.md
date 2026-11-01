# Cascade

Placeholder for workspace package `cascade`. Canonical design: [`P-CASCADE`](../../../project-planning/components/cascade.md).

| Fact | Value |
|---|---|
| Status | `exploring` — spec only, no code |
| Languages (design space) | Rust, Go |
| Declared ecosystem consumers | Forge |
| Declared superplatforms | ForgeCloud |
| Depends on | C-CAS, P-HATCH, P-PRISM, P-POLICYCORE |

This directory fixes the component's path in the workspace ([D-0012](../../../project-planning/decisions/0012-component-repositories.md)). Add the manifest, source, and tests when the first slice exists. Do not add CI, release configuration, or a published version before then; conventions are in [AGENTS.md](../../AGENTS.md).
