use crate::{player::Player, suit::Suit};

pub enum BidResult {
    Called {
        trump: Suit,
        caller: Player,
    },
    CalledAlone {
        trump: Suit,
        caller: Player,
    },
    DefendedAlone {
        trump: Suit,
        caller: Player,
        defender: Player,
    },
    EveryonePassed,
}
