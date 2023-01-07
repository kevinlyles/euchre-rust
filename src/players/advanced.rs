use crate::{hand::Hand, player::Player, position::Position, rank_with_bowers::RankWithBowers};

#[derive(Clone)]
pub(crate) struct AdvancedPlayer {
    position: Position,
    trump_has_been_lead: bool,
}

impl AdvancedPlayer {
    pub(crate) fn create(position: Position) -> AdvancedPlayer {
        AdvancedPlayer {
            position,
            trump_has_been_lead: false,
        }
    }
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

mod tests {
    use crate::{
        bid_result::BidResultAll,
        bid_state::BidState,
        card::Card,
        players::{preprogrammed_bidder::PreprogrammedBidder, wrapper::Wrapper},
        suit::Suit,
    };

    use super::*;

    #[test]
    fn test_cases() {
        test_bidding(
            "All nines and tens, dealer, candidate trump matches",
            Hand {
                cards: vec![
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Ten,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Nine,
                    },
                    Card {
                        suit: Suit::Diamonds,
                        rank: RankWithBowers::Nine,
                    },
                    Card {
                        suit: Suit::Clubs,
                        rank: RankWithBowers::Nine,
                    },
                    Card {
                        suit: Suit::Hearts,
                        rank: RankWithBowers::Nine,
                    },
                ],
            },
            Card {
                suit: Suit::Spades,
                rank: RankWithBowers::King,
            },
            Position::South,
            false,
            None,
            None,
            false,
        );

        test_bidding(
            "Right nine, off ace, off king, off king, candidate trump does not match",
            Hand {
                cards: vec![
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Nine,
                    },
                    Card {
                        suit: Suit::Diamonds,
                        rank: RankWithBowers::Ace,
                    },
                    Card {
                        suit: Suit::Clubs,
                        rank: RankWithBowers::King,
                    },
                    Card {
                        suit: Suit::Hearts,
                        rank: RankWithBowers::King,
                    },
                ],
            },
            Card {
                suit: Suit::Diamonds,
                rank: RankWithBowers::King,
            },
            Position::West,
            false,
            None,
            None,
            false,
        );

        test_bidding(
            "Right, off ace, off jack ten nine, dealer, candidate trump matches",
            Hand {
                cards: vec![
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Clubs,
                        rank: RankWithBowers::Ace,
                    },
                    Card {
                        suit: Suit::Diamonds,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Diamonds,
                        rank: RankWithBowers::Ten,
                    },
                    Card {
                        suit: Suit::Diamonds,
                        rank: RankWithBowers::Nine,
                    },
                ],
            },
            Card {
                suit: Suit::Spades,
                rank: RankWithBowers::Nine,
            },
            Position::South,
            true,
            Some(Card {
                suit: Suit::Diamonds,
                rank: RankWithBowers::Nine,
            }),
            None,
            false,
        );

        test_bidding(
            "Right nine, off ace, off ten nine, dealer, candidate trump matches",
            Hand {
                cards: vec![
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Nine,
                    },
                    Card {
                        suit: Suit::Clubs,
                        rank: RankWithBowers::Ace,
                    },
                    Card {
                        suit: Suit::Diamonds,
                        rank: RankWithBowers::Ten,
                    },
                    Card {
                        suit: Suit::Diamonds,
                        rank: RankWithBowers::Nine,
                    },
                ],
            },
            Card {
                suit: Suit::Spades,
                rank: RankWithBowers::Ten,
            },
            Position::South,
            true,
            Some(Card {
                suit: Suit::Diamonds,
                rank: RankWithBowers::Nine,
            }),
            None,
            false,
        );

        test_bidding(
            "Right, off ace, off jack ten nine, dealer, candidate trump matches",
            Hand {
                cards: vec![
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Clubs,
                        rank: RankWithBowers::Ace,
                    },
                    Card {
                        suit: Suit::Diamonds,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Diamonds,
                        rank: RankWithBowers::Ten,
                    },
                    Card {
                        suit: Suit::Diamonds,
                        rank: RankWithBowers::Nine,
                    },
                ],
            },
            Card {
                suit: Suit::Spades,
                rank: RankWithBowers::Nine,
            },
            Position::South,
            true,
            Some(Card {
                suit: Suit::Diamonds,
                rank: RankWithBowers::Nine,
            }),
            None,
            false,
        );

        test_bidding(
            "Perfect hand, candidate trump does not match",
            Hand {
                cards: vec![
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Clubs,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Ace,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::King,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Queen,
                    },
                ],
            },
            Card {
                suit: Suit::Diamonds,
                rank: RankWithBowers::Ace,
            },
            Position::West,
            false,
            None,
            Some(Suit::Spades),
            true,
        );

        test_bidding(
            "Right king queen, two off queens, candidate trump does not match",
            Hand {
                cards: vec![
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::King,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Queen,
                    },
                    Card {
                        suit: Suit::Hearts,
                        rank: RankWithBowers::Queen,
                    },
                    Card {
                        suit: Suit::Diamonds,
                        rank: RankWithBowers::Queen,
                    },
                ],
            },
            Card {
                suit: Suit::Diamonds,
                rank: RankWithBowers::Ace,
            },
            Position::West,
            false,
            None,
            Some(Suit::Spades),
            false,
        );

        test_bidding(
            "Right left king, off king nine, candidate trump does not match",
            Hand {
                cards: vec![
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::King,
                    },
                    Card {
                        suit: Suit::Clubs,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Hearts,
                        rank: RankWithBowers::King,
                    },
                    Card {
                        suit: Suit::Hearts,
                        rank: RankWithBowers::Nine,
                    },
                ],
            },
            Card {
                suit: Suit::Diamonds,
                rank: RankWithBowers::Ace,
            },
            Position::West,
            false,
            None,
            Some(Suit::Spades),
            true,
        );

        test_bidding(
            "Right nine, off ace, off ten nine, candidate trump does not match",
            Hand {
                cards: vec![
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Nine,
                    },
                    Card {
                        suit: Suit::Diamonds,
                        rank: RankWithBowers::Ace,
                    },
                    Card {
                        suit: Suit::Hearts,
                        rank: RankWithBowers::Ten,
                    },
                    Card {
                        suit: Suit::Hearts,
                        rank: RankWithBowers::Nine,
                    },
                ],
            },
            Card {
                suit: Suit::Diamonds,
                rank: RankWithBowers::King,
            },
            Position::West,
            false,
            None,
            Some(Suit::Spades),
            false,
        );

        test_bidding(
            "Right nine, off ace, off king queen, dealer, candidate trump is ten",
            Hand {
                cards: vec![
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Nine,
                    },
                    Card {
                        suit: Suit::Diamonds,
                        rank: RankWithBowers::Ace,
                    },
                    Card {
                        suit: Suit::Hearts,
                        rank: RankWithBowers::King,
                    },
                    Card {
                        suit: Suit::Hearts,
                        rank: RankWithBowers::Queen,
                    },
                ],
            },
            Card {
                suit: Suit::Spades,
                rank: RankWithBowers::Ten,
            },
            Position::South,
            true,
            Some(Card {
                suit: Suit::Hearts,
                rank: RankWithBowers::Queen,
            }),
            None,
            false,
        );

        test_bidding(
            "Right left, off ace, off king queen, dealer, candidate trump is nine",
            Hand {
                cards: vec![
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Clubs,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Diamonds,
                        rank: RankWithBowers::Ace,
                    },
                    Card {
                        suit: Suit::Hearts,
                        rank: RankWithBowers::King,
                    },
                    Card {
                        suit: Suit::Hearts,
                        rank: RankWithBowers::Queen,
                    },
                ],
            },
            Card {
                suit: Suit::Spades,
                rank: RankWithBowers::Ten,
            },
            Position::South,
            true,
            Some(Card {
                suit: Suit::Hearts,
                rank: RankWithBowers::Queen,
            }),
            None,
            true,
        );

        test_bidding(
            "Right nine, off ace, off ten nine, candidate trump matches but goes to opponents",
            Hand {
                cards: vec![
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Nine,
                    },
                    Card {
                        suit: Suit::Clubs,
                        rank: RankWithBowers::Ace,
                    },
                    Card {
                        suit: Suit::Diamonds,
                        rank: RankWithBowers::Ten,
                    },
                    Card {
                        suit: Suit::Diamonds,
                        rank: RankWithBowers::Nine,
                    },
                ],
            },
            Card {
                suit: Suit::Spades,
                rank: RankWithBowers::Ten,
            },
            Position::West,
            false,
            None,
            None,
            false,
        );

        test_bidding(
            "Right nine, off ace, off ten nine, candidate trump matches and goes to partner",
            Hand {
                cards: vec![
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Nine,
                    },
                    Card {
                        suit: Suit::Clubs,
                        rank: RankWithBowers::Ace,
                    },
                    Card {
                        suit: Suit::Diamonds,
                        rank: RankWithBowers::Ten,
                    },
                    Card {
                        suit: Suit::Diamonds,
                        rank: RankWithBowers::Nine,
                    },
                ],
            },
            Card {
                suit: Suit::Spades,
                rank: RankWithBowers::Ten,
            },
            Position::North,
            true,
            None,
            None,
            false,
        );

        test_bidding(
            "Right nine, off ace, off ace, off nine, candidate trump does not match",
            Hand {
                cards: vec![
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Nine,
                    },
                    Card {
                        suit: Suit::Clubs,
                        rank: RankWithBowers::Ace,
                    },
                    Card {
                        suit: Suit::Hearts,
                        rank: RankWithBowers::Ace,
                    },
                    Card {
                        suit: Suit::Diamonds,
                        rank: RankWithBowers::Nine,
                    },
                ],
            },
            Card {
                suit: Suit::Diamonds,
                rank: RankWithBowers::Ace,
            },
            Position::West,
            false,
            None,
            Some(Suit::Spades),
            false,
        );

        test_bidding(
            "Perfect hand, candidate trump matches",
            Hand {
                cards: vec![
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Ace,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::King,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Queen,
                    },
                    Card {
                        suit: Suit::Clubs,
                        rank: RankWithBowers::Jack,
                    },
                ],
            },
            Card {
                suit: Suit::Spades,
                rank: RankWithBowers::Ten,
            },
            Position::West,
            true,
            None,
            None,
            true,
        );

        test_bidding(
            "Right left ace, off ace king, follow dealer, other suit is better",
            Hand {
                cards: vec![
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Ace,
                    },
                    Card {
                        suit: Suit::Clubs,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Clubs,
                        rank: RankWithBowers::Ace,
                    },
                    Card {
                        suit: Suit::Clubs,
                        rank: RankWithBowers::King,
                    },
                ],
            },
            Card {
                suit: Suit::Spades,
                rank: RankWithBowers::King,
            },
            Position::East,
            false,
            None,
            Some(Suit::Clubs),
            true,
        );

        test_bidding(
            "Right left ace, off ace king, second after dealer, other suit is better",
            Hand {
                cards: vec![
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Ace,
                    },
                    Card {
                        suit: Suit::Clubs,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Clubs,
                        rank: RankWithBowers::Ace,
                    },
                    Card {
                        suit: Suit::Clubs,
                        rank: RankWithBowers::King,
                    },
                ],
            },
            Card {
                suit: Suit::Spades,
                rank: RankWithBowers::King,
            },
            Position::North,
            true,
            None,
            None,
            true,
        );

        test_bidding(
            "Right nine, off ace, off king, off king, candidate trump does not match but makes one of the off kings good",
            Hand {cards:vec![
                Card{suit:Suit::Spades, rank:RankWithBowers::Jack},
                Card{suit:Suit::Spades, rank:RankWithBowers::Nine},
                Card{suit:Suit::Clubs, rank:RankWithBowers::Ace},
                Card{suit:Suit::Diamonds, rank:RankWithBowers::King},
                Card{suit:Suit::Hearts, rank:RankWithBowers::King},
            ]},
            Card{suit:Suit::Diamonds, rank:RankWithBowers::Ace},
            Position::West,
            false,
            None,
            Some(Suit::Spades),
            false,
        );

        test_bidding(
            "King queen ten nine, off nine, candidate trump does not match",
            Hand {
                cards: vec![
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::King,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Queen,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Ten,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Nine,
                    },
                    Card {
                        suit: Suit::Hearts,
                        rank: RankWithBowers::Nine,
                    },
                ],
            },
            Card {
                suit: Suit::Diamonds,
                rank: RankWithBowers::Ace,
            },
            Position::West,
            false,
            None,
            Some(Suit::Spades),
            false,
        );

        test_bidding(
            "Perfect hand, candidate trump matches, dealer",
            Hand {
                cards: vec![
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Clubs,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Ace,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::King,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Queen,
                    },
                ],
            },
            Card {
                suit: Suit::Spades,
                rank: RankWithBowers::Ten,
            },
            Position::South,
            true,
            Some(Card {
                suit: Suit::Spades,
                rank: RankWithBowers::Ten,
            }),
            None,
            true,
        );

        test_bidding(
            "Perfect hand after picking it up, candidate trump matches, dealer",
            Hand {
                cards: vec![
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Clubs,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Ace,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::King,
                    },
                    Card {
                        suit: Suit::Diamonds,
                        rank: RankWithBowers::Ten,
                    },
                ],
            },
            Card {
                suit: Suit::Spades,
                rank: RankWithBowers::Queen,
            },
            Position::South,
            true,
            Some(Card {
                suit: Suit::Diamonds,
                rank: RankWithBowers::Ten,
            }),
            None,
            true,
        );

        test_bidding(
            "Right left ace, off queen, off ten, candidate trump matches, dealer",
            Hand {
                cards: vec![
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Clubs,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Ace,
                    },
                    Card {
                        suit: Suit::Diamonds,
                        rank: RankWithBowers::Queen,
                    },
                    Card {
                        suit: Suit::Hearts,
                        rank: RankWithBowers::Ten,
                    },
                ],
            },
            Card {
                suit: Suit::Spades,
                rank: RankWithBowers::King,
            },
            Position::South,
            true,
            Some(Card {
                suit: Suit::Hearts,
                rank: RankWithBowers::Ten,
            }),
            None,
            false,
        );

        test_bidding(
            "Right left ace, off queen, off ten, candidate trump matches, dealer (other order of offsuits)",
            Hand {cards:vec![
                Card{suit:Suit::Spades, rank:RankWithBowers::Jack},
                Card{suit:Suit::Clubs, rank:RankWithBowers::Jack},
                Card{suit:Suit::Spades, rank:RankWithBowers::Ace},
                Card{suit:Suit::Hearts, rank:RankWithBowers::Queen},
                Card{suit:Suit::Diamonds, rank:RankWithBowers::Ten},
            ]},
            Card{suit:Suit::Spades, rank:RankWithBowers::King},
            Position::South,
            true,
            Some(Card{suit:Suit::Diamonds, rank:RankWithBowers::Ten}),
            None,
            false,
        );

        test_bidding(
            "Right left ace, off king queen, candidate trump matches, dealer",
            Hand {
                cards: vec![
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Clubs,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Ace,
                    },
                    Card {
                        suit: Suit::Hearts,
                        rank: RankWithBowers::King,
                    },
                    Card {
                        suit: Suit::Hearts,
                        rank: RankWithBowers::Queen,
                    },
                ],
            },
            Card {
                suit: Suit::Spades,
                rank: RankWithBowers::King,
            },
            Position::South,
            true,
            Some(Card {
                suit: Suit::Hearts,
                rank: RankWithBowers::Queen,
            }),
            None,
            true,
        );

        test_bidding(
            "Right left ace, off king queen, candidate trump matches, dealer (other order of offsuits)",
            Hand {cards:vec![
                Card{suit:Suit::Spades, rank:RankWithBowers::Jack},
                Card{suit:Suit::Clubs, rank:RankWithBowers::Jack},
                Card{suit:Suit::Spades, rank:RankWithBowers::Ace},
                Card{suit:Suit::Hearts, rank:RankWithBowers::Queen},
                Card{suit:Suit::Hearts, rank:RankWithBowers::King},
            ]},
            Card{suit:Suit::Spades, rank:RankWithBowers::King},
            Position::South,
            true,
            Some(Card{suit:Suit::Hearts, rank:RankWithBowers::Queen}),
            None,
            true,
        );

        test_bidding(
            "Right left ace king, off ace, candidate trump matches, dealer",
            Hand {
                cards: vec![
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Clubs,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Ace,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::King,
                    },
                    Card {
                        suit: Suit::Hearts,
                        rank: RankWithBowers::Ace,
                    },
                ],
            },
            Card {
                suit: Suit::Spades,
                rank: RankWithBowers::Queen,
            },
            Position::South,
            true,
            Some(Card {
                suit: Suit::Hearts,
                rank: RankWithBowers::Ace,
            }),
            None,
            true,
        );

        test_bidding(
            "Right ace king, off king queen, candidate trump is left, dealer",
            Hand {
                cards: vec![
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Jack,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::Ace,
                    },
                    Card {
                        suit: Suit::Spades,
                        rank: RankWithBowers::King,
                    },
                    Card {
                        suit: Suit::Hearts,
                        rank: RankWithBowers::King,
                    },
                    Card {
                        suit: Suit::Hearts,
                        rank: RankWithBowers::Queen,
                    },
                ],
            },
            Card {
                suit: Suit::Clubs,
                rank: RankWithBowers::Jack,
            },
            Position::East,
            false,
            None,
            Some(Suit::Spades),
            true,
        );
    }

    fn test_bidding(
        description: &str,
        hand: Hand,
        trump_candidate: Card,
        dealer: Position,
        orders_up: bool,
        discard: Option<Card>,
        calls_suit: Option<Suit>,
        goes_alone: bool,
    ) -> () {
        let am_dealer = dealer == Position::South;

        assert!(
            discard.is_some() || !orders_up || !am_dealer,
                "{description}: You must specify a discard if things are ordered up and you're the dealer",
        );
        if discard.is_some() && orders_up && am_dealer {
            let mut found = hand.cards.contains(&discard.unwrap());
            if !found && orders_up && am_dealer {
                found = trump_candidate == discard.unwrap();
            }
            assert!(
                found,
                "{description}: Expected discard is neither in hand nor trump candidate"
            );
        }
        if !orders_up {
            assert!(
                calls_suit.is_none() || calls_suit.unwrap() != trump_candidate.suit,
                "{description}: Expected to call trump candidate's suit in round 2"
            );
        }

        let ai: AdvancedPlayer;
        let mut hands = [
            Hand { cards: Vec::new() },
            Hand { cards: Vec::new() },
            hand,
            Hand { cards: Vec::new() },
        ];
        ai = AdvancedPlayer::create(Position::South);
        let mut players = [
            Wrapper::create_single_player(Box::new(PreprogrammedBidder::does_nothing())),
            Wrapper::create_single_player(Box::new(PreprogrammedBidder::does_nothing())),
            Wrapper::create_single_player(Box::new(ai)),
            Wrapper::create_single_player(Box::new(PreprogrammedBidder::does_nothing())),
        ];
        let mut bid = BidState::create(dealer, trump_candidate);
        let bid_result = loop {
            match bid.step(&mut players, &mut hands) {
                Some(bid_result) => {
                    break bid_result;
                }
                None => (),
            }
        };

        if orders_up {
            match bid_result {
                BidResultAll::Called { trump, caller }
                | BidResultAll::CalledAlone { trump, caller } => {
                    assert!(caller == Position::South, "{description}: Wrong caller");
                    assert!(trump == trump_candidate.suit, "{description}: Wrong suit");
                }
                _ => assert!(false, "{description}: Did not order it up"),
            }

            if am_dealer {
                let discard = discard.unwrap();
                assert!(
                    !hands[Position::South.index()].cards.contains(&discard),
                    "{description}: Did not discard {}",
                    discard
                );
            }
        }

        if !orders_up {
            if let Some(called_suit) = calls_suit {
                match bid_result {
                    BidResultAll::Called { trump, caller }
                    | BidResultAll::CalledAlone { trump, caller } => {
                        assert!(caller == Position::South, "{description}: Wrong caller");
                        assert!(trump == called_suit, "{description}: Wrong suit");
                    }
                    _ => assert!(false, "{description}: Did not call a suit"),
                }
            }
        }

        if orders_up || calls_suit.is_some() {
            match bid_result {
                BidResultAll::Called { .. } => {
                    assert!(!goes_alone, "{description}: Did not go alone")
                }
                BidResultAll::CalledAlone { .. } => {
                    assert!(goes_alone, "{description}: Went alone")
                }
                _ => assert!(false, "{description}: Nothing was called"),
            }
        }
    }
}
