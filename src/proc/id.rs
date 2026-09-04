//! `id`: print user and group ids.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "id", trait_path = "::xccute_runtime::ShellCommand")]
pub struct IdBuilder {
    #[builder(opt_into)]
    #[shell(positional)]
    pub username: Option<String>,
    #[builder(flag)]
    #[shell(flag = "-u")]
    pub user_id_only: bool,
    #[builder(flag)]
    #[shell(flag = "-g")]
    pub group_id_only: bool,
    #[builder(flag)]
    #[shell(flag = "-G")]
    pub groups: bool,
    #[builder(flag)]
    #[shell(flag = "-n")]
    pub name: bool,
}
