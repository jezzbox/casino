use crate::{cards, players};
use crate::common;

mod hands;


enum RoundState {
    TakingBets,
    InPlay,
    Finished,
}

pub struct Round<'a> {
    dealer_hand: hands::SubHand,
    player_hands: Vec<hands::PlayerHand<'a>>,
    deck: cards::Deck,
    state: RoundState,
}

impl<'a> Round<'a> {
    pub fn new() -> Round<'a> {
        Round { player_hands : Vec::new(),
            dealer_hand: hands::SubHand::new(), 
            deck: cards::Deck::standard(), 
            state: RoundState::TakingBets}  
    }

    pub fn take_bet(&mut self, player: &'a mut players::Player, bet: f64) {
        match self.state {
            RoundState::TakingBets => {
                if let Some(bet) = player.bet(bet) {
                    let player_hand = hands::PlayerHand::new(player, bet);
                    self.player_hands.push(player_hand);
                }
            }
            _ => println!("Currently not taking bets.")
        }
    }

    pub fn play(&mut self) {
        if let RoundState::TakingBets = self.state {
            self.state = RoundState::InPlay;

            self.deal();
            self.play_player_hands();
            self.play_dealer_hand();

            self.state = RoundState::Finished;

            let dealer_hand_state = self.dealer_hand.state();
            for hand in self.player_hands.iter_mut() {
                hand.pay_out(dealer_hand_state);
            }         
        }
    }

    fn deal(&mut self) {
        for i in 0..2 {
            for player_hand in self.player_hands.iter_mut() {
                player_hand.hit(&mut self.deck);
            }

            let card = self.deck.draw().expect("no more cards!");
            match i {
                0 => println!("dealer -> hidden"),
                _ => println!("dealer -> {}", card),
            }
            self.dealer_hand.add_card(card);
        }

    }

    fn play_player_hands(&mut self) {
        for hand in self.player_hands.iter_mut() {
            println!("{}", hand.player_name());
            while !hand.is_resolved() {
                hand.print_hand();
                println!("Choose: hit, stand, split");
                if hand.is_original() {
                    println!("double down, surrender");
                }
                let decision = common::read_user_input();
                hand.play_decision(decision.as_str(), &mut self.deck)
            }
        }
    }

    fn play_dealer_hand(&mut self) {
        while self.dealer_hand.value() < 17 {
            self.dealer_hand.print_hand();
            let card = self.deck.draw().expect("no more cards");
            println!("dealer hits -> {}", card);
            self.dealer_hand.add_card(card);
            }
    }
}

impl<'a> Default for Round<'a> {
    fn default() -> Self {
        Round::new()
    }
}

