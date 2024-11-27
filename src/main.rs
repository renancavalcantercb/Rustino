use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct RustinoCli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start {
        #[arg(short, long, default_value_t = 1000)]
        balance: u32,
    },
    History,
}

fn main() {
    let cli = RustinoCli::parse();

    match cli.command {
        Commands::Start { balance } => {
            println!("Welcome to Rustino! Your starting balance is ${}.", balance);
        }
        Commands::History => {
            println!("Game history will be displayed here.");
        }
    }
}

