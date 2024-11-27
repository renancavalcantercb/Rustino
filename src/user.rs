pub struct User {
    pub name: String,
    pub email: String,
    pub balance: u32,
}

impl User {
    pub fn new(name: &str, email: &str) -> Self {
        Self {
            name: name.to_string(),
            email: email.to_string(),
            balance: 1000,
        }
    }

    pub fn update_balance(&mut self, amount: i32) {
        self.balance = (self.balance as i32 + amount).max(0) as u32;
    }
}

