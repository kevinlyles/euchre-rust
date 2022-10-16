use crate::card::*;
use crate::hand::*;

#[derive(PartialEq)]
pub struct GameState {
    pub hands: [HandProps; 4],
    pub trump_candidate: CardProps,
}
