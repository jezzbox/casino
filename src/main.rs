use casino::{Deck, Player };

fn main() {
    let mut player = Player::new();
    let mut deck = Deck::new();
    deck.shuffle();

    let card = deck.draw();
    player.add(card);
    let card2 = deck.draw();
    player.add(card2);

    println!("Value of hand is: {}", player.calculate_hand());
}
