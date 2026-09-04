//! `cp`: copy files and directories.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "cp", trait_path = "::xccute_runtime::ShellCommand")]
pub struct CpBuilder {
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub from: String,
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub to: String,
    #[builder(flag)]
    #[shell(flag = "-r")]
    pub recursive: bool,
    #[builder(flag)]
    #[shell(flag = "-f")]
    pub force: bool,
    #[builder(flag)]
    #[shell(flag = "-v")]
    pub verbose: bool,
}
