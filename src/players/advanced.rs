use enum_iterator::IntoEnumIterator;

use crate::{
    card::{Card, CardBeforeBidding},
    hand::{Hand, HandBeforeBidding},
    player::Player,
    position::Position,
    rank::Rank,
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
        hand: &HandBeforeBidding,
        &dealer: &Position,
        &trump_candidate: &CardBeforeBidding,
    ) -> bool {
        let to_me = self.position == dealer;
        let to_partner = self.position.partner() == dealer;
        let trump_cards = hand.cards.iter().filter(|card| {
            card.suit == trump_candidate.suit
                || card.rank == Rank::Jack
                    && card.suit == trump_candidate.suit.other_suit_of_same_color()
        });
        let has_right = trump_cards
            .clone()
            .any(|&card| card.suit == trump_candidate.suit && card.rank == Rank::Jack);
        let has_off_ace = hand
            .cards
            .iter()
            .any(|card| card.rank == Rank::Ace && card.suit != trump_candidate.suit);

        let trump_count = trump_cards.count() as i32
            + if to_me {
                1
            } else if !to_partner {
                -1
            } else {
                0
            };
        match trump_count {
            6 | 5 | 4 => true,
            3 if to_partner || has_right => true,
            2 if has_right && has_off_ace => true,
            _ => false,
        }
    }

    fn should_order_up_alone(
        &mut self,
        hand: &HandBeforeBidding,
        &dealer: &Position,
        &trump_candidate: &CardBeforeBidding,
    ) -> bool {
        let to_me = self.position == dealer;
        let mut cards = hand.cards.clone();
        if to_me {
            cards.push(trump_candidate);
            let discard = self.choose_discard(hand, &trump_candidate.suit);
            cards.retain(|&card| card != discard);
        }
        let trump_cards: Vec<&CardBeforeBidding> = cards
            .iter()
            .filter(|card| {
                card.suit == trump_candidate.suit
                    || card.rank == Rank::Jack
                        && card.suit == trump_candidate.suit.other_suit_of_same_color()
            })
            .collect();
        if trump_cards.len() < 3
            || trump_cards
                .iter()
                .filter(|card| card.rank == Rank::Jack || card.rank > Rank::Queen)
                .count()
                < 2
        {
            return false;
        }
        let mut highest_card_in_suit = [None; 4];
        for &card in &cards {
            if card.rank == Rank::Jack && card.suit == trump_candidate.suit {
                highest_card_in_suit[card.suit.index()] = Some(card);
            } else if card.rank == Rank::Jack
                && card.suit.other_suit_of_same_color() == trump_candidate.suit
            {
                match highest_card_in_suit[card.suit.other_suit_of_same_color().index()] {
                    Some(highest_card) if highest_card.rank == Rank::Jack => (),
                    _ => {
                        highest_card_in_suit[card.suit.other_suit_of_same_color().index()] =
                            Some(card)
                    }
                }
            } else if card.suit == trump_candidate.suit {
                match highest_card_in_suit[card.suit.index()] {
                    Some(highest_card)
                        if highest_card.rank == Rank::Jack || highest_card.rank > card.rank =>
                    {
                        ()
                    }
                    _ => highest_card_in_suit[card.suit.index()] = Some(card),
                }
            } else {
                match highest_card_in_suit[card.suit.index()] {
                    Some(highest_card) if highest_card.rank > card.rank => (),
                    _ => highest_card_in_suit[card.suit.index()] = Some(card),
                }
            }
        }
        let mut cards_that_could_beat_my_highest = 0;
        for suit in Suit::into_enum_iter() {
            if suit == trump_candidate.suit {
                match highest_card_in_suit[suit.index()] {
                    Some(card) => match card.rank {
                        Rank::Jack if card.suit != trump_candidate.suit => {
                            cards_that_could_beat_my_highest += 1
                        }
                        Rank::Jack => (),
                        Rank::Ace => cards_that_could_beat_my_highest += 2,
                        Rank::King => cards_that_could_beat_my_highest += 3,
                        Rank::Queen => cards_that_could_beat_my_highest += 4,
                        Rank::Ten => cards_that_could_beat_my_highest += 5,
                        Rank::Nine => cards_that_could_beat_my_highest += 6,
                    },
                    _ => (),
                }
                continue;
            }
            match highest_card_in_suit[suit.index()] {
                Some(card) => match card.rank {
                    Rank::Ace => (),
                    Rank::King => cards_that_could_beat_my_highest += 1,
                    Rank::Queen => cards_that_could_beat_my_highest += 2,
                    Rank::Jack => cards_that_could_beat_my_highest += 3,
                    Rank::Ten => cards_that_could_beat_my_highest += 4,
                    Rank::Nine => cards_that_could_beat_my_highest += 5,
                },
                _ => (),
            }
        }
        return cards_that_could_beat_my_highest <= 2;
    }

    fn call_trump(
        &mut self,
        hand: &HandBeforeBidding,
        _dealer: &Position,
        turned_down: &CardBeforeBidding,
    ) -> Option<Suit> {
        let mut suit_scores = [0; 4];
        for trump_candidate in Suit::into_enum_iter().filter(|&suit| suit != turned_down.suit) {
            let trump_cards = hand.cards.iter().filter(|card| {
                card.suit == trump_candidate
                    || card.rank == Rank::Jack
                        && card.suit == trump_candidate.other_suit_of_same_color()
            });
            let has_right = trump_cards
                .clone()
                .any(|&card| card.suit == trump_candidate && card.rank == Rank::Jack);
            let has_off_ace = hand
                .cards
                .iter()
                .any(|card| card.rank == Rank::Ace && card.suit != trump_candidate);

            let trump_count = trump_cards.count();
            match trump_count {
                5 | 4 => suit_scores[trump_candidate.index()] = trump_count,
                3 if has_right => suit_scores[trump_candidate.index()] = trump_count,
                2 if has_right && has_off_ace => suit_scores[trump_candidate.index()] = trump_count,
                _ => (),
            }
        }
        let mut max_score = 0;
        let mut max_suit = None;
        for suit in Suit::into_enum_iter() {
            if suit_scores[suit.index()] > max_score {
                max_score = suit_scores[suit.index()];
                max_suit = Some(suit);
            }
        }
        max_suit
    }

    fn should_defend_alone_ordered(
        &mut self,
        _hand: &HandBeforeBidding,
        _dealer: &Position,
        _trump_candidate: &CardBeforeBidding,
    ) -> bool {
        false
    }

    fn should_call_alone(
        &mut self,
        hand: &HandBeforeBidding,
        _dealer: &Position,
        &trump: &Suit,
        _turned_down: &CardBeforeBidding,
    ) -> bool {
        let cards = &hand.cards;
        let trump_cards: Vec<&CardBeforeBidding> = cards
            .iter()
            .filter(|card| {
                card.suit == trump
                    || card.rank == Rank::Jack && card.suit == trump.other_suit_of_same_color()
            })
            .collect();
        if trump_cards.len() < 3
            || trump_cards
                .iter()
                .filter(|card| card.rank == Rank::Jack || card.rank > Rank::Queen)
                .count()
                < 2
        {
            return false;
        }
        let mut highest_card_in_suit = [None; 4];
        for &card in cards {
            if card.rank == Rank::Jack && card.suit == trump {
                highest_card_in_suit[card.suit.index()] = Some(card);
            } else if card.rank == Rank::Jack && card.suit.other_suit_of_same_color() == trump {
                match highest_card_in_suit[card.suit.other_suit_of_same_color().index()] {
                    Some(highest_card) if highest_card.rank == Rank::Jack => (),
                    _ => {
                        highest_card_in_suit[card.suit.other_suit_of_same_color().index()] =
                            Some(card)
                    }
                }
            } else if card.suit == trump {
                match highest_card_in_suit[card.suit.index()] {
                    Some(highest_card)
                        if highest_card.rank == Rank::Jack || highest_card.rank > card.rank =>
                    {
                        ()
                    }
                    _ => highest_card_in_suit[card.suit.index()] = Some(card),
                }
            } else {
                match highest_card_in_suit[card.suit.index()] {
                    Some(highest_card) if highest_card.rank > card.rank => (),
                    _ => highest_card_in_suit[card.suit.index()] = Some(card),
                }
            }
        }
        let mut cards_that_could_beat_my_highest = 0;
        for suit in Suit::into_enum_iter() {
            if suit == trump {
                match highest_card_in_suit[suit.index()] {
                    Some(card) => match card.rank {
                        Rank::Jack if card.suit != trump => cards_that_could_beat_my_highest += 1,
                        Rank::Jack => (),
                        Rank::Ace => cards_that_could_beat_my_highest += 2,
                        Rank::King => cards_that_could_beat_my_highest += 3,
                        Rank::Queen => cards_that_could_beat_my_highest += 4,
                        Rank::Ten => cards_that_could_beat_my_highest += 5,
                        Rank::Nine => cards_that_could_beat_my_highest += 6,
                    },
                    _ => (),
                }
                continue;
            }
            match highest_card_in_suit[suit.index()] {
                Some(card) => match card.rank {
                    Rank::Ace => (),
                    Rank::King => cards_that_could_beat_my_highest += 1,
                    Rank::Queen => cards_that_could_beat_my_highest += 2,
                    Rank::Jack => cards_that_could_beat_my_highest += 3,
                    Rank::Ten => cards_that_could_beat_my_highest += 4,
                    Rank::Nine => cards_that_could_beat_my_highest += 5,
                },
                _ => (),
            }
        }
        return cards_that_could_beat_my_highest <= 2;
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

    fn choose_discard(&mut self, hand: &HandBeforeBidding, &trump: &Suit) -> CardBeforeBidding {
        let mut suit_counts: [u8; 4] = [0; 4];
        let mut has_ace: [bool; 4] = [false; 4];
        let mut lowest_cards: [Option<CardBeforeBidding>; 4] = [None; 4];
        for &card in &hand.cards {
            suit_counts[card.suit.index()] += 1;
            if card.rank == Rank::Ace {
                has_ace[card.suit.index()] = true;
            } else if card.rank == Rank::Jack
                && (card.suit == trump || card.suit.other_suit_of_same_color() == trump)
            {
                continue;
            }
            match lowest_cards[card.suit.index()] {
                Some(lowest_card) if lowest_card.rank < card.rank => (),
                _ => lowest_cards[card.suit.index()] = Some(card),
            }
        }

        fn get_discard<F>(
            lowest_cards: &[Option<CardBeforeBidding>; 4],
            filter: F,
        ) -> Option<CardBeforeBidding>
        where
            F: Fn(Suit) -> bool,
        {
            let mut lowest_card: Option<CardBeforeBidding> = None;

            for suit in Suit::into_enum_iter() {
                match lowest_cards[suit.index()] {
                    Some(card) if filter(suit) => match lowest_card {
                        Some(lowest_card) if lowest_card.rank < card.rank => (),
                        _ => lowest_card = Some(card),
                    },
                    _ => (),
                }
            }

            lowest_card
        }

        if let Some(card) = get_discard(&lowest_cards, |suit| {
            suit != trump && suit_counts[suit.index()] == 1 && !has_ace[suit.index()]
        }) {
            card
        } else if let Some(card) = get_discard(&lowest_cards, |suit| {
            suit != trump && !has_ace[suit.index()]
        }) {
            card
        } else if let Some(card) = get_discard(&lowest_cards, |suit| suit != trump) {
            card
        } else {
            get_discard(&lowest_cards, |_| true).unwrap()
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        bid_result::BidResultAll,
        bid_state::BidState,
        players::{preprogrammed_bidder::PreprogrammedBidder, wrapper::Wrapper},
    };
    use test_case::test_case;

    #[test]
    fn test_cases() {
        test_bidding_old(
            "All nines and tens, dealer, candidate trump matches",
            HandBeforeBidding {
                cards: vec![
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Ten,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Nine,
                    },
                    CardBeforeBidding {
                        suit: Suit::Diamonds,
                        rank: Rank::Nine,
                    },
                    CardBeforeBidding {
                        suit: Suit::Clubs,
                        rank: Rank::Nine,
                    },
                    CardBeforeBidding {
                        suit: Suit::Hearts,
                        rank: Rank::Nine,
                    },
                ],
            },
            CardBeforeBidding {
                suit: Suit::Spades,
                rank: Rank::King,
            },
            Position::South,
            false,
            None,
            None,
            false,
        );

        test_bidding_old(
            "Right nine, off ace, off king, off king, candidate trump does not match",
            HandBeforeBidding {
                cards: vec![
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Nine,
                    },
                    CardBeforeBidding {
                        suit: Suit::Diamonds,
                        rank: Rank::Ace,
                    },
                    CardBeforeBidding {
                        suit: Suit::Clubs,
                        rank: Rank::King,
                    },
                    CardBeforeBidding {
                        suit: Suit::Hearts,
                        rank: Rank::King,
                    },
                ],
            },
            CardBeforeBidding {
                suit: Suit::Diamonds,
                rank: Rank::King,
            },
            Position::West,
            false,
            None,
            None,
            false,
        );

        test_bidding_old(
            "Right, off ace, off jack ten nine, dealer, candidate trump matches",
            HandBeforeBidding {
                cards: vec![
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Clubs,
                        rank: Rank::Ace,
                    },
                    CardBeforeBidding {
                        suit: Suit::Diamonds,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Diamonds,
                        rank: Rank::Ten,
                    },
                    CardBeforeBidding {
                        suit: Suit::Diamonds,
                        rank: Rank::Nine,
                    },
                ],
            },
            CardBeforeBidding {
                suit: Suit::Spades,
                rank: Rank::Nine,
            },
            Position::South,
            true,
            Some(CardBeforeBidding {
                suit: Suit::Diamonds,
                rank: Rank::Nine,
            }),
            None,
            false,
        );

        test_bidding_old(
            "Right nine, off ace, off ten nine, dealer, candidate trump matches",
            HandBeforeBidding {
                cards: vec![
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Nine,
                    },
                    CardBeforeBidding {
                        suit: Suit::Clubs,
                        rank: Rank::Ace,
                    },
                    CardBeforeBidding {
                        suit: Suit::Diamonds,
                        rank: Rank::Ten,
                    },
                    CardBeforeBidding {
                        suit: Suit::Diamonds,
                        rank: Rank::Nine,
                    },
                ],
            },
            CardBeforeBidding {
                suit: Suit::Spades,
                rank: Rank::Ten,
            },
            Position::South,
            true,
            Some(CardBeforeBidding {
                suit: Suit::Diamonds,
                rank: Rank::Nine,
            }),
            None,
            false,
        );

        test_bidding_old(
            "Right, off ace, off jack ten nine, dealer, candidate trump matches",
            HandBeforeBidding {
                cards: vec![
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Clubs,
                        rank: Rank::Ace,
                    },
                    CardBeforeBidding {
                        suit: Suit::Diamonds,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Diamonds,
                        rank: Rank::Ten,
                    },
                    CardBeforeBidding {
                        suit: Suit::Diamonds,
                        rank: Rank::Nine,
                    },
                ],
            },
            CardBeforeBidding {
                suit: Suit::Spades,
                rank: Rank::Nine,
            },
            Position::South,
            true,
            Some(CardBeforeBidding {
                suit: Suit::Diamonds,
                rank: Rank::Nine,
            }),
            None,
            false,
        );

        test_bidding_old(
            "Perfect hand, candidate trump does not match",
            HandBeforeBidding {
                cards: vec![
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Clubs,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Ace,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::King,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Queen,
                    },
                ],
            },
            CardBeforeBidding {
                suit: Suit::Diamonds,
                rank: Rank::Ace,
            },
            Position::West,
            false,
            None,
            Some(Suit::Spades),
            true,
        );

        test_bidding_old(
            "Right king queen, two off queens, candidate trump does not match",
            HandBeforeBidding {
                cards: vec![
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::King,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Queen,
                    },
                    CardBeforeBidding {
                        suit: Suit::Hearts,
                        rank: Rank::Queen,
                    },
                    CardBeforeBidding {
                        suit: Suit::Diamonds,
                        rank: Rank::Queen,
                    },
                ],
            },
            CardBeforeBidding {
                suit: Suit::Diamonds,
                rank: Rank::Ace,
            },
            Position::West,
            false,
            None,
            Some(Suit::Spades),
            false,
        );

        test_bidding_old(
            "Right left king, off king nine, candidate trump does not match",
            HandBeforeBidding {
                cards: vec![
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::King,
                    },
                    CardBeforeBidding {
                        suit: Suit::Clubs,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Hearts,
                        rank: Rank::King,
                    },
                    CardBeforeBidding {
                        suit: Suit::Hearts,
                        rank: Rank::Nine,
                    },
                ],
            },
            CardBeforeBidding {
                suit: Suit::Diamonds,
                rank: Rank::Ace,
            },
            Position::West,
            false,
            None,
            Some(Suit::Spades),
            true,
        );

        test_bidding_old(
            "Right nine, off ace, off ten nine, candidate trump does not match",
            HandBeforeBidding {
                cards: vec![
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Nine,
                    },
                    CardBeforeBidding {
                        suit: Suit::Diamonds,
                        rank: Rank::Ace,
                    },
                    CardBeforeBidding {
                        suit: Suit::Hearts,
                        rank: Rank::Ten,
                    },
                    CardBeforeBidding {
                        suit: Suit::Hearts,
                        rank: Rank::Nine,
                    },
                ],
            },
            CardBeforeBidding {
                suit: Suit::Diamonds,
                rank: Rank::King,
            },
            Position::West,
            false,
            None,
            Some(Suit::Spades),
            false,
        );

        test_bidding_old(
            "Right nine, off ace, off king queen, dealer, candidate trump is ten",
            HandBeforeBidding {
                cards: vec![
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Nine,
                    },
                    CardBeforeBidding {
                        suit: Suit::Diamonds,
                        rank: Rank::Ace,
                    },
                    CardBeforeBidding {
                        suit: Suit::Hearts,
                        rank: Rank::King,
                    },
                    CardBeforeBidding {
                        suit: Suit::Hearts,
                        rank: Rank::Queen,
                    },
                ],
            },
            CardBeforeBidding {
                suit: Suit::Spades,
                rank: Rank::Ten,
            },
            Position::South,
            true,
            Some(CardBeforeBidding {
                suit: Suit::Hearts,
                rank: Rank::Queen,
            }),
            None,
            false,
        );

        test_bidding_old(
            "Right left, off ace, off king queen, dealer, candidate trump is nine",
            HandBeforeBidding {
                cards: vec![
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Clubs,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Diamonds,
                        rank: Rank::Ace,
                    },
                    CardBeforeBidding {
                        suit: Suit::Hearts,
                        rank: Rank::King,
                    },
                    CardBeforeBidding {
                        suit: Suit::Hearts,
                        rank: Rank::Queen,
                    },
                ],
            },
            CardBeforeBidding {
                suit: Suit::Spades,
                rank: Rank::Ten,
            },
            Position::South,
            true,
            Some(CardBeforeBidding {
                suit: Suit::Hearts,
                rank: Rank::Queen,
            }),
            None,
            true,
        );

        test_bidding_old(
            "Right nine, off ace, off ten nine, candidate trump matches but goes to opponents",
            HandBeforeBidding {
                cards: vec![
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Nine,
                    },
                    CardBeforeBidding {
                        suit: Suit::Clubs,
                        rank: Rank::Ace,
                    },
                    CardBeforeBidding {
                        suit: Suit::Diamonds,
                        rank: Rank::Ten,
                    },
                    CardBeforeBidding {
                        suit: Suit::Diamonds,
                        rank: Rank::Nine,
                    },
                ],
            },
            CardBeforeBidding {
                suit: Suit::Spades,
                rank: Rank::Ten,
            },
            Position::West,
            false,
            None,
            None,
            false,
        );

        test_bidding_old(
            "Right nine, off ace, off ten nine, candidate trump matches and goes to partner",
            HandBeforeBidding {
                cards: vec![
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Nine,
                    },
                    CardBeforeBidding {
                        suit: Suit::Clubs,
                        rank: Rank::Ace,
                    },
                    CardBeforeBidding {
                        suit: Suit::Diamonds,
                        rank: Rank::Ten,
                    },
                    CardBeforeBidding {
                        suit: Suit::Diamonds,
                        rank: Rank::Nine,
                    },
                ],
            },
            CardBeforeBidding {
                suit: Suit::Spades,
                rank: Rank::Ten,
            },
            Position::North,
            true,
            None,
            None,
            false,
        );

        test_bidding_old(
            "Right nine, off ace, off ace, off nine, candidate trump does not match",
            HandBeforeBidding {
                cards: vec![
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Nine,
                    },
                    CardBeforeBidding {
                        suit: Suit::Clubs,
                        rank: Rank::Ace,
                    },
                    CardBeforeBidding {
                        suit: Suit::Hearts,
                        rank: Rank::Ace,
                    },
                    CardBeforeBidding {
                        suit: Suit::Diamonds,
                        rank: Rank::Nine,
                    },
                ],
            },
            CardBeforeBidding {
                suit: Suit::Diamonds,
                rank: Rank::Ace,
            },
            Position::West,
            false,
            None,
            Some(Suit::Spades),
            false,
        );

        test_bidding_old(
            "Perfect hand, candidate trump matches",
            HandBeforeBidding {
                cards: vec![
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Ace,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::King,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Queen,
                    },
                    CardBeforeBidding {
                        suit: Suit::Clubs,
                        rank: Rank::Jack,
                    },
                ],
            },
            CardBeforeBidding {
                suit: Suit::Spades,
                rank: Rank::Ten,
            },
            Position::West,
            true,
            None,
            None,
            true,
        );

        test_bidding_old(
            "Right left ace, off ace king, follow dealer, other suit is better",
            HandBeforeBidding {
                cards: vec![
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Ace,
                    },
                    CardBeforeBidding {
                        suit: Suit::Clubs,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Clubs,
                        rank: Rank::Ace,
                    },
                    CardBeforeBidding {
                        suit: Suit::Clubs,
                        rank: Rank::King,
                    },
                ],
            },
            CardBeforeBidding {
                suit: Suit::Spades,
                rank: Rank::King,
            },
            Position::East,
            false,
            None,
            Some(Suit::Clubs),
            true,
        );

        test_bidding_old(
            "Right left ace, off ace king, second after dealer, other suit is better",
            HandBeforeBidding {
                cards: vec![
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Ace,
                    },
                    CardBeforeBidding {
                        suit: Suit::Clubs,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Clubs,
                        rank: Rank::Ace,
                    },
                    CardBeforeBidding {
                        suit: Suit::Clubs,
                        rank: Rank::King,
                    },
                ],
            },
            CardBeforeBidding {
                suit: Suit::Spades,
                rank: Rank::King,
            },
            Position::North,
            true,
            None,
            None,
            true,
        );

        test_bidding_old(
            "Right nine, off ace, off king, off king, candidate trump does not match but makes one of the off kings good",
            HandBeforeBidding {cards:vec![
                CardBeforeBidding{suit:Suit::Spades, rank:Rank::Jack},
                CardBeforeBidding{suit:Suit::Spades, rank:Rank::Nine},
                CardBeforeBidding{suit:Suit::Clubs, rank:Rank::Ace},
                CardBeforeBidding{suit:Suit::Diamonds, rank:Rank::King},
                CardBeforeBidding{suit:Suit::Hearts, rank:Rank::King},
            ]},
            CardBeforeBidding{suit:Suit::Diamonds, rank:Rank::Ace},
            Position::West,
            false,
            None,
            Some(Suit::Spades),
            false,
        );

        test_bidding_old(
            "King queen ten nine, off nine, candidate trump does not match",
            HandBeforeBidding {
                cards: vec![
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::King,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Queen,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Ten,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Nine,
                    },
                    CardBeforeBidding {
                        suit: Suit::Hearts,
                        rank: Rank::Nine,
                    },
                ],
            },
            CardBeforeBidding {
                suit: Suit::Diamonds,
                rank: Rank::Ace,
            },
            Position::West,
            false,
            None,
            Some(Suit::Spades),
            false,
        );

        test_bidding_old(
            "Perfect hand, candidate trump matches, dealer",
            HandBeforeBidding {
                cards: vec![
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Clubs,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Ace,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::King,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Queen,
                    },
                ],
            },
            CardBeforeBidding {
                suit: Suit::Spades,
                rank: Rank::Ten,
            },
            Position::South,
            true,
            Some(CardBeforeBidding {
                suit: Suit::Spades,
                rank: Rank::Ten,
            }),
            None,
            true,
        );

        test_bidding_old(
            "Perfect hand after picking it up, candidate trump matches, dealer",
            HandBeforeBidding {
                cards: vec![
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Clubs,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Ace,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::King,
                    },
                    CardBeforeBidding {
                        suit: Suit::Diamonds,
                        rank: Rank::Ten,
                    },
                ],
            },
            CardBeforeBidding {
                suit: Suit::Spades,
                rank: Rank::Queen,
            },
            Position::South,
            true,
            Some(CardBeforeBidding {
                suit: Suit::Diamonds,
                rank: Rank::Ten,
            }),
            None,
            true,
        );

        test_bidding_old(
            "Right left ace, off queen, off ten, candidate trump matches, dealer",
            HandBeforeBidding {
                cards: vec![
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Clubs,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Ace,
                    },
                    CardBeforeBidding {
                        suit: Suit::Diamonds,
                        rank: Rank::Queen,
                    },
                    CardBeforeBidding {
                        suit: Suit::Hearts,
                        rank: Rank::Ten,
                    },
                ],
            },
            CardBeforeBidding {
                suit: Suit::Spades,
                rank: Rank::King,
            },
            Position::South,
            true,
            Some(CardBeforeBidding {
                suit: Suit::Hearts,
                rank: Rank::Ten,
            }),
            None,
            false,
        );

        test_bidding_old(
            "Right left ace, off queen, off ten, candidate trump matches, dealer (other order of offsuits)",
            HandBeforeBidding {cards:vec![
                CardBeforeBidding{suit:Suit::Spades, rank:Rank::Jack},
                CardBeforeBidding{suit:Suit::Clubs, rank:Rank::Jack},
                CardBeforeBidding{suit:Suit::Spades, rank:Rank::Ace},
                CardBeforeBidding{suit:Suit::Hearts, rank:Rank::Queen},
                CardBeforeBidding{suit:Suit::Diamonds, rank:Rank::Ten},
            ]},
            CardBeforeBidding{suit:Suit::Spades, rank:Rank::King},
            Position::South,
            true,
            Some(CardBeforeBidding{suit:Suit::Diamonds, rank:Rank::Ten}),
            None,
            false,
        );

        test_bidding_old(
            "Right left ace, off king queen, candidate trump matches, dealer",
            HandBeforeBidding {
                cards: vec![
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Clubs,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Ace,
                    },
                    CardBeforeBidding {
                        suit: Suit::Hearts,
                        rank: Rank::King,
                    },
                    CardBeforeBidding {
                        suit: Suit::Hearts,
                        rank: Rank::Queen,
                    },
                ],
            },
            CardBeforeBidding {
                suit: Suit::Spades,
                rank: Rank::King,
            },
            Position::South,
            true,
            Some(CardBeforeBidding {
                suit: Suit::Hearts,
                rank: Rank::Queen,
            }),
            None,
            true,
        );

        test_bidding_old(
            "Right left ace, off king queen, candidate trump matches, dealer (other order of offsuits)",
            HandBeforeBidding {cards:vec![
                CardBeforeBidding{suit:Suit::Spades, rank:Rank::Jack},
                CardBeforeBidding{suit:Suit::Clubs, rank:Rank::Jack},
                CardBeforeBidding{suit:Suit::Spades, rank:Rank::Ace},
                CardBeforeBidding{suit:Suit::Hearts, rank:Rank::Queen},
                CardBeforeBidding{suit:Suit::Hearts, rank:Rank::King},
            ]},
            CardBeforeBidding{suit:Suit::Spades, rank:Rank::King},
            Position::South,
            true,
            Some(CardBeforeBidding{suit:Suit::Hearts, rank:Rank::Queen}),
            None,
            true,
        );

        test_bidding_old(
            "Right left ace king, off ace, candidate trump matches, dealer",
            HandBeforeBidding {
                cards: vec![
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Clubs,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Ace,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::King,
                    },
                    CardBeforeBidding {
                        suit: Suit::Hearts,
                        rank: Rank::Ace,
                    },
                ],
            },
            CardBeforeBidding {
                suit: Suit::Spades,
                rank: Rank::Queen,
            },
            Position::South,
            true,
            Some(CardBeforeBidding {
                suit: Suit::Hearts,
                rank: Rank::Ace,
            }),
            None,
            true,
        );

        test_bidding_old(
            "Right ace king, off king queen, candidate trump is left, dealer",
            HandBeforeBidding {
                cards: vec![
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Jack,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::Ace,
                    },
                    CardBeforeBidding {
                        suit: Suit::Spades,
                        rank: Rank::King,
                    },
                    CardBeforeBidding {
                        suit: Suit::Hearts,
                        rank: Rank::King,
                    },
                    CardBeforeBidding {
                        suit: Suit::Hearts,
                        rank: Rank::Queen,
                    },
                ],
            },
            CardBeforeBidding {
                suit: Suit::Clubs,
                rank: Rank::Jack,
            },
            Position::East,
            false,
            None,
            Some(Suit::Spades),
            true,
        );
    }

    #[test_case(["TS", "NS", "ND", "NC", "NH"], "KS", Position::South, BidResultAll::NoOneCalled, None)]
    #[test_case(["JS", "NS", "AD", "KC", "KH"], "KD", Position::West, BidResultAll::NoOneCalled, None)]
    #[test_case(["JS", "AC", "JD", "TD", "ND"], "NS", Position::South, BidResultAll::called("S"), Some("ND"))]
    #[test_case(["JS", "NS", "AC", "TD", "ND"], "TS", Position::South, BidResultAll::called("S"), Some("ND"))]
    #[test_case(["JS", "JC", "AS", "KS", "QS"], "AD", Position::West, BidResultAll::alone("S"), None)]
    #[test_case(["JS", "KS", "QS", "QH", "QD"], "AD", Position::West, BidResultAll::called("S"), None)]
    #[test_case(["JS", "NS", "AD", "TH", "NH"], "KD", Position::West, BidResultAll::called("S"), None)]
    #[test_case(["JS", "NS", "AC", "KH", "QH"], "TS", Position::South, BidResultAll::called("S"), Some("QH"))]
    #[test_case(["JS", "JC", "AD", "KH", "QH"], "TS", Position::South, BidResultAll::called("S"), Some("QH"))]
    #[test_case(["JS", "NS", "AC", "TD", "ND"], "TS", Position::West, BidResultAll::NoOneCalled, None)]
    #[test_case(["JS", "NS", "AC", "TD", "ND"], "TS", Position::North, BidResultAll::called("S"), None)]
    #[test_case(["JS", "NS", "AC", "AH", "ND"], "AD", Position::West, BidResultAll::called("S"), None)]
    #[test_case(["JS", "AS", "KS", "QS", "JC"], "TS", Position::West, BidResultAll::alone("S"), None)]
    #[test_case(["JS", "AS", "JC", "AC", "KC"], "KS", Position::East, BidResultAll::alone("C"), None)]
    #[test_case(["JS", "AS", "JC", "AC", "KC"], "KS", Position::North, BidResultAll::called("S"), None)]
    #[test_case(["JS", "NS", "AC", "KD", "KH"], "AD", Position::West, BidResultAll::called("S"), None)]
    #[test_case(["KS", "QS", "TS", "NS", "NH"], "AD", Position::West, BidResultAll::called("S"), None)]
    #[test_case(["JS", "JC", "AS", "KS", "QS"], "TS", Position::South, BidResultAll::alone("S"), Some("TS"))]
    #[test_case(["JS", "JC", "AS", "KS", "TD"], "QS", Position::South, BidResultAll::alone("S"), Some("TD"))]
    #[test_case(["JS", "JC", "AS", "QD", "TH"], "KS", Position::South, BidResultAll::called("S"), Some("TH"))]
    #[test_case(["JS", "JC", "AS", "QH", "TD"], "KS", Position::South, BidResultAll::called("S"), Some("TD"))]
    #[test_case(["JS", "JC", "AS", "KH", "QH"], "KS", Position::South, BidResultAll::alone("S"), Some("QH"))]
    #[test_case(["JS", "JC", "AS", "QH", "KH"], "KS", Position::South, BidResultAll::alone("S"), Some("QH"))]
    #[test_case(["JS", "JC", "AS", "KS", "AH"], "QS", Position::South, BidResultAll::alone("S"), Some("AH"))]
    #[test_case(["JS", "AS", "KS", "KH", "QH"], "JC", Position::East, BidResultAll::alone("S"), None)]
    //TODO: test discarding a higher card if it drops a suit
    fn test_bidding(
        hand: [&str; 5],
        trump_candidate: &str,
        dealer: Position,
        expected_bid_result: BidResultAll,
        discard: Option<&str>,
    ) -> () {
        let hand = HandBeforeBidding {
            cards: hand
                .iter()
                .map(|&card| CardBeforeBidding::try_create(card).unwrap())
                .collect(),
        };
        let trump_candidate = CardBeforeBidding::try_create(trump_candidate).unwrap();
        let expected_hand = match discard {
            Some(discard) => {
                let discard = CardBeforeBidding::try_create(discard).unwrap();
                assert!(
                    hand.cards.contains(&discard) || trump_candidate == discard,
                    "Invalid discard"
                );
                HandBeforeBidding {
                    cards: hand
                        .cards
                        .clone()
                        .into_iter()
                        .chain(std::iter::once(trump_candidate))
                        .filter(|&card| card != discard)
                        .collect(),
                }
            }
            None => {
                match expected_bid_result {
                    BidResultAll::Called { trump, .. }
                    | BidResultAll::CalledAlone { trump, .. }
                    | BidResultAll::DefendedAlone { trump, .. } => assert!(
                        dealer != Position::South || trump != trump_candidate.suit,
                        "Discard required"
                    ),
                    BidResultAll::NoOneCalled => (),
                }
                hand.clone()
            }
        };
        assert_eq!(
            5,
            expected_hand.cards.len(),
            "Discard didn't work correctly",
        );
        let mut hands = [
            HandBeforeBidding { cards: Vec::new() },
            HandBeforeBidding { cards: Vec::new() },
            HandBeforeBidding { cards: Vec::new() },
            HandBeforeBidding { cards: Vec::new() },
        ];
        hands[Position::South.index()] = hand;
        let mut players = [
            Wrapper::create_single_player(Box::new(PreprogrammedBidder::does_nothing())),
            Wrapper::create_single_player(Box::new(PreprogrammedBidder::does_nothing())),
            Wrapper::create_single_player(Box::new(PreprogrammedBidder::does_nothing())),
            Wrapper::create_single_player(Box::new(PreprogrammedBidder::does_nothing())),
        ];
        players[Position::South.index()] =
            Wrapper::create_single_player(Box::new(AdvancedPlayer::create(Position::South)));
        let mut bid = BidState::create(dealer, trump_candidate);
        let bid_result = loop {
            match bid.step(&mut players, &mut hands) {
                Some(bid_result) => {
                    break bid_result;
                }
                None => (),
            }
        };
        assert_eq!(expected_bid_result, bid_result, "Incorrect bid result");
        assert_eq!(
            expected_hand,
            hands[Position::South.index()],
            "Incorrect discard: expected {:?}",
            discard,
        );
    }

    fn test_bidding_old(
        description: &str,
        hand: HandBeforeBidding,
        trump_candidate: CardBeforeBidding,
        dealer: Position,
        orders_up: bool,
        discard: Option<CardBeforeBidding>,
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
                found = trump_candidate == discard.unwrap().into();
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
            HandBeforeBidding { cards: Vec::new() },
            HandBeforeBidding { cards: Vec::new() },
            hand,
            HandBeforeBidding { cards: Vec::new() },
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
