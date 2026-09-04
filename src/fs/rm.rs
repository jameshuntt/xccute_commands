//! `rm`: remove files or directories.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "rm", trait_path = "::xccute_runtime::ShellCommand")]
pub struct RmBuilder {
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub path: String,
    #[builder(flag)]
    #[shell(flag = "-r")]
    pub recursive: bool,
    #[builder(flag)]
    #[shell(flag = "-f")]
    pub force: bool,
    #[builder(flag)]
    #[shell(flag = "-i")]
    pub interactive: bool,
    #[builder(flag)]
    #[shell(flag = "-v")]
    pub verbose: bool,
}
