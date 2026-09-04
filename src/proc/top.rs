//! `top`: display processes.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "top", trait_path = "::xccute_runtime::ShellCommand")]
pub struct TopBuilder {
    #[builder(flag)]
    #[shell(flag = "-b")]
    pub batch_mode: bool,
    #[builder(opt)]
    #[shell(opt_kv = "-d")]
    pub delay: Option<f32>,
    #[builder(opt)]
    #[shell(opt_kv = "-n")]
    pub iterations: Option<u32>,
    #[builder(opt_into)]
    #[shell(opt_kv = "-u")]
    pub user: Option<String>,
    #[builder(opt)]
    #[shell(opt_kv = "-p")]
    pub pid: Option<u32>,
}
