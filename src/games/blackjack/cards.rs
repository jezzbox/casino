use crate::cards;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Card(cards::Card);

impl cards::Standard for Card {
    fn new(rank: cards::Rank, suit: cards::Suit) -> Card {
        let card = cards::Card::new(rank, suit);
        Card(card)
    }

    fn rank(&self) -> cards::Rank {
        self.0.rank()
    }

    fn is_rank(&self, rank: cards::Rank) -> bool {
        self.0.is_rank(rank)
    }

    fn is_suit(&self, suit: cards::Suit) -> bool {
        self.0.is_suit(suit)
    }
}

impl BlackjackCard {
    fn value(&self) -> u32 {
        match self.rank() {
            cards::Rank::Ace => 1,
            cards::Rank::Two => 2,
            cards::Rank::Three => 3,
            cards::Rank::Four => 4,
            cards::Rank::Five => 5,
            cards::Rank::Six => 6,
            cards::Rank::Seven => 7,
            cards::Rank::Eight => 8,
            cards::Rank::Nine => 9,
            // Card{ rank: Rank::Joker, ..} => None,
            _ => 10,
        }
    }

    fn is_ace(&self) -> bool {
        self.is_rank(cards::Rank::Ace)
    }

    fn eq_rank(&self, other: &Self) -> bool {
        self.rank() == other.rank()
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub use crate::cards::Deck;
