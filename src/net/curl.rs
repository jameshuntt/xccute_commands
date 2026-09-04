//! `curl`: transfer a URL.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "curl", require_order, trait_path = "::xccute_runtime::ShellCommand")]
pub struct CurlBuilder {
    #[builder(required, into)]
    #[shell(arg_clone, order = "last")]
    pub url: String,
    #[builder(flag)]
    #[shell(flag = "-s", order = 1)]
    pub silent: bool,
    #[builder(flag)]
    #[shell(flag = "-v", order = 2)]
    pub verbose: bool,
    #[builder(flag)]
    #[shell(flag = "-L", order = 3)]
    pub follow_redirects: bool,
    #[builder(opt_into)]
    #[shell(opt_kv = "-o", order = 4)]
    pub output: Option<String>,
    #[builder(push, method = "header")]
    #[shell(multi_opt_kv = "-H", fmt = "'{}'", order = 5)]
    pub headers: Vec<String>,
    #[builder(opt_into, method = "method")]
    #[shell(opt_kv = "-X", order = 6)]
    pub request_method: Option<String>,
    #[builder(opt_into)]
    #[shell(opt_kv = "-d", fmt = "'{}'", order = 7)]
    pub data: Option<String>,
    #[builder(opt_into)]
    #[shell(opt_kv = "-u", order = 8)]
    pub user: Option<String>,
    #[builder(flag)]
    #[shell(flag = "-k", order = 9)]
    pub insecure: bool,
}
