//! `chmod`: change file mode bits.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "chmod", trait_path = "::xccute_runtime::ShellCommand")]
pub struct ChmodBuilder {
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub mode: String,
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub target: String,
    #[builder(flag)]
    #[shell(flag = "-R")]
    pub recursive: bool,
    #[builder(opt_into)]
    #[shell(opt_kv = "--reference")]
    pub reference: Option<String>,
}
