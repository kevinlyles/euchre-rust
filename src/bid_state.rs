use crate::{
    bid_result::BidResult, card::CardLogic, hand::HandLogic, position::Position, suit::Suit,
};

#[derive(Clone, PartialEq)]
pub struct BidState {
    pub dealer: Position,
    pub hands: [HandLogic; 4],
    pub phase: BidPhase,
}

#[derive(Clone, PartialEq)]
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
        trump: Suit,
    },
    OrderedUpAlone {
        caller: Position,
        trump: Suit,
    },
    OrderedUpDefendedAlone {
        caller: Position,
        trump: Suit,
        defender: Position,
    },
    SecondRoundFirstPlayer {
        forbidden_suit: Suit,
    },
    SecondRoundSecondPlayer {
        forbidden_suit: Suit,
    },
    SecondRoundThirdPlayer {
        forbidden_suit: Suit,
    },
    SecondRoundFourthPlayer {
        forbidden_suit: Suit,
    },
    Done {
        bid_result: BidResult,
    },
}

impl BidState {
    pub fn create(dealer: Position, hands: [HandLogic; 4], trump_candidate: CardLogic) -> BidState {
        BidState {
            dealer,
            hands,
            phase: BidPhase::FirstRoundFirstPlayer { trump_candidate },
        }
    }

    pub fn step(&mut self) -> Option<BidResult> {
        match &mut self.phase {
            BidPhase::FirstRoundFirstPlayer { trump_candidate } => todo!(),
            BidPhase::FirstRoundSecondPlayer { trump_candidate } => todo!(),
            BidPhase::FirstRoundThirdPlayer { trump_candidate } => todo!(),
            BidPhase::FirstRoundFourthPlayer { trump_candidate } => todo!(),
            BidPhase::OrderedUp { caller, trump } => todo!(),
            BidPhase::OrderedUpAlone { caller, trump } => todo!(),
            BidPhase::OrderedUpDefendedAlone {
                caller,
                trump,
                defender,
            } => todo!(),
            BidPhase::SecondRoundFirstPlayer { forbidden_suit } => todo!(),
            BidPhase::SecondRoundSecondPlayer { forbidden_suit } => todo!(),
            BidPhase::SecondRoundThirdPlayer { forbidden_suit } => todo!(),
            BidPhase::SecondRoundFourthPlayer { forbidden_suit } => todo!(),
            BidPhase::Done { bid_result } => todo!(),
        }
    }

    pub fn get_active_player(&self) -> Position {
        match self.phase {
            BidPhase::FirstRoundFirstPlayer { .. }
            | BidPhase::SecondRoundFirstPlayer { .. }
            | BidPhase::Called { .. }
            | BidPhase::NoOneCalled => self.dealer.next_player(None, None),
            BidPhase::FirstRoundSecondPlayer { .. } | BidPhase::SecondRoundSecondPlayer { .. } => {
                self.dealer.partner()
            }
            BidPhase::FirstRoundThirdPlayer { .. } | BidPhase::SecondRoundThirdPlayer { .. } => {
                self.dealer.partner().next_player(None, None)
            }
            BidPhase::FirstRoundFourthPlayer { .. }
            | BidPhase::OrderedUp { .. }
            | BidPhase::OrderedUpAlone { .. }
            | BidPhase::OrderedUpDefendedAlone { .. }
            | BidPhase::SecondRoundFourthPlayer { .. } => self.dealer,
            BidPhase::CalledAlone { caller, .. } => self.dealer.next_player(Some(caller), None),
            BidPhase::DefendedAlone {
                caller, defender, ..
            } => self.dealer.next_player(Some(caller), Some(defender)),
        }
    }

    pub fn get_trump_candidate(&self) -> Option<CardLogic> {
        match self.phase {
            BidPhase::FirstRoundFirstPlayer { trump_candidate }
            | BidPhase::FirstRoundSecondPlayer { trump_candidate }
            | BidPhase::FirstRoundThirdPlayer { trump_candidate }
            | BidPhase::FirstRoundFourthPlayer { trump_candidate } => Some(trump_candidate),
            _ => None,
        }
    }

    pub fn pass(mut self) -> bool {
        match &self.phase {
            BidPhase::FirstRoundFirstPlayer { trump_candidate } => {
                self.phase = BidPhase::FirstRoundSecondPlayer {
                    trump_candidate: *trump_candidate,
                };
                true
            }
            BidPhase::FirstRoundSecondPlayer { trump_candidate } => {
                self.phase = BidPhase::FirstRoundThirdPlayer {
                    trump_candidate: *trump_candidate,
                };
                true
            }
            BidPhase::FirstRoundThirdPlayer { trump_candidate } => {
                self.phase = BidPhase::FirstRoundFourthPlayer {
                    trump_candidate: *trump_candidate,
                };
                true
            }
            BidPhase::FirstRoundFourthPlayer { trump_candidate } => {
                self.phase = BidPhase::SecondRoundFirstPlayer {
                    forbidden_suit: trump_candidate.suit,
                };
                true
            }
            BidPhase::SecondRoundFirstPlayer { forbidden_suit } => {
                self.phase = BidPhase::SecondRoundSecondPlayer {
                    forbidden_suit: *forbidden_suit,
                };
                true
            }
            BidPhase::SecondRoundSecondPlayer { forbidden_suit } => {
                self.phase = BidPhase::SecondRoundThirdPlayer {
                    forbidden_suit: *forbidden_suit,
                };
                true
            }
            BidPhase::SecondRoundThirdPlayer { forbidden_suit } => {
                self.phase = BidPhase::SecondRoundFourthPlayer {
                    forbidden_suit: *forbidden_suit,
                };
                true
            }
            BidPhase::SecondRoundFourthPlayer { .. } => {
                self.phase = BidPhase::NoOneCalled;
                true
            }
            _ => false,
        }
    }

    pub fn order_it_up(&mut self, alone: bool, defending_alone: Option<Position>) -> bool {
        match self.phase {
            BidPhase::FirstRoundFirstPlayer { trump_candidate }
            | BidPhase::FirstRoundSecondPlayer { trump_candidate }
            | BidPhase::FirstRoundThirdPlayer { trump_candidate }
            | BidPhase::FirstRoundFourthPlayer { trump_candidate } => {
                let caller = self.get_active_player();
                self.phase = match alone {
                    true => match defending_alone {
                        Some(defender) => BidPhase::OrderedUpDefendedAlone {
                            caller,
                            trump: trump_candidate.suit,
                            defender,
                        },
                        None => BidPhase::OrderedUpAlone {
                            caller,
                            trump: trump_candidate.suit,
                        },
                    },
                    false => BidPhase::OrderedUp {
                        caller,
                        trump: trump_candidate.suit,
                    },
                };
                true
            }
            _ => false,
        }
    }

    pub fn discard(&mut self, card: CardLogic) -> bool {
        match self.phase {
            BidPhase::OrderedUp { trump, caller }
                if self.hands[self.dealer.index()].cards.contains(&card) =>
            {
                self.phase = BidPhase::Called { caller, trump };
                /*
                self.finished_callback.emit(BidResult {
                    caller,
                    trump,
                    called_alone: false,
                    defender: None,
                });
                */
                true
            }
            BidPhase::OrderedUpAlone { trump, caller } => {
                BidPhase::CalledAlone { caller, trump };
                /*
                self.finished_callback.emit(BidResult {
                    caller,
                    trump,
                    called_alone: true,
                    defender: None,
                });
                */
                true
            }
            BidPhase::OrderedUpDefendedAlone {
                trump,
                caller,
                defender,
            } => {
                self.phase = BidPhase::DefendedAlone {
                    trump,
                    caller,
                    defender,
                };
                /*
                self.finished_callback.emit(BidResult {
                    caller,
                    trump,
                    called_alone: true,
                    defender: Some(defender),
                })
                */

                true
            }
            _ => false,
        }
    }

    pub fn call(&mut self, trump: Suit, alone: bool, defending_alone: Option<Position>) -> bool {
        match self.phase {
            BidPhase::SecondRoundFirstPlayer { forbidden_suit }
            | BidPhase::SecondRoundSecondPlayer { forbidden_suit }
            | BidPhase::SecondRoundThirdPlayer { forbidden_suit }
            | BidPhase::SecondRoundFourthPlayer { forbidden_suit }
                if trump != forbidden_suit =>
            {
                let caller = self.get_active_player();
                self.phase = match alone {
                    true => match defending_alone {
                        Some(defender) => BidPhase::DefendedAlone {
                            trump,
                            caller,
                            defender,
                        },
                        None => BidPhase::CalledAlone { caller, trump },
                    },
                    false => BidPhase::Called { caller, trump },
                };
                true
            }
            _ => false,
        }
    }
}

fn discard_card(
    mut hands: [HandLogic; 4],
    dealer: &Position,
    discard: &CardLogic,
) -> [HandLogic; 4] {
    hands[dealer.index()].cards.retain(|card| card != discard);
    hands
}

fn get_ordered_up_hands(
    mut hands: [HandLogic; 4],
    dealer: &Position,
    trump_candidate: CardLogic,
) -> [HandLogic; 4] {
    hands[dealer.index()].cards.push(trump_candidate);
    hands
}
