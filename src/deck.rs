use crate::card::*;
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
}
