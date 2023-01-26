pub mod blackjack;

pub trait PayOut {
    fn pay_out(&self, player_name: &str) -> f64;
}
