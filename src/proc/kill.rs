//! `kill`: send a signal to processes.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "kill", trait_path = "::xccute_runtime::ShellCommand")]
pub struct KillBuilder {
    #[builder(opt_into)]
    #[shell(opt_prefix = "-")]
    pub signal: Option<String>,
    #[builder(opt)]
    #[shell(positional)]
    pub pid: Option<u32>,
    #[builder(vec_into)]
    #[shell(positional)]
    pub pids: Vec<u32>,
}
