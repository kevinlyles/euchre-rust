use crate::card::*;
use crate::game_state::GameState;
use crate::hand::HandProps;
use crate::rank_with_bowers::RankWithBowers;
use enum_iterator::IntoEnumIterator;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Deck {
    pub cards: Vec<CardProps>,
}

impl Deck {
    fn create_all_cards() -> Vec<CardProps> {
        let mut cards = Vec::with_capacity(24);
        for suit in Suit::into_enum_iter() {
            for rank in RankWithBowers::into_enum_iter().skip(2) {
                cards.push(CardProps { suit, rank });
            }
        }
        cards
    }

    pub fn create_shuffled_deck() -> Deck {
        let mut cards = Deck::create_all_cards();
        cards.shuffle(&mut thread_rng());
        Deck { cards }
    }

    pub fn deal(&mut self) -> GameState {
        let mut hands = [
            HandProps {
                cards: Vec::new(),
                visible: false,
            },
            HandProps {
                cards: Vec::new(),
                visible: false,
            },
            HandProps {
                cards: Vec::new(),
                visible: false,
            },
            HandProps {
                cards: Vec::new(),
                visible: true,
            },
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
            hands,
            trump_candidate: self.cards.pop().unwrap(),
        }
    }
}
