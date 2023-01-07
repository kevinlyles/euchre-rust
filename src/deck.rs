use crate::card::*;
use crate::hand::HandBeforeBidding;
use crate::rank::Rank;
use crate::suit::Suit;
use enum_iterator::IntoEnumIterator;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub(crate) struct Deck {
    pub(crate) cards: Vec<CardBeforeBidding>,
}

impl Deck {
    pub(crate) fn create_all_cards() -> Vec<CardBeforeBidding> {
        let mut cards = Vec::with_capacity(24);
        for suit in Suit::into_enum_iter() {
            for rank in Rank::into_enum_iter() {
                cards.push(CardBeforeBidding { suit, rank });
            }
        }
        cards
    }

    pub(crate) fn create_shuffled_deck() -> Deck {
        let mut cards = Deck::create_all_cards();
        cards.shuffle(&mut thread_rng());
        Deck { cards }
    }

    pub(crate) fn deal(&mut self) -> ([HandBeforeBidding; 4], CardBeforeBidding) {
        let mut hands = [
            HandBeforeBidding { cards: Vec::new() },
            HandBeforeBidding { cards: Vec::new() },
            HandBeforeBidding { cards: Vec::new() },
            HandBeforeBidding { cards: Vec::new() },
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
