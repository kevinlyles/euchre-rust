use crate::{
    bid_result::BidResult, bid_state::BidState, card::CardLogic, hand::HandLogic, player::Player,
    trick_state::TrickState,
};

pub struct HandState {
    pub dealer: Player,
    pub phase: HandPhase,
}

impl PartialEq for HandState {
    fn eq(&self, other: &Self) -> bool {
        self.dealer == other.dealer && self.phase == other.phase
    }
}

#[derive(PartialEq)]
pub enum HandPhase {
    Initializing,
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
    pub fn create(dealer: Player, hands: [HandLogic; 4], trump_candidate: CardLogic) -> HandState {
        let mut state = HandState {
            dealer,
            phase: HandPhase::Initializing,
        };
        state.phase = HandPhase::Bidding {
            bid_state: BidState::create(dealer, hands, trump_candidate),
        };
        state
    }

    fn update_bid_state(&mut self, bid_state: BidState) {
        self.phase = HandPhase::Bidding {
            bid_state: bid_state,
        };
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

    fn trick_finished(&mut self, trick_winner: Player) -> () {
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
