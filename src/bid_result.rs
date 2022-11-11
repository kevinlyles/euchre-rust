use crate::{position::Position, suit::Suit};

#[derive(Clone)]
pub enum BidResult {
    Called {
        trump: Suit,
        caller: Position,
    },
    CalledAlone {
        trump: Suit,
        caller: Position,
    },
    DefendedAlone {
        trump: Suit,
        caller: Position,
        defender: Position,
    },
    NoOneCalled,
}
