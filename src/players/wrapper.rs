use crate::{
    card::{Card, CardBeforeBidding},
    hand::{Hand, HandBeforeBidding},
    player::Player,
    position::Position,
    suit::Suit,
};

#[derive(Clone)]
pub(crate) struct Wrapper<'a> {
    bidder: Box<dyn Player + Send + Sync + 'a>,
    player: Box<dyn Player + Send + Sync + 'a>,
}

impl<'a> Wrapper<'a> {
    pub(crate) fn create_separate_bidder(
        bidder: Box<dyn Player + Send + Sync + 'a>,
        player: Box<dyn Player + Send + Sync + 'a>,
    ) -> Wrapper<'a> {
        Wrapper { bidder, player }
    }

    pub(crate) fn create_single_player(player: Box<dyn Player + Send + Sync + 'a>) -> Wrapper<'a> {
        Wrapper {
            bidder: player.clone(),
            player: player.clone(),
        }
    }
}

impl Player for Wrapper<'_> {
    fn should_order_up(
        &mut self,
        hand: &HandBeforeBidding,
        dealer: &Position,
        trump_candidate: &CardBeforeBidding,
    ) -> bool {
        self.bidder.should_order_up(hand, dealer, trump_candidate)
    }

    fn should_order_up_alone(
        &mut self,
        hand: &HandBeforeBidding,
        dealer: &Position,
        trump_candidate: &CardBeforeBidding,
    ) -> bool {
        self.bidder
            .should_order_up_alone(hand, dealer, trump_candidate)
    }

    fn should_defend_alone_ordered(
        &mut self,
        hand: &HandBeforeBidding,
        dealer: &Position,
        trump_candidate: &CardBeforeBidding,
    ) -> bool {
        self.bidder
            .should_defend_alone_ordered(hand, dealer, trump_candidate)
    }

    fn call_trump(
        &mut self,
        hand: &HandBeforeBidding,
        dealer: &Position,
        turned_down: &CardBeforeBidding,
    ) -> Option<Suit> {
        self.bidder.call_trump(hand, dealer, turned_down)
    }

    fn should_call_alone(
        &mut self,
        hand: &HandBeforeBidding,
        dealer: &Position,
        trump: &Suit,
        turned_down: &CardBeforeBidding,
    ) -> bool {
        self.bidder
            .should_call_alone(hand, dealer, trump, turned_down)
    }

    fn should_defend_alone_called(
        &mut self,
        hand: &HandBeforeBidding,
        dealer: &Position,
        trump: &Suit,
        turned_down: &CardBeforeBidding,
    ) -> bool {
        self.bidder
            .should_defend_alone_called(hand, dealer, trump, turned_down)
    }

    fn choose_discard(&mut self, hand: &HandBeforeBidding, trump: &Suit) -> CardBeforeBidding {
        self.player.choose_discard(hand, trump)
    }

    fn play_card(
        &mut self,
        hand: &Hand,
        caller: &Position,
        trump: &Suit,
        led: Option<Suit>,
    ) -> Card {
        self.player.play_card(hand, caller, trump, led)
    }
}
