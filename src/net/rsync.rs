//! `rsync`: fast, versatile file copying.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "rsync", trait_path = "::xccute_runtime::ShellCommand")]
pub struct RsyncBuilder {
    #[builder(flag)]
    #[shell(flag = "-a")]
    pub archive: bool,
    #[builder(flag)]
    #[shell(flag = "-v")]
    pub verbose: bool,
    #[builder(flag)]
    #[shell(flag = "-r")]
    pub recursive: bool,
    #[builder(flag)]
    #[shell(flag = "-z")]
    pub compress: bool,
    #[builder(flag)]
    #[shell(flag = "--delete")]
    pub delete: bool,
    #[builder(flag)]
    #[shell(flag = "--dry-run")]
    pub dry_run: bool,
    #[builder(flag)]
    #[shell(flag = "-h")]
    pub human_readable: bool,
    #[builder(flag)]
    #[shell(flag = "--progress")]
    pub progress: bool,
    #[builder(flag)]
    #[shell(flag = "-t")]
    pub preserve_times: bool,
    #[builder(flag)]
    #[shell(flag = "-p")]
    pub preserve_permissions: bool,
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub source: String,
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub destination: String,
}
