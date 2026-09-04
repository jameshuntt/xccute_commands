//! `ftp`: the classic file transfer client.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "ftp", trait_path = "::xccute_runtime::ShellCommand")]
pub struct FtpBuilder {
    #[builder(required, into)]
    #[shell(arg_join_opt = "port", sep = ":")]
    pub host: String,
    #[builder(opt)]
    pub port: Option<u16>,
    #[builder(opt_into)]
    pub user: Option<String>,
    #[builder(opt_into)]
    pub password: Option<String>,
    #[builder(opt_into, method = "script")]
    #[shell(opt_kv = "-s:")]
    pub script_path: Option<String>,
    #[builder(flag)]
    pub passive: bool,
    #[builder(flag)]
    #[shell(flag = "-n")]
    pub no_auto_login: bool,
}
