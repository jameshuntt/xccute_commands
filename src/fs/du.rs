//! `du`: estimate file space usage.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "du", trait_path = "::xccute_runtime::ShellCommand")]
pub struct DuBuilder {
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub path: String,
    #[builder(flag)]
    #[shell(flag = "-s")]
    pub summarize: bool,
    #[builder(flag)]
    #[shell(flag = "-h")]
    pub human_readable: bool,
    #[builder(flag)]
    #[shell(flag = "--apparent-size")]
    pub apparent_size: bool,
    #[builder(flag)]
    #[shell(flag = "-a")]
    pub all: bool,
    #[builder(opt)]
    #[shell(opt_eq = "--max-depth")]
    pub max_depth: Option<u32>,
}
