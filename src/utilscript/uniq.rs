//! `uniq`: report or omit repeated lines.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "uniq", trait_path = "::xccute_runtime::ShellCommand")]
pub struct UniqBuilder {
    #[builder(flag)]
    #[shell(flag = "-c")]
    pub count: bool,
    #[builder(flag)]
    #[shell(flag = "-d")]
    pub repeated: bool,
    #[builder(flag)]
    #[shell(flag = "-u")]
    pub unique_only: bool,
    #[builder(flag)]
    #[shell(flag = "-i")]
    pub ignore_case: bool,
    #[builder(opt)]
    #[shell(opt_kv = "-f")]
    pub skip_fields: Option<usize>,
    #[builder(opt)]
    #[shell(opt_kv = "-s")]
    pub skip_chars: Option<usize>,
    #[builder(opt_into)]
    #[shell(positional)]
    pub input: Option<String>,
}
