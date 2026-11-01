# Backfire

Placeholder for workspace package `backfire`. Canonical design: [`P-BACKFIRE`](../../../project-planning/components/backfire.md).

| Fact | Value |
|---|---|
| Status | `exploring` — spec only, no code |
| Languages (design space) | Rust, Go, C# |
| Declared ecosystem consumers | Forge, Chronicle, Sentinel |
| Declared superplatforms | ForgeCloud, Verity |
| Depends on | C-CAS, P-ACCORD, P-CASCADE, P-PRISM |

This directory fixes the component's path in the workspace ([D-0012](../../../project-planning/decisions/0012-component-repositories.md)). Add the manifest, source, and tests when the first slice exists. Do not add CI, release configuration, or a published version before then; conventions are in [AGENTS.md](../../AGENTS.md).
