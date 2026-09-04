//! `zsh`: run a command line through zsh.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "zsh", trait_path = "::xccute_runtime::ShellCommand")]
pub struct ZshBuilder {
    #[builder(flag)]
    #[shell(flag = "-l")]
    pub login: bool,
    #[builder(flag)]
    #[shell(flag = "-i")]
    pub interactive: bool,
    #[builder(flag)]
    #[shell(flag = "-e")]
    pub emacs: bool,
    #[builder(opt_into)]
    #[shell(opt_kv = "-c", fmt = "\"{}\"")]
    pub command: Option<String>,
}
