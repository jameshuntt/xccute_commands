//! `useradd`: create a new user.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "useradd", trait_path = "::xccute_runtime::ShellCommand")]
pub struct UserAddBuilder {
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub username: String,
    #[builder(flag)]
    #[shell(flag = "--system")]
    pub system: bool,
    #[builder(flag)]
    #[shell(flag = "-m")]
    pub create_home: bool,
    #[builder(opt_into)]
    #[shell(opt_kv = "-d")]
    pub home_dir: Option<String>,
    #[builder(opt_into)]
    #[shell(opt_kv = "-s")]
    pub shell: Option<String>,
    #[builder(opt_into)]
    #[shell(opt_kv = "-g")]
    pub group: Option<String>,
    #[builder(opt)]
    #[shell(opt_kv = "-u")]
    pub uid: Option<u32>,
}
