//! `git`: a subcommand and its arguments.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "git", trait_path = "::xccute_runtime::ShellCommand")]
pub struct GitBuilder {
    #[builder(required, into)]
    #[shell(subcommand)]
    pub subcommand: String,
    #[builder(push, method = "arg")]
    #[shell(positional)]
    pub args: Vec<String>,
}

impl GitBuilder {
    /// Add several arguments at once.
    pub fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.args.extend(args.into_iter().map(Into::into));
        self
    }

    /// `git clone --mirror <url> <path>`.
    pub fn clone_mirror(url: &str, path: &str) -> Self {
        GitBuilder::new("clone").args(["--mirror", url, path])
    }

    /// `git clone <mirror_dir> <working_copy>`.
    pub fn clone_working(mirror_dir: &str, working_copy: &str) -> Self {
        GitBuilder::new("clone").args([mirror_dir, working_copy])
    }

    /// `git worktree add --detach <dest> <tag>`.
    pub fn worktree_add(tag: &str, dest: &str) -> Self {
        GitBuilder::new("worktree").args(["add", "--detach", dest, tag])
    }
}
