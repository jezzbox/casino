use rand::{seq::SliceRandom, thread_rng};
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub trait Value {
    fn value(&self) -> u8;
}
#[derive(Debug, Clone, Copy, EnumIter, PartialEq)]
enum Suit {
    Diamonds,
    Clubs,
    Hearts,
    Spades,
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone, Copy, EnumIter, PartialEq)]
enum Rank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Joker,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }
}

impl Value for Card {
    fn value(&self) -> u8 {
        match self.rank {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 10,
            Rank::Queen => 10,
            Rank::King => 10,
            Rank::Ace => 11,
            _ => 11,
        }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::with_capacity(52);

        for rank in Rank::iter().filter(|&x| x != Rank::Joker) {
            for suit in Suit::iter() {
                cards.push(Card::new(rank, suit));
            }
        }

        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng())
    }

    pub fn draw(&mut self) -> Card {
        if let Some(card) = self.cards.pop() {
            println!("drawn card is: {}", card);
            card
        } else {
            Card {
                rank: Rank::Ace,
                suit: Suit::Spades,
            }
        }
    }
}

pub struct BlackJack {
    players: Vec<Player>,
    dealer: Player,
    deck: Deck,
}

impl BlackJack {
    fn new(players: Vec<Player>, dealer: Player, deck: Deck) -> BlackJack {
        BlackJack {
            players,
            dealer,
            deck,
        }
    }
}

pub struct Player {
    name: String,
    hand: Vec<Card>,
}

impl Player {
    pub fn new(name: &str, hand: Vec<Card>) -> Player {
        Player { name, hand }
    }

    pub fn add(&mut self, card: Card) {
        self.hand.push(card);
    }

    pub fn calculate_hand(&self) -> u8 {
        let values: Vec<u8> = self.hand.iter().map(|card| card.value()).collect();

        values.iter().sum::<u8>()
    }
}
