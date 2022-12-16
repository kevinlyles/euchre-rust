use crate::card::*;
use crate::hand::Hand;
use crate::rank_with_bowers::RankWithBowers;
use crate::suit::Suit;
use enum_iterator::IntoEnumIterator;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    fn create_all_cards() -> Vec<Card> {
        let mut cards = Vec::with_capacity(24);
        for suit in Suit::into_enum_iter() {
            for rank in RankWithBowers::into_enum_iter().skip(2) {
                cards.push(Card { suit, rank });
            }
        }
        cards
    }

    pub fn create_shuffled_deck() -> Deck {
        let mut cards = Deck::create_all_cards();
        cards.shuffle(&mut thread_rng());
        Deck { cards }
    }

    pub fn deal(&mut self) -> ([Hand; 4], Card) {
        let mut hands = [
            Hand { cards: Vec::new() },
            Hand { cards: Vec::new() },
            Hand { cards: Vec::new() },
            Hand { cards: Vec::new() },
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
        (hands, self.cards.pop().unwrap())
    }
}
