use yew::Callback;

use crate::{card::CardProps, player::Player, suit::Suit};

pub struct TrickState {
    pub phase: TrickPhase,
    update: Callback<bool>,
    finish: Callback<Player>,
}

impl PartialEq for TrickState {
    fn eq(&self, other: &Self) -> bool {
        self.phase == other.phase
    }
}

#[derive(Clone, PartialEq)]
pub enum TrickPhase {
    Start {
        leader: Player,
    },
    FirstCardPlayed {
        suit_lead: Suit,
        cards_played: [CardProps; 1],
    },
    SecondCardPlayed {
        suit_lead: Suit,
        cards_played: [CardProps; 2],
    },
    ThirdCardPlayed {
        suit_lead: Suit,
        cards_played: [CardProps; 3],
    },
    FourthCardPlayed {
        suit_lead: Suit,
        cards_played: [CardProps; 4],
    },
}

impl TrickState {
    pub fn create(leader: Player, update: Callback<bool>, finish: Callback<Player>) -> TrickState {
        TrickState {
            phase: TrickPhase::Start { leader },
            update,
            finish,
        }
    }
}
