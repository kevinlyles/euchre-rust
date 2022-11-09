use crate::{card::CardLogic, hand::HandLogic, position::Position, suit::Suit};

pub trait Player {
    fn should_order_up(hand: &HandLogic, dealer: Position, trump_candidate: CardLogic) -> bool {
        false
    }

    fn call_trump(hand: &HandLogic, dealer: Position, turned_down: CardLogic) -> Option<Suit> {
        None
    }

    fn should_go_alone(
        hand: &HandLogic,
        dealer: Position,
        trump_candidate: Option<CardLogic>,
    ) -> bool {
        false
    }

    fn should_defend_alone(
        hand: &HandLogic,
        dealer: Position,
        trump_candidate: Option<CardLogic>,
    ) -> bool {
        false
    }

    fn choose_discard(hand: &HandLogic) -> CardLogic {
        hand.cards[0]
    }

    fn play_card(hand: &HandLogic, led: Option<Suit>) -> CardLogic {
        match led {
            Some(suit) => match hand.cards.iter().filter(|card| card.suit == suit).nth(0) {
                Some(card) => *card,
                None => hand.cards[0],
            },
            None => hand.cards[0],
        }
    }
}
