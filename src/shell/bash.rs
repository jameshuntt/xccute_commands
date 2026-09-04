//! `bash`: run a command line through bash.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "bash", trait_path = "::xccute_runtime::ShellCommand")]
pub struct BashBuilder {
    #[builder(flag)]
    #[shell(flag = "-e")]
    pub exit_on_error: bool,
    #[builder(flag)]
    #[shell(flag = "-x")]
    pub verbose: bool,
    #[builder(flag)]
    #[shell(flag = "-l")]
    pub login: bool,
    #[builder(opt_into)]
    #[shell(opt_kv = "-c", fmt = "\"{}\"")]
    pub command: Option<String>,
}
