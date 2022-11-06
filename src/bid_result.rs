use crate::{player::Player, suit::Suit};

#[derive(Clone, PartialEq)]
pub struct BidResult {
    pub caller: Player,
    pub trump: Suit,
    pub called_alone: bool,
    pub defender: Option<Player>,
}
