//! Every net builder, the exact command it renders.

use xccute_commands::net::*;
use xccute_runtime::ShellCommand;

fn assert_build<T: ShellCommand>(cmd: T, expected: &str) {
    assert_eq!(cmd.build(), expected);
}

#[test]
fn network_builders_render_their_commands() {
    assert_build(CurlBuilder::new("https://example.invalid").silent().verbose().follow_redirects().output("out.json").header("Accept: application/json").method("POST").data("{x:1}").user("me:secret").insecure(), "curl -s -v -L -o out.json -H 'Accept: application/json' -X POST -d '{x:1}' -u me:secret -k https://example.invalid");
    assert_build(CurlBuilder::new("https://example.invalid").header("A: 1").header("B: 2"), "curl -H 'A: 1' -H 'B: 2' https://example.invalid");
    assert_build(FtpBuilder::new("ftp.example.invalid").no_auto_login().script("commands.ftp").port(2121), "ftp -n -s: commands.ftp ftp.example.invalid:2121");
    assert_build(FtpBuilder::new("h").user("u").password("p").passive(), "ftp h");
    assert_build(IpBuilder::new().command(IpCommand::Addr).args(&["show", "eth0"]), "ip addr show eth0");
    assert_build(IpBuilder::new().command(IpCommand::Custom("foo".into())).arg("x"), "ip foo x");
    assert_build(NetcatBuilder::new().listen().udp().zero_io().input("in").output("out").host("127.0.0.1").port(8080), "nc -l -u -z -i in -o out 127.0.0.1 8080");
    assert_build(NetstatBuilder::new().tcp().udp().listening().programs().numeric().routes().all().interfaces(), "netstat -t -u -l -p -n -r -a -i");
    assert_build(PingBuilder::new("::1").ipv6().count(4).interval(1.5).timeout(2).packet_size(64).quiet(), "ping6 -c 4 -i 1.5 -W 2 -s 64 -q ::1");
    assert_build(PingBuilder::new("h"), "ping h");
    assert_build(RsyncBuilder::new("src/", "dst/").archive().verbose().recursive().compress().delete().dry_run().human_readable().progress().preserve_times().preserve_permissions(), "rsync -a -v -r -z --delete --dry-run -h --progress -t -p src/ dst/");
    assert_build(ScpBuilder::new("file", "host:/tmp/file").recursive().preserve_times().quiet().port(2222).identity_file("id_rsa"), "scp -r -p -q -P 2222 -i id_rsa file host:/tmp/file");
    assert_build(SftpBuilder::new("example.invalid").user("admin").identity_file("id_rsa").port(22).batch_file("batch.sftp").interactive(), "sftp -i id_rsa -P 22 -b batch.sftp -C admin@example.invalid");
    assert_build(SftpBuilder::new("example.invalid"), "sftp example.invalid");
    assert_build(SsBuilder::new().all().listening().tcp().udp().unix().process().state("ESTABLISHED").port(443), "ss -a -l -t -u -x -p state ESTABLISHED sport = :443");
    assert_build(TelnetBuilder::new("towel.blinkenlights.nl").timeout(5).escape_char('^').port(23), "telnet -t 5 -e ^ towel.blinkenlights.nl 23");
    assert_build(WgetBuilder::new("https://example.invalid/file").recursive().no_clobber().quiet().limit_rate("500k").user_agent("xccute-test").output_file("file"), "wget --recursive --no-clobber --quiet --limit-rate 500k --user-agent xccute-test -O file https://example.invalid/file");
}
