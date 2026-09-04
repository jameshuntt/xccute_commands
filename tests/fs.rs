//! Every fs builder, the exact command it renders.

use xccute_commands::fs::*;
use xccute_runtime::ShellCommand;

fn assert_build<T: ShellCommand>(cmd: T, expected: &str) {
    assert_eq!(cmd.build(), expected);
}

#[test]
fn filesystem_builders_render_their_commands() {
    assert_build(CdBuilder::new("work").join("src"), "cd work/src");
    assert_build(ChmodBuilder::new("755", "bin/tool").recursive(), "chmod -R 755 bin/tool");
    assert_build(ChmodBuilder::new("755", "bin/tool").reference("ref"), "chmod --reference ref 755 bin/tool");
    assert_build(
        ChownBuilder::new("james", "/var/log/app.log").group("staff").recursive().no_dereference().from("root", Some("wheel".to_string())),
        "chown -R -h --from root:wheel james:staff /var/log/app.log",
    );
    assert_build(ChownBuilder::new("james", "f").from("root", None), "chown --from root james f");
    assert_build(CpBuilder::new("a", "b").recursive().force().verbose(), "cp -r -f -v a b");
    assert_build(DfBuilder::new().human_readable().all().local().inodes().path("/"), "df -h -a -l -i /");
    assert_build(DfBuilder::new(), "df");
    assert_build(DuBuilder::new("/var").summarize().human_readable().apparent_size().all().max_depth(2), "du -s -h --apparent-size -a --max-depth=2 /var");
    assert_build(EchoBuilder::new("hello").quoted().no_newline().to_file("out.txt").append(), "echo -n \"hello\" >> out.txt");
    assert_build(EchoBuilder::new("hello").to_file("out.txt"), "echo hello > out.txt");
    assert_build(EchoBuilder::new("hello"), "echo hello");
    assert_build(FileBuilder::new("archive.tar.gz").mime().brief().dereference().special_files(), "file --mime-type --brief --dereference --special-files archive.tar.gz");
    assert_build(LnBuilder::new("src", "dst").symbolic().force().no_dereference().verbose(), "ln -s -f -n -v src dst");
    assert_build(LinkBuilder::new("src", "dst"), "ln src dst");
    assert_build(LsBuilder::new().long().all().human_readable().target("/tmp"), "ls -l -a -h /tmp");
    assert_build(MkdirBuilder::new("target/demo").parents().verbose().mode("755"), "mkdir -p -v -m 755 target/demo");
    assert_build(MvBuilder::new("old", "new").force().interactive().verbose().no_clobber().backup(), "mv -f -i -v -n -b old new");
    assert_build(NanoBuilder::new("notes.md").sudo(), "sudo nano notes.md");
    assert_build(NanoBuilder::new("notes.md"), "nano notes.md");
    assert_build(RmBuilder::new("target/tmp").recursive().force().interactive().verbose(), "rm -r -f -i -v target/tmp");
    assert_build(RmdirBuilder::new("empty-dir").recursive().verbose(), "rmdir -r -v empty-dir");
    assert_build(StatBuilder::new("/tmp/file").follow_symlinks().format("%n:%s"), "stat -L --format %n:%s /tmp/file");
    assert_build(TouchBuilder::new().create().no_create().date("2024-12-25").reference("ref").time("202412250101").file("target"), "touch -c -d 2024-12-25 -r ref -t 202412250101 target");
}
