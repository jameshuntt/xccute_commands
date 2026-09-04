//! `echo`: print a line, optionally quoted and redirected to a file.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "echo", trait_path = "::xccute_runtime::ShellCommand")]
pub struct EchoBuilder {
    #[builder(required, into)]
    #[shell(arg_expr = "if self.quoted { format!(\"\\\"{}\\\"\", self.message) } else { self.message.clone() }")]
    pub message: String,
    #[builder(flag)]
    #[shell(flag = "-n")]
    pub no_newline: bool,
    #[builder(flag)]
    pub quoted: bool,
    #[builder(opt_into)]
    #[shell(opt_expr = "self.to_file.as_ref().map(|f| format!(\"{} {}\", if self.append { \">>\" } else { \">\" }, f))")]
    pub to_file: Option<String>,
    #[builder(flag)]
    pub append: bool,
}
