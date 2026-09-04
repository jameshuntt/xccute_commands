//! `touch`: change file timestamps, creating the file unless told not to.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "touch", trait_path = "::xccute_runtime::ShellCommand")]
pub struct TouchBuilder {
    /// Creating is the default; this records the intent and emits nothing.
    #[builder(flag)]
    pub create: bool,
    #[builder(flag)]
    #[shell(flag = "-c")]
    pub no_create: bool,
    #[builder(opt_into)]
    #[shell(opt_kv = "-d")]
    pub date: Option<String>,
    #[builder(opt_into)]
    #[shell(opt_kv = "-r")]
    pub reference: Option<String>,
    #[builder(opt_into)]
    #[shell(opt_kv = "-t")]
    pub time: Option<String>,
    #[builder(opt_into)]
    #[shell(positional)]
    pub file: Option<String>,
}
