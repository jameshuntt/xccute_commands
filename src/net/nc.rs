//! `nc`: netcat.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "nc", trait_path = "::xccute_runtime::ShellCommand")]
pub struct NetcatBuilder {
    #[builder(flag)]
    #[shell(flag = "-l")]
    pub listen: bool,
    #[builder(flag)]
    #[shell(flag = "-u")]
    pub udp: bool,
    #[builder(flag)]
    #[shell(flag = "-z")]
    pub zero_io: bool,
    #[builder(opt_into, method = "input")]
    #[shell(opt_kv = "-i")]
    pub input_file: Option<String>,
    #[builder(opt_into, method = "output")]
    #[shell(opt_kv = "-o")]
    pub output_file: Option<String>,
    #[builder(opt_into)]
    #[shell(positional)]
    pub host: Option<String>,
    #[builder(opt)]
    #[shell(positional)]
    pub port: Option<u16>,
}
