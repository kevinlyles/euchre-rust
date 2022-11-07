use yew::UseStateHandle;

use crate::{
    bid_result::BidResult,
    bid_state::{BidState, BidStateKind},
    hand::HandLogic,
    player::Player,
    trick_state::TrickState,
};

#[derive(Clone, PartialEq)]
pub struct HandState {
    pub dealer: Player,
    pub phase: HandStateKind,
}

#[derive(Clone, PartialEq)]
pub enum HandStateKind {
    Bidding {
        bid_state: UseStateHandle<BidState>,
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
    pub fn finish_bidding(&self) -> Option<HandState> {
        match &self.phase {
            HandStateKind::Bidding { bid_state } => {
                let bid_result = match bid_state.phase {
                    BidStateKind::Called { caller, trump } => Some(BidResult {
                        caller,
                        trump,
                        called_alone: false,
                        defender: None,
                    }),
                    BidStateKind::CalledAlone { caller, trump } => Some(BidResult {
                        caller,
                        trump,
                        called_alone: true,
                        defender: None,
                    }),
                    BidStateKind::DefendedAlone {
                        trump,
                        caller,
                        defender,
                    } => Some(BidResult {
                        caller,
                        trump,
                        called_alone: true,
                        defender: Some(defender),
                    }),
                    _ => panic!("BidState was {:?}", bid_state),
                };
                match bid_result {
                    Some(bid_result) => Some(HandState {
                        dealer: self.dealer,
                        phase: HandStateKind::FirstTrick {
                            bid_result: bid_result.clone(),
                            hands: bid_state.hands.clone(),
                            trick_state: TrickState::Start {
                                leader: self.dealer.next_player(
                                    if bid_result.called_alone {
                                        Some(bid_result.caller)
                                    } else {
                                        None
                                    },
                                    bid_result.defender,
                                ),
                            },
                        },
                    }),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}
