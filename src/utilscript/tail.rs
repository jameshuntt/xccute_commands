//! `tail`: output the last part of files.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "tail", require_order, trait_path = "::xccute_runtime::ShellCommand")]
pub struct TailBuilder {
    #[builder(opt)]
    #[shell(opt_kv = "-n", order = 1)]
    pub lines: Option<usize>,
    #[builder(opt)]
    #[shell(opt_kv = "-c", order = 2)]
    pub bytes: Option<usize>,
    #[builder(flag)]
    #[shell(flag = "-f", order = 3)]
    pub follow: bool,
    #[builder(flag)]
    #[shell(flag = "-q", order = 4)]
    pub quiet: bool,
    #[builder(opt_into)]
    #[shell(positional, order = 5)]
    pub file: Option<String>,
}
