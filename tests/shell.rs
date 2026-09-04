//! The shell builders, the exact command each renders.

use xccute_commands::shell::*;
use xccute_runtime::ShellCommand;

fn assert_build<T: ShellCommand>(cmd: T, expected: &str) {
    assert_eq!(cmd.build(), expected);
}

#[test]
fn shell_builders_render_their_commands() {
    assert_build(BashBuilder::new().exit_on_error().verbose().login().command("echo hi"), "bash -e -x -l -c \"echo hi\"");
    assert_build(ShBuilder::new().login().restricted().command("echo hi"), "sh -l -r -c \"echo hi\"");
    assert_build(ZshBuilder::new().login().interactive().emacs().command("echo hi"), "zsh -l -i -e -c \"echo hi\"");
    assert_build(BashBuilder::new(), "bash");
}
