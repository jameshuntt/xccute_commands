//! `chown`: change file owner and group.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "chown", trait_path = "::xccute_runtime::ShellCommand")]
pub struct ChownBuilder {
    #[builder(required, into)]
    #[shell(arg_join_opt = "group", sep = ":")]
    pub owner: String,
    #[builder(opt_into)]
    pub group: Option<String>,
    #[builder(required, into)]
    #[shell(arg_clone)]
    pub target: String,
    #[builder(flag)]
    #[shell(flag = "-R")]
    pub recursive: bool,
    #[builder(flag)]
    #[shell(flag = "-h")]
    pub no_dereference: bool,
    /// `owner` or `owner:group`, set by [`ChownBuilder::from`].
    #[builder(skip)]
    #[shell(opt_kv = "--from")]
    pub from: Option<String>,
}

impl ChownBuilder {
    /// Only change files currently owned by `owner` (and `group`, when given).
    pub fn from(mut self, owner: impl Into<String>, group: Option<String>) -> Self {
        let mut value = owner.into();
        if let Some(group) = group {
            value.push(':');
            value.push_str(&group);
        }
        self.from = Some(value);
        self
    }
}
