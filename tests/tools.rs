//! git, cargo and psql, the exact command each renders.

use xccute_commands::{CargoBuilder, GitBuilder, PsqlCommandBuilder};
use xccute_runtime::ShellCommand;

fn assert_build<T: ShellCommand>(cmd: T, expected: &str) {
    assert_eq!(cmd.build(), expected);
}

#[test]
fn tool_builders_render_their_commands() {
    assert_build(GitBuilder::clone_mirror("https://example.invalid/repo.git", "repo.git"), "git clone --mirror https://example.invalid/repo.git repo.git");
    assert_build(GitBuilder::clone_working("m", "w"), "git clone m w");
    assert_build(GitBuilder::worktree_add("v1", "out"), "git worktree add --detach out v1");
    assert_build(GitBuilder::new("status").arg("--short"), "git status --short");
    assert_build(CargoBuilder::new("test").package("xccute").manifest_path("Cargo.toml").features(&["a", "b"]).release().target("wasm32-unknown-unknown"), "cargo test --release --package xccute --manifest-path Cargo.toml --features a,b --target wasm32-unknown-unknown");
    assert_build(CargoBuilder::new("new").name("my_crate").lib().args(&["--vcs", "none"]), "cargo new my_crate --lib --vcs none");
    assert_build(CargoBuilder::new("build").feature("x").bin(), "cargo build --bin --features x");
    assert_build(PsqlCommandBuilder::new("SELECT 1;").database("app").user("postgres").quiet(), "psql -d app -U postgres -q -c \"SELECT 1;\"");
    assert_build(PsqlCommandBuilder::new("SELECT 1;").output(), "psql -c \"SELECT 1;\"");
}
