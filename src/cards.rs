use rand::{seq::SliceRandom, thread_rng};
use std::fmt;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Debug, Clone, Copy, EnumIter, PartialEq)]
pub enum Suit {
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
pub enum Rank {
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
    // Joker,
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub fn rank(&self) -> Rank {
        self.rank
    }

    fn new(rank: Rank, suit: Suit) -> Card {
        Card{ rank, suit }
    }

    pub fn is_rank(&self, rank: Rank) -> bool {
        self.rank == rank
    }
    pub fn is_suit(&self, suit: Suit) -> bool {
        self.suit == suit
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
    pub fn standard() -> Deck { 
        let mut cards = Vec::with_capacity(52);

        for rank in Rank::iter() {
            for suit in Suit::iter() {
                cards.push(Card::new(rank, suit));
            }
        }

        let mut deck = Deck{cards};
        deck.shuffle();
        deck
    }

    fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng())
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
        }
}
