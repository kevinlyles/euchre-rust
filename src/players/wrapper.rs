use crate::player::Player;

#[derive(Clone)]
pub struct Wrapper<'a> {
    bidder: Box<dyn Player + Send + Sync + 'a>,
    player: Box<dyn Player + Send + Sync + 'a>,
}

impl<'a> Wrapper<'a> {
    pub fn create_separate_bidder(
        bidder: Box<dyn Player + Send + Sync + 'a>,
        player: Box<dyn Player + Send + Sync + 'a>,
    ) -> Wrapper<'a> {
        Wrapper { bidder, player }
    }

    pub fn create_single_player(player: Box<dyn Player + Send + Sync + 'a>) -> Wrapper<'a> {
        Wrapper {
            bidder: player.clone(),
            player: player.clone(),
        }
    }
}

impl Player for Wrapper<'_> {
    fn should_order_up(
        &mut self,
        hand: &crate::hand::Hand,
        dealer: &crate::position::Position,
        trump_candidate: &crate::card::Card,
    ) -> bool {
        self.bidder.should_order_up(hand, dealer, trump_candidate)
    }

    fn should_order_up_alone(
        &mut self,
        hand: &crate::hand::Hand,
        dealer: &crate::position::Position,
        trump_candidate: &crate::card::Card,
    ) -> bool {
        self.bidder
            .should_order_up_alone(hand, dealer, trump_candidate)
    }

    fn should_defend_alone_ordered(
        &mut self,
        hand: &crate::hand::Hand,
        dealer: &crate::position::Position,
        trump_candidate: &crate::card::Card,
    ) -> bool {
        self.bidder
            .should_defend_alone_ordered(hand, dealer, trump_candidate)
    }

    fn call_trump(
        &mut self,
        hand: &crate::hand::Hand,
        dealer: &crate::position::Position,
        turned_down: &crate::card::Card,
    ) -> Option<crate::suit::Suit> {
        self.bidder.call_trump(hand, dealer, turned_down)
    }

    fn should_call_alone(
        &mut self,
        hand: &crate::hand::Hand,
        dealer: &crate::position::Position,
        trump: &crate::suit::Suit,
        turned_down: &crate::card::Card,
    ) -> bool {
        self.bidder
            .should_call_alone(hand, dealer, trump, turned_down)
    }

    fn should_defend_alone_called(
        &mut self,
        hand: &crate::hand::Hand,
        dealer: &crate::position::Position,
        trump: &crate::suit::Suit,
        turned_down: &crate::card::Card,
    ) -> bool {
        self.bidder
            .should_defend_alone_called(hand, dealer, trump, turned_down)
    }

    fn choose_discard(
        &mut self,
        hand: &crate::hand::Hand,
        trump: &crate::suit::Suit,
    ) -> crate::card::Card {
        self.player.choose_discard(hand, trump)
    }

    fn play_card(
        &mut self,
        hand: &crate::hand::Hand,
        trump: &crate::suit::Suit,
        led: Option<crate::suit::Suit>,
    ) -> crate::card::Card {
        self.player.play_card(hand, trump, led)
    }
}
