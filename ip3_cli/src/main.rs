use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}
#[derive(Subcommand)]
enum Commands {
    /// Display current ip3
    Me {
        #[arg(short, long)]
        json: bool,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Me {
            json
        }) => {
            commands::me::display_me(*json).await;
        }
        None => {
            println!("No command! Exiting...");
        }
    }
}
