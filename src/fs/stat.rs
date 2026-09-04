//! `stat`: display file status.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "stat", trait_path = "::xccute_runtime::ShellCommand")]
pub struct StatBuilder {
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub path: String,
    #[builder(flag)]
    #[shell(flag = "-L")]
    pub follow_symlinks: bool,
    #[builder(opt_into)]
    #[shell(opt_kv = "--format")]
    pub format: Option<String>,
}
