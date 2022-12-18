use crate::{
    bid_result::{BidResultAll, BidResultCalled},
    bid_state::BidState,
    card::Card,
    hand::Hand,
    hands_iterator::HandsIterator,
    player::Player,
    position::Position,
    rank_with_bowers::RankWithBowers,
    suit::Suit,
    trick_state::TrickState,
};

pub struct HandState {
    pub dealer: Position,
    hands: [Hand; 4],
    pub phase: HandPhase,
}

pub enum HandPhase {
    Bidding {
        bid_state: BidState,
    },
    FirstTrick {
        bid_result: BidResultCalled,
        trick_state: TrickState,
    },
    SecondTrick {
        bid_result: BidResultCalled,
        trick_state: TrickState,
        tricks_taken: [u8; 4],
    },
    ThirdTrick {
        bid_result: BidResultCalled,
        trick_state: TrickState,
        tricks_taken: [u8; 4],
    },
    FourthTrick {
        bid_result: BidResultCalled,
        trick_state: TrickState,
        tricks_taken: [u8; 4],
    },
    FifthTrick {
        bid_result: BidResultCalled,
        trick_state: TrickState,
        tricks_taken: [u8; 4],
    },
    Scoring {
        bid_result: BidResultAll,
        tricks_taken: [u8; 4],
    },
}

impl HandState {
    pub fn create(dealer: Position, trump_candidate: Card, hands: [Hand; 4]) -> HandState {
        HandState {
            dealer,
            hands,
            phase: HandPhase::Bidding {
                bid_state: BidState::create(dealer, trump_candidate),
            },
        }
    }

    pub fn create_with_scenario(
        players: &mut [impl Player; 4],
        dealer: Position,
        my_hand: Hand,
        trump_candidate: Card,
        bid_result: BidResultCalled,
    ) -> impl Iterator<Item = HandState> + '_ {
        HandsIterator::create(my_hand, trump_candidate)
            .into_iter()
            .map(move |hands| {
                HandState::create_hand_state(
                    players,
                    dealer,
                    hands,
                    trump_candidate,
                    bid_result.clone(),
                )
            })
    }

    fn create_hand_state(
        players: &mut [impl Player; 4],
        dealer: Position,
        hands: [Hand; 4],
        trump_candidate: Card,
        bid_result: BidResultCalled,
    ) -> HandState {
        match bid_result {
            BidResultCalled::Called { trump, .. }
            | BidResultCalled::CalledAlone { trump, .. }
            | BidResultCalled::DefendedAlone { trump, .. } => {
                let mut hands = hands;
                if trump == trump_candidate.suit {
                    let hand = &mut hands[dealer.index()];
                    hand.cards.push(trump_candidate);
                    let mut discard =
                        players[dealer.index()].choose_discard(&hand, &trump_candidate.suit);
                    if !hand.cards.contains(&discard) {
                        discard = hand.cards[0];
                    }
                    hand.cards.retain(|&card| card != discard);
                }
                HandState {
                    dealer,
                    hands,
                    phase: HandPhase::FirstTrick {
                        bid_result: bid_result.clone(),
                        trick_state: TrickState::create(
                            bid_result.clone(),
                            dealer.next_position_playing(&bid_result),
                        ),
                    },
                }
            }
        }
    }

    pub fn step(&mut self, players: &mut [impl Player; 4]) -> Option<(Position, u8)> {
        match &mut self.phase {
            HandPhase::Bidding { bid_state } => {
                match bid_state.step(players, &mut self.hands) {
                    Some(bid_result) => {
                        self.phase = match bid_result {
                            BidResultAll::Called { .. }
                            | BidResultAll::CalledAlone { .. }
                            | BidResultAll::DefendedAlone { .. } => {
                                let bid_result: BidResultCalled = bid_result.try_into().unwrap();
                                HandState::update_bowers(&mut self.hands, &bid_result.trump());
                                HandPhase::FirstTrick {
                                    trick_state: TrickState::create(
                                        bid_result.clone(),
                                        self.dealer.next_position_playing(&bid_result),
                                    ),
                                    bid_result,
                                }
                            }
                            BidResultAll::NoOneCalled => HandPhase::Scoring {
                                bid_result,
                                tricks_taken: [0; 4],
                            },
                        }
                    }
                    None => (),
                };
                None
            }
            HandPhase::FirstTrick {
                bid_result,
                trick_state,
            } => {
                match trick_state.step(players, &mut self.hands) {
                    Some(trick_winner) => {
                        let mut tricks_taken = [0; 4];
                        tricks_taken[trick_winner.index()] += 1;
                        self.phase = HandPhase::SecondTrick {
                            bid_result: bid_result.clone(),
                            trick_state: TrickState::create(bid_result.clone(), trick_winner),
                            tricks_taken,
                        }
                    }
                    None => (),
                };
                None
            }
            HandPhase::SecondTrick {
                bid_result,
                trick_state,
                tricks_taken,
            } => {
                match trick_state.step(players, &mut self.hands) {
                    Some(trick_winner) => {
                        tricks_taken[trick_winner.index()] += 1;
                        self.phase = HandPhase::ThirdTrick {
                            bid_result: bid_result.clone(),
                            trick_state: TrickState::create(bid_result.clone(), trick_winner),
                            tricks_taken: *tricks_taken,
                        }
                    }
                    None => (),
                };
                None
            }
            HandPhase::ThirdTrick {
                bid_result,
                trick_state,
                tricks_taken,
            } => {
                match trick_state.step(players, &mut self.hands) {
                    Some(trick_winner) => {
                        tricks_taken[trick_winner.index()] += 1;
                        self.phase = HandPhase::FourthTrick {
                            bid_result: bid_result.clone(),
                            trick_state: TrickState::create(bid_result.clone(), trick_winner),
                            tricks_taken: *tricks_taken,
                        }
                    }
                    None => (),
                };
                None
            }
            HandPhase::FourthTrick {
                bid_result,
                trick_state,
                tricks_taken,
            } => {
                match trick_state.step(players, &mut self.hands) {
                    Some(trick_winner) => {
                        tricks_taken[trick_winner.index()] += 1;
                        self.phase = HandPhase::FifthTrick {
                            bid_result: bid_result.clone(),
                            trick_state: TrickState::create(bid_result.clone(), trick_winner),
                            tricks_taken: *tricks_taken,
                        }
                    }
                    None => (),
                };
                None
            }
            HandPhase::FifthTrick {
                bid_result,
                trick_state,
                tricks_taken,
            } => {
                match trick_state.step(players, &mut self.hands) {
                    Some(trick_winner) => {
                        tricks_taken[trick_winner.index()] += 1;
                        self.phase = HandPhase::Scoring {
                            bid_result: bid_result.clone().into(),
                            tricks_taken: *tricks_taken,
                        }
                    }
                    None => (),
                };
                None
            }
            HandPhase::Scoring {
                bid_result,
                tricks_taken,
            } => Some(HandState::get_score(&bid_result, &tricks_taken)),
        }
    }

    fn update_bowers(hands: &mut [Hand; 4], trump: &Suit) -> () {
        for hand in hands.iter_mut() {
            for card in hand.cards.iter_mut() {
                if card.rank != RankWithBowers::Jack {
                    continue;
                }
                if card.suit == *trump {
                    *card = Card {
                        rank: RankWithBowers::RightBower,
                        suit: *trump,
                    };
                } else if card.suit.other_suit_of_same_color() == *trump {
                    *card = Card {
                        rank: RankWithBowers::LeftBower,
                        suit: *trump,
                    };
                }
            }
        }
    }

    fn get_score(bid_result: &BidResultAll, tricks_taken: &[u8; 4]) -> (Position, u8) {
        match bid_result {
            BidResultAll::Called { caller, .. } => {
                let caller_tricks =
                    tricks_taken[caller.index()] + tricks_taken[caller.partner().index()];
                if caller_tricks >= 3 {
                    if caller_tricks >= 5 {
                        (*caller, 2)
                    } else {
                        (*caller, 1)
                    }
                } else {
                    (caller.next_position_bidding(), 2)
                }
            }
            BidResultAll::CalledAlone { caller, .. } => {
                let caller_tricks = tricks_taken[caller.index()];
                if caller_tricks >= 3 {
                    if caller_tricks >= 5 {
                        (*caller, 4)
                    } else {
                        (*caller, 1)
                    }
                } else {
                    (caller.next_position_bidding(), 2)
                }
            }
            BidResultAll::DefendedAlone {
                caller, defender, ..
            } => {
                let caller_tricks = tricks_taken[caller.index()];
                if caller_tricks >= 3 {
                    if caller_tricks >= 5 {
                        (*caller, 4)
                    } else {
                        (*caller, 1)
                    }
                } else {
                    (*defender, 4)
                }
            }
            BidResultAll::NoOneCalled => (Position::South, 0),
        }
    }
}
