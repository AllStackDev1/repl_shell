mod command;
mod errors;
mod helpers;

use errors::CrateResult;
use helpers::pwd;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt},
    task::JoinHandle,
};

use command::Command;

/// This function spawns a new task that listens for user input and handles it.
/// It returns a JoinHandle that can be awaited to wait for the task to finish.
fn spawn_user_input_handler() -> JoinHandle<CrateResult<()>> {
    tokio::spawn(async {
        // Initialize stdin and stdout
        let stdin = tokio::io::stdin();
        let stdout = tokio::io::stdout();

        let mut reader = tokio::io::BufReader::new(stdin).lines();
        let mut stdout = tokio::io::BufWriter::new(stdout);

        stdout.write_all(b"Welcome to the shell!\n").await?;

        stdout.write_all(pwd()?.as_bytes()).await?;
        stdout.write_all(b"\n>").await?;
        stdout.flush().await?;

        while let Ok(Some(line)) = reader.next_line().await {
            let command = handle_new_line(&line).await;

            if let Ok(command) = &command {
                if let Command::Exit = command {
                    println!("Exiting shell...");
                    break;
                }
            } else {
                eprintln!("Error parsing command: {}", command.err().unwrap());
            }

            stdout.write_all(pwd()?.as_bytes()).await?;
            stdout.write_all(b"\n>").await?;
            stdout.flush().await?;
        }

        Ok(())
    })
}

#[tokio::main]
async fn main() {
    let user_input_handler = spawn_user_input_handler();

    if let Err(e) = user_input_handler.await {
        eprintln!("Error: {}", e);
    }
}

/// This function takes a line of user input and handles it.
/// It returns a Command enum that represents the command that was parsed.
async fn handle_new_line(line: &str) -> CrateResult<Command> {
    // Leverages the TryFrom trait implemented
    let command: Command = line.try_into()?;

    match command.clone() {
        Command::Echo(s) => {
            println!("{}", s);
        }
        Command::Ls => {
            helpers::ls()?;
        }
        Command::Pwd => {
            println!("{}", helpers::pwd()?);
        }
        Command::Cd(s) => {
            helpers::cd(&s)?;
        }
        Command::Touch(s) => {
            helpers::touch(&s)?;
        }
        Command::Rm(s) => {
            helpers::rm(&s)?;
        }
        Command::Cat(s) => {
            println!("{}", helpers::cat(&s)?);
        }
        _ => {}
    }

    Ok(command)
}
