//! `w`: who is logged on and what they are doing.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "w", trait_path = "::xccute_runtime::ShellCommand")]
pub struct WBuilder {
    #[builder(flag)]
    #[shell(flag = "-s")]
    pub short: bool,
    #[builder(opt_into)]
    #[shell(positional)]
    pub user: Option<String>,
}
