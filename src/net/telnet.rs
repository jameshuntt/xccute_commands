//! `telnet`.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "telnet", trait_path = "::xccute_runtime::ShellCommand")]
pub struct TelnetBuilder {
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub host: String,
    #[builder(opt)]
    #[shell(positional)]
    pub port: Option<u16>,
    #[builder(opt)]
    #[shell(opt_kv = "-t")]
    pub timeout: Option<u64>,
    #[builder(opt)]
    #[shell(opt_kv = "-e")]
    pub escape_char: Option<char>,
}
