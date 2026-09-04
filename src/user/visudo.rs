//! `visudo`: edit or check the sudoers file.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "visudo", trait_path = "::xccute_runtime::ShellCommand")]
pub struct VisudoBuilder {
    #[builder(flag)]
    #[shell(flag = "-q")]
    pub quiet: bool,
    #[builder(flag, method = "check_syntax")]
    #[shell(flag = "-c")]
    pub check: bool,
    #[builder(opt_into)]
    #[shell(opt_kv = "-f")]
    pub file: Option<String>,
}
