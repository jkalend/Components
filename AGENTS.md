# Components agent guide

## Workspace boundaries

This directory is **one repository** holding every component package. It is not an umbrella of per-component repositories, and it holds no ecosystem product code.

- One package per component spec. Directory name equals the spec file name (`components/bailiff.md` → `crates/bailiff/`), so a path and a spec resolve to the same string.
- Ecosystem product code lives in `Ecosystems/<Name>/`; superplatform integration code lives in its shell repository.
- Three components are adapters over an ecosystem surface and live in their consuming shell instead: `P-BINVERIFY` (Verity), `P-RIGEL` (DataSentinel), `P-SIM` (OpsMesh). Do not move them here; the dependency graph must stay acyclic.
- **Dependency direction is a hard rule:** no package here may depend on an ecosystem or a superplatform. If a component needs one, it is an adapter, not a library, and belongs in the shell.
- `bailiff` and `knowledge` were extracted from working product code (PeerVault, Scribe) on 2026-11-01; their product-side glue, evidence inputs and evaluation harnesses stay in those repositories. Any future product code that predates a package moves here against a contract, never as a copy (D-0012 rule 7).
- This is a Git repository with a Cargo workspace whose `members` list is explicit. It has no CI or release pipeline yet; adding a package means adding its manifest and its `members` entry together ([D-0012](../project-planning/decisions/0012-component-repositories.md)).

## Placeholder state

Every `crates/<name>/` currently contains only a README pointing at its spec. That is the intended state.

- Do not add a manifest, CI job, release configuration, or package version before that component's first slice exists.
- Do not delete a placeholder directory because the component has no code; it is the component's fixed path.
- When the slice lands, replace the stub README with real build and test instructions, and keep the spec link.

## Find the design before writing code

The planning corpus is at `../project-planning/`, with a [GitHub fallback](https://github.com/jkalend/project-planning).

1. Read the canonical spec: `components/<name>.md`. The grouped index is `components/INDEX.md`; layout rules are in `decisions/0012-component-repositories.md` and `decisions/0005-repository-layout.md`.
2. For shared foundations (CAS, scheduler, event log, policy, provenance, identity) read `components/shared-foundations.md`; the first foundation is fixed by `decisions/0002-first-foundation.md`.
3. The spec wins over this directory's README when they disagree. Surface material contradictions; do not invent architectural commitments to resolve them.
4. Specs are intent, not installed tooling: planned commands, languages, performance targets, and demos are unverified.

## Languages and build roots

- 47 of 49 specs list Rust first; 6 are Rust-only; 32 also list Go. Those `languages:` lines describe a design space, not a decision. `D-004` (build engine language) and `D-006` (GPU stack) remain open.
- Choose one implementation language per slice and record the choice. Do not add a second language for a component that already has one without a stated reason.
- One build root per toolchain inside this repository: `Cargo.toml` for Rust members, `go.work` when a Go slice exists, project files for C#/Java. A mixed-language package still lives under `crates/<name>/`.
- Components listing CUDA/C++ (`bufferline`, `kernelsmith`, `vecforge`, `sketchbook`, `colformat`) carry a native toolchain prerequisite in `build.rs`; they are not pure Rust builds.
- Build and test one package at a time: `cargo test -p <name>`. A workspace-wide build is not the default entry point; add `default-members` only after the first slice.

## Contract discipline

- Public API is versioned per package: semver and `<name>-vX.Y.Z` tags. A breaking change to a released crate needs a major bump plus a note for consumers.
- Define canonical serialization, hash domain, version negotiation, and compatibility before persisting formats, signatures, or cached artifacts.
- No absolute workstation paths, no sibling source imports at runtime, no secrets in fixtures or logs.
- Consumers use a path dependency during co-development and a pinned tag or package version afterwards. Never a vendored copy, never a submodule.

## Verification

- Test at the package boundary: round-trips, malformed input, interrupted writes, version negotiation, and bounded resource use.
- Before releasing a package, one consumer integration test must pass against the pinned version.
- Performance claims need a reproducible harness with reported variance and a pinned workload. One run is not evidence.
- Discover real commands from manifests. Before a manifest exists, state that execution is unavailable instead of claiming a planned demo ran.

## Handoff

- Report what changed, the commands actually run, and remaining limits.
- Keep each package README linked to its spec and update the stub when code lands.
- When delegating, supply the package path, spec, scope, commands, and acceptance criteria; give concurrent agents non-overlapping write ownership.
- The roster in [README.md](README.md) and [components/INDEX.md](../project-planning/components/INDEX.md) are generated from spec frontmatter. Regenerate instead of editing consumer lists by hand.
