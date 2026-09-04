//! `awk`: pattern scanning and processing.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "awk", trait_path = "::xccute_runtime::ShellCommand")]
pub struct AwkBuilder {
    #[builder(opt_into)]
    #[shell(opt_kv = "-F", fmt = "'{}'")]
    pub field_separator: Option<String>,
    /// `name=value` assignments, set by [`AwkBuilder::with_var`].
    #[builder(skip)]
    #[shell(positional)]
    pub var_assignments: Vec<String>,
    #[builder(required, into)]
    #[shell(positional, fmt = "'{}'")]
    pub program: String,
    #[builder(opt_into)]
    #[shell(positional)]
    pub file: Option<String>,
}

impl AwkBuilder {
    /// Assign an awk variable before the program runs.
    pub fn with_var(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.var_assignments.push(format!("{}={}", name.into(), value.into()));
        self
    }
}
