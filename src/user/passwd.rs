//! `passwd`: change a password, optionally fed from stdin.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd_expr = "if self.stdin { \"echo \\\"<password>\\\" | passwd\" } else { \"passwd\" }", trait_path = "::xccute_runtime::ShellCommand")]
pub struct PasswdBuilder {
    #[builder(opt_into, method = "user")]
    #[shell(positional)]
    pub username: Option<String>,
    #[builder(flag)]
    pub stdin: bool,
}
