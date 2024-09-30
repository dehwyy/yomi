use std::fmt::Display;
use tokio::process::Command as TokioCommand;

type ProcessOutputResult = Result<std::process::Output, tokio::io::Error>;

async fn execute(command: impl Display, block_executable_stdout: bool) -> ProcessOutputResult {
    let command = command.to_string();
    let mut command_iter = command.split(" ");
    let command = command_iter.next().expect("Empty command");
    let args = command_iter.collect::<Vec<_>>();

    let mut c = TokioCommand::new(command);
    c.args(args);

    if block_executable_stdout {
        c.output().await
    } else {
        c.spawn()?.wait_with_output().await
    }
}
pub struct Command;

impl Command {
    /// Executes a command.
    ///
    /// Blocks `command's` stdout.
    pub async fn execute(command: impl Display) -> ProcessOutputResult {
        execute(command, true).await
    }

    /// Executes a command.
    ///
    /// Does not block `command's` stdout.
    pub async fn execute_non_blocking(command: impl Display) -> ProcessOutputResult {
        execute(command, false).await
    }
}
