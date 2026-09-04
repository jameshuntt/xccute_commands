//! `pgrep`: look up processes by name and attributes.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "pgrep", trait_path = "::xccute_runtime::ShellCommand")]
pub struct PgrepBuilder {
    #[builder(opt_into)]
    #[shell(positional)]
    pub pattern: Option<String>,
    #[builder(flag)]
    #[shell(flag = "-f")]
    pub full: bool,
    #[builder(flag)]
    #[shell(flag = "-n")]
    pub newest: bool,
    #[builder(flag)]
    #[shell(flag = "-o")]
    pub oldest: bool,
    #[builder(flag)]
    #[shell(flag = "-x")]
    pub exact: bool,
    #[builder(opt_into)]
    #[shell(opt_kv = "-u")]
    pub user: Option<String>,
    #[builder(opt_into)]
    #[shell(opt_kv = "-G")]
    pub group: Option<String>,
}
