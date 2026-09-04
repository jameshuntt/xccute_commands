//! `sh`: run a command line through the POSIX shell.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "sh", trait_path = "::xccute_runtime::ShellCommand")]
pub struct ShBuilder {
    #[builder(flag)]
    #[shell(flag = "-l")]
    pub login: bool,
    #[builder(flag)]
    #[shell(flag = "-r")]
    pub restricted: bool,
    #[builder(opt_into)]
    #[shell(opt_kv = "-c", fmt = "\"{}\"")]
    pub command: Option<String>,
}
