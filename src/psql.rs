//! `psql`: run SQL through the PostgreSQL client.
use simple_impl_derive::SimpleImpl;

#[derive(Debug, Clone, Default, SimpleImpl)]
#[shell(cmd = "psql", require_order, trait_path = "::xccute_runtime::ShellCommand")]
pub struct PsqlCommandBuilder {
    #[builder(required, into)]
    #[shell(kv = "-c", fmt = "\"{}\"", order = 4)]
    pub sql: String,
    #[builder(opt_into)]
    #[shell(opt_kv = "-d", order = 1)]
    pub database: Option<String>,
    #[builder(opt_into, method = "user")]
    #[shell(opt_kv = "-U", order = 2)]
    pub username: Option<String>,
    /// Recorded for callers; `psql` itself takes no flag for it.
    #[builder(flag)]
    pub output: bool,
    #[builder(flag)]
    #[shell(flag = "-q", order = 3)]
    pub quiet: bool,
}
