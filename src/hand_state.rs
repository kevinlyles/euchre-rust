use crate::{
    bid_result::BidResult, bid_state::BidState, card::CardLogic, hand::HandLogic,
    position::Position, trick_state::TrickState,
};

pub struct HandState {
    pub dealer: Position,
    pub phase: HandPhase,
}

impl PartialEq for HandState {
    fn eq(&self, other: &Self) -> bool {
        self.dealer == other.dealer && self.phase == other.phase
    }
}

#[derive(PartialEq)]
pub enum HandPhase {
    Bidding {
        bid_state: BidState,
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
        dealer: Position,
        hands: [HandLogic; 4],
        trump_candidate: CardLogic,
    ) -> HandState {
        HandState {
            dealer,
            phase: HandPhase::Bidding {
                bid_state: BidState::create(dealer, hands, trump_candidate),
            },
        }
    }

    pub fn step(&mut self) -> Option<[u8; 4]> {
        match &mut self.phase {
            HandPhase::Bidding { bid_state } => match bid_state.step() {},
            HandPhase::FirstTrick {
                bid_result,
                hands,
                trick_state,
            } => todo!(),
            HandPhase::SecondTrick {
                bid_result,
                hands,
                trick_state,
                tricks_taken,
            } => todo!(),
            HandPhase::ThirdTrick {
                bid_result,
                hands,
                trick_state,
                tricks_taken,
            } => todo!(),
            HandPhase::FourthTrick {
                bid_result,
                hands,
                trick_state,
                tricks_taken,
            } => todo!(),
            HandPhase::FifthTrick {
                bid_result,
                hands,
                trick_state,
                tricks_taken,
            } => todo!(),
            HandPhase::Scoring { tricks_taken } => todo!(),
        }
    }

    pub fn finish_bidding(&mut self, bid_result: BidResult) -> () {
        match &self.phase {
            HandPhase::Bidding { bid_state } => {
                self.phase = HandPhase::FirstTrick {
                    bid_result: bid_result.clone(),
                    hands: bid_state.hands.clone(),
                    trick_state: TrickState::create(self.dealer.next_player(
                        if bid_result.called_alone {
                            Some(bid_result.caller)
                        } else {
                            None
                        },
                        bid_result.defender,
                    )),
                }
            }
            _ => (),
        }
    }

    fn trick_finished(&mut self, trick_winner: Position) -> () {
        match &mut self.phase {
            HandPhase::FirstTrick {
                bid_result, hands, ..
            } => {
                let mut tricks_taken = [0u8; 4];
                tricks_taken[trick_winner.index()] += 1;
                self.phase = HandPhase::SecondTrick {
                    bid_result: bid_result.clone(),
                    hands: hands.clone(),
                    trick_state: TrickState::create(trick_winner),
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
                    bid_result: bid_result.clone(),
                    hands: hands.clone(),
                    trick_state: TrickState::create(trick_winner),
                    tricks_taken: *tricks_taken,
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
                    bid_result: bid_result.clone(),
                    hands: hands.clone(),
                    trick_state: TrickState::create(trick_winner),
                    tricks_taken: *tricks_taken,
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
                    bid_result: bid_result.clone(),
                    hands: hands.clone(),
                    trick_state: TrickState::create(trick_winner),
                    tricks_taken: *tricks_taken,
                }
            }
            HandPhase::FifthTrick {
                bid_result,
                hands,
                tricks_taken,
                ..
            } => {
                tricks_taken[trick_winner.index()] += 1;
                self.phase = HandPhase::Scoring {
                    tricks_taken: *tricks_taken,
                }
            }
            _ => (),
        }
    }
}
