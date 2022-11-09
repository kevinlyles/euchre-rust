use crate::{position::Position, suit::Suit};

#[derive(Clone, PartialEq)]
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
