use crate::{card::CardProps, player::Player, suit::Suit};

pub struct TrickState {
    pub phase: TrickPhase,
    update_in_parent: Box<dyn FnMut(TrickState) -> ()>,
    finish: Box<dyn FnMut(Player) -> ()>,
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
    pub fn create(
        leader: Player,
        update_in_parent: Box<dyn FnMut(TrickState) -> ()>,
        finish: Box<dyn FnMut(Player) -> ()>,
    ) -> TrickState {
        TrickState {
            phase: TrickPhase::Start { leader },
            update_in_parent,
            finish,
        }
    }
}
