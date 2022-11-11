use crate::{
    bid_result::BidResult, bid_state::BidState, card::CardLogic, hand::HandLogic, player::Player,
    position::Position, trick_state::TrickState,
};

pub struct HandState {
    pub dealer: Position,
    pub phase: HandPhase,
}

pub enum HandPhase {
    Bidding {
        bid_state: BidState,
    },
    FirstTrick {
        bid_result: BidResult,
        trick_state: TrickState,
    },
    SecondTrick {
        bid_result: BidResult,
        trick_state: TrickState,
        tricks_taken: [u8; 4],
    },
    ThirdTrick {
        bid_result: BidResult,
        trick_state: TrickState,
        tricks_taken: [u8; 4],
    },
    FourthTrick {
        bid_result: BidResult,
        trick_state: TrickState,
        tricks_taken: [u8; 4],
    },
    FifthTrick {
        bid_result: BidResult,
        trick_state: TrickState,
        tricks_taken: [u8; 4],
    },
    Scoring {
        tricks_taken: [u8; 4],
    },
}

impl HandState {
    pub fn create(dealer: Position, trump_candidate: CardLogic) -> HandState {
        HandState {
            dealer,
            phase: HandPhase::Bidding {
                bid_state: BidState::create(dealer, trump_candidate),
            },
        }
    }

    pub fn step(
        &mut self,
        players: &mut [Box<dyn Player>; 4],
        hands: &mut [HandLogic; 4],
    ) -> Option<[u8; 4]> {
        match &mut self.phase {
            HandPhase::Bidding { bid_state } => {
                match bid_state.step(players) {
                    Some(bid_result) => {
                        self.phase = HandPhase::FirstTrick {
                            trick_state: TrickState::create(
                                self.dealer.next_position_playing(&bid_result),
                            ),
                            bid_result,
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
                match trick_state.step(players, hands) {
                    Some(trick_winner) => {
                        let mut tricks_taken = [0; 4];
                        tricks_taken[trick_winner.index()] += 1;
                        self.phase = HandPhase::SecondTrick {
                            bid_result: bid_result.clone(),
                            trick_state: TrickState::create(trick_winner),
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
                match trick_state.step(players, hands) {
                    Some(trick_winner) => {
                        tricks_taken[trick_winner.index()] += 1;
                        self.phase = HandPhase::ThirdTrick {
                            bid_result: bid_result.clone(),
                            trick_state: TrickState::create(trick_winner),
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
                match trick_state.step(players, hands) {
                    Some(trick_winner) => {
                        tricks_taken[trick_winner.index()] += 1;
                        self.phase = HandPhase::FourthTrick {
                            bid_result: bid_result.clone(),
                            trick_state: TrickState::create(trick_winner),
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
                match trick_state.step(players, hands) {
                    Some(trick_winner) => {
                        tricks_taken[trick_winner.index()] += 1;
                        self.phase = HandPhase::FifthTrick {
                            bid_result: bid_result.clone(),
                            trick_state: TrickState::create(trick_winner),
                            tricks_taken: *tricks_taken,
                        }
                    }
                    None => (),
                };
                None
            }
            HandPhase::FifthTrick {
                bid_result: _,
                trick_state,
                tricks_taken,
            } => {
                match trick_state.step(players, hands) {
                    Some(trick_winner) => {
                        tricks_taken[trick_winner.index()] += 1;
                        self.phase = HandPhase::Scoring {
                            tricks_taken: *tricks_taken,
                        }
                    }
                    None => (),
                };
                None
            }
            HandPhase::Scoring { tricks_taken } => Some(*tricks_taken),
        }
    }
}
