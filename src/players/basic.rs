use crate::{hand::Hand, player::Player, position::Position, rank_with_bowers::RankWithBowers};

#[derive(Clone)]
pub(crate) struct BasicPlayer {
    pub(crate) position: Position,
}

impl Player for BasicPlayer {
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
}
