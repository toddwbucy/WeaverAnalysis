//! conforms: analysis-no-internal-dependency
//! conforms: analysis-no-runtime-no-socket-crate
//! conforms: analysis-no-crate-depends-on-it
//!
//! The manifest assertions of `weaver-analysis-Spec` section 1: no
//! `weaver-*` dependency at all - this crate stands outside the agent and
//! linking any interior crate would make an outside consumer a compile-time
//! dependent of the interior - and no async runtime and no socket crate in
//! the resolved tree, the standard library's own client being the whole of
//! what one dialed Unix socket needs. And the other direction: no crate in
//! the workspace depends on this one, the operator's condition of 2026-09-26
//! for it staying in the repository.

use std::process::Command;

fn resolved_tree() -> String {
    let out = Command::new(env!("CARGO"))
        // Every edge kind and every target: a dev or build dependency on
        // an interior crate, or one reached only on another platform,
        // crosses the same boundary the normal edges do.
        .args([
            "tree",
            "-p",
            "weaver-analysis",
            "--edges",
            "all",
            "--all-targets",
            "--prefix",
            "none",
        ])
        .output()
        .expect("cargo tree runs");
    assert!(out.status.success());
    String::from_utf8(out.stdout).expect("utf8")
}

#[test]
fn no_weaver_dependency_at_all() {
    let names: Vec<String> = resolved_tree()
        .lines()
        .filter_map(|l| l.split_whitespace().next())
        .map(str::to_string)
        .collect();
    assert!(
        !names
            .iter()
            .any(|n| n.starts_with("weaver-") && n != "weaver-analysis"),
        "the boundary is the manifest: {names:?}"
    );
}

#[test]
fn no_runtime_and_no_socket_crate() {
    let tree = resolved_tree();
    for forbidden in ["tokio", "async-std", "smol", "mio", "socket2", "nix"] {
        assert!(
            !tree
                .lines()
                .any(|l| l.split_whitespace().next() == Some(forbidden)),
            "{forbidden} stands in the resolved tree"
        );
    }
}

/// **No crate in the workspace depends on this one, of any kind.** Read from
/// cargo's declared dependencies of every workspace member, where a normal,
/// build or dev edge, renamed, target-qualified or optional, all name the
/// package, so every route to this crate is seen and not only a manifest line
/// spelled `weaver-analysis =`.
///
/// **Since 2026-09-30 the workspace is this crate alone.** The condition was
/// written against a monorepo, where it was checked against every
/// `crates/*/Cargo.toml` so a crate standing outside the workspace refused
/// rather than going unread. The split into its own repository made the
/// condition structural: an interior crate that wanted this one would have to
/// reach across repositories to name it. What this test still reads is that
/// the workspace cargo answers for is exactly this crate, so a second member
/// added here, the one way the condition could quietly return, refuses.
///
/// Perturbation: add a second package to `[workspace] members`, or give it a
/// dependency on this crate, and this fails naming it. Watched under exactly
/// those changes.
#[test]
fn no_workspace_crate_depends_on_this_one() {
    let out = Command::new(env!("CARGO"))
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .args([
            "metadata",
            "--no-deps",
            "--format-version",
            "1",
            "--locked",
            "--offline",
        ])
        .output()
        .expect("cargo metadata runs");
    assert!(
        out.status.success(),
        "cargo metadata failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let metadata: serde_json::Value =
        serde_json::from_slice(&out.stdout).expect("cargo metadata is json");
    let packages = metadata["packages"].as_array().expect("a package list");

    let names: Vec<&str> = packages
        .iter()
        .map(|p| p["name"].as_str().expect("a package name"))
        .collect();
    assert_eq!(
        names,
        ["weaver-analysis"],
        "the workspace is this crate alone, and cargo answers for these"
    );
    let manifest = std::path::PathBuf::from(
        packages[0]["manifest_path"]
            .as_str()
            .expect("a manifest path"),
    )
    .canonicalize()
    .expect("a member manifest resolves");
    assert_eq!(
        manifest,
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("Cargo.toml")
            .canonicalize()
            .expect("this manifest resolves"),
        "the one member is the crate at the repository root"
    );

    let dependents: Vec<String> = packages
        .iter()
        .filter(|p| p["name"] != "weaver-analysis")
        .filter(|p| {
            p["dependencies"]
                .as_array()
                .expect("a dependency list")
                .iter()
                .any(|d| d["name"] == "weaver-analysis")
        })
        .map(|p| p["name"].as_str().expect("a package name").to_string())
        .collect();
    assert!(
        dependents.is_empty(),
        "weaver-analysis stays a leaf, and these depend on it: {dependents:?}"
    );
}
