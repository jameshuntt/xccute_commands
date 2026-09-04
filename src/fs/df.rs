//! `df`: report file system disk space usage.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "df", trait_path = "::xccute_runtime::ShellCommand")]
pub struct DfBuilder {
    #[builder(flag)]
    #[shell(flag = "-h")]
    pub human_readable: bool,
    #[builder(flag)]
    #[shell(flag = "-a")]
    pub all: bool,
    #[builder(flag)]
    #[shell(flag = "-l")]
    pub local: bool,
    #[builder(flag)]
    #[shell(flag = "-i")]
    pub inodes: bool,
    #[builder(opt_into)]
    #[shell(positional)]
    pub path: Option<String>,
}
