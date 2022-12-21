use crate::card::Card;

#[derive(Clone, PartialEq, Debug)]
pub struct Hand {
    pub cards: Vec<Card>,
}
