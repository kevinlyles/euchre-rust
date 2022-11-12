use crate::{card::CardLogic, hand::HandLogic, position::Position, suit::Suit};

pub trait Player {
    fn should_order_up(
        &mut self,
        hand: &HandLogic,
        dealer: &Position,
        trump_candidate: &CardLogic,
    ) -> bool {
        false
    }

    fn should_order_up_alone(
        &mut self,
        hand: &HandLogic,
        dealer: &Position,
        trump_candidate: &CardLogic,
    ) -> bool {
        false
    }

    fn should_defend_alone_ordered(
        &mut self,
        hand: &HandLogic,
        dealer: &Position,
        trump_candidate: &CardLogic,
    ) -> bool {
        false
    }

    fn call_trump(
        &mut self,
        hand: &HandLogic,
        dealer: &Position,
        turned_down: &CardLogic,
    ) -> Option<Suit> {
        None
    }

    fn should_call_alone(
        &mut self,
        hand: &HandLogic,
        dealer: &Position,
        trump: &Suit,
        turned_down: &CardLogic,
    ) -> bool {
        false
    }

    fn should_defend_alone_called(
        &mut self,
        hand: &HandLogic,
        dealer: &Position,
        trump: &Suit,
        turned_down: &CardLogic,
    ) -> bool {
        false
    }

    fn choose_discard(&mut self, hand: &HandLogic) -> CardLogic {
        hand.cards[0]
    }

    fn play_card(&mut self, hand: &HandLogic, led: Option<Suit>) -> CardLogic {
        match led {
            Some(suit) => match hand.cards.iter().filter(|card| card.suit == suit).nth(0) {
                Some(card) => *card,
                None => hand.cards[0],
            },
            None => hand.cards[0],
        }
    }
}
