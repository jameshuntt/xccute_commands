//! `rmdir`: remove empty directories.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "rmdir", trait_path = "::xccute_runtime::ShellCommand")]
pub struct RmdirBuilder {
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub directory: String,
    #[builder(flag)]
    #[shell(flag = "-r")]
    pub recursive: bool,
    #[builder(flag)]
    #[shell(flag = "-v")]
    pub verbose: bool,
}
