# Components

Shared libraries and CLIs for the project family: **one repository, one package per component**. Governed by proposed [D-0012](../project-planning/decisions/0012-component-repositories.md), which amends [D-0005](../project-planning/decisions/0005-repository-layout.md). Working conventions are in [AGENTS.md](AGENTS.md).

Every component has a fixed directory under `crates/`. Two are buildable today: `bailiff` (extracted from PeerVault's R0 harness) and `knowledge` (extracted from Scribe's engine); the remaining directories hold a README pointing at their spec and no manifest. This is a Git repository with a Cargo workspace; `cargo test` runs the component tests.

## Ownership rule

| Code | Home |
|---|---|
| Any component (`P-*`) | Here, one package per component |
| Component that `depends_on` an ecosystem | Integration adapter in the consuming superplatform shell |
| Ecosystem product code | Its own repository under `Ecosystems/` |
| Superplatform integration code | Its own shell repository |

Placement matches [components/INDEX.md](../project-planning/components/INDEX.md). The dependency graph is acyclic by rule: nothing here depends on an ecosystem or superplatform, and ecosystem repositories contain no component source. Consumer count is not a placement criterion — a component that gains a second consumer changes a dependency, not its location.

## Workspace members (50)

| ID | Component | Declared consumers | Package |
|---|---|---|---|
| P-ACCORD | [Accord](../project-planning/components/accord.md) | Forge, AgentWorks | [`accord`](crates/accord/) |
| P-ANCHOR | [Anchor](../project-planning/components/anchor.md) | CloudMesh | [`anchor`](crates/anchor/) |
| P-BACKFIRE | [Backfire](../project-planning/components/backfire.md) | Forge, Chronicle, Sentinel | [`backfire`](crates/backfire/) |
| P-BAILIFF | [Bailiff](../project-planning/components/bailiff.md) | PeerVault, CloudMesh, ModGrid, Sentinel | [`bailiff`](crates/bailiff/) |
| P-BLAST | [Blast-radius policy engine](../project-planning/components/blast.md) | — | [`blast`](crates/blast/) |
| P-BUFFERLINE | [Bufferline](../project-planning/components/bufferline.md) | Quarry, ModelGrid | [`bufferline`](crates/bufferline/) |
| P-CASCADE | [Cascade](../project-planning/components/cascade.md) | Forge | [`cascade`](crates/cascade/) |
| P-CASEBOX | [Casebox](../project-planning/components/casebox.md) | Sentinel | [`casebox`](crates/casebox/) |
| P-CAUSEWAY | [Causeway](../project-planning/components/causeway.md) | Forge | [`causeway`](crates/causeway/) |
| P-COLFORMAT | [ColFormat](../project-planning/components/colformat.md) | Quarry | [`colformat`](crates/colformat/) |
| P-CROSSCUT | [CrossCut](../project-planning/components/crosscut.md) | Forge, AgentWorks | [`crosscut`](crates/crosscut/) |
| P-DETECTA | [Detecta](../project-planning/components/detecta.md) | Sentinel | [`detecta`](crates/detecta/) |
| P-DIAG | [Operations diagnosis agent](../project-planning/components/diag.md) | — | [`diag`](crates/diag/) |
| P-DIFFTEST | [Differential test engine](../project-planning/components/difftest.md) | — | [`difftest`](crates/difftest/) |
| P-EVIDENCEGRAPH | [Incident evidence graph](../project-planning/components/evidencegraph.md) | — | [`evidencegraph`](crates/evidencegraph/) |
| P-FERRY | [Ferry](../project-planning/components/ferry.md) | CloudMesh, PeerVault | [`ferry`](crates/ferry/) |
| P-FLIGHTDECK | [Flightdeck](../project-planning/components/flightdeck.md) | Quarry | [`flightdeck`](crates/flightdeck/) |
| P-FREIGHT | [Freight](../project-planning/components/freight.md) | CloudMesh, Forge | [`freight`](crates/freight/) |
| P-GRIDBENCH | [GridBench](../project-planning/components/gridbench.md) | Quarry, ModelGrid | [`gridbench`](crates/gridbench/) |
| P-HARNESS | [Verification harness](../project-planning/components/harness.md) | — | [`harness`](crates/harness/) |
| P-HATCH | [Hatch](../project-planning/components/hatch.md) | Forge | [`hatch`](crates/hatch/) |
| P-HPLAN | [HPlan](../project-planning/components/hplan.md) | Quarry | [`hplan`](crates/hplan/) |
| P-KERNELSMITH | [KernelSmith](../project-planning/components/kernelsmith.md) | Quarry, ModelGrid | [`kernelsmith`](crates/kernelsmith/) |
| C-KNOWLEDGE | [Knowledge index](../project-planning/components/knowledge.md) | AgentWorks, Atlas | [`knowledge`](crates/knowledge/) |
| P-LENS | [Lens](../project-planning/components/lens.md) | Sentinel | [`lens`](crates/lens/) |
| P-LENSCOPE | [Lenscope](../project-planning/components/lenscope.md) | Quarry, ModelGrid | [`lenscope`](crates/lenscope/) |
| P-MERIDIAN | [Meridian](../project-planning/components/meridian.md) | — | [`meridian`](crates/meridian/) |
| P-PALISADE | [Palisade](../project-planning/components/palisade.md) | Sentinel | [`palisade`](crates/palisade/) |
| P-PARAPET | [Parapet](../project-planning/components/parapet.md) | CloudMesh, PeerVault | [`parapet`](crates/parapet/) |
| P-PLINTH | [Plinth](../project-planning/components/plinth.md) | CloudMesh | [`plinth`](crates/plinth/) |
| P-POLICYCORE | [PolicyCore](../project-planning/components/policycore.md) | Forge | [`policycore`](crates/policycore/) |
| P-PORTCULLIS | [Portcullis](../project-planning/components/portcullis.md) | CloudMesh, ModGrid | [`portcullis`](crates/portcullis/) |
| P-POSTMORTEM | [Postmortem generator](../project-planning/components/postmortem.md) | Forge | [`postmortem`](crates/postmortem/) |
| P-PRISM | [Prism](../project-planning/components/prism.md) | Forge | [`prism`](crates/prism/) |
| P-PROOF | [Formal proof and SMT adapter](../project-planning/components/proof.md) | — | [`proof`](crates/proof/) |
| P-PROPERTY | [Property and invariant specification library](../project-planning/components/property.md) | — | [`property`](crates/property/) |
| P-PYLON | [Pylon](../project-planning/components/pylon.md) | CloudMesh | [`pylon`](crates/pylon/) |
| P-REVIEW | [Review Orchestrator](../project-planning/components/review-orchestrator.md) | AgentWorks, CodeHost | [`review-orchestrator`](crates/review-orchestrator/) |
| P-REWIND | [Rewind](../project-planning/components/rewind.md) | Quarry | [`rewind`](crates/rewind/) |
| P-RISK | [Change risk classifier](../project-planning/components/risk.md) | — | [`risk`](crates/risk/) |
| P-RUNBOOK | [Executable runbooks](../project-planning/components/runbook.md) | — | [`runbook`](crates/runbook/) |
| P-SABLE | [Sable](../project-planning/components/sable.md) | Sentinel | [`sable`](crates/sable/) |
| P-SCHEMAFORGE | [Schemaforge](../project-planning/components/schemaforge.md) | Sentinel | [`schemaforge`](crates/schemaforge/) |
| P-SKETCHBOOK | [Sketchbook](../project-planning/components/sketchbook.md) | Quarry, ModelGrid | [`sketchbook`](crates/sketchbook/) |
| P-SKETEL | [Sketel](../project-planning/components/sketel.md) | Sentinel | [`sketel`](crates/sketel/) |
| P-SPEK | [Spek](../project-planning/components/spek.md) | Forge | [`spek`](crates/spek/) |
| P-TELLER | [Teller](../project-planning/components/teller.md) | CloudMesh, ModGrid, Sentinel | [`teller`](crates/teller/) |
| P-TOPOSCHED | [TopoSched](../project-planning/components/toposched.md) | Quarry, ModelGrid | [`toposched`](crates/toposched/) |
| P-TRIBUNE | [Tribune](../project-planning/components/tribune.md) | Forge, AgentWorks | [`tribune`](crates/tribune/) |
| P-VECFORGE | [VecForge](../project-planning/components/vecforge.md) | Quarry, ModelGrid | [`vecforge`](crates/vecforge/) |

## Adapters in superplatform shells (3)

| Component | Wraps | Lives in |
|---|---|---|
| `P-BINVERIFY` | BinaryLab | Verity shell |
| `P-RIGEL` | Sentinel | DataSentinel shell |
| `P-SIM` | CloudMesh | OpsMesh shell |

Each `depends_on` an ecosystem, so it adapts that surface rather than implementing a library. Adapters are not workspace members and never copies of ecosystem source.

## Layout

```text
Components/
├─ AGENTS.md             # conventions for this repository
├─ Cargo.toml            # workspace; members are listed explicitly
├─ Cargo.lock
├─ crates/
│  ├─ bailiff/           # member: extracted PeerVault R0 harness
│  ├─ knowledge/         # member: extracted Scribe engine
│  ├─ crosscut/          # placeholder: README only
│  └─ <component>/       # one directory per package, 50 present
└─ README.md
```

- Workspace `members` lists buildable packages only. Placeholder directories are deliberately absent: a glob over `crates/*` fails on directories without a manifest.
- Adding a package means adding its manifest and its entry in the workspace `members` list in the same change.

- Package directory names equal spec file names, so `components/crosscut.md` ↔ `crates/crosscut/`.
- `crates/` is the package root for every language. Rust members join the Cargo workspace; a Go slice adds `go.work`; C#/Java slices add their own project files. A mixed-language package stays under `crates/<name>/`.
- Placeholder directories stay until their component ships; they are the fixed path, not an invitation to scaffold.
- Three foundation concepts have no spec of their own and get a package when their contract lands: `cas`, `sched`, `event`. The other foundation concepts overlap existing specs (`columnar`/`colformat`, `gpu`/`bufferline`+`toposched`+`kernelsmith`, `policy`/`policycore`+`parapet`, `provenance`/`accord`, `identity`/`anchor`) and need reconciliation before they become packages.

## Language reality

`languages:` in a spec is a design space, not a decision. Of the 50: 48 list Rust first, 7 are Rust-only, 32 also list Go, 5 list Python, 4 CUDA, 3 C++, 3 C#. `D-004` (build engine language) and `D-006` (GPU stack) are open. Expect one build root per toolchain, not one build graph for all 50 packages.

## Consumption and its cost

- Co-development: path dependency on the workspace member. An ecosystem build then needs this directory checked out beside `Ecosystems/`.
- After first release: pinned Git tag (`<component>-v<semver>`) or package feed entry.
- Ecosystem CI either checks out both repositories or resolves the pinned version; a build that silently follows a moved local path is not a valid integration run.
- Never a vendored copy, never a submodule (D-0005).
- A package may still split into its own repository under D-0012 rule 6 (standalone product with an independent release cadence). That is an option, not a default, and not a prerequisite for use by several ecosystems.

## Migrated implementations

Two packages were extracted from working product code on 2026-11-01. Both keep their tests with the package; the product keeps what it owns.

| Package | Extracted from | Product retains |
|---|---|---|
| [`bailiff`](crates/bailiff/) | PeerVault's R0 harness: fixture generation, backend pinning, bounded process execution, media identity, comparison, restore-drill orchestration | CLI (`src/main.rs`), pin document (`backend-lock.json`), evidence inputs (`src/pins.rs`) |
| [`knowledge`](crates/knowledge/) | Scribe's `kb` engine: storage, ingest, retrieval, context packs, graph, MCP server | CLI (`src/main.rs`), product evaluation harness (`src/eval.rs`, `src/answer_eval.rs`), fixtures and config documents |

- PeerVault consumes `bailiff` by path dependency and passes `EvidenceInputs`, so the evidence records the product's own build inputs and the crate version.
- Scribe's `kb` crate depends on `knowledge` and re-exports it; commands, configuration and MCP behaviour are unchanged.
- Fixture data exists in both repositories where product and package tests need it (`v1-a.json`/`v1-b.json` in PeerVault, `canonical/`/`graph-errors/` in Scribe and `knowledge`). It is test data, not duplicated logic.
- ModGrid holds no component code: its `internal/` packages are product internals with no component specification.

## Gates

| Gate | Condition |
|---|---|
| Initialize a package | First slice exists; add the manifest and the workspace `members` entry together |
| Add a manifest for a package | Spec exists with a defined first demo; the package builds and tests standalone |
| Release a member | Contract tests pass plus at least one consumer integration test |
| Split a member into its own repository | Standalone product surface with an independent release cadence, or build-time isolation need |
