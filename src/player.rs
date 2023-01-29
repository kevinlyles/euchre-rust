use dyn_clonable::clonable;

use crate::{
    bid_result::BidResultCalled,
    card::{Card, CardBeforeBidding},
    hand::{Hand, HandBeforeBidding},
    position::Position,
    suit::Suit,
    trick_state::PlayedCard,
};

#[clonable]
pub trait Player: Clone + Send + Sync {
    fn should_order_up(
        &mut self,
        _hand: &HandBeforeBidding,
        _dealer: &Position,
        _trump_candidate: &CardBeforeBidding,
    ) -> bool {
        false
    }

    fn should_order_up_alone(
        &mut self,
        _hand: &HandBeforeBidding,
        _dealer: &Position,
        _trump_candidate: &CardBeforeBidding,
    ) -> bool {
        false
    }

    fn should_defend_alone_ordered(
        &mut self,
        _hand: &HandBeforeBidding,
        _dealer: &Position,
        _trump_candidate: &CardBeforeBidding,
    ) -> bool {
        false
    }

    fn call_trump(
        &mut self,
        _hand: &HandBeforeBidding,
        _dealer: &Position,
        _turned_down: &CardBeforeBidding,
    ) -> Option<Suit> {
        None
    }

    fn should_call_alone(
        &mut self,
        _hand: &HandBeforeBidding,
        _dealer: &Position,
        _trump: &Suit,
        _turned_down: &CardBeforeBidding,
    ) -> bool {
        false
    }

    fn should_defend_alone_called(
        &mut self,
        _hand: &HandBeforeBidding,
        _dealer: &Position,
        _trump: &Suit,
        _turned_down: &CardBeforeBidding,
    ) -> bool {
        false
    }

    fn choose_discard(&mut self, hand: &HandBeforeBidding, _trump: &Suit) -> CardBeforeBidding {
        hand.cards[0]
    }

    fn play_card(
        &mut self,
        hand: &Hand,
        _bid_result: &BidResultCalled,
        cards_played: &[PlayedCard],
    ) -> Card {
        match cards_played.first() {
            Some(PlayedCard {
                card: Card { suit, .. },
                ..
            }) => match hand.cards.iter().find(|card| card.suit == *suit) {
                Some(card) => *card,
                None => hand.cards[0],
            },
            None => hand.cards[0],
        }
    }

    fn trick_end(&mut self, _bid_result: &BidResultCalled, _cards_played: &[PlayedCard]) {}
}
