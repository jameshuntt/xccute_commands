//! `ln`: make links between files.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "ln", trait_path = "::xccute_runtime::ShellCommand")]
pub struct LnBuilder {
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub source: String,
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub target: String,
    #[builder(flag)]
    #[shell(flag = "-s")]
    pub symbolic: bool,
    #[builder(flag)]
    #[shell(flag = "-f")]
    pub force: bool,
    #[builder(flag)]
    #[shell(flag = "-n")]
    pub no_dereference: bool,
    #[builder(flag)]
    #[shell(flag = "-v")]
    pub verbose: bool,
}

/// The older name of [`LnBuilder`].
pub type LinkBuilder = LnBuilder;
