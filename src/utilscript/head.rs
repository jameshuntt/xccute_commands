//! `head`: output the first part of files.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "head", require_order, trait_path = "::xccute_runtime::ShellCommand")]
pub struct HeadBuilder {
    #[builder(opt)]
    #[shell(opt_kv = "-n", order = 1)]
    pub lines: Option<u32>,
    #[builder(opt)]
    #[shell(opt_kv = "-c", order = 2)]
    pub bytes: Option<u32>,
    #[builder(flag)]
    #[shell(flag = "-q", order = 3)]
    pub quiet: bool,
    #[builder(flag)]
    #[shell(flag = "-v", order = 4)]
    pub verbose: bool,
    #[builder(opt_into)]
    #[shell(positional, order = 5)]
    pub file: Option<String>,
}
