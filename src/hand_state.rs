use crate::{card::CardProps, hand::HandProps, suit::Suit};

enum HandState {
    BiddingRoundOne {
        hands: [[HandProps; 5]; 4],
        trump_candidate: CardProps,
    },
    Discarding {
        hands: [[HandProps; 5]; 4],
        trump_candidate: CardProps,
    },
    BiddingRoundTwo {
        hands: [[HandProps; 5]; 4],
        disallowed_suit: Suit,
    },
    FirstTrick {
        hands: [[HandProps; 5]; 4],
        trump: Suit,
    },
    SecondTrick {
        hands: [[HandProps; 4]; 4],
        trump: Suit,
        tricks_taken: [u8; 4],
    },
    ThirdTrick {
        hands: [[HandProps; 3]; 4],
        trump: Suit,
        tricks_taken: [u8; 4],
    },
    FourthTrick {
        hands: [[HandProps; 2]; 4],
        trump: Suit,
        tricks_taken: [u8; 4],
    },
    FifthTrick {
        hands: [[HandProps; 1]; 4],
        trump: Suit,
        tricks_taken: [u8; 4],
    },
    Scoring {
        tricks_taken: [u8; 4],
    },
}
