//! `wget`: non-interactive network downloader.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "wget", trait_path = "::xccute_runtime::ShellCommand")]
pub struct WgetBuilder {
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub url: String,
    #[builder(flag)]
    #[shell(flag = "--recursive")]
    pub recursive: bool,
    #[builder(flag)]
    #[shell(flag = "--no-clobber")]
    pub no_clobber: bool,
    #[builder(flag)]
    #[shell(flag = "--quiet")]
    pub quiet: bool,
    #[builder(opt_into)]
    #[shell(opt_kv = "--limit-rate")]
    pub limit_rate: Option<String>,
    #[builder(opt_into)]
    #[shell(opt_kv = "--user-agent")]
    pub user_agent: Option<String>,
    #[builder(opt_into)]
    #[shell(opt_kv = "-O")]
    pub output_file: Option<String>,
}
