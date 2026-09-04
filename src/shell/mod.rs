//! Run a command line through a shell.

pub mod bash;
pub use bash::*;

pub mod sh;
pub use sh::*;

pub mod zsh;
pub use zsh::*;
