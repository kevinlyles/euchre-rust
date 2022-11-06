use crate::bid_state::{BidState, BidStateKind};
use crate::card::*;
use crate::game_state::GameState;
use crate::hand::{HandLogic, HandProps};
use crate::hand_state::{HandState, HandStateKind};
use crate::player::Player;
use crate::rank_with_bowers::RankWithBowers;
use crate::suit::Suit;
use enum_iterator::IntoEnumIterator;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Deck {
    pub cards: Vec<CardLogic>,
}

impl Deck {
    fn create_all_cards() -> Vec<CardLogic> {
        let mut cards = Vec::with_capacity(24);
        for suit in Suit::into_enum_iter() {
            for rank in RankWithBowers::into_enum_iter().skip(2) {
                cards.push(CardLogic { suit, rank });
            }
        }
        cards
    }

    pub fn create_shuffled_deck() -> Deck {
        let mut cards = Deck::create_all_cards();
        cards.shuffle(&mut thread_rng());
        Deck { cards }
    }

    pub fn deal(&mut self, dealer: Player) -> GameState {
        let mut hands = [
            HandLogic { cards: Vec::new() },
            HandLogic { cards: Vec::new() },
            HandLogic { cards: Vec::new() },
            HandLogic { cards: Vec::new() },
        ];
        hands[0].cards.push(self.cards.pop().unwrap());
        hands[0].cards.push(self.cards.pop().unwrap());
        hands[0].cards.push(self.cards.pop().unwrap());
        hands[1].cards.push(self.cards.pop().unwrap());
        hands[1].cards.push(self.cards.pop().unwrap());
        hands[2].cards.push(self.cards.pop().unwrap());
        hands[2].cards.push(self.cards.pop().unwrap());
        hands[2].cards.push(self.cards.pop().unwrap());
        hands[3].cards.push(self.cards.pop().unwrap());
        hands[3].cards.push(self.cards.pop().unwrap());
        hands[0].cards.push(self.cards.pop().unwrap());
        hands[0].cards.push(self.cards.pop().unwrap());
        hands[1].cards.push(self.cards.pop().unwrap());
        hands[1].cards.push(self.cards.pop().unwrap());
        hands[1].cards.push(self.cards.pop().unwrap());
        hands[2].cards.push(self.cards.pop().unwrap());
        hands[2].cards.push(self.cards.pop().unwrap());
        hands[3].cards.push(self.cards.pop().unwrap());
        hands[3].cards.push(self.cards.pop().unwrap());
        hands[3].cards.push(self.cards.pop().unwrap());
        GameState {
            hand_state: HandState {
                dealer,
                phase: HandStateKind::Bidding {
                    bid_state: BidState {
                        dealer,
                        hands,
                        phase: BidStateKind::FirstRoundFirstPlayer {
                            trump_candidate: self.cards.pop().unwrap(),
                        },
                    },
                },
            },
        }
    }
}
