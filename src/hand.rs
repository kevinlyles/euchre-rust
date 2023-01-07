use crate::{
    card::{Card, CardBeforeBidding},
    suit::Suit,
};

#[derive(Clone, PartialEq, Debug)]
pub struct HandBeforeBidding {
    pub cards: Vec<CardBeforeBidding>,
}

impl HandBeforeBidding {
    pub fn update_bowers(hand: HandBeforeBidding, trump: &Suit) -> Hand {
        Hand {
            cards: Card::update_bowers(hand.cards, trump),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
}
