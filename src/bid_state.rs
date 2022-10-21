use crate::{card::CardProps, player::Player, suit::Suit};

#[derive(PartialEq)]
pub enum BidState {
    FirstRoundFirstPlayer {
        dealer: Player,
        trump_candidate: CardProps,
    },
    FirstRoundSecondPlayer {
        dealer: Player,
        trump_candidate: CardProps,
    },
    FirstRoundThirdPlayer {
        dealer: Player,
        trump_candidate: CardProps,
    },
    FirstRoundFourthPlayer {
        dealer: Player,
        trump_candidate: CardProps,
    },
    OrderedUp {
        dealer: Player,
        caller: Player,
        trump_candidate: CardProps,
    },
    SecondRoundFirstPlayer {
        dealer: Player,
        forbidden_suit: Suit,
    },
    SecondRoundSecondPlayer {
        dealer: Player,
        forbidden_suit: Suit,
    },
    SecondRoundThirdPlayer {
        dealer: Player,
        forbidden_suit: Suit,
    },
    SecondRoundFourthPlayer {
        dealer: Player,
        forbidden_suit: Suit,
    },
    Called {
        dealer: Player,
        caller: Player,
        trump: Suit,
    },
}

impl BidState {}
