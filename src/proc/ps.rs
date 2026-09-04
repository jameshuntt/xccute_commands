//! `ps`: report a snapshot of current processes.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "ps", trait_path = "::xccute_runtime::ShellCommand")]
pub struct PsBuilder {
    #[builder(flag)]
    #[shell(flag = "a")]
    pub all: bool,
    #[builder(flag)]
    #[shell(flag = "u")]
    pub user: bool,
    #[builder(opt_into)]
    #[shell(opt_eq = "--format")]
    pub format: Option<String>,
    #[builder(opt_into)]
    #[shell(opt_eq = "--sort")]
    pub sort: Option<String>,
    #[builder(push, method = "arg")]
    #[shell(positional)]
    pub custom_args: Vec<String>,
}
