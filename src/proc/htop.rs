//! `htop`: interactive process viewer.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "htop", require_order, trait_path = "::xccute_runtime::ShellCommand")]
pub struct HtopBuilder {
    #[builder(opt_into)]
    #[shell(opt_kv = "--user", order = 1)]
    pub user: Option<String>,
    #[builder(opt_into)]
    #[shell(opt_kv = "--sort-key", order = 2)]
    pub sort_key: Option<String>,
    #[builder(flag)]
    #[shell(flag = "--tree", order = 3)]
    pub tree: bool,
    #[builder(opt)]
    #[shell(opt_kv = "--delay", order = 4)]
    pub delay: Option<u64>,
    #[builder(flag)]
    #[shell(flag = "--no-color", order = 5)]
    pub no_colors: bool,
    #[builder(flag)]
    #[shell(flag = "--no-mouse", order = 6)]
    pub no_mouse: bool,
}
