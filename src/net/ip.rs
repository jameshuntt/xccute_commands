//! `ip`: show and manipulate routing, devices and tunnels.
use simple_impl_derive::SimpleImpl;

/// The `ip` object to operate on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpCommand {
    Addr,
    Link,
    Route,
    Neigh,
    Rule,
    TcpMetrics,
    Maddr,
    Monitor,
    Xfrm,
    Netns,
    Tuntap,
    Token,
    MacSec,
    Netconf,
    L2tp,
    Mroute,
    Mptcp,
    Vrfs,
    Help,
    /// Any object this list does not name.
    Custom(String),
}

impl IpCommand {
    /// The word `ip` takes for this object.
    pub fn as_str(&self) -> &str {
        match self {
            IpCommand::Addr => "addr",
            IpCommand::Link => "link",
            IpCommand::Route => "route",
            IpCommand::Neigh => "neigh",
            IpCommand::Rule => "rule",
            IpCommand::TcpMetrics => "tcp_metrics",
            IpCommand::Maddr => "maddr",
            IpCommand::Monitor => "monitor",
            IpCommand::Xfrm => "xfrm",
            IpCommand::Netns => "netns",
            IpCommand::Tuntap => "tuntap",
            IpCommand::Token => "token",
            IpCommand::MacSec => "macsec",
            IpCommand::Netconf => "netconf",
            IpCommand::L2tp => "l2tp",
            IpCommand::Mroute => "mroute",
            IpCommand::Mptcp => "mptcp",
            IpCommand::Vrfs => "vrf",
            IpCommand::Help => "help",
            IpCommand::Custom(s) => s,
        }
    }
}

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "ip", trait_path = "::xccute_runtime::ShellCommand")]
pub struct IpBuilder {
    #[builder(opt)]
    #[shell(opt_expr = "self.command.as_ref().map(IpCommand::as_str)")]
    pub command: Option<IpCommand>,
    #[builder(push, method = "arg")]
    #[shell(positional)]
    pub args: Vec<String>,
}

impl IpBuilder {
    /// Add several arguments at once.
    pub fn args(mut self, args: &[&str]) -> Self {
        self.args.extend(args.iter().map(|s| s.to_string()));
        self
    }
}
