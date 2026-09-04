//! `uptime`: how long the system has been running.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "uptime", trait_path = "::xccute_runtime::ShellCommand")]
pub struct UptimeBuilder {}
