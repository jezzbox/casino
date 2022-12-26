use rand::{seq::SliceRandom, thread_rng};
use std::fmt;

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
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
        let ranks = [
            Rank::Ace, Rank::Two, Rank::Three,
            Rank::Four, Rank::Five, Rank::Six,
            Rank::Seven, Rank::Eight, Rank::Nine,
            Rank::Ten, Rank::Jack, Rank::Queen,
            Rank::King];

        let suits = [Suit::Diamonds, Suit::Clubs, Suit::Hearts, Suit::Spades];

        for rank in ranks.iter() {
            for suit in suits.iter() {
                cards.push(Card::new(rank.clone(), suit.clone()));
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
            };
        }
    }
}

pub struct Player {
    hand: Vec<Card>,
}

impl Player {
    pub fn new() -> Player {
        Player { hand: Vec::new() }
    }
    
    pub fn add(&mut self, card: Card) {
        self.hand.push(card);
    }

    pub fn calculate_hand(&self) -> u8 {
        let values: Vec<u8> = self.hand.iter().map(|card| match card.rank {
              Rank::Two => 2,
              Rank::Three => 3,
              Rank::Four => 4,
              Rank::Five => 5,
              Rank::Six => 6,
              Rank::Seven => 7,
              Rank::Eight => 9,
              Rank::Ten => 10,
              Rank::Jack => 10,
              Rank::Queen => 10,
              Rank::King => 10,
              Rank::Ace => 11,
              _ => 11,
        }).collect();
        
        values.iter().sum::<u8>()
    }
}