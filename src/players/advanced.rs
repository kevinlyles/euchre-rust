use enum_iterator::IntoEnumIterator;

use crate::{
    card::Card, hand::Hand, player::Player, position::Position, rank_with_bowers::RankWithBowers,
    suit::Suit,
};

#[derive(Clone)]
pub(crate) struct AdvancedPlayer {
    position: Position,
    trump_has_been_led: bool,
}

impl AdvancedPlayer {
    pub(crate) fn create(position: Position) -> AdvancedPlayer {
        AdvancedPlayer {
            position,
            trump_has_been_led: false,
        }
    }
}

impl Player for AdvancedPlayer {
    fn should_order_up(
        &mut self,
        hand: &Hand,
        &dealer: &Position,
        &trump_candidate: &Card,
    ) -> bool {
        let to_me = self.position == dealer;
        let to_partner = self.position.partner() == dealer;
        let mut trump_cards = hand.cards.iter().filter(|card| {
            card.suit == trump_candidate.suit
                || card.rank == RankWithBowers::Jack
                    && card.suit == trump_candidate.suit.other_suit_of_same_color()
        });
        match trump_cards.clone().count() + if to_me { 1 } else { 0 } {
            6 | 5 | 4 => true,
            3 if to_me || to_partner => true,
            2 if trump_cards.any(|&card| {
                card.suit == trump_candidate.suit && card.rank == RankWithBowers::Jack
            }) && hand.cards.iter().any(|card| {
                card.rank == RankWithBowers::Ace && card.suit != trump_candidate.suit
            }) =>
            {
                true
            }
            _ => false,
        }
    }

    fn should_order_up_alone(
        &mut self,
        hand: &Hand,
        &dealer: &Position,
        trump_candidate: &Card,
    ) -> bool {
        let to_me = self.position == dealer;
        let trump_cards = hand.cards.iter().filter(|card| {
            card.suit == trump_candidate.suit
                || card.rank == RankWithBowers::Jack
                    && card.suit == trump_candidate.suit.other_suit_of_same_color()
        });
        match trump_cards.count() + if to_me { 1 } else { 0 } {
            6 | 5 => true,
            _ => false,
        }
    }

    fn call_trump(&mut self, hand: &Hand, _dealer: &Position, _turned_down: &Card) -> Option<Suit> {
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

    fn should_defend_alone_ordered(
        &mut self,
        _hand: &Hand,
        _dealer: &Position,
        _trump_candidate: &Card,
    ) -> bool {
        false
    }

    fn should_call_alone(
        &mut self,
        hand: &Hand,
        _dealer: &Position,
        &trump: &Suit,
        _turned_down: &Card,
    ) -> bool {
        let trump_cards = hand.cards.iter().filter(|card| {
            card.suit == trump
                || card.rank == RankWithBowers::Jack
                    && card.suit == trump.other_suit_of_same_color()
        });
        match trump_cards.count() {
            5 => true,
            _ => false,
        }
    }

    fn should_defend_alone_called(
        &mut self,
        _hand: &Hand,
        _dealer: &Position,
        _trump: &Suit,
        _turned_down: &Card,
    ) -> bool {
        false
    }

    fn choose_discard(&mut self, hand: &Hand, trump: &Suit) -> Card {
        let mut suit_counts: [u8; 4] = [0; 4];
        let mut has_ace: [bool; 4] = [false; 4];
        let mut lowest_cards: [Option<Card>; 4] = [None; 4];
        for &card in &hand.cards {
            suit_counts[card.suit as usize] += 1;
            if card.rank == RankWithBowers::Ace {
                has_ace[card.suit as usize] = true;
            }
            match lowest_cards[card.suit as usize] {
                Some(lowest_card) if lowest_card.rank < card.rank => (),
                _ => lowest_cards[card.suit as usize] = Some(card),
            }
        }

        fn get_discard<F>(
            &trump: &Suit,
            lowest_cards: &[Option<Card>; 4],
            filter: F,
        ) -> Option<Card>
        where
            F: Fn(Suit) -> bool,
        {
            let mut lowest_card: Option<Card> = None;

            for suit in Suit::into_enum_iter().filter(|&suit| suit != trump) {
                match lowest_cards[suit as usize] {
                    Some(card) if filter(suit) => match lowest_card {
                        Some(lowest_card) if lowest_card.rank < card.rank => (),
                        _ => lowest_card = Some(card),
                    },
                    _ => (),
                }
            }

            lowest_card
        }

        if let Some(card) = get_discard(trump, &lowest_cards, |suit| {
            suit_counts[suit as usize] == 1 && !has_ace[suit as usize]
        }) {
            card
        } else if let Some(card) = get_discard(trump, &lowest_cards, |suit| !has_ace[suit as usize])
        {
            card
        } else {
            get_discard(trump, &lowest_cards, |_| true).unwrap()
        }
    }

    fn play_card(
        &mut self,
        hand: &Hand,
        &caller: &Position,
        &trump: &Suit,
        led: Option<Suit>,
    ) -> Card {
        match led {
            Some(suit) => match hand.cards.iter().filter(|card| card.suit == suit).nth(0) {
                Some(card) => *card,
                None => hand.cards[0],
            },
            None => {
                if self.position == caller
                    || self.position.partner() == caller && !self.trump_has_been_led
                {
                    match hand.cards.iter().filter(|card| card.suit == trump).nth(0) {
                        Some(&card) => card,
                        None => hand.cards[0],
                    }
                } else {
                    hand.cards[0]
                }
            }
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
