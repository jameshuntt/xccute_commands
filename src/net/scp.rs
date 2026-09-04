//! `scp`: secure copy.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "scp", trait_path = "::xccute_runtime::ShellCommand")]
pub struct ScpBuilder {
    #[builder(flag)]
    #[shell(flag = "-r")]
    pub recursive: bool,
    #[builder(flag)]
    #[shell(flag = "-p")]
    pub preserve_times: bool,
    #[builder(flag)]
    #[shell(flag = "-q")]
    pub quiet: bool,
    #[builder(opt)]
    #[shell(opt_kv = "-P")]
    pub port: Option<u16>,
    #[builder(opt_into)]
    #[shell(opt_kv = "-i")]
    pub identity_file: Option<String>,
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub from: String,
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub to: String,
}
