//! `pkill`: signal processes by name.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "pkill", trait_path = "::xccute_runtime::ShellCommand")]
pub struct PkillBuilder {
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub pattern: String,
    #[builder(opt_into)]
    #[shell(opt_prefix = "-")]
    pub signal: Option<String>,
}
