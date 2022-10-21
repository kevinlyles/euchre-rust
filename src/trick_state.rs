use crate::{card::CardProps, player::Player, suit::Suit};

#[derive(PartialEq)]
pub enum TrickState {
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
