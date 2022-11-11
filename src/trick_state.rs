use crate::{card::CardLogic, hand::HandLogic, player::Player, position::Position, suit::Suit};

pub struct TrickState {
    leader: Position,
    pub phase: TrickPhase,
}

pub enum TrickPhase {
    Start,
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
            leader,
            phase: TrickPhase::Start,
        }
    }

    pub fn step(
        &mut self,
        players: &mut [Box<dyn Player>; 4],
        hands: &mut [HandLogic; 4],
    ) -> Option<Position> {
        todo!()
    }
}
