use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, propagate_version = true)]
pub struct Cli<C: Subcommand> {
    #[command(subcommand)]
    command: C,
}

impl<C> Cli<C>
where
    C: Subcommand,
{
    pub fn get(self) -> C {
        self.command
    }
}
