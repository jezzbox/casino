pub struct Player {
    pub name: String,
    money: f64,
}

impl Player {
    pub fn new(name: &str, money: f64) -> Player {
        Player { name: name.to_string(), money }      
    }

    pub fn bet(&mut self, amount: f64) -> Option<f64> {
        match self.money - amount {
            x if x >= 0f64 => {
                self.money -= amount;
                Some(amount)
                }
            _ => None
        }
    }

    pub fn collect_winnings(&mut self, winnings: f64) {
        self.money += winnings;
    }

    pub fn print_money(&self) {
        println!("{}", self.money);
    }
}
