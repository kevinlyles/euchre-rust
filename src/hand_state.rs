use crate::{
    bid_result::BidResult, bid_state::BidState, card::CardLogic, hand::HandLogic, player::Player,
    trick_state::TrickState,
};

#[derive(PartialEq)]
pub struct HandState {
    pub dealer: Player,
    pub phase: HandPhase,
    pub update_in_parent: fn(HandState) -> (),
}

#[derive(PartialEq)]
pub enum HandPhase {
    Initializing,
    Bidding {
        bid_state: Box<BidState>,
    },
    FirstTrick {
        bid_result: BidResult,
        hands: [HandLogic; 4],
        trick_state: TrickState,
    },
    SecondTrick {
        bid_result: BidResult,
        hands: [HandLogic; 4],
        trick_state: TrickState,
        tricks_taken: [u8; 4],
    },
    ThirdTrick {
        bid_result: BidResult,
        hands: [HandLogic; 4],
        trick_state: TrickState,
        tricks_taken: [u8; 4],
    },
    FourthTrick {
        bid_result: BidResult,
        hands: [HandLogic; 4],
        trick_state: TrickState,
        tricks_taken: [u8; 4],
    },
    FifthTrick {
        bid_result: BidResult,
        hands: [HandLogic; 4],
        trick_state: TrickState,
        tricks_taken: [u8; 4],
    },
    Scoring {
        tricks_taken: [u8; 4],
    },
}

impl HandState {
    pub fn create(
        dealer: Player,
        hands: [HandLogic; 4],
        trump_candidate: CardLogic,
        update_in_parent: fn(HandState) -> (),
    ) -> HandState {
        let mut state = HandState {
            dealer,
            phase: HandPhase::Initializing,
            update_in_parent,
        };
        state.phase = HandPhase::Bidding {
            bid_state: BidState::create(
                dealer,
                hands,
                trump_candidate,
                Box::new(|bid_state| state.update_bid_state(bid_state)),
                Box::new(|bid_result| state.finish_bidding(bid_result)),
            ),
        };
        state
    }

    fn update_bid_state(&mut self, bid_state: BidState) {
        self.phase = HandPhase::Bidding {
            bid_state: Box::new(bid_state),
        };
        (self.update_in_parent)(*self);
    }

    pub fn finish_bidding(&mut self, bid_result: BidResult) -> () {
        match self.phase {
            HandPhase::Bidding { bid_state } => {
                self.phase = HandPhase::FirstTrick {
                    bid_result,
                    hands: bid_state.hands,
                    trick_state: TrickState::create(
                        self.dealer.next_player(
                            if bid_result.called_alone {
                                Some(bid_result.caller)
                            } else {
                                None
                            },
                            bid_result.defender,
                        ),
                        Box::new(|trick_state| self.update_trick_state(trick_state)),
                        Box::new(|trick_winner| self.trick_finished(trick_winner)),
                    ),
                }
            }
            _ => (),
        }
    }

    fn update_trick_state(&mut self, trick_state: TrickState) -> () {
        match self.phase {
            HandPhase::FirstTrick {
                bid_result, hands, ..
            } => {
                self.phase = HandPhase::FirstTrick {
                    bid_result,
                    hands,
                    trick_state,
                }
            }
            HandPhase::SecondTrick {
                bid_result,
                hands,
                tricks_taken,
                ..
            } => {
                self.phase = HandPhase::SecondTrick {
                    bid_result,
                    hands,
                    trick_state,
                    tricks_taken,
                }
            }
            HandPhase::ThirdTrick {
                bid_result,
                hands,
                tricks_taken,
                ..
            } => {
                self.phase = HandPhase::ThirdTrick {
                    bid_result,
                    hands,
                    trick_state,
                    tricks_taken,
                }
            }
            HandPhase::FourthTrick {
                bid_result,
                hands,
                tricks_taken,
                ..
            } => {
                self.phase = HandPhase::FourthTrick {
                    bid_result,
                    hands,
                    trick_state,
                    tricks_taken,
                }
            }
            HandPhase::FifthTrick {
                bid_result,
                hands,
                tricks_taken,
                ..
            } => {
                self.phase = HandPhase::FifthTrick {
                    bid_result,
                    hands,
                    trick_state,
                    tricks_taken,
                }
            }
            _ => (),
        }
    }

    fn trick_finished(&mut self, trick_winner: Player) -> () {
        let update_in_parent = Box::new(|trick_state| self.update_trick_state(trick_state));
        let finish = Box::new(|trick_winner| self.trick_finished(trick_winner));
        match self.phase {
            HandPhase::FirstTrick {
                bid_result, hands, ..
            } => {
                let tricks_taken = [0u8; 4];
                tricks_taken[trick_winner.index()] += 1;
                self.phase = HandPhase::SecondTrick {
                    bid_result,
                    hands,
                    trick_state: TrickState::create(trick_winner, update_in_parent, finish),
                    tricks_taken,
                }
            }
            HandPhase::SecondTrick {
                bid_result,
                hands,
                tricks_taken,
                ..
            } => {
                tricks_taken[trick_winner.index()] += 1;
                self.phase = HandPhase::ThirdTrick {
                    bid_result,
                    hands,
                    trick_state: TrickState::create(trick_winner, update_in_parent, finish),
                    tricks_taken,
                }
            }
            HandPhase::ThirdTrick {
                bid_result,
                hands,
                tricks_taken,
                ..
            } => {
                tricks_taken[trick_winner.index()] += 1;
                self.phase = HandPhase::FourthTrick {
                    bid_result,
                    hands,
                    trick_state: TrickState::create(trick_winner, update_in_parent, finish),
                    tricks_taken,
                }
            }
            HandPhase::FourthTrick {
                bid_result,
                hands,
                tricks_taken,
                ..
            } => {
                tricks_taken[trick_winner.index()] += 1;
                self.phase = HandPhase::FifthTrick {
                    bid_result,
                    hands,
                    trick_state: TrickState::create(trick_winner, update_in_parent, finish),
                    tricks_taken,
                }
            }
            HandPhase::FifthTrick {
                bid_result,
                hands,
                tricks_taken,
                ..
            } => {
                tricks_taken[trick_winner.index()] += 1;
                self.phase = HandPhase::Scoring { tricks_taken }
            }
            _ => (),
        }
    }
}
