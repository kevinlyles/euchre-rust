use crate::{
    bid_result::BidResultAll, card::CardLogic, hand::HandLogic, player::Player, position::Position,
};

#[derive(Debug, PartialEq, Eq)]
pub struct BidState {
    pub dealer: Position,
    pub phase: BidPhase,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BidPhase {
    FirstRoundFirstPlayer {
        trump_candidate: CardLogic,
    },
    FirstRoundSecondPlayer {
        trump_candidate: CardLogic,
    },
    FirstRoundThirdPlayer {
        trump_candidate: CardLogic,
    },
    FirstRoundFourthPlayer {
        trump_candidate: CardLogic,
    },
    OrderedUp {
        caller: Position,
        card_ordered: CardLogic,
    },
    OrderedUpAlone {
        caller: Position,
        card_ordered: CardLogic,
    },
    OrderedUpDefendedAlone {
        caller: Position,
        card_ordered: CardLogic,
        defender: Position,
    },
    SecondRoundFirstPlayer {
        turned_down: CardLogic,
    },
    SecondRoundSecondPlayer {
        turned_down: CardLogic,
    },
    SecondRoundThirdPlayer {
        turned_down: CardLogic,
    },
    SecondRoundFourthPlayer {
        turned_down: CardLogic,
    },
    Done {
        bid_result: BidResultAll,
    },
}

impl BidState {
    pub fn create(dealer: Position, trump_candidate: CardLogic) -> BidState {
        BidState {
            dealer,
            phase: BidPhase::FirstRoundFirstPlayer { trump_candidate },
        }
    }

    pub fn step(
        &mut self,
        players: &mut [impl Player; 4],
        hands: &mut [HandLogic; 4],
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
                            trump_candidate: trump_candidate.clone(),
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
                            trump_candidate: trump_candidate.clone(),
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
                            trump_candidate: trump_candidate.clone(),
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
                hand.cards.push(*card_ordered);
                let discard =
                    players[self.dealer.index()].choose_discard(&hand, &card_ordered.suit);
                hand.cards.retain(|card| *card != discard);
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
                hand.cards.push(*card_ordered);
                let discard =
                    players[self.dealer.index()].choose_discard(&hand, &card_ordered.suit);
                hand.cards.retain(|card| *card != discard);
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
                hand.cards.push(*card_ordered);
                let discard =
                    players[self.dealer.index()].choose_discard(&hand, &card_ordered.suit);
                hand.cards.retain(|card| *card != discard);
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
        trump_candidate: &CardLogic,
        players: &mut [impl Player; 4],
        hands: &[HandLogic; 4],
    ) -> Option<BidPhase> {
        let bidder_index = bidder.index();
        if !players[bidder_index].should_order_up(&hands[bidder_index], dealer, trump_candidate) {
            return None;
        }
        log::info!("{:?} ordered up {}", bidder, trump_candidate);
        if !players[bidder_index].should_order_up_alone(
            &hands[bidder_index],
            dealer,
            &trump_candidate,
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
            &trump_candidate,
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

    fn call(
        dealer: &Position,
        bidder: Position,
        players: &mut [impl Player; 4],
        hands: &[HandLogic; 4],
        turned_down: &CardLogic,
    ) -> Option<BidResultAll> {
        match players[bidder.index()].call_trump(&hands[bidder.index()], &dealer, turned_down) {
            Some(trump) if trump != turned_down.suit => {
                log::info!("{:?} called {}", bidder, trump);
                if !players[bidder.index()].should_call_alone(
                    &hands[bidder.index()],
                    &dealer,
                    &trump,
                    &turned_down,
                ) {
                    return Some(BidResultAll::Called {
                        trump,
                        caller: bidder,
                    });
                }
                let defender = bidder.next_position_bidding();
                if players[defender.index()].should_defend_alone_called(
                    &hands[defender.index()],
                    &dealer,
                    &trump,
                    &turned_down,
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
                        &dealer,
                        &trump,
                        &turned_down,
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
        };
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{rank_with_bowers::RankWithBowers, suit::Suit};

    struct TestBidder {
        order_up: bool,
        order_up_alone: bool,
        defend_alone_ordered: bool,
        trump_to_call: Option<Suit>,
        call_alone: bool,
        defend_alone_called: bool,
    }

    impl TestBidder {
        pub fn does_nothing() -> TestBidder {
            TestBidder {
                order_up: false,
                order_up_alone: false,
                defend_alone_ordered: false,
                trump_to_call: None,
                call_alone: false,
                defend_alone_called: false,
            }
        }

        pub fn orders_up() -> TestBidder {
            TestBidder {
                order_up: true,
                order_up_alone: false,
                defend_alone_ordered: false,
                trump_to_call: None,
                call_alone: false,
                defend_alone_called: false,
            }
        }

        pub fn orders_up_alone() -> TestBidder {
            TestBidder {
                order_up: true,
                order_up_alone: true,
                defend_alone_ordered: false,
                trump_to_call: None,
                call_alone: false,
                defend_alone_called: false,
            }
        }
    }

    impl Player for TestBidder {
        fn should_order_up(
            &mut self,
            _hand: &HandLogic,
            _dealer: &Position,
            _trump_candidate: &CardLogic,
        ) -> bool {
            self.order_up
        }

        fn should_order_up_alone(
            &mut self,
            _hand: &HandLogic,
            _dealer: &Position,
            _trump_candidate: &CardLogic,
        ) -> bool {
            self.order_up_alone
        }

        fn should_defend_alone_ordered(
            &mut self,
            _hand: &HandLogic,
            _dealer: &Position,
            _trump_candidate: &CardLogic,
        ) -> bool {
            self.defend_alone_ordered
        }

        fn call_trump(
            &mut self,
            _hand: &HandLogic,
            _dealer: &Position,
            _turned_down: &CardLogic,
        ) -> Option<Suit> {
            self.trump_to_call
        }

        fn should_call_alone(
            &mut self,
            _hand: &HandLogic,
            _dealer: &Position,
            _trump: &Suit,
            _turned_down: &CardLogic,
        ) -> bool {
            self.call_alone
        }

        fn should_defend_alone_called(
            &mut self,
            _hand: &HandLogic,
            _dealer: &Position,
            _trump: &Suit,
            _turned_down: &CardLogic,
        ) -> bool {
            self.defend_alone_called
        }
    }

    #[test]
    fn everyone_passes() {
        let dealer = Position::North;
        let trump_candidate = CardLogic {
            suit: Suit::Hearts,
            rank: RankWithBowers::Ace,
        };
        let turned_down = trump_candidate.clone();
        let mut players = [
            TestBidder::does_nothing(),
            TestBidder::does_nothing(),
            TestBidder::does_nothing(),
            TestBidder::does_nothing(),
        ];
        let mut hands = [
            HandLogic { cards: vec![] },
            HandLogic { cards: vec![] },
            HandLogic { cards: vec![] },
            HandLogic { cards: vec![] },
        ];
        let expected_results = [
            BidPhase::FirstRoundFirstPlayer { trump_candidate },
            BidPhase::FirstRoundSecondPlayer { trump_candidate },
            BidPhase::FirstRoundThirdPlayer { trump_candidate },
            BidPhase::FirstRoundFourthPlayer { trump_candidate },
            BidPhase::SecondRoundFirstPlayer { turned_down },
            BidPhase::SecondRoundSecondPlayer { turned_down },
            BidPhase::SecondRoundThirdPlayer { turned_down },
            BidPhase::SecondRoundFourthPlayer { turned_down },
            BidPhase::Done {
                bid_result: BidResultAll::NoOneCalled,
            },
        ];
        let expected_return_value = BidResultAll::NoOneCalled;
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
    fn ordered_up() {
        let dealer = Position::North;
        let original_trump_candidate = CardLogic {
            suit: Suit::Hearts,
            rank: RankWithBowers::Ace,
        };
        let mut players: [TestBidder; 4] = [
            TestBidder::does_nothing(),
            TestBidder::does_nothing(),
            TestBidder::orders_up(),
            TestBidder::does_nothing(),
        ];
        let mut hands = [
            HandLogic { cards: vec![] },
            HandLogic { cards: vec![] },
            HandLogic { cards: vec![] },
            HandLogic { cards: vec![] },
        ];
        let mut bid_state = BidState::create(dealer, original_trump_candidate);
        assert_eq!(bid_state.dealer, dealer);
        match bid_state.phase {
            BidPhase::FirstRoundFirstPlayer { trump_candidate } => {
                assert_eq!(trump_candidate, original_trump_candidate)
            }
            _ => assert!(false),
        }
        match bid_state.step(&mut players, &mut hands) {
            None => {
                assert_eq!(bid_state.dealer, dealer);
                match bid_state.phase {
                    BidPhase::FirstRoundSecondPlayer { trump_candidate } => {
                        assert_eq!(trump_candidate, original_trump_candidate)
                    }
                    _ => assert!(false),
                }
            }
            _ => assert!(false),
        }
        match bid_state.step(&mut players, &mut hands) {
            None => {
                assert_eq!(bid_state.dealer, dealer);
                match bid_state.phase {
                    BidPhase::OrderedUp {
                        caller,
                        card_ordered,
                    } => {
                        assert_eq!(caller, Position::South);
                        assert_eq!(card_ordered, original_trump_candidate)
                    }
                    _ => assert!(false),
                }
            }
            _ => assert!(false),
        }
        match bid_state.step(&mut players, &mut hands) {
            None => {
                assert_eq!(bid_state.dealer, dealer);
                match bid_state.phase {
                    BidPhase::Done { ref bid_result } => match bid_result {
                        BidResultAll::Called { trump, caller } => {
                            assert_eq!(caller, &Position::South);
                            assert_eq!(trump, &original_trump_candidate.suit)
                        }
                        _ => assert!(false),
                    },
                    _ => assert!(false),
                }
            }
            _ => assert!(false),
        }
        match bid_state.step(&mut players, &mut hands) {
            Some(bid_result) => match bid_result {
                BidResultAll::Called { trump, caller } => {
                    assert_eq!(caller, Position::South);
                    assert_eq!(trump, original_trump_candidate.suit)
                }
                _ => assert!(false),
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn ordered_up_alone() {
        let dealer = Position::North;
        let original_trump_candidate = CardLogic {
            suit: Suit::Hearts,
            rank: RankWithBowers::Ace,
        };
        let mut players: [TestBidder; 4] = [
            TestBidder::does_nothing(),
            TestBidder::does_nothing(),
            TestBidder::orders_up_alone(),
            TestBidder::does_nothing(),
        ];
        let mut hands = [
            HandLogic { cards: vec![] },
            HandLogic { cards: vec![] },
            HandLogic { cards: vec![] },
            HandLogic { cards: vec![] },
        ];
        let mut bid_state = BidState::create(dealer, original_trump_candidate);
        assert_eq!(bid_state.dealer, dealer);
        match bid_state.phase {
            BidPhase::FirstRoundFirstPlayer { trump_candidate } => {
                assert_eq!(trump_candidate, original_trump_candidate)
            }
            _ => assert!(false),
        }
        match bid_state.step(&mut players, &mut hands) {
            None => {
                assert_eq!(bid_state.dealer, dealer);
                match bid_state.phase {
                    BidPhase::FirstRoundSecondPlayer { trump_candidate } => {
                        assert_eq!(trump_candidate, original_trump_candidate)
                    }
                    _ => assert!(false),
                }
            }
            _ => assert!(false),
        }
        match bid_state.step(&mut players, &mut hands) {
            None => {
                assert_eq!(bid_state.dealer, dealer);
                match bid_state.phase {
                    BidPhase::OrderedUpAlone {
                        caller,
                        card_ordered,
                    } => {
                        assert_eq!(caller, Position::South);
                        assert_eq!(card_ordered, original_trump_candidate)
                    }
                    _ => assert!(false),
                }
            }
            _ => assert!(false),
        }
        match bid_state.step(&mut players, &mut hands) {
            None => {
                assert_eq!(bid_state.dealer, dealer);
                match bid_state.phase {
                    BidPhase::Done { ref bid_result } => match bid_result {
                        BidResultAll::CalledAlone { trump, caller } => {
                            assert_eq!(caller, &Position::South);
                            assert_eq!(trump, &original_trump_candidate.suit)
                        }
                        _ => assert!(false),
                    },
                    _ => assert!(false),
                }
            }
            _ => assert!(false),
        }
        match bid_state.step(&mut players, &mut hands) {
            Some(bid_result) => match bid_result {
                BidResultAll::CalledAlone { trump, caller } => {
                    assert_eq!(caller, Position::South);
                    assert_eq!(trump, original_trump_candidate.suit)
                }
                _ => assert!(false),
            },
            _ => assert!(false),
        }
    }

    #[test]
    fn ordered_up_defended_alone_first_opponent() {
        todo!()
    }

    #[test]
    fn ordered_up_defended_alone_second_opponent() {
        todo!()
    }

    #[test]
    fn called() {
        todo!()
    }

    #[test]
    fn called_alone() {
        todo!()
    }

    #[test]
    fn called_defended_alone_first_opponent() {
        todo!()
    }

    #[test]
    fn called_defended_alone_second_opponent() {
        todo!()
    }

    fn check_sequence(
        dealer: Position,
        trump_candidate: CardLogic,
        players: &mut [impl Player; 4],
        hands: &mut [HandLogic; 4],
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
