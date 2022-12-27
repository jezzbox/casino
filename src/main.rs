use casino::{Deck, Player, Value};

fn main() {
    let mut player = Player::new();
    let mut dealer = Player::new();

    let mut deck = Deck::new();
    deck.shuffle();

    let card = deck.draw();
    player.add(card);
    let card2 = deck.draw();
    player.add(card2);

    dealer.add(deck.draw());
    let card4 = deck.draw();
    println!("the dealers faceup card is: {}", card4.value());
    dealer.add(card4);

    println!("Value of hand is: {}", player.calculate_hand());
}
