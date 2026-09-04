//! `usermod`: modify a user account.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "usermod", require_order, trait_path = "::xccute_runtime::ShellCommand")]
pub struct UserModBuilder {
    #[builder(required, into)]
    #[shell(arg_clone, order = "last")]
    pub username: String,
    #[builder(opt_into, method = "rename")]
    #[shell(opt_kv = "-l", order = 1)]
    pub new_name: Option<String>,
    #[builder(opt_into)]
    #[shell(opt_kv = "-d", order = 2)]
    pub home_dir: Option<String>,
    #[builder(flag)]
    #[shell(flag = "-m", order = 3)]
    pub move_home: bool,
    #[builder(opt_into)]
    #[shell(opt_kv = "-s", order = 4)]
    pub shell: Option<String>,
    #[builder(opt_into)]
    #[shell(opt_kv = "-g", order = 5)]
    pub group: Option<String>,
    #[builder(opt_into)]
    #[shell(opt_kv = "-a -G", order = 6)]
    pub append_group: Option<String>,
    #[builder(opt)]
    #[shell(opt_kv = "-u", order = 7)]
    pub uid: Option<u32>,
    #[builder(opt)]
    #[shell(opt_kv = "-g", order = 8)]
    pub gid: Option<u32>,
    #[builder(flag)]
    #[shell(flag = "-L", order = 9)]
    pub lock: bool,
    #[builder(flag)]
    #[shell(flag = "-U", order = 10)]
    pub unlock: bool,
}
