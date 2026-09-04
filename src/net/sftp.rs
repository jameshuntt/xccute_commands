//! `sftp`: secure file transfer.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "sftp", require_order, trait_path = "::xccute_runtime::ShellCommand")]
pub struct SftpBuilder {
    #[builder(opt_into)]
    pub user: Option<String>,
    #[builder(required, into)]
    #[shell(arg_expr = "match &self.user { Some(user) => format!(\"{}@{}\", user, self.host), None => self.host.clone() }", order = "last")]
    pub host: String,
    #[builder(opt_into)]
    #[shell(opt_kv = "-i", order = 1)]
    pub identity_file: Option<String>,
    #[builder(opt)]
    #[shell(opt_kv = "-P", order = 2)]
    pub port: Option<u16>,
    #[builder(opt_into)]
    #[shell(opt_kv = "-b", order = 3)]
    pub batch_file: Option<String>,
    #[builder(flag)]
    #[shell(flag = "-C", order = 4)]
    pub interactive: bool,
}
