use crate::{card::CardLogic, position::Position, suit::Suit};

pub struct TrickState {
    pub phase: TrickPhase,
}

impl PartialEq for TrickState {
    fn eq(&self, other: &Self) -> bool {
        self.phase == other.phase
    }
}

#[derive(Clone, PartialEq)]
pub enum TrickPhase {
    Start {
        leader: Position,
    },
    FirstCardPlayed {
        suit_lead: Suit,
        cards_played: [CardLogic; 1],
    },
    SecondCardPlayed {
        suit_lead: Suit,
        cards_played: [CardLogic; 2],
    },
    ThirdCardPlayed {
        suit_lead: Suit,
        cards_played: [CardLogic; 3],
    },
    FourthCardPlayed {
        suit_lead: Suit,
        cards_played: [CardLogic; 4],
    },
}

impl TrickState {
    pub fn create(leader: Position) -> TrickState {
        TrickState {
            phase: TrickPhase::Start { leader },
        }
    }
}
