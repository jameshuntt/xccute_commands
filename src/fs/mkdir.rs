//! `mkdir`: make directories.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "mkdir", trait_path = "::xccute_runtime::ShellCommand")]
pub struct MkdirBuilder {
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub directory: String,
    #[builder(flag)]
    #[shell(flag = "-p")]
    pub parents: bool,
    #[builder(flag)]
    #[shell(flag = "-v")]
    pub verbose: bool,
    #[builder(opt_into)]
    #[shell(opt_kv = "-m")]
    pub mode: Option<String>,
}
