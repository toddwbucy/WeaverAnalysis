# WeaverAnalysis

The diagnostic consumer of the WeaverTools suite: one crate, outside the agent's
boundary, that holds both ends of a replay. It reads a finished record the operator
holds, derives the declaration a replay needs from it, preloads it through the state
member's second door, and reads the diagnostic-trace the run produces off the sink
admin opened for it. Everything downstream of the record is here: the reading and its
gate, the projection, the lens over the residual stream, the field at a position, and
the exact comparison of two captures. Nothing here touches a device or sees a weight
the record did not carry.

It is one binary with seven verbs, each answering one JSON line on stdout and refusing
by name where it will not act:

```text
weaver-analysis derive  <trace> --as <session> --devices <n,..> --sink <path> [--readout] [--surprisal] ...
weaver-analysis preload <trace> <socket> [--through <run>:<turn>] [--as <session>] [--diagnostic]
weaver-analysis read    <diagnostic-trace>
weaver-analysis compare <capture> <capture>
weaver-analysis signals <record> [spike-bar] [--deposit <path>]
weaver-analysis lens    <capture> --lens <path> --weights <path> [--layers ..] [--positions ..]
weaver-analysis field   <record> --position <turn>:<position> [--run <run>]
```

Run it with no arguments for the full usage line. A serving trace given to `read`
answers `the record is not a diagnostic-trace`; a record holding more than one run
given to `derive` names the two runs it met and refuses. Those are the crate working.

## Where it sits

```text
   WeaverAgents (the agent)                 WeaverWeb (the frontend)
   admin opens the sink ----> record ----> weaver-analysis ----> per-position series,
   state member's 2nd door <-- preload                          per-generation summary
```

Two seams, both defined by documents rather than shared types:

- **weaver-state**, `docs/weaver-analysis-state-contract.md`: the preload, over the
  state member's socket.
- **weaver-web**, `docs/weaver-analysis-web-contract.md`: the emission of finished
  records, this crate the emitter that asks nothing of the reader.

The crate links no `weaver-*` crate, no async runtime and no socket crate;
`tests/manifest.rs` reads `cargo tree` and refuses any of the three. No crate depends
on it, and the same test asserts the workspace is exactly this crate. Those are the
conditions under which it left the monorepo on 2026-09-30 and became this repository.

## Documents

- `docs/weaver-analysis-PRD.md`: the charter. What the crate is, is not, does in order,
  and leaves open.
- `docs/weaver-analysis-Spec.md`: every representation election, the parse, the
  projection, the preload, the reading and its gate, and the table of what is enforced
  by which instrument.
- The two contracts above.

The record it reads is authored inside the agent by `weaver-diagnostic`, the harness's
third member, whose documents live in WeaverAgents under `docs/crates/`.

## Building

Pinned to `nightly-2026-02-13` by `rust-toolchain.toml`, edition 2024, four
dependencies. Keep `--locked` on every command that resolves.

```sh
cargo build --locked
cargo test --locked                       # 85 tests, 9 binaries, fixtures under tests/fixtures/
cargo test --locked --test manifest       # the leaf conditions
cargo clippy --all-targets --locked -- -D warnings
cargo fmt --check
```

## License

See `LICENSE`, carried from the workspace the crate left.
