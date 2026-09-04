//! `grep`: print lines matching a pattern.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "grep", trait_path = "::xccute_runtime::ShellCommand")]
pub struct GrepBuilder {
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub pattern: String,
    #[builder(flag)]
    #[shell(flag = "-i")]
    pub ignore_case: bool,
    #[builder(flag)]
    #[shell(flag = "-r")]
    pub recursive: bool,
    #[builder(flag)]
    #[shell(flag = "-n")]
    pub line_number: bool,
    #[builder(flag)]
    #[shell(flag = "-v")]
    pub invert_match: bool,
    #[builder(flag)]
    #[shell(flag = "-w")]
    pub word_regexp: bool,
    #[builder(flag)]
    #[shell(flag = "-E")]
    pub extended_regex: bool,
    #[builder(push, method = "file")]
    #[shell(positional)]
    pub files: Vec<String>,
}

impl GrepBuilder {
    /// Add several files at once.
    pub fn files(mut self, files: Vec<impl Into<String>>) -> Self {
        self.files.extend(files.into_iter().map(Into::into));
        self
    }
}
