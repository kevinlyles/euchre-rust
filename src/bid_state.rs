use std::ops::Deref;

use yew::html::IntoPropValue;

use crate::{card::CardProps, player::Player, suit::Suit};

#[derive(PartialEq)]
pub struct BidState {
    pub dealer: Player,
    pub phase: BidStateKind,
}

#[derive(PartialEq)]
pub enum BidStateKind {
    FirstRoundFirstPlayer {
        trump_candidate: CardProps,
    },
    FirstRoundSecondPlayer {
        trump_candidate: CardProps,
    },
    FirstRoundThirdPlayer {
        trump_candidate: CardProps,
    },
    FirstRoundFourthPlayer {
        trump_candidate: CardProps,
    },
    OrderedUp {
        caller: Player,
        trump_candidate: CardProps,
    },
    SecondRoundFirstPlayer {
        forbidden_suit: Suit,
    },
    SecondRoundSecondPlayer {
        forbidden_suit: Suit,
    },
    SecondRoundThirdPlayer {
        forbidden_suit: Suit,
    },
    SecondRoundFourthPlayer {
        forbidden_suit: Suit,
    },
    Called {
        caller: Player,
        trump: Suit,
    },
}

impl BidState {}

impl IntoPropValue<Option<CardProps>> for &BidState {
    fn into_prop_value(self) -> Option<CardProps> {
        match self.phase {
            BidStateKind::FirstRoundFirstPlayer { trump_candidate }
            | BidStateKind::FirstRoundSecondPlayer { trump_candidate }
            | BidStateKind::FirstRoundThirdPlayer { trump_candidate }
            | BidStateKind::FirstRoundFourthPlayer { trump_candidate }
            | BidStateKind::OrderedUp {
                caller: _,
                trump_candidate,
            } => Some(trump_candidate),
            BidStateKind::SecondRoundFirstPlayer { forbidden_suit: _ }
            | BidStateKind::SecondRoundSecondPlayer { forbidden_suit: _ }
            | BidStateKind::SecondRoundThirdPlayer { forbidden_suit: _ }
            | BidStateKind::SecondRoundFourthPlayer { forbidden_suit: _ }
            | BidStateKind::Called {
                caller: _,
                trump: _,
            } => None,
        }
    }
}
