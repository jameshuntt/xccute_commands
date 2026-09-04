//! `sudo`: run a command as another user.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "sudo", trait_path = "::xccute_runtime::ShellCommand")]
pub struct SudoBuilder {
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub command: String,
    #[builder(flag)]
    #[shell(flag = "-E")]
    pub preserve_env: bool,
    #[builder(flag)]
    #[shell(flag = "-i")]
    pub login_shell: bool,
    #[builder(opt_into)]
    #[shell(opt_kv = "-u")]
    pub user: Option<String>,
    #[builder(opt_into)]
    #[shell(opt_kv = "-g")]
    pub group: Option<String>,
}
