//! `groupadd`: create a new group.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "groupadd", trait_path = "::xccute_runtime::ShellCommand")]
pub struct GroupAddBuilder {
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub groupname: String,
    #[builder(flag)]
    #[shell(flag = "--system")]
    pub system: bool,
    #[builder(opt)]
    #[shell(opt_kv = "-g")]
    pub gid: Option<u32>,
}
