use crate::{
    bid_result::BidResultAll, card::CardBeforeBidding, hand::HandBeforeBidding, player::Player,
    position::Position,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct BidState {
    pub(crate) dealer: Position,
    pub(crate) phase: BidPhase,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum BidPhase {
    FirstRoundFirstPlayer {
        trump_candidate: CardBeforeBidding,
    },
    FirstRoundSecondPlayer {
        trump_candidate: CardBeforeBidding,
    },
    FirstRoundThirdPlayer {
        trump_candidate: CardBeforeBidding,
    },
    FirstRoundFourthPlayer {
        trump_candidate: CardBeforeBidding,
    },
    OrderedUp {
        caller: Position,
        card_ordered: CardBeforeBidding,
    },
    OrderedUpAlone {
        caller: Position,
        card_ordered: CardBeforeBidding,
    },
    OrderedUpDefendedAlone {
        caller: Position,
        card_ordered: CardBeforeBidding,
        defender: Position,
    },
    SecondRoundFirstPlayer {
        turned_down: CardBeforeBidding,
    },
    SecondRoundSecondPlayer {
        turned_down: CardBeforeBidding,
    },
    SecondRoundThirdPlayer {
        turned_down: CardBeforeBidding,
    },
    SecondRoundFourthPlayer {
        turned_down: CardBeforeBidding,
    },
    Done {
        bid_result: BidResultAll,
    },
}

impl BidState {
    pub(crate) fn create(dealer: Position, trump_candidate: CardBeforeBidding) -> BidState {
        BidState {
            dealer,
            phase: BidPhase::FirstRoundFirstPlayer { trump_candidate },
        }
    }

    pub(crate) fn step(
        &mut self,
        players: &mut [impl Player; 4],
        hands: &mut [HandBeforeBidding; 4],
    ) -> Option<BidResultAll> {
        match &mut self.phase {
            BidPhase::FirstRoundFirstPlayer {
                ref trump_candidate,
            } => {
                let bidder = self.dealer.next_position_bidding();
                self.phase =
                    match BidState::order_up(&self.dealer, bidder, trump_candidate, players, hands)
                    {
                        Some(phase) => phase,
                        None => BidPhase::FirstRoundSecondPlayer {
                            trump_candidate: *trump_candidate,
                        },
                    };
                None
            }
            BidPhase::FirstRoundSecondPlayer { trump_candidate } => {
                let bidder = self.dealer.partner();
                self.phase =
                    match BidState::order_up(&self.dealer, bidder, trump_candidate, players, hands)
                    {
                        Some(phase) => phase,
                        None => BidPhase::FirstRoundThirdPlayer {
                            trump_candidate: *trump_candidate,
                        },
                    };
                None
            }
            BidPhase::FirstRoundThirdPlayer { trump_candidate } => {
                let bidder = self.dealer.partner().next_position_bidding();
                self.phase =
                    match BidState::order_up(&self.dealer, bidder, trump_candidate, players, hands)
                    {
                        Some(phase) => phase,
                        None => BidPhase::FirstRoundFourthPlayer {
                            trump_candidate: *trump_candidate,
                        },
                    };
                None
            }
            BidPhase::FirstRoundFourthPlayer { trump_candidate } => {
                let bidder = self.dealer;
                self.phase =
                    match BidState::order_up(&self.dealer, bidder, trump_candidate, players, hands)
                    {
                        Some(phase) => phase,
                        None => BidPhase::SecondRoundFirstPlayer {
                            turned_down: *trump_candidate,
                        },
                    };
                None
            }
            BidPhase::OrderedUp {
                caller,
                card_ordered,
            } => {
                let hand = &mut hands[self.dealer.index()];
                let player = &mut players[self.dealer.index()];
                BidState::discard(player, hand, *card_ordered);
                self.phase = BidPhase::Done {
                    bid_result: BidResultAll::Called {
                        trump: card_ordered.suit,
                        caller: *caller,
                    },
                };
                None
            }
            BidPhase::OrderedUpAlone {
                caller,
                card_ordered,
            } => {
                let hand = &mut hands[self.dealer.index()];
                let player = &mut players[self.dealer.index()];
                BidState::discard(player, hand, *card_ordered);
                self.phase = BidPhase::Done {
                    bid_result: BidResultAll::CalledAlone {
                        trump: card_ordered.suit,
                        caller: *caller,
                    },
                };
                None
            }
            BidPhase::OrderedUpDefendedAlone {
                caller,
                card_ordered,
                defender,
            } => {
                let hand = &mut hands[self.dealer.index()];
                let player = &mut players[self.dealer.index()];
                BidState::discard(player, hand, *card_ordered);
                self.phase = BidPhase::Done {
                    bid_result: BidResultAll::DefendedAlone {
                        trump: card_ordered.suit,
                        caller: *caller,
                        defender: *defender,
                    },
                };
                None
            }
            BidPhase::SecondRoundFirstPlayer { ref turned_down } => {
                let bidder = self.dealer.next_position_bidding();
                self.phase = match BidState::call(&self.dealer, bidder, players, hands, turned_down)
                {
                    Some(bid_result) => BidPhase::Done { bid_result },
                    _ => BidPhase::SecondRoundSecondPlayer {
                        turned_down: *turned_down,
                    },
                };
                None
            }
            BidPhase::SecondRoundSecondPlayer { ref turned_down } => {
                let bidder = self.dealer.partner();
                self.phase = match BidState::call(&self.dealer, bidder, players, hands, turned_down)
                {
                    Some(bid_result) => BidPhase::Done { bid_result },
                    _ => BidPhase::SecondRoundThirdPlayer {
                        turned_down: *turned_down,
                    },
                };
                None
            }
            BidPhase::SecondRoundThirdPlayer { ref turned_down } => {
                let bidder = self.dealer.partner().next_position_bidding();
                self.phase = match BidState::call(&self.dealer, bidder, players, hands, turned_down)
                {
                    Some(bid_result) => BidPhase::Done { bid_result },
                    _ => BidPhase::SecondRoundFourthPlayer {
                        turned_down: *turned_down,
                    },
                };
                None
            }
            BidPhase::SecondRoundFourthPlayer { ref turned_down } => {
                let bidder = self.dealer;
                self.phase = match BidState::call(&self.dealer, bidder, players, hands, turned_down)
                {
                    Some(bid_result) => BidPhase::Done { bid_result },
                    _ => {
                        log::info!("{}", "No one called a trump suit");
                        BidPhase::Done {
                            bid_result: BidResultAll::NoOneCalled,
                        }
                    }
                };
                None
            }
            BidPhase::Done { bid_result } => Some(bid_result.clone()),
        }
    }

    fn order_up(
        dealer: &Position,
        bidder: Position,
        trump_candidate: &CardBeforeBidding,
        players: &mut [impl Player; 4],
        hands: &[HandBeforeBidding; 4],
    ) -> Option<BidPhase> {
        let bidder_index = bidder.index();
        if !players[bidder_index].should_order_up(&hands[bidder_index], dealer, trump_candidate) {
            return None;
        }
        log::info!("{:?} ordered up {}", bidder, trump_candidate);
        if !players[bidder_index].should_order_up_alone(
            &hands[bidder_index],
            dealer,
            trump_candidate,
        ) {
            return Some(BidPhase::OrderedUp {
                caller: bidder,
                card_ordered: *trump_candidate,
            });
        }
        let defender = bidder.next_position_bidding();
        let defender_index = defender.index();
        if players[defender_index].should_defend_alone_ordered(
            &hands[defender_index],
            dealer,
            trump_candidate,
        ) {
            Some(BidPhase::OrderedUpDefendedAlone {
                caller: bidder,
                card_ordered: *trump_candidate,
                defender,
            })
        } else {
            let defender = defender.partner();
            let defender_index = defender.index();
            if players[defender_index].should_defend_alone_ordered(
                &hands[defender_index],
                dealer,
                trump_candidate,
            ) {
                Some(BidPhase::OrderedUpDefendedAlone {
                    caller: bidder,
                    card_ordered: *trump_candidate,
                    defender,
                })
            } else {
                Some(BidPhase::OrderedUpAlone {
                    caller: bidder,
                    card_ordered: *trump_candidate,
                })
            }
        }
    }

    fn discard(
        dealer: &mut impl Player,
        hand: &mut HandBeforeBidding,
        card_ordered: CardBeforeBidding,
    ) {
        hand.cards.push(card_ordered);
        let mut discard = dealer.choose_discard(hand, &card_ordered.suit);
        if !hand.cards.contains(&discard) {
            discard = hand.cards[0];
        }
        hand.cards.retain(|card| *card != discard);
    }

    fn call(
        dealer: &Position,
        bidder: Position,
        players: &mut [impl Player; 4],
        hands: &[HandBeforeBidding; 4],
        turned_down: &CardBeforeBidding,
    ) -> Option<BidResultAll> {
        match players[bidder.index()].call_trump(&hands[bidder.index()], dealer, turned_down) {
            Some(trump) if trump != turned_down.suit => {
                log::info!("{:?} called {}", bidder, trump);
                if !players[bidder.index()].should_call_alone(
                    &hands[bidder.index()],
                    dealer,
                    &trump,
                    turned_down,
                ) {
                    return Some(BidResultAll::Called {
                        trump,
                        caller: bidder,
                    });
                }
                let defender = bidder.next_position_bidding();
                if players[defender.index()].should_defend_alone_called(
                    &hands[defender.index()],
                    dealer,
                    &trump,
                    turned_down,
                ) {
                    Some(BidResultAll::DefendedAlone {
                        trump,
                        caller: bidder,
                        defender,
                    })
                } else {
                    let defender = defender.partner();
                    if players[defender.index()].should_defend_alone_called(
                        &hands[defender.index()],
                        dealer,
                        &trump,
                        turned_down,
                    ) {
                        Some(BidResultAll::DefendedAlone {
                            trump,
                            caller: bidder,
                            defender,
                        })
                    } else {
                        Some(BidResultAll::CalledAlone {
                            trump,
                            caller: bidder,
                        })
                    }
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{players::preprogrammed_bidder::PreprogrammedBidder, rank::Rank, suit::Suit};

    #[test]
    fn everyone_passes() {
        let dealer = Position::North;
        let trump_candidate = CardBeforeBidding {
            suit: Suit::Hearts,
            rank: Rank::Ace,
        };
        let turned_down = trump_candidate;
        let mut players = make_players();
        let mut hands = make_hands();
        let expected_return_value = BidResultAll::NoOneCalled;
        let bid_result = expected_return_value.clone();
        let expected_results = [
            BidPhase::FirstRoundFirstPlayer { trump_candidate },
            BidPhase::FirstRoundSecondPlayer { trump_candidate },
            BidPhase::FirstRoundThirdPlayer { trump_candidate },
            BidPhase::FirstRoundFourthPlayer { trump_candidate },
            BidPhase::SecondRoundFirstPlayer { turned_down },
            BidPhase::SecondRoundSecondPlayer { turned_down },
            BidPhase::SecondRoundThirdPlayer { turned_down },
            BidPhase::SecondRoundFourthPlayer { turned_down },
            BidPhase::Done { bid_result },
        ];
        check_sequence(
            dealer,
            trump_candidate,
            &mut players,
            &mut hands,
            &expected_results,
            expected_return_value,
        )
    }

    #[test]
    fn ordered_up_only_allows_valid_discards() {
        let dealer = Position::North;
        let trump_candidate = CardBeforeBidding {
            suit: Suit::Hearts,
            rank: Rank::Ace,
        };
        let trump = trump_candidate.suit;
        let card_ordered = trump_candidate;
        let mut players = make_players();
        players[dealer.index()] = PreprogrammedBidder::discards(CardBeforeBidding {
            suit: Suit::Spades,
            rank: Rank::Nine,
        });
        let caller = Position::South;
        players[caller.index()] = PreprogrammedBidder::orders_up();
        let mut hands = make_hands();
        let expected_return_value = BidResultAll::Called { trump, caller };
        let bid_result = expected_return_value.clone();
        let expected_results = [
            BidPhase::FirstRoundFirstPlayer { trump_candidate },
            BidPhase::FirstRoundSecondPlayer { trump_candidate },
            BidPhase::OrderedUp {
                caller,
                card_ordered,
            },
            BidPhase::Done { bid_result },
        ];
        check_sequence(
            dealer,
            trump_candidate,
            &mut players,
            &mut hands,
            &expected_results,
            expected_return_value,
        );
        assert_eq!(hands[dealer.index()].cards, vec![trump_candidate])
    }

    #[test]
    fn ordered_up() {
        let dealer = Position::North;
        let trump_candidate = CardBeforeBidding {
            suit: Suit::Hearts,
            rank: Rank::Ace,
        };
        let trump = trump_candidate.suit;
        let card_ordered = trump_candidate;
        let mut players = make_players();
        let caller = Position::South;
        players[caller.index()] = PreprogrammedBidder::orders_up();
        let mut hands = make_hands();
        let expected_return_value = BidResultAll::Called { trump, caller };
        let bid_result = expected_return_value.clone();
        let expected_results = [
            BidPhase::FirstRoundFirstPlayer { trump_candidate },
            BidPhase::FirstRoundSecondPlayer { trump_candidate },
            BidPhase::OrderedUp {
                caller,
                card_ordered,
            },
            BidPhase::Done { bid_result },
        ];
        check_sequence(
            dealer,
            trump_candidate,
            &mut players,
            &mut hands,
            &expected_results,
            expected_return_value,
        )
    }

    #[test]
    fn ordered_up_alone() {
        let dealer = Position::North;
        let trump_candidate = CardBeforeBidding {
            suit: Suit::Hearts,
            rank: Rank::Ace,
        };
        let card_ordered = trump_candidate;
        let mut players = make_players();
        let caller = Position::South;
        players[caller.index()] = PreprogrammedBidder::orders_up_alone();
        let mut hands = make_hands();
        let expected_return_value = BidResultAll::CalledAlone {
            trump: trump_candidate.suit,
            caller: Position::South,
        };
        let bid_result = expected_return_value.clone();
        let expected_results = [
            BidPhase::FirstRoundFirstPlayer { trump_candidate },
            BidPhase::FirstRoundSecondPlayer { trump_candidate },
            BidPhase::OrderedUpAlone {
                caller,
                card_ordered,
            },
            BidPhase::Done { bid_result },
        ];
        check_sequence(
            dealer,
            trump_candidate,
            &mut players,
            &mut hands,
            &expected_results,
            expected_return_value,
        )
    }

    #[test]
    fn ordered_up_defended_alone_first_opponent() {
        let dealer = Position::North;
        let trump_candidate = CardBeforeBidding {
            suit: Suit::Hearts,
            rank: Rank::Ace,
        };
        let trump = trump_candidate.suit;
        let card_ordered = trump_candidate;
        let mut players = make_players();
        let caller = Position::South;
        players[caller.index()] = PreprogrammedBidder::orders_up_alone();
        let defender = Position::West;
        players[defender.index()] = PreprogrammedBidder::defends_alone();
        players[defender.partner().index()] = PreprogrammedBidder::defends_alone();
        let mut hands = make_hands();
        let expected_return_value = BidResultAll::DefendedAlone {
            trump,
            caller,
            defender,
        };
        let bid_result = expected_return_value.clone();
        let expected_results = [
            BidPhase::FirstRoundFirstPlayer { trump_candidate },
            BidPhase::FirstRoundSecondPlayer { trump_candidate },
            BidPhase::OrderedUpDefendedAlone {
                caller,
                card_ordered,
                defender,
            },
            BidPhase::Done { bid_result },
        ];
        check_sequence(
            dealer,
            trump_candidate,
            &mut players,
            &mut hands,
            &expected_results,
            expected_return_value,
        )
    }

    #[test]
    fn ordered_up_defended_alone_second_opponent() {
        let dealer = Position::North;
        let trump_candidate = CardBeforeBidding {
            suit: Suit::Hearts,
            rank: Rank::Ace,
        };
        let trump = trump_candidate.suit;
        let card_ordered = trump_candidate;
        let mut players = make_players();
        let caller = Position::South;
        players[caller.index()] = PreprogrammedBidder::orders_up_alone();
        let defender = Position::East;
        players[defender.index()] = PreprogrammedBidder::defends_alone();
        let mut hands = make_hands();
        let expected_return_value = BidResultAll::DefendedAlone {
            trump,
            caller,
            defender,
        };
        let bid_result = expected_return_value.clone();
        let expected_results = [
            BidPhase::FirstRoundFirstPlayer { trump_candidate },
            BidPhase::FirstRoundSecondPlayer { trump_candidate },
            BidPhase::OrderedUpDefendedAlone {
                caller,
                card_ordered,
                defender,
            },
            BidPhase::Done { bid_result },
        ];
        check_sequence(
            dealer,
            trump_candidate,
            &mut players,
            &mut hands,
            &expected_results,
            expected_return_value,
        )
    }

    #[test]
    fn called_requires_new_suit() {
        let dealer = Position::North;
        let trump_candidate = CardBeforeBidding {
            suit: Suit::Hearts,
            rank: Rank::Ace,
        };
        let turned_down = trump_candidate;
        let mut players = make_players();
        let caller = Position::South;
        players[caller.index()] = PreprogrammedBidder::calls(Suit::Hearts);
        let mut hands = make_hands();
        let expected_return_value = BidResultAll::NoOneCalled;
        let bid_result = expected_return_value.clone();
        let expected_results = [
            BidPhase::FirstRoundFirstPlayer { trump_candidate },
            BidPhase::FirstRoundSecondPlayer { trump_candidate },
            BidPhase::FirstRoundThirdPlayer { trump_candidate },
            BidPhase::FirstRoundFourthPlayer { trump_candidate },
            BidPhase::SecondRoundFirstPlayer { turned_down },
            BidPhase::SecondRoundSecondPlayer { turned_down },
            BidPhase::SecondRoundThirdPlayer { turned_down },
            BidPhase::SecondRoundFourthPlayer { turned_down },
            BidPhase::Done { bid_result },
        ];
        check_sequence(
            dealer,
            trump_candidate,
            &mut players,
            &mut hands,
            &expected_results,
            expected_return_value,
        )
    }

    #[test]
    fn called() {
        let dealer = Position::North;
        let trump_candidate = CardBeforeBidding {
            suit: Suit::Hearts,
            rank: Rank::Ace,
        };
        let turned_down = trump_candidate;
        let trump = Suit::Spades;
        let mut players = make_players();
        let caller = Position::South;
        players[caller.index()] = PreprogrammedBidder::calls(trump);
        let mut hands = make_hands();
        let expected_return_value = BidResultAll::Called { trump, caller };
        let bid_result = expected_return_value.clone();
        let expected_results = [
            BidPhase::FirstRoundFirstPlayer { trump_candidate },
            BidPhase::FirstRoundSecondPlayer { trump_candidate },
            BidPhase::FirstRoundThirdPlayer { trump_candidate },
            BidPhase::FirstRoundFourthPlayer { trump_candidate },
            BidPhase::SecondRoundFirstPlayer { turned_down },
            BidPhase::SecondRoundSecondPlayer { turned_down },
            BidPhase::Done { bid_result },
        ];
        check_sequence(
            dealer,
            trump_candidate,
            &mut players,
            &mut hands,
            &expected_results,
            expected_return_value,
        )
    }

    #[test]
    fn called_alone() {
        let dealer = Position::North;
        let trump_candidate = CardBeforeBidding {
            suit: Suit::Hearts,
            rank: Rank::Ace,
        };
        let turned_down = trump_candidate;
        let trump = Suit::Spades;
        let mut players = make_players();
        let caller = Position::South;
        players[caller.index()] = PreprogrammedBidder::calls_alone(trump);
        let mut hands = make_hands();
        let expected_return_value = BidResultAll::CalledAlone { trump, caller };
        let bid_result = expected_return_value.clone();
        let expected_results = [
            BidPhase::FirstRoundFirstPlayer { trump_candidate },
            BidPhase::FirstRoundSecondPlayer { trump_candidate },
            BidPhase::FirstRoundThirdPlayer { trump_candidate },
            BidPhase::FirstRoundFourthPlayer { trump_candidate },
            BidPhase::SecondRoundFirstPlayer { turned_down },
            BidPhase::SecondRoundSecondPlayer { turned_down },
            BidPhase::Done { bid_result },
        ];
        check_sequence(
            dealer,
            trump_candidate,
            &mut players,
            &mut hands,
            &expected_results,
            expected_return_value,
        )
    }

    #[test]
    fn called_defended_alone_first_opponent() {
        let dealer = Position::North;
        let trump_candidate = CardBeforeBidding {
            suit: Suit::Hearts,
            rank: Rank::Ace,
        };
        let turned_down = trump_candidate;
        let trump = Suit::Spades;
        let mut players = make_players();
        let caller = Position::South;
        players[caller.index()] = PreprogrammedBidder::calls_alone(trump);
        let defender = Position::West;
        players[defender.index()] = PreprogrammedBidder::defends_alone();
        players[defender.partner().index()] = PreprogrammedBidder::defends_alone();
        let mut hands = make_hands();
        let expected_return_value = BidResultAll::DefendedAlone {
            trump,
            caller,
            defender,
        };
        let bid_result = expected_return_value.clone();
        let expected_results = [
            BidPhase::FirstRoundFirstPlayer { trump_candidate },
            BidPhase::FirstRoundSecondPlayer { trump_candidate },
            BidPhase::FirstRoundThirdPlayer { trump_candidate },
            BidPhase::FirstRoundFourthPlayer { trump_candidate },
            BidPhase::SecondRoundFirstPlayer { turned_down },
            BidPhase::SecondRoundSecondPlayer { turned_down },
            BidPhase::Done { bid_result },
        ];
        check_sequence(
            dealer,
            trump_candidate,
            &mut players,
            &mut hands,
            &expected_results,
            expected_return_value,
        )
    }

    #[test]
    fn called_defended_alone_second_opponent() {
        let dealer = Position::North;
        let trump_candidate = CardBeforeBidding {
            suit: Suit::Hearts,
            rank: Rank::Ace,
        };
        let turned_down = trump_candidate;
        let trump = Suit::Spades;
        let mut players = make_players();
        let caller = Position::South;
        players[caller.index()] = PreprogrammedBidder::calls_alone(trump);
        let defender = Position::East;
        players[defender.index()] = PreprogrammedBidder::defends_alone();
        let mut hands = make_hands();
        let expected_return_value = BidResultAll::DefendedAlone {
            trump,
            caller,
            defender,
        };
        let bid_result = expected_return_value.clone();
        let expected_results = [
            BidPhase::FirstRoundFirstPlayer { trump_candidate },
            BidPhase::FirstRoundSecondPlayer { trump_candidate },
            BidPhase::FirstRoundThirdPlayer { trump_candidate },
            BidPhase::FirstRoundFourthPlayer { trump_candidate },
            BidPhase::SecondRoundFirstPlayer { turned_down },
            BidPhase::SecondRoundSecondPlayer { turned_down },
            BidPhase::Done { bid_result },
        ];
        check_sequence(
            dealer,
            trump_candidate,
            &mut players,
            &mut hands,
            &expected_results,
            expected_return_value,
        )
    }

    fn make_players() -> [PreprogrammedBidder; 4] {
        [
            PreprogrammedBidder::does_nothing(),
            PreprogrammedBidder::does_nothing(),
            PreprogrammedBidder::does_nothing(),
            PreprogrammedBidder::does_nothing(),
        ]
    }

    fn make_hands() -> [HandBeforeBidding; 4] {
        [
            HandBeforeBidding {
                cards: vec![CardBeforeBidding {
                    rank: Rank::King,
                    suit: Suit::Spades,
                }],
            },
            HandBeforeBidding {
                cards: vec![CardBeforeBidding {
                    rank: Rank::King,
                    suit: Suit::Hearts,
                }],
            },
            HandBeforeBidding {
                cards: vec![CardBeforeBidding {
                    rank: Rank::King,
                    suit: Suit::Diamonds,
                }],
            },
            HandBeforeBidding {
                cards: vec![CardBeforeBidding {
                    rank: Rank::King,
                    suit: Suit::Clubs,
                }],
            },
        ]
    }

    fn check_sequence(
        dealer: Position,
        trump_candidate: CardBeforeBidding,
        players: &mut [impl Player; 4],
        hands: &mut [HandBeforeBidding; 4],
        expected_results: &[BidPhase],
        expected_return_value: BidResultAll,
    ) {
        let mut bid_state = BidState::create(dealer, trump_candidate);
        let mut return_value_received = false;
        for expected_result in expected_results {
            assert_eq!(
                false, return_value_received,
                "Return value received too early"
            );
            assert_eq!(bid_state.dealer, dealer);
            assert_eq!(bid_state.phase, *expected_result);
            match bid_state.step(players, hands) {
                Some(bid_result) => {
                    return_value_received = true;
                    assert_eq!(bid_result, expected_return_value);
                }
                None => (),
            }
        }
        assert_eq!(true, return_value_received, "Return value not receieved")
    }
}
