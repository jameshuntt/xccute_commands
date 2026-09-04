//! `file`: determine file type.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "file", trait_path = "::xccute_runtime::ShellCommand")]
pub struct FileBuilder {
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub filename: String,
    #[builder(flag)]
    #[shell(flag = "--mime-type")]
    pub mime: bool,
    #[builder(flag)]
    #[shell(flag = "--brief")]
    pub brief: bool,
    #[builder(flag)]
    #[shell(flag = "--dereference")]
    pub dereference: bool,
    #[builder(flag)]
    #[shell(flag = "--special-files")]
    pub special_files: bool,
}
