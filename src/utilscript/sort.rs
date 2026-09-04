//! `sort`: sort lines of text files.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "sort", trait_path = "::xccute_runtime::ShellCommand")]
pub struct SortBuilder {
    #[builder(flag)]
    #[shell(flag = "-n")]
    pub numeric: bool,
    #[builder(flag)]
    #[shell(flag = "-r")]
    pub reverse: bool,
    #[builder(flag)]
    #[shell(flag = "-h")]
    pub human_numeric: bool,
    #[builder(flag)]
    #[shell(flag = "-f")]
    pub ignore_case: bool,
    #[builder(flag)]
    #[shell(flag = "-u")]
    pub unique: bool,
    #[builder(opt_into)]
    #[shell(opt_kv = "-k")]
    pub key: Option<String>,
    #[builder(opt_into)]
    #[shell(positional)]
    pub file: Option<String>,
}
