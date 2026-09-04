//! `who`: who is logged on.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "who", trait_path = "::xccute_runtime::ShellCommand")]
pub struct WhoBuilder {
    #[builder(flag)]
    #[shell(flag = "-a")]
    pub all: bool,
    #[builder(flag)]
    #[shell(flag = "-q")]
    pub count: bool,
    #[builder(flag)]
    #[shell(flag = "-H")]
    pub heading: bool,
}
