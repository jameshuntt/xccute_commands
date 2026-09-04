//! `wc`: count lines, words and bytes.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "wc", trait_path = "::xccute_runtime::ShellCommand")]
pub struct WcBuilder {
    #[builder(flag)]
    #[shell(flag = "-c")]
    pub bytes: bool,
    #[builder(flag)]
    #[shell(flag = "-m")]
    pub chars: bool,
    #[builder(flag)]
    #[shell(flag = "-l")]
    pub lines: bool,
    #[builder(flag)]
    #[shell(flag = "-w")]
    pub words: bool,
    #[builder(flag)]
    #[shell(flag = "-L")]
    pub max_line_length: bool,
    #[builder(opt_into)]
    #[shell(positional)]
    pub file: Option<String>,
}
