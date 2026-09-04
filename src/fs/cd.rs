//! `cd`: change directory.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "cd", trait_path = "::xccute_runtime::ShellCommand")]
pub struct CdBuilder {
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub path: String,
}

impl CdBuilder {
    /// Append a path segment to the directory.
    pub fn join(mut self, subpath: impl Into<String>) -> Self {
        let mut p = std::path::PathBuf::from(&self.path);
        p.push(subpath.into());
        self.path = p.to_string_lossy().into_owned();
        self
    }
}
