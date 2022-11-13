use crate::{
    bid_result::{BidResultAll, BidResultCalled},
    bid_state::BidState,
    card::CardLogic,
    hand::HandLogic,
    player::Player,
    position::Position,
    trick_state::TrickState,
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
    ) -> Option<(Position, u8)> {
        match &mut self.phase {
            HandPhase::Bidding { bid_state } => {
                match bid_state.step(players, hands) {
                    Some(bid_result) => {
                        self.phase = match bid_result {
                            BidResultAll::Called { trump, .. }
                            | BidResultAll::CalledAlone { trump, .. }
                            | BidResultAll::DefendedAlone { trump, .. } => {
                                let bid_result: BidResultCalled = bid_result.try_into().unwrap();
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
                match trick_state.step(players, hands) {
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
                match trick_state.step(players, hands) {
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
                match trick_state.step(players, hands) {
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
                match trick_state.step(players, hands) {
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
                match trick_state.step(players, hands) {
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

    fn get_score(bid_result: &BidResultAll, tricks_taken: &[u8; 4]) -> (Position, u8) {
        todo!()
    }
}
