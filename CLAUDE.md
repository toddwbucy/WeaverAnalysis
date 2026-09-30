# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this repository is

**WeaverAnalysis is the diagnostic consumer of the WeaverTools suite**, one crate at the
repository root, outside the agent's boundary. It reads the record the agent's admin
custodies, derives and preloads a replay through the state member's second door, reads
the diagnostic-trace that comes back, and emits finished records toward WeaverWeb.
`README.md` has the verbs and the picture; `docs/weaver-analysis-PRD.md` is the charter
and `docs/weaver-analysis-Spec.md` the Spec, and section 6 of the Spec is the table of
what is enforced and by which instrument.

The tree was `crates/weaver-analysis` in the WeaverTools monorepo (now
`toddwbucy/WeaverTools-old2`) until 2026-09-30, when the suite split into
WeaverAgents, WeaverAnalysis, WeaverWeb and a suite repository. The split's 44 commits
are on the local `seed-history` branch, which is **never pushed**, and in the monorepo.
This repository opens at its own root commit. The suite-level `CLAUDE.md` one directory
up describes the four repositories together.

## Hard boundaries

- **No `weaver-*` dependency, no async runtime, no socket crate.** Spec section 1;
  `tests/manifest.rs` reads `cargo tree` and refuses all three. Both seams draw their
  vocabulary from the contracts, not from shared types. A shared type is a compile-time
  dependency of the interior and is what this crate exists not to be.
- **No crate depends on this one, and the workspace is exactly this crate.** The same
  test holds it.
- **It never sees weights and never touches a device.** What reaches it is what the
  record carried.
- **Do not edit the interior from here.** A gap in what the record or a contract carries
  is an issue on `toddwbucy/WeaverAgents`, one per interface question: what was
  measured, what is asked, which document would move.

## Commands

Pinned to `nightly-2026-02-13`, edition 2024. Keep `--locked` on every cargo command
that resolves; `fmt` takes no flag.

```sh
cargo build --locked
cargo test --locked                       # 85 tests across 9 binaries
cargo test --locked --test manifest       # the three leaf pins
cargo test --locked <name_fragment>       # one test by substring
cargo clippy --all-targets --locked -- -D warnings
cargo fmt --check
```

Fixtures under `tests/fixtures/` are pinned outputs: `derived-surrogate.toml` is what
`derive` writes from `serving-source.ndjson`, held byte for byte by the driver test, and
WeaverAgents' `weaver-types` config test and the suite's determinism-matrix grammar test
both read it from this repository by path. Changing it changes three trees.

To exercise the binary rather than its tests, `target/debug/weaver-analysis` with no
arguments prints the usage line; `read tests/fixtures/diagnostic-certified.ndjson`
answers `certified`; `derive tests/fixtures/serving-source.ndjson --as s --devices 0
--sink /tmp/x` prints a declaration. A live serving trace from a WeaverAgents box is
refused by `read` (not a diagnostic-trace) and, when it holds several runs, by `derive`
(`MemberDisagrees` naming the two runs). Read a refusal before treating it as a defect:
each names its ground.

Cargo writes diagnostics to stderr, so pipe with `2>&1`; a crate that fails to compile
emits no `test result` line at all. Check the exit status where the answer matters.

## How we work

Two Claude Code sessions and the operator (Todd) closes every loop. The **thinkpad**
seat plans, reviews drafts and handles the third-party review; the **olympus** seat or
an executor session writes code on a branch from `main` and opens a **draft** pull
request. The executor never undrafts or merges; the planner undrafts when the draft
passes, which fires Codex's review; only the operator merges. Nobody pushes to `main`
after the root commit. Every pull request body carries `Implements: <Spec> <sections>`,
and each named section gets one verdict: conforms, drifted, better way, or Spec gap.
Where code and the Spec disagree, the same act changes whichever is wrong.

Issue and pull request numbers in inherited code and documents (`#628`, `#545`, ...)
are `toddwbucy/WeaverTools-old2` numbers. Open issues were transferred to
`toddwbucy/WeaverAgents` on 2026-09-30; #515, which straddles the harness and this
crate, is there too.

## Inherited documents, and what is unsettled

- The four documents under `docs/` are byte-identical copies of files that also stand
  in WeaverAgents under `docs/crates/weaver-analysis/` and `docs/crates/contracts/`.
  Which copy is authoritative is unruled. Say so in any act that edits one.
- The Spec's `conforms:` headers and assertion records are kept and not maintained until
  release, per the suite's ruling of 2026-09-27. Files written to the Spec open with
  `//! conforms: analysis-<assertion>` lines naming the claims they are held to.
- House style: ASCII only, no em-dashes; absolute dates; each election's reasoning
  stated where it is made; superseded text removed, since git is the archive; commit
  subjects `code:`, `docs:`, `process:`. The canonical vocabulary is `trace` /
  `reflection` / `substrate-state`.
- The repository is public. No commercial or strategy material and no single-operator
  versus multi-tenant distinction enters it.
