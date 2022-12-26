use rand::{seq::SliceRandom, thread_rng};
use std::fmt;

#[derive(Debug)]
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

#[derive(Debug)]
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
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct Card {
    rank: Rank,
    suit: Suit,
    value: u8,
}

impl Card {
    fn new(rank: u8, suit: &str) -> Card {
        let value = rank;

        let rank = match rank {
            1 => Rank::Ace,
            2 => Rank::Two,
            3 => Rank::Three,
            4 => Rank::Four,
            5 => Rank::Five,
            6 => Rank::Six,
            7 => Rank::Seven,
            8 => Rank::Eight,
            9 => Rank::Nine,
            10 => Rank::Ten,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => Rank::Ace,
        };

        let suit = suit.to_lowercase();

        let suit = match &suit[..] {
            "diamonds" => Suit::Diamonds,
            "clubs" => Suit::Clubs,
            "hearts" => Suit::Hearts,
            "spades" => Suit::Spades,
            _ => Suit::Hearts,
        };

        Card { rank, suit, value }
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
        let suits = ["Diamonds", "Clubs", "Hearts", "Spades"];
        for i in 1..14 {
            for suit in suits.iter() {
                cards.push(Card::new(i, suit));
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
            return card;
        } else {
            return Card {
                rank: Rank::Ace,
                suit: Suit::Spades,
                value: 1,
            };
        }
    }
}
