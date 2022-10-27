use crate::{
    bid_state::BidState, hand::HandLogic, player::Player, suit::Suit, trick_state::TrickState,
};

#[derive(Clone, PartialEq)]
pub struct HandState {
    pub dealer: Player,
    pub phase: HandStateKind,
}

#[derive(Clone, PartialEq)]
pub enum HandStateKind {
    Bidding {
        hands: [HandLogic; 4],
        bid_state: BidState,
    },
    FirstTrick {
        hands: [HandLogic; 4],
        trump: Suit,
        trick_state: TrickState,
    },
    SecondTrick {
        hands: [HandLogic; 4],
        trump: Suit,
        trick_state: TrickState,
        tricks_taken: [u8; 4],
    },
    ThirdTrick {
        hands: [HandLogic; 4],
        trump: Suit,
        trick_state: TrickState,
        tricks_taken: [u8; 4],
    },
    FourthTrick {
        hands: [HandLogic; 4],
        trump: Suit,
        trick_state: TrickState,
        tricks_taken: [u8; 4],
    },
    FifthTrick {
        hands: [HandLogic; 4],
        trump: Suit,
        trick_state: TrickState,
        tricks_taken: [u8; 4],
    },
    Scoring {
        tricks_taken: [u8; 4],
    },
}
