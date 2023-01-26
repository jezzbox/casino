use crate::{cards, players};

fn card_value(card: &cards::Card) -> u32 {
    match card.rank() {
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

#[derive(PartialEq, Clone, Copy)]
pub enum HandState {
    Resolved(u32),
    Unresolved,
    Bust,
    Surrendered,
}

pub struct SubHand {
    cards: Vec<cards::Card>,
    state: HandState,
}

impl SubHand {
    pub fn new() -> SubHand {
        SubHand {cards: Vec::new(), state: HandState::Unresolved }
    }


    pub fn add_card(&mut self, card: cards::Card) {
        if !self.is_resolved() {
            self.cards.push(card);

            match self.value() {
                22.. => { self.state = HandState::Bust },
                21 => { self.state = HandState::Resolved(21) },
                _ => (),
            }
        } else { println!("can't add card to resolved hand");}
    }

    fn stand(&mut self) {
        if !self.is_resolved() {
            self.state = HandState::Resolved(self.value());
        } else {println!("can't stand a resolved hand");}
    }
    
    pub fn state(&self) -> &HandState {
        &self.state
    }
    fn is_resolved(&self) -> bool {
        !(self.state == HandState::Unresolved)
    }

    pub fn value(&self) -> u32 {
        let hand_value = self.cards.iter().map(card_value).sum();
        if self.is_soft() && (hand_value + 10) <= 21 {
            hand_value + 10
        } else {
            hand_value
        }
    }

    fn result(&self) -> HandState {
        self.state
    }

    fn is_soft(&self) -> bool {
        self.cards.iter().any(|c| c.is_rank(cards::Rank::Ace))
    }

    pub fn print_hand(&self) {
        for card in self.cards.iter() {
            println!("{}", card)
        }
    }

    pub fn is_splittable(&self) -> bool {
        let hand_len = self.cards.len();
        let first_card = self.cards.first();
        let last_card = self.cards.last();

        if let (2, Some(first_card), Some(last_card)) = (hand_len, first_card, last_card) {
            first_card.is_rank(last_card.rank())
        } else {false }
    }

}

pub struct PlayerHand<'a> {
    player: &'a mut players::Player,
    sub_hands: Vec<SubHand>,
    bet: f64,
}

impl<'a> PlayerHand<'a> {
    pub fn new(player: &mut players::Player, bet: f64) -> PlayerHand {
        let sub_hand: SubHand = SubHand::new();
        let sub_hands: Vec<SubHand> = vec![sub_hand];
        PlayerHand { player, bet, sub_hands }
    }

    fn active_hand(&self) -> Option<&SubHand> {
        self.sub_hands.iter().filter(|&h| !h.is_resolved()).last()
        }

    fn active_hand_mut(&mut self) -> Option<&mut SubHand> {
        self.sub_hands.iter_mut().filter(|h| !h.is_resolved()).last()
    }

    pub fn is_resolved(&self) -> bool {
        self.sub_hands.iter().all(|h| h.is_resolved())
        }

    pub fn hit(&mut self, deck: &mut cards::Deck) {
        let card = deck.draw().expect("deck is empty");
        self.add_card(card);
    }

    pub fn stand(&mut self) {
        if let Some(sub_hand) = self.active_hand_mut() {
            sub_hand.stand();
        }
    }

    pub fn add_card(&mut self, card: cards::Card) {
        if let Some(sub_hand) = self.active_hand_mut() {
            sub_hand.add_card(card);
            println!("-> {}", sub_hand.cards.last().expect("no card"));
        }
    }

    fn add_sub_hand(&mut self, sub_hand: SubHand) {
        self.sub_hands.push(sub_hand);
        }

    fn result(&self) -> Vec<HandState> {
        let mut result = Vec::new();
        for sub_hand in self.sub_hands.iter() {
            result.push(sub_hand.result());
                }
        result
    }

    pub fn is_splittable(&self) -> bool {
        match self.active_hand() {
            Some(sub_hand) => sub_hand.is_splittable(),
            _ => false
        }
    }

    pub fn split(&mut self) {
      if self.is_splittable() {
          if self.player.bet(self.bet).is_some() {
              let mut new_hand = SubHand::new();
              let card = self.active_hand_mut()
                  .expect("error").cards.pop()
                  .expect("no card to pop");
              new_hand.add_card(card);
              self.add_sub_hand(new_hand);
          }
      } else { 
          println!("can't split hand");
      }
    }

    pub fn is_original(&self) -> bool {
        let sub_hand_count = self.sub_hands.len();
        let card_count = self.active_hand().expect("no hand").cards.len();
        matches!((sub_hand_count, card_count), (1, 2))
    }


    pub fn double_down(&mut self, deck: &mut cards::Deck) {
        let player_bet = self.player.bet(self.bet);
        match (self.is_original(), player_bet) {
            (true, Some(bet)) => {
                self.bet += bet;
                self.hit(deck);
                self.stand();
            }
            _ => println!("Can't double down")
        }
    }

    pub fn surrender(&mut self) {
        if self.is_original() {
            for sub_hand in self.sub_hands.iter_mut() {
                sub_hand.state = HandState::Surrendered;
            }
        } else {
            println!("can't surrender!");
        }
    }

    pub fn player_name(&self) -> &str {
        &self.player.name
    }

    fn calculate_winnings(&self, dealer_hand_state: &HandState) -> f64 {
        let mut winnings: f64 = 0f64;
        for hand_state in self.result() {
            match (hand_state, dealer_hand_state) {
                (HandState::Resolved(x), HandState::Resolved(y)) if x < *y => (),
                (HandState::Resolved(x), HandState::Resolved(y)) if x == *y => winnings += self.bet,
                (HandState::Resolved(_), _) => winnings += self.bet * 2f64,
                (HandState::Surrendered, _) => winnings += self.bet / 2f64,
                _ => (),
            }
        }
        winnings
    }

    pub fn pay_out(&mut self, dealer_hand_state: &HandState) {
        let winnings = self.calculate_winnings(dealer_hand_state);
        self.player.collect_winnings(winnings);
    }

    pub fn play_decision(&mut self, decision: &str, deck: &mut cards::Deck) {
        match decision {
            "hit" => self.hit(deck),
            "stand" => self.stand(),
            "split" => self.split(),
            "double down" => self.double_down(deck),
            "surrender" => self.surrender(),
            _ => println!("decision not recognized")
        }
    }

    pub fn print_hand(&self) {
        if let Some(sub_hand) = self.active_hand() {
            sub_hand.print_hand();
        }
    }
}
