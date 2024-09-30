mod core;
#[cfg(test)]
mod tests;

pub use core::cli::Cli;
pub use core::command::Command as CommandExecutor;

pub mod anim {
    pub use super::core::animation::{Animation, Process as AnimatedProcess};
}

pub mod prelude {
    pub use clap::{Parser, Subcommand};
}
