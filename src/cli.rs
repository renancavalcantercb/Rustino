use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
pub struct RustinoCli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Start ,
    Game {
        #[arg()]
        game: String,

        #[arg(short, long)]
        bet: u32,
    },
    History,
}

