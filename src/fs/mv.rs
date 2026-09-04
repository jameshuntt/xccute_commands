//! `mv`: move or rename files.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "mv", trait_path = "::xccute_runtime::ShellCommand")]
pub struct MvBuilder {
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub source: String,
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub destination: String,
    #[builder(flag)]
    #[shell(flag = "-f")]
    pub force: bool,
    #[builder(flag)]
    #[shell(flag = "-i")]
    pub interactive: bool,
    #[builder(flag)]
    #[shell(flag = "-v")]
    pub verbose: bool,
    #[builder(flag)]
    #[shell(flag = "-n")]
    pub no_clobber: bool,
    #[builder(flag)]
    #[shell(flag = "-b")]
    pub backup: bool,
}
