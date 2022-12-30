use crate::{card::Card, suit::Suit};

#[derive(Clone, PartialEq, Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn update_bowers(hand: &mut Hand, trump: &Suit) -> () {
        Card::update_bowers(&mut hand.cards, trump);
    }
}
