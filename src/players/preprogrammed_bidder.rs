use crate::{card::Card, hand::Hand, player::Player, position::Position, suit::Suit};

#[derive(Clone)]
pub struct PreprogrammedBidder {
    order_up: bool,
    order_up_alone: bool,
    defend_alone: bool,
    trump_to_call: Option<Suit>,
    call_alone: bool,
    card_to_discard: Option<Card>,
}

impl PreprogrammedBidder {
    pub fn does_nothing() -> PreprogrammedBidder {
        PreprogrammedBidder {
            order_up: false,
            order_up_alone: false,
            defend_alone: false,
            trump_to_call: None,
            call_alone: false,
            card_to_discard: None,
        }
    }

    pub fn orders_up() -> PreprogrammedBidder {
        PreprogrammedBidder {
            order_up: true,
            order_up_alone: false,
            defend_alone: false,
            trump_to_call: None,
            call_alone: false,
            card_to_discard: None,
        }
    }

    pub fn orders_up_alone() -> PreprogrammedBidder {
        PreprogrammedBidder {
            order_up: true,
            order_up_alone: true,
            defend_alone: false,
            trump_to_call: None,
            call_alone: false,
            card_to_discard: None,
        }
    }

    pub fn defends_alone() -> PreprogrammedBidder {
        PreprogrammedBidder {
            order_up: false,
            order_up_alone: false,
            defend_alone: true,
            trump_to_call: None,
            call_alone: false,
            card_to_discard: None,
        }
    }

    pub fn calls(trump: Suit) -> PreprogrammedBidder {
        PreprogrammedBidder {
            order_up: false,
            order_up_alone: false,
            defend_alone: false,
            trump_to_call: Some(trump),
            call_alone: false,
            card_to_discard: None,
        }
    }

    pub fn calls_alone(trump: Suit) -> PreprogrammedBidder {
        PreprogrammedBidder {
            order_up: false,
            order_up_alone: false,
            defend_alone: false,
            trump_to_call: Some(trump),
            call_alone: true,
            card_to_discard: None,
        }
    }

    pub fn discards(card: Card) -> PreprogrammedBidder {
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
        _hand: &Hand,
        _dealer: &Position,
        _trump_candidate: &Card,
    ) -> bool {
        self.order_up
    }

    fn should_order_up_alone(
        &mut self,
        _hand: &Hand,
        _dealer: &Position,
        _trump_candidate: &Card,
    ) -> bool {
        self.order_up_alone
    }

    fn should_defend_alone_ordered(
        &mut self,
        _hand: &Hand,
        _dealer: &Position,
        _trump_candidate: &Card,
    ) -> bool {
        self.defend_alone
    }

    fn call_trump(
        &mut self,
        _hand: &Hand,
        _dealer: &Position,
        _turned_down: &Card,
    ) -> Option<Suit> {
        self.trump_to_call
    }

    fn should_call_alone(
        &mut self,
        _hand: &Hand,
        _dealer: &Position,
        _trump: &Suit,
        _turned_down: &Card,
    ) -> bool {
        self.call_alone
    }

    fn should_defend_alone_called(
        &mut self,
        _hand: &Hand,
        _dealer: &Position,
        _trump: &Suit,
        _turned_down: &Card,
    ) -> bool {
        self.defend_alone
    }

    fn choose_discard(&mut self, hand: &Hand, _trump: &Suit) -> Card {
        match self.card_to_discard {
            Some(card) => card,
            None => hand.cards[0],
        }
    }
}
