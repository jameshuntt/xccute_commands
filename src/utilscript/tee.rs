//! `tee`: copy stdin to files and stdout.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "tee", trait_path = "::xccute_runtime::ShellCommand")]
pub struct TeeBuilder {
    #[builder(flag)]
    #[shell(flag = "-a")]
    pub append: bool,
    #[builder(flag)]
    #[shell(flag = "-i")]
    pub ignore_interrupts: bool,
    #[builder(push, method = "file")]
    #[shell(positional)]
    pub files: Vec<String>,
}
