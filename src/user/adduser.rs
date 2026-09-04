//! `adduser`: add a user, Debian style.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "adduser", trait_path = "::xccute_runtime::ShellCommand")]
pub struct AddUserBuilder {
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub username: String,
    #[builder(flag)]
    #[shell(flag = "--system")]
    pub system: bool,
    #[builder(flag)]
    #[shell(flag = "--disabled-login")]
    pub disabled_login: bool,
    #[builder(opt_into)]
    #[shell(opt_kv = "--home")]
    pub home: Option<String>,
    #[builder(opt_into)]
    #[shell(opt_kv = "--shell")]
    pub shell: Option<String>,
    #[builder(opt_into)]
    #[shell(opt_kv = "--ingroup")]
    pub group: Option<String>,
    #[builder(opt_into)]
    #[shell(opt_kv = "--gecos", fmt = "\"{}\"")]
    pub gecos: Option<String>,
}
