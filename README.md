# xccute_commands

Typed builders for the shell commands people actually run.

```rust
use xccute_commands::{fs::LsBuilder, net::CurlBuilder, proc::KillBuilder, user::SudoBuilder};
use xccute_runtime::ShellCommand;

assert_eq!(LsBuilder::new().long().all().target("/tmp").build(), "ls -l -a /tmp");
assert_eq!(KillBuilder::new().signal("9").pid(1234).build(), "kill -9 1234");
assert_eq!(
    CurlBuilder::new("https://example.invalid").silent().header("Accept: application/json").build(),
    "curl -s -H 'Accept: application/json' https://example.invalid"
);
assert_eq!(
    SudoBuilder::new("systemctl status app").user("root").build(),
    "sudo -u root systemctl status app"
);
```

Every builder is a struct: `new()` takes what the command cannot run without,
a setter per option, and `build()` from `xccute_runtime::ShellCommand`
renders the command line. `to_command()` wraps it in `sh -c`; for untrusted
values use the argv commands of `xccute_contract` instead.

| family | commands |
|---|---|
| `fs` | cd, chmod, chown, cp, df, du, echo, file, ln, ls, mkdir, mv, nano, rm, rmdir, stat, touch |
| `net` | curl, ftp, ip, nc, netstat, ping, rsync, scp, sftp, ss, telnet, wget |
| `proc` | htop, id, kill, pgrep, pkill, ps, ps aux \| grep, top, uptime, w, who |
| `user` | adduser, groupadd, passwd, su, sudo, useradd, usermod, visudo |
| `utilscript` | awk, cut, grep, head, sed, sort, tail, tee, uniq, wc, xargs |
| `shell` | bash, sh, zsh |
| `third_party`, `cargo`, `psql` | git, cargo, psql |

Each builder is `#[derive(SimpleImpl)]` from
[`simple_impl_derive`](https://crates.io/crates/simple_impl_derive): the
struct's attributes are the whole definition, and a test per family pins
the exact string every builder renders.

## License

MIT OR Apache-2.0.
