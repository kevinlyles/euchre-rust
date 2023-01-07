use crate::{card::Card, hand::Hand, player::Player, position::Position, suit::Suit};

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
    fn should_order_up(&mut self, hand: &Hand, dealer: &Position, trump_candidate: &Card) -> bool {
        self.bidder.should_order_up(hand, dealer, trump_candidate)
    }

    fn should_order_up_alone(
        &mut self,
        hand: &Hand,
        dealer: &Position,
        trump_candidate: &Card,
    ) -> bool {
        self.bidder
            .should_order_up_alone(hand, dealer, trump_candidate)
    }

    fn should_defend_alone_ordered(
        &mut self,
        hand: &Hand,
        dealer: &Position,
        trump_candidate: &Card,
    ) -> bool {
        self.bidder
            .should_defend_alone_ordered(hand, dealer, trump_candidate)
    }

    fn call_trump(&mut self, hand: &Hand, dealer: &Position, turned_down: &Card) -> Option<Suit> {
        self.bidder.call_trump(hand, dealer, turned_down)
    }

    fn should_call_alone(
        &mut self,
        hand: &Hand,
        dealer: &Position,
        trump: &Suit,
        turned_down: &Card,
    ) -> bool {
        self.bidder
            .should_call_alone(hand, dealer, trump, turned_down)
    }

    fn should_defend_alone_called(
        &mut self,
        hand: &Hand,
        dealer: &Position,
        trump: &Suit,
        turned_down: &Card,
    ) -> bool {
        self.bidder
            .should_defend_alone_called(hand, dealer, trump, turned_down)
    }

    fn choose_discard(&mut self, hand: &Hand, trump: &Suit) -> Card {
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
