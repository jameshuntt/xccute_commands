//! `nano`: open a file in the editor, optionally through `sudo`.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd_expr = "if self.sudo { \"sudo nano\" } else { \"nano\" }", trait_path = "::xccute_runtime::ShellCommand")]
pub struct NanoBuilder {
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub file: String,
    #[builder(flag)]
    pub sudo: bool,
}
