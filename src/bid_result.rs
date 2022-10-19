use crate::{player::Player, suit::Suit};

struct BidResult {
    trump: Suit,
    caller: Player,
    went_alone: bool,
    defended_alone: Option<Player>,
}
