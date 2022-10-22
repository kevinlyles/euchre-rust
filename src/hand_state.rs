use crate::{
    bid_state::BidState, hand::HandProps, player::Player, suit::Suit, trick_state::TrickState,
};

#[derive(Clone, PartialEq)]
pub struct HandState {
    pub dealer: Player,
    pub phase: HandStateKind,
}

#[derive(Clone, PartialEq)]
pub enum HandStateKind {
    Bidding {
        hands: [HandProps; 4],
        bid_state: BidState,
    },
    FirstTrick {
        hands: [HandProps; 4],
        trump: Suit,
        trick_state: TrickState,
    },
    SecondTrick {
        hands: [HandProps; 4],
        trump: Suit,
        trick_state: TrickState,
        tricks_taken: [u8; 4],
    },
    ThirdTrick {
        hands: [HandProps; 4],
        trump: Suit,
        trick_state: TrickState,
        tricks_taken: [u8; 4],
    },
    FourthTrick {
        hands: [HandProps; 4],
        trump: Suit,
        trick_state: TrickState,
        tricks_taken: [u8; 4],
    },
    FifthTrick {
        hands: [HandProps; 4],
        trump: Suit,
        trick_state: TrickState,
        tricks_taken: [u8; 4],
    },
    Scoring {
        tricks_taken: [u8; 4],
    },
}
