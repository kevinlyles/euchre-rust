use enum_iterator::IntoEnumIterator;

use crate::{
    bid_result::BidResultCalled,
    card::{Card, CardBeforeBidding},
    hand::{Hand, HandBeforeBidding},
    player::Player,
    position::Position,
    rank::Rank,
    rank_with_bowers::RankWithBowers,
    suit::Suit,
    trick_state::PlayedCard,
};

#[derive(Clone)]
pub(crate) struct AdvancedPlayer {
    position: Position,
    trump_has_been_led: bool,
    is_definitely_out_of_trump: [bool; 4],
    trump_played: [bool; RankWithBowers::RightBower as usize + 1],
}

impl AdvancedPlayer {
    pub(crate) fn create(position: Position) -> AdvancedPlayer {
        AdvancedPlayer {
            position,
            trump_has_been_led: false,
            is_definitely_out_of_trump: [false; 4],
            trump_played: [false; RankWithBowers::RightBower as usize + 1],
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
        if dealer.next_position_bidding() == self.position {
            if let Some(suit) = self.call_trump(hand, &dealer, &trump_candidate) {
                if suit == trump_candidate.suit.other_suit_of_same_color() {
                    return false;
                }
            }
        }

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
            let mut suits: Vec<Suit> = hand
                .cards
                .iter()
                .map(|card| {
                    if card.rank == Rank::Jack
                        && card.suit == trump_candidate.other_suit_of_same_color()
                    {
                        trump_candidate
                    } else {
                        card.suit
                    }
                })
                .collect();
            suits.sort();
            suits.dedup();
            let suit_count = suits.len();
            match trump_count {
                5 | 4 => suit_scores[trump_candidate.index()] = trump_count,
                3 if has_right => suit_scores[trump_candidate.index()] = trump_count,
                2 if has_right && has_off_ace && suit_count < 4 => {
                    suit_scores[trump_candidate.index()] = trump_count
                }
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
        bid_result: &BidResultCalled,
        cards_played: &Vec<PlayedCard>,
    ) -> Card {
        let caller = bid_result.caller();
        let trump = bid_result.trump();
        match cards_played.first() {
            Some(first) => match hand
                .cards
                .iter()
                .filter(|card| card.suit == first.card.suit)
                .nth(0)
            {
                Some(&card) => card,
                None => hand.cards[0],
            },
            None => {
                if (self.position == caller || self.position.partner() == caller)
                    && !self.trump_has_been_led
                {
                    let my_trump: Vec<&Card> = hand
                        .cards
                        .iter()
                        .filter(|card| card.suit == trump)
                        .collect();

                    match my_trump.iter().max_by_key(|card| card.rank) {
                        Some(&&card) if card.rank >= RankWithBowers::Ace => card,
                        Some(_) => **(my_trump.iter().min_by_key(|card| card.rank).unwrap()),
                        None => hand.cards[0],
                    }
                } else {
                    hand.cards[0]
                }
            }
        }
    }

    fn trick_end(&mut self, bid_result: &BidResultCalled, cards_played: &Vec<PlayedCard>) -> () {
        let trump = bid_result.trump();

        for played_card in cards_played {
            if played_card.card.suit == trump {
                self.trump_played[played_card.card.rank as usize] = true;
            }
        }

        match cards_played[0] {
            played_card if played_card.card.suit == trump => {
                self.trump_has_been_led = true;
                for played_card in cards_played {
                    if played_card.card.suit != trump {
                        self.is_definitely_out_of_trump[played_card.player.index()] = true;
                    }
                }
            }
            _ => (),
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

    #[test_case(["TS", "NS", "ND", "NC", "NH"], "KS", Position::South,
        BidResultAll::NoOneCalled, None ; "All nines and tens, dealer, trump matches, pass")]
    #[test_case(["JS", "NS", "AD", "KC", "KH"], "KD", Position::West,
        BidResultAll::NoOneCalled, None ; "Right nine off ace 4 suited, pass")]
    #[test_case(["JS", "AC", "JD", "TD", "ND"], "NS", Position::South,
        BidResultAll::called("S"), Some("ND") ; "Right off ace 3 suited, pick up the nine")]
    #[test_case(["JS", "NS", "AC", "TD", "ND"], "TS", Position::South,
        BidResultAll::called("S"), Some("ND") ; "Right nine off ace 3 suited, pick up the ten")]
    #[test_case(["JS", "JC", "AS", "KS", "QS"], "AD", Position::West,
        BidResultAll::alone("S"), None ; "Perfect hand, call alone")]
    #[test_case(["JS", "KS", "QS", "QH", "QD"], "AD", Position::West,
        BidResultAll::called("S"), None ; "Right king queen 3 suited, call")]
    #[test_case(["JS", "KS", "JC", "KH", "NH"], "AD", Position::West,
        BidResultAll::alone("S"), None ; "Right left king, off king nine, call alone")]
    #[test_case(["JS", "NS", "AD", "TH", "NH"], "KD", Position::West,
        BidResultAll::called("S"), None ; "Right nine off ace 3 suited, call")]
    #[test_case(["JS", "NS", "AD", "KH", "QH"], "TS", Position::South,
        BidResultAll::called("S"), Some("QH") ; "Right nine off ace 3 suited, pick up the ten (not alone)")]
    #[test_case(["JS", "JC", "AD", "KH", "QH"], "TS", Position::South,
        BidResultAll::alone("S"), Some("QH") ; "Right left off ace off king queen, pick up the ten alone")]
    #[test_case(["JS", "NS", "AC", "TD", "ND"], "TS", Position::West,
        BidResultAll::NoOneCalled, None ; "Right nine off ace 3 suited, do not order to opponent")]
    #[test_case(["JS", "NS", "AC", "TD", "ND"], "TS", Position::North,
        BidResultAll::called("S"), None ; "Right nine off ace 3 suited, order to partner")]
    #[test_case(["JS", "NS", "AC", "AH", "ND"], "AD", Position::West,
        BidResultAll::NoOneCalled, None ; "Right nine 2 off aces 4 suited, pass")]
    #[test_case(["JS", "AS", "KS", "QS", "JC"], "TS", Position::West,
        BidResultAll::alone("S"), None ; "Perfect hand, candidate trump matches")]
    #[test_case(["JS", "AS", "JC", "AC", "KC"], "KS", Position::East,
        BidResultAll::alone("C"), None ; "Right left ace off ace king, follow dealer, wait for next (better) suit")]
    #[test_case(["JS", "AS", "JC", "AC", "KC"], "KS", Position::North,
        BidResultAll::alone("S"), None ; "Right left ace off ace king, opposite dealer, order up alone")]
    #[test_case(["JS", "NS", "AC", "KD", "KH"], "AD", Position::West,
        BidResultAll::NoOneCalled, None ; "Right nine off ace 4 suited, trump makes off king good, passes")]
    #[test_case(["KS", "QS", "TS", "NS", "NH"], "AD", Position::West,
        BidResultAll::called("S"), None ; "Bottom four trump off nine, call")]
    #[test_case(["JS", "JC", "AS", "KS", "QS"], "TS", Position::South,
        BidResultAll::alone("S"), Some("TS") ; "Perfect hand, pick up and discard ten alone")]
    #[test_case(["JS", "JC", "AS", "KS", "TD"], "QS", Position::South,
        BidResultAll::alone("S"), Some("TD") ; "Perfect hand after picking up, pick up alone")]
    #[test_case(["JS", "JC", "AS", "QD", "TH"], "KS", Position::South,
        BidResultAll::alone("S"), Some("TH") ; "Right left ace off queen 3 suited, pick up the king alone")]
    #[test_case(["JS", "JC", "AS", "TD", "QH"], "KS", Position::South,
        BidResultAll::alone("S"), Some("TD") ; "Right left ace off queen 3 suited, pick up the king alone (other order)")]
    #[test_case(["JS", "JC", "AS", "KH", "QH"], "KS", Position::South,
        BidResultAll::alone("S"), Some("QH") ; "Right left ace off king queen, pick up the king alone")]
    #[test_case(["JS", "JC", "AS", "QH", "KH"], "KS", Position::South,
        BidResultAll::alone("S"), Some("QH") ; "Right left ace off king queen, pick up the king alone (other order)")]
    #[test_case(["JS", "JC", "AS", "KS", "AH"], "QS", Position::South,
        BidResultAll::alone("S"), Some("AH") ; "Right left ace king off ace, pick up the queen alone")]
    #[test_case(["JS", "AS", "KS", "KH", "QH"], "JC", Position::East,
        BidResultAll::alone("S"), None ; "Right ace king, off king queen, call alone")]
    //TODO: test right nine 3 off aces
    //TODO: test not waiting for the next suit if the current one is better
    //TODO: better test logic around turned down card making things good
    //TODO: test discarding a higher card if it drops a suit
    //TODO: #[test_case(["JS", "NS", "AC", "KD", "KH"], "AD", Position::West,
    //    BidResultAll::called("S"), None ; "Right nine 2 off aces 4 suited, trump makes off king good, calls")]
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
}
