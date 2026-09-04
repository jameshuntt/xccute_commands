//! `cut`: remove sections from each line.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "cut", trait_path = "::xccute_runtime::ShellCommand")]
pub struct CutBuilder {
    #[builder(opt_into)]
    #[shell(opt_kv = "-d")]
    pub delimiter: Option<String>,
    #[builder(opt_into)]
    #[shell(opt_kv = "-f")]
    pub fields: Option<String>,
    #[builder(opt_into)]
    #[shell(opt_kv = "-b")]
    pub bytes: Option<String>,
    #[builder(opt_into)]
    #[shell(opt_kv = "-c")]
    pub characters: Option<String>,
    #[builder(flag)]
    #[shell(flag = "--complement")]
    pub complement: bool,
    #[builder(flag)]
    #[shell(flag = "--only-delimited")]
    pub only_delimited: bool,
    #[builder(opt_into)]
    #[shell(positional)]
    pub file: Option<String>,
}
