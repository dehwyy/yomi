use yomi::anim::{AnimatedProcess, Animation};
use yomi::prelude::{Parser, Subcommand};
use yomi::{Cli, CommandExecutor};

#[derive(Subcommand)]
enum InputCommand {
    /// cargo {crate_name} gen.
    #[command(name = "gen")]
    Generate,
}

async fn generate_files() {
    Animation::builder()
        .add(AnimatedProcess::new(async {
            CommandExecutor::execute("mkdir random_directory")
                .await
                .unwrap();
        }))
        .add(AnimatedProcess::new(async {
            CommandExecutor::execute("mdkir hello!").await.unwrap();
        }))
        .invoke_parallel()
        .await;
}

#[tokio::main]
async fn main() {
    let cli = Cli::<InputCommand>::parse();

    match cli.get() {
        InputCommand::Generate => generate_files().await,
    };
}
