//! `ping` and `ping6`.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd_expr = "if self.ipv6 { \"ping6\" } else { \"ping\" }", require_order, trait_path = "::xccute_runtime::ShellCommand")]
pub struct PingBuilder {
    #[builder(required, into)]
    #[shell(arg_clone, order = "last")]
    pub host: String,
    #[builder(opt)]
    #[shell(opt_kv = "-c", order = 1)]
    pub count: Option<u32>,
    #[builder(opt)]
    #[shell(opt_kv = "-i", order = 2)]
    pub interval: Option<f32>,
    #[builder(opt)]
    #[shell(opt_kv = "-W", order = 3)]
    pub timeout: Option<u32>,
    #[builder(opt)]
    #[shell(opt_kv = "-s", order = 4)]
    pub packet_size: Option<u32>,
    #[builder(flag)]
    pub ipv6: bool,
    #[builder(flag)]
    #[shell(flag = "-q", order = 5)]
    pub quiet: bool,
}
