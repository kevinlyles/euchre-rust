use crate::{
    card::CardBeforeBidding, hand::HandBeforeBidding, player::Player, position::Position,
    suit::Suit,
};

#[derive(Clone)]
pub(crate) struct PreprogrammedBidder {
    order_up: bool,
    order_up_alone: bool,
    defend_alone: bool,
    trump_to_call: Option<Suit>,
    call_alone: bool,
    card_to_discard: Option<CardBeforeBidding>,
}

impl PreprogrammedBidder {
    pub(crate) fn does_nothing() -> PreprogrammedBidder {
        PreprogrammedBidder {
            order_up: false,
            order_up_alone: false,
            defend_alone: false,
            trump_to_call: None,
            call_alone: false,
            card_to_discard: None,
        }
    }

    pub(crate) fn orders_up() -> PreprogrammedBidder {
        PreprogrammedBidder {
            order_up: true,
            order_up_alone: false,
            defend_alone: false,
            trump_to_call: None,
            call_alone: false,
            card_to_discard: None,
        }
    }

    pub(crate) fn orders_up_alone() -> PreprogrammedBidder {
        PreprogrammedBidder {
            order_up: true,
            order_up_alone: true,
            defend_alone: false,
            trump_to_call: None,
            call_alone: false,
            card_to_discard: None,
        }
    }

    pub(crate) fn defends_alone() -> PreprogrammedBidder {
        #![allow(unused)]
        PreprogrammedBidder {
            order_up: false,
            order_up_alone: false,
            defend_alone: true,
            trump_to_call: None,
            call_alone: false,
            card_to_discard: None,
        }
    }

    pub(crate) fn calls(trump: Suit) -> PreprogrammedBidder {
        PreprogrammedBidder {
            order_up: false,
            order_up_alone: false,
            defend_alone: false,
            trump_to_call: Some(trump),
            call_alone: false,
            card_to_discard: None,
        }
    }

    pub(crate) fn calls_alone(trump: Suit) -> PreprogrammedBidder {
        PreprogrammedBidder {
            order_up: false,
            order_up_alone: false,
            defend_alone: false,
            trump_to_call: Some(trump),
            call_alone: true,
            card_to_discard: None,
        }
    }

    pub(crate) fn discards(card: CardBeforeBidding) -> PreprogrammedBidder {
        #![allow(unused)]
        PreprogrammedBidder {
            order_up: false,
            order_up_alone: false,
            defend_alone: false,
            trump_to_call: None,
            call_alone: false,
            card_to_discard: Some(card),
        }
    }
}

impl Player for PreprogrammedBidder {
    fn should_order_up(
        &mut self,
        _hand: &HandBeforeBidding,
        _dealer: &Position,
        _trump_candidate: &CardBeforeBidding,
    ) -> bool {
        self.order_up
    }

    fn should_order_up_alone(
        &mut self,
        _hand: &HandBeforeBidding,
        _dealer: &Position,
        _trump_candidate: &CardBeforeBidding,
    ) -> bool {
        self.order_up_alone
    }

    fn should_defend_alone_ordered(
        &mut self,
        _hand: &HandBeforeBidding,
        _dealer: &Position,
        _trump_candidate: &CardBeforeBidding,
    ) -> bool {
        self.defend_alone
    }

    fn call_trump(
        &mut self,
        _hand: &HandBeforeBidding,
        _dealer: &Position,
        _turned_down: &CardBeforeBidding,
    ) -> Option<Suit> {
        self.trump_to_call
    }

    fn should_call_alone(
        &mut self,
        _hand: &HandBeforeBidding,
        _dealer: &Position,
        _trump: &Suit,
        _turned_down: &CardBeforeBidding,
    ) -> bool {
        self.call_alone
    }

    fn should_defend_alone_called(
        &mut self,
        _hand: &HandBeforeBidding,
        _dealer: &Position,
        _trump: &Suit,
        _turned_down: &CardBeforeBidding,
    ) -> bool {
        self.defend_alone
    }

    fn choose_discard(&mut self, hand: &HandBeforeBidding, _trump: &Suit) -> CardBeforeBidding {
        match self.card_to_discard {
            Some(card) => card,
            None => hand.cards[0],
        }
    }
}
