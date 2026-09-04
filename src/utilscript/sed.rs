//! `sed`: stream editor.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "sed", require_order, trait_path = "::xccute_runtime::ShellCommand")]
pub struct SedBuilder {
    #[builder(flag)]
    #[shell(flag = "-n", order = 1)]
    pub suppress_output: bool,
    #[builder(flag)]
    #[shell(flag = "-i", order = 2)]
    pub in_place: bool,
    /// Only emitted with `in_place`.
    #[builder(opt_into)]
    #[shell(opt_expr = "if self.in_place { self.backup_extension.as_ref() } else { None }", order = 3)]
    pub backup_extension: Option<String>,
    #[builder(opt_into)]
    #[shell(opt_kv = "-e", order = 4)]
    pub expression: Option<String>,
    #[builder(opt_into)]
    #[shell(positional, order = 5)]
    pub file: Option<String>,
}
