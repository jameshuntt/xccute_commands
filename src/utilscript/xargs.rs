//! `xargs`: build and run command lines from stdin.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "xargs", require_order, trait_path = "::xccute_runtime::ShellCommand")]
pub struct XargsBuilder {
    #[builder(opt_into)]
    #[shell(positional, order = 1)]
    pub command: Option<String>,
    #[builder(opt)]
    #[shell(opt_kv = "-n", order = 2)]
    pub max_args: Option<usize>,
    #[builder(opt_into)]
    #[shell(opt_kv = "-d", fmt = "'{}'", order = 3)]
    pub delimiter: Option<String>,
    #[builder(opt_into, method = "replace")]
    #[shell(opt_kv = "-I", order = 4)]
    pub replace_str: Option<String>,
    #[builder(flag)]
    #[shell(flag = "-p", order = 5)]
    pub interactive: bool,
    #[builder(flag)]
    #[shell(flag = "--verbose", order = 6)]
    pub verbose: bool,
}
