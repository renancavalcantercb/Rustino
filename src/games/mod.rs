pub mod coin_flip;

use crate::{history::GameHistory, user::User};

pub fn play_game(game: &str, bet: u32, user: &mut User, history: &mut GameHistory) {
    match game {
        "coin_flip" => coin_flip::play(bet, user, history),
        _ => println!("Invalid game choice."),
    }
}

