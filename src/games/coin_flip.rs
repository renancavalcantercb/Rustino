use crate::{history::GameHistory, user::User};

pub fn play(bet: u32, user: &mut User, history: &mut GameHistory) {
    if bet > user.balance {
        println!("Insufficient balance!");
        return;
    }

    let result = if rand::random() { "Heads" } else { "Tails" };
    let outcome = if result == "Heads" {
        user.update_balance(bet as i32);
        "Victory"
    } else {
        user.update_balance(-(bet as i32));
        "Defeat"
    };

    println!("The coin landed on: {}. You {}!", result, outcome);

    history.add_entry(crate::history::GameHistoryEntry {
        timestamp: chrono::Utc::now(),
        game_type: "Coin Flip".to_string(),
        bet_amount: bet,
        result: outcome.to_string(),
        final_balance: user.balance,
    });
}

