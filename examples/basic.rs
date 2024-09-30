use yomi::prelude::{Parser, Subcommand};
use yomi::{Cli, CommandExecutor};

#[derive(Subcommand)]
enum InputCommand {
    /// cargo {crate_name} mk.
    #[command(name = "mk")]
    MakeDirectory,

    /// cargo {crate_name} rm.
    #[command(name = "rm")]
    RemoveDirectory,
}

#[tokio::main]
async fn main() {
    let cli = Cli::<InputCommand>::parse();

    let r = match cli.get() {
        InputCommand::MakeDirectory => CommandExecutor::execute("mkdir random_directory"),
        InputCommand::RemoveDirectory => CommandExecutor::execute("rm -rf random_directory"),
    };

    r.await.unwrap();
}
