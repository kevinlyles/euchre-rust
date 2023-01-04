use crate::{hand::Hand, player::Player, position::Position, rank_with_bowers::RankWithBowers};

#[derive(Clone)]
pub(crate) struct AdvancedPlayer {
    pub(crate) position: Position,
}

impl Player for AdvancedPlayer {
    fn should_order_up(
        &mut self,
        hand: &Hand,
        dealer: &Position,
        trump_candidate: &crate::card::Card,
    ) -> bool {
        let trump_cards = hand.cards.iter().filter(|card| {
            card.suit == trump_candidate.suit
                || card.rank == RankWithBowers::Jack
                    && card.suit == trump_candidate.suit.other_suit_of_same_color()
        });
        match trump_cards.count() {
            4 | 5 => true,
            3 if *dealer == self.position || *dealer == self.position.partner() => true,
            _ => false,
        }
    }

    fn call_trump(
        &mut self,
        hand: &Hand,
        _dealer: &Position,
        _turned_down: &crate::card::Card,
    ) -> Option<crate::suit::Suit> {
        if hand
            .cards
            .iter()
            .filter(|card| card.suit == hand.cards[0].suit)
            .count()
            >= 4
        {
            Some(hand.cards[0].suit)
        } else if hand
            .cards
            .iter()
            .filter(|card| card.suit == hand.cards[1].suit)
            .count()
            >= 4
        {
            Some(hand.cards[1].suit)
        } else {
            None
        }
    }

    fn should_order_up_alone(
        &mut self,
        _hand: &Hand,
        _dealer: &Position,
        _trump_candidate: &crate::card::Card,
    ) -> bool {
        false
    }

    fn should_defend_alone_ordered(
        &mut self,
        _hand: &Hand,
        _dealer: &Position,
        _trump_candidate: &crate::card::Card,
    ) -> bool {
        false
    }

    fn should_call_alone(
        &mut self,
        _hand: &Hand,
        _dealer: &Position,
        _trump: &crate::suit::Suit,
        _turned_down: &crate::card::Card,
    ) -> bool {
        false
    }

    fn should_defend_alone_called(
        &mut self,
        _hand: &Hand,
        _dealer: &Position,
        _trump: &crate::suit::Suit,
        _turned_down: &crate::card::Card,
    ) -> bool {
        false
    }

    fn choose_discard(&mut self, hand: &Hand, trump: &crate::suit::Suit) -> crate::card::Card {
        *hand
            .cards
            .iter()
            .filter(|card| card.suit != *trump)
            .nth(0)
            .unwrap_or(&hand.cards[0])
    }

    fn play_card(
        &mut self,
        hand: &Hand,
        _trump: &crate::suit::Suit,
        led: Option<crate::suit::Suit>,
    ) -> crate::card::Card {
        match led {
            Some(suit) => match hand.cards.iter().filter(|card| card.suit == suit).nth(0) {
                Some(card) => *card,
                None => hand.cards[0],
            },
            None => hand.cards[0],
        }
    }
}
