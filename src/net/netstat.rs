//! `netstat`: network connections, routing tables, interface statistics.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "netstat", trait_path = "::xccute_runtime::ShellCommand")]
pub struct NetstatBuilder {
    #[builder(flag)]
    #[shell(flag = "-t")]
    pub tcp: bool,
    #[builder(flag)]
    #[shell(flag = "-u")]
    pub udp: bool,
    #[builder(flag)]
    #[shell(flag = "-l")]
    pub listening: bool,
    #[builder(flag)]
    #[shell(flag = "-p")]
    pub programs: bool,
    #[builder(flag)]
    #[shell(flag = "-n")]
    pub numeric: bool,
    #[builder(flag)]
    #[shell(flag = "-r")]
    pub routes: bool,
    #[builder(flag)]
    #[shell(flag = "-a")]
    pub all: bool,
    #[builder(flag)]
    #[shell(flag = "-i")]
    pub interfaces: bool,
}
