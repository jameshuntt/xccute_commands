//! `su`: run a shell or command as another user.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "su", require_order, trait_path = "::xccute_runtime::ShellCommand")]
pub struct SuBuilder {
    #[builder(flag)]
    #[shell(flag = "-l", order = 1)]
    pub login: bool,
    #[builder(opt_into)]
    #[shell(positional, order = 2)]
    pub user: Option<String>,
    #[builder(opt_into)]
    #[shell(opt_kv = "-c", fmt = "'{}'", order = 3)]
    pub command: Option<String>,
}
