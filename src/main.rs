use casino::Deck;

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();

    let card = deck.draw();
    println!("card is {}", card);
    let card2 = deck.draw();
    println!("card is {}", card2);
}
