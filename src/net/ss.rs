//! `ss`: socket statistics.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "ss", trait_path = "::xccute_runtime::ShellCommand")]
pub struct SsBuilder {
    #[builder(flag)]
    #[shell(flag = "-a")]
    pub all: bool,
    #[builder(flag)]
    #[shell(flag = "-l")]
    pub listening: bool,
    #[builder(flag)]
    #[shell(flag = "-t")]
    pub tcp: bool,
    #[builder(flag)]
    #[shell(flag = "-u")]
    pub udp: bool,
    #[builder(flag)]
    #[shell(flag = "-x")]
    pub unix: bool,
    #[builder(flag)]
    #[shell(flag = "-p")]
    pub process: bool,
    #[builder(opt_into)]
    #[shell(opt_kv = "state")]
    pub state: Option<String>,
    #[builder(opt)]
    #[shell(opt_prefix = "sport = :")]
    pub port: Option<u16>,
}
