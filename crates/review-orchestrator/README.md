# Review Orchestrator

Placeholder for workspace package `review-orchestrator`. Canonical design: [`P-REVIEW`](../../../project-planning/components/review-orchestrator.md).

| Fact | Value |
|---|---|
| Status | `exploring` — spec only, no code |
| Languages (design space) | Rust, Go |
| Declared ecosystem consumers | AgentWorks, CodeHost |
| Declared superplatforms | ForgeCloud, Verity |
| Depends on | C-CAS, P-CROSSCUT, P-POLICYCORE |

This directory fixes the component's path in the workspace ([D-0012](../../../project-planning/decisions/0012-component-repositories.md)). Add the manifest, source, and tests when the first slice exists. Do not add CI, release configuration, or a published version before then; conventions are in [AGENTS.md](../../AGENTS.md).
