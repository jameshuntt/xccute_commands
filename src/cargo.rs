//! `cargo`: the Rust package manager.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "cargo", require_order, trait_path = "::xccute_runtime::ShellCommand")]
pub struct CargoBuilder {
    #[builder(required, into)]
    #[shell(subcommand, order = 1)]
    pub subcommand: String,
    #[builder(opt_into)]
    #[shell(positional, order = 2)]
    pub name: Option<String>,
    #[builder(flag)]
    #[shell(flag = "--bin", order = 3)]
    pub bin: bool,
    #[builder(flag)]
    #[shell(flag = "--lib", order = 4)]
    pub lib: bool,
    #[builder(flag)]
    #[shell(flag = "--release", order = 5)]
    pub release: bool,
    #[builder(opt_into)]
    #[shell(opt_kv = "--package", order = 6)]
    pub package: Option<String>,
    #[builder(opt_into)]
    #[shell(opt_kv = "--manifest-path", order = 7)]
    pub manifest_path: Option<String>,
    #[builder(push, method = "feature")]
    #[shell(multi_arg_flag = "--features", join = ",", order = 8)]
    pub features: Vec<String>,
    #[builder(push, method = "arg")]
    #[shell(positional, order = 9)]
    pub extra_args: Vec<String>,
}

impl CargoBuilder {
    /// Enable several features at once.
    pub fn features(mut self, features: &[&str]) -> Self {
        self.features.extend(features.iter().map(|s| s.to_string()));
        self
    }

    /// Add several trailing arguments at once.
    pub fn args(mut self, args: &[&str]) -> Self {
        self.extra_args.extend(args.iter().map(|s| s.to_string()));
        self
    }

    /// `--target <triple>`.
    pub fn target(mut self, triple: impl Into<String>) -> Self {
        self.extra_args.push("--target".to_string());
        self.extra_args.push(triple.into());
        self
    }
}
