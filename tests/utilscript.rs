//! Every utilscript builder, the exact command it renders.

use xccute_commands::utilscript::*;
use xccute_runtime::ShellCommand;

fn assert_build<T: ShellCommand>(cmd: T, expected: &str) {
    assert_eq!(cmd.build(), expected);
}

#[test]
fn text_utility_builders_render_their_commands() {
    assert_build(AwkBuilder::new("{print $1}").field_separator(",").with_var("threshold", "10").file("input.csv"), "awk -F ',' threshold=10 '{print $1}' input.csv");
    assert_build(AwkBuilder::new("{print $1}"), "awk '{print $1}'");
    assert_build(CutBuilder::new().complement().only_delimited().delimiter(",").fields("1,3").bytes("1-10").characters("1-5").file("input.csv"), "cut --complement --only-delimited -d , -f 1,3 -b 1-10 -c 1-5 input.csv");
    assert_build(GrepBuilder::new("NEEDLE").ignore_case().recursive().line_number().invert_match().word_regexp().extended_regex().file("a.log").file("b.log"), "grep -i -r -n -v -w -E NEEDLE a.log b.log");
    assert_build(GrepBuilder::new("NEEDLE").files(vec!["a", "b"]), "grep NEEDLE a b");
    assert_build(HeadBuilder::new().lines(5).bytes(100).quiet().verbose().file("input.log"), "head -n 5 -c 100 -q -v input.log");
    assert_build(SedBuilder::new().suppress_output().in_place().backup_extension(".bak").expression("s/a/b/g").file("input.txt"), "sed -n -i .bak -e s/a/b/g input.txt");
    assert_build(SedBuilder::new().backup_extension(".bak").expression("s/a/b/g"), "sed -e s/a/b/g");
    assert_build(SortBuilder::new().numeric().reverse().human_numeric().ignore_case().unique().key("2,2").file("input.txt"), "sort -n -r -h -f -u -k 2,2 input.txt");
    assert_build(TailBuilder::new().lines(20).bytes(200).follow().quiet().file("input.log"), "tail -n 20 -c 200 -f -q input.log");
    assert_build(TeeBuilder::new().append().ignore_interrupts().file("a.out").file("b.out"), "tee -a -i a.out b.out");
    assert_build(UniqBuilder::new().count().repeated().unique_only().ignore_case().skip_fields(1).skip_chars(2).input("input.txt"), "uniq -c -d -u -i -f 1 -s 2 input.txt");
    assert_build(WcBuilder::new().bytes().chars().lines().words().max_line_length().file("input.txt"), "wc -c -m -l -w -L input.txt");
    assert_build(XargsBuilder::new().command("rm").max_args(2).delimiter("\\n").replace("{}").interactive().verbose(), "xargs rm -n 2 -d '\\n' -I {} -p --verbose");
    assert_build(xccute_commands::grep::GrepBuilder::new("NEEDLE").file("a.log"), "grep NEEDLE a.log");
}
