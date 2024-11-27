mod cli;
mod games;
mod history;
mod user;

use clap::Parser;
use cli::Commands;
use games::play_game;
use history::GameHistory;
use user::User;

fn main() {
    let cli = cli::RustinoCli::parse();

    let mut user = User::new("Player", "player@example.com");
    let mut history = GameHistory::new();

    match cli.command {
        Commands::Start => {
            println!("Welcome to Rustino, {}!", user.name);
        }
        Commands::Game { game, bet } => {
            play_game(&game, bet, &mut user, &mut history);
        }
        Commands::History => {
            history.display();
        }
    }
}

