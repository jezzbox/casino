use casino::{games::blackjack, players};


fn main() {
    println!("Welcome to blackjack");

    // let mut players = blackjack::setup_players();
    let mut player = players::Player::new("jez", 500f64);
    let mut round = blackjack::Round::new();
    round.take_bet(&mut player, 50f64);
    round.play();
    player.print_money();
//    println!("{}", player.money);

    // for player in players.iter_mut() {
    //     dealer.add_player(player);
    // }

    // dealer.deal();

    // for player in dealer.players {
    //     println!("Welcome {}", player.name);
    //     player.play_turn(&mut dealer);
    // }

    // println!("Dealer's hand: ");
    // dealer.hand.print_hand();
    // dealer.play_turn();

    // for player in players.iter() {
    //     println!("{} results: ", player.name);
    //     for hand in player.hand.iter() {
    //         hand.reveal_cards();
    //         hand.print_result(&dealer)

    //     }
    // }
    
}
    
