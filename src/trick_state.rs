use crate::{card::Card, player::Player, suit::Suit};

enum TrickState {
    Start {
        leader: Player,
    },
    FirstCardPlayed {
        suit_lead: Suit,
        cards_played: [Card; 1],
    },
    SecondCardPlayed {
        suit_lead: Suit,
        cards_played: [Card; 2],
    },
    ThirdCardPlayed {
        suit_lead: Suit,
        cards_played: [Card; 3],
    },
    FourthCardPlayed {
        suit_lead: Suit,
        cards_played: [Card; 4],
    },
}
