//! `ps aux | grep <pattern>`.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "ps aux | grep", trait_path = "::xccute_runtime::ShellCommand")]
pub struct PsGrepBuilder {
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub pattern: String,
}
