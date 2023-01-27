use casino::{games::blackjack, players};


fn main() {
    println!("Welcome to blackjack");

    let mut player = players::Player::new("jez", 500f64);
    let mut round = blackjack::Round::new();
    round.take_bet(&mut player, 50f64);
    round.play();
    player.print_money();
}
    
