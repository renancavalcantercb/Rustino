use chrono::{DateTime, Utc};

pub struct GameHistoryEntry {
    pub timestamp: DateTime<Utc>,
    pub game_type: String,
    pub bet_amount: u32,
    pub result: String,
    pub final_balance: u32,
}

pub struct GameHistory {
    pub entries: Vec<GameHistoryEntry>,
}

impl GameHistory {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn add_entry(&mut self, entry: GameHistoryEntry) {
        self.entries.push(entry);
    }

    pub fn display(&self) {
        println!("Game History:");
        for entry in &self.entries {
            println!(
                "{} | {} | Bet: ${} | Result: {} | Balance: ${}",
                entry.timestamp,
                entry.game_type,
                entry.bet_amount,
                entry.result,
                entry.final_balance
            );
        }
    }
}

