//! Every proc builder, the exact command it renders.

use xccute_commands::proc::*;
use xccute_runtime::ShellCommand;

fn assert_build<T: ShellCommand>(cmd: T, expected: &str) {
    assert_eq!(cmd.build(), expected);
}

#[test]
fn process_builders_render_their_commands() {
    assert_build(HtopBuilder::new().user("james").sort_key("PERCENT_CPU").tree().delay(10).no_colors().no_mouse(), "htop --user james --sort-key PERCENT_CPU --tree --delay 10 --no-color --no-mouse");
    assert_build(IdBuilder::new().user_id_only().group_id_only().groups().name().username("james"), "id -u -g -G -n james");
    assert_build(KillBuilder::new().signal("9").pid(1234), "kill -9 1234");
    assert_build(KillBuilder::new().signal("TERM").pids(vec![1, 2]), "kill -TERM 1 2");
    assert_build(PgrepBuilder::new().full().newest().oldest().exact().user("james").group("staff").pattern("node"), "pgrep -f -n -o -x -u james -G staff node");
    assert_build(PkillBuilder::new("node").signal("TERM"), "pkill -TERM node");
    assert_build(PkillBuilder::new("node"), "pkill node");
    assert_build(PsBuilder::new().all().user().format("pid,cmd").sort("-pid").arg("x"), "ps a u --format=pid,cmd --sort=-pid x");
    assert_build(PsGrepBuilder::new("node"), "ps aux | grep node");
    assert_build(TopBuilder::new().batch_mode().delay(1.5).iterations(5).user("james").pid(1234), "top -b -d 1.5 -n 5 -u james -p 1234");
    assert_build(UptimeBuilder::new(), "uptime");
    assert_build(WBuilder::new().short().user("james"), "w -s james");
    assert_build(WhoBuilder::new().all().count().heading(), "who -a -q -H");
}
