//! `ls`: list directory contents.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "ls", trait_path = "::xccute_runtime::ShellCommand")]
pub struct LsBuilder {
    #[builder(flag)]
    #[shell(flag = "-l")]
    pub long: bool,
    #[builder(flag)]
    #[shell(flag = "-a")]
    pub all: bool,
    #[builder(flag)]
    #[shell(flag = "-h")]
    pub human_readable: bool,
    #[builder(opt_into, method = "target")]
    #[shell(positional)]
    pub path: Option<String>,
}
