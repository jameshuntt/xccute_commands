//! Every user builder, the exact command it renders.

use xccute_commands::user::*;
use xccute_runtime::ShellCommand;

fn assert_build<T: ShellCommand>(cmd: T, expected: &str) {
    assert_eq!(cmd.build(), expected);
}

#[test]
fn user_builders_render_their_commands() {
    assert_build(AddUserBuilder::new("agent").system().disabled_login().home("/srv/agent").shell("/usr/sbin/nologin").group("agents").gecos("Service Agent"), "adduser --system --disabled-login --home /srv/agent --shell /usr/sbin/nologin --ingroup agents --gecos \"Service Agent\" agent");
    assert_build(GroupAddBuilder::new("agents").system().gid(900), "groupadd --system -g 900 agents");
    assert_build(PasswdBuilder::new().stdin().user("james"), "echo \"<password>\" | passwd james");
    assert_build(PasswdBuilder::new().user("james"), "passwd james");
    assert_build(SuBuilder::new().login().user("root").command("whoami"), "su -l root -c 'whoami'");
    assert_build(SudoBuilder::new("systemctl status app").preserve_env().login_shell().user("root").group("wheel"), "sudo -E -i -u root -g wheel systemctl status app");
    assert_build(UserAddBuilder::new("agent").system().create_home().home_dir("/srv/agent").shell("/usr/sbin/nologin").group("agents").uid(900), "useradd --system -m -d /srv/agent -s /usr/sbin/nologin -g agents -u 900 agent");
    assert_build(UserModBuilder::new("agent").rename("agent2").home_dir("/srv/agent2").move_home().shell("/bin/bash").group("agents").append_group("docker").uid(901).gid(902).lock(), "usermod -l agent2 -d /srv/agent2 -m -s /bin/bash -g agents -a -G docker -u 901 -g 902 -L agent");
    assert_build(UserModBuilder::new("agent").unlock(), "usermod -U agent");
    assert_build(VisudoBuilder::new().quiet().check_syntax().file("/etc/sudoers.d/app"), "visudo -q -c -f /etc/sudoers.d/app");
}
