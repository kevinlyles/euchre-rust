use crate::{card::CardLogic, hand::HandLogic, player::Player, suit::Suit};

#[derive(Clone, PartialEq)]
pub struct BidState {
    pub dealer: Player,
    pub hands: [HandLogic; 4],
    pub phase: BidStateKind,
}

#[derive(Clone, PartialEq)]
pub enum BidStateKind {
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
        trump_candidate: CardLogic,
        caller: Player,
    },
    OrderedUpAlone {
        trump_candidate: CardLogic,
        caller: Player,
    },
    OrderedUpDefendedAlone {
        trump_candidate: CardLogic,
        caller: Player,
        defender: Player,
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
    Called {
        caller: Player,
        trump: Suit,
    },
    CalledAlone {
        caller: Player,
        trump: Suit,
    },
    DefendedAlone {
        trump: Suit,
        caller: Player,
        defender: Player,
    },
    NoOneCalled,
}

impl BidState {
    pub fn get_active_player(&self) -> Player {
        match self.phase {
            BidStateKind::FirstRoundFirstPlayer { .. }
            | BidStateKind::SecondRoundFirstPlayer { .. }
            | BidStateKind::Called { .. }
            | BidStateKind::NoOneCalled => self.dealer.next_player(None, None),
            BidStateKind::FirstRoundSecondPlayer { .. }
            | BidStateKind::SecondRoundSecondPlayer { .. } => self.dealer.partner(),
            BidStateKind::FirstRoundThirdPlayer { .. }
            | BidStateKind::SecondRoundThirdPlayer { .. } => {
                self.dealer.partner().next_player(None, None)
            }
            BidStateKind::FirstRoundFourthPlayer { .. }
            | BidStateKind::OrderedUp { .. }
            | BidStateKind::OrderedUpAlone { .. }
            | BidStateKind::OrderedUpDefendedAlone { .. }
            | BidStateKind::SecondRoundFourthPlayer { .. } => self.dealer,
            BidStateKind::CalledAlone { caller, .. } => self.dealer.next_player(Some(caller), None),
            BidStateKind::DefendedAlone {
                caller, defender, ..
            } => self.dealer.next_player(Some(caller), Some(defender)),
        }
    }

    pub fn order_it_up(&self, alone: bool, defending_alone: Option<Player>) -> Option<BidState> {
        match self.phase {
            BidStateKind::FirstRoundFirstPlayer { trump_candidate }
            | BidStateKind::FirstRoundSecondPlayer { trump_candidate }
            | BidStateKind::FirstRoundThirdPlayer { trump_candidate }
            | BidStateKind::FirstRoundFourthPlayer { trump_candidate } => {
                let caller = self.get_active_player();
                Some(BidState {
                    dealer: self.dealer,
                    hands: self.hands.clone(),
                    phase: match alone {
                        true => match defending_alone {
                            Some(defender) => BidStateKind::OrderedUpDefendedAlone {
                                trump_candidate,
                                caller,
                                defender,
                            },
                            None => BidStateKind::OrderedUpAlone {
                                trump_candidate,
                                caller,
                            },
                        },
                        false => BidStateKind::OrderedUp {
                            trump_candidate,
                            caller,
                        },
                    },
                })
            }
            _ => None,
        }
    }

    pub fn call(
        &self,
        trump: Suit,
        alone: bool,
        defending_alone: Option<Player>,
    ) -> Option<BidState> {
        match self.phase {
            BidStateKind::SecondRoundFirstPlayer { forbidden_suit }
            | BidStateKind::SecondRoundSecondPlayer { forbidden_suit }
            | BidStateKind::SecondRoundThirdPlayer { forbidden_suit }
            | BidStateKind::SecondRoundFourthPlayer { forbidden_suit }
                if trump != forbidden_suit =>
            {
                let caller = self.get_active_player();
                Some(BidState {
                    dealer: self.dealer,
                    hands: self.hands.clone(),
                    phase: match alone {
                        true => match defending_alone {
                            Some(defender) => BidStateKind::DefendedAlone {
                                trump,
                                caller,
                                defender,
                            },
                            None => BidStateKind::CalledAlone { caller, trump },
                        },
                        false => BidStateKind::Called { caller, trump },
                    },
                })
            }
            _ => None,
        }
    }

    pub fn pass(&self) -> Option<BidState> {
        match self.phase {
            BidStateKind::FirstRoundFirstPlayer { trump_candidate } => Some(BidState {
                dealer: self.dealer,
                hands: self.hands.clone(),
                phase: BidStateKind::FirstRoundSecondPlayer { trump_candidate },
            }),
            BidStateKind::FirstRoundSecondPlayer { trump_candidate } => Some(BidState {
                dealer: self.dealer,
                hands: self.hands.clone(),
                phase: BidStateKind::FirstRoundThirdPlayer { trump_candidate },
            }),
            BidStateKind::FirstRoundThirdPlayer { trump_candidate } => Some(BidState {
                dealer: self.dealer,
                hands: self.hands.clone(),
                phase: BidStateKind::FirstRoundFourthPlayer { trump_candidate },
            }),
            BidStateKind::FirstRoundFourthPlayer { trump_candidate } => Some(BidState {
                dealer: self.dealer,
                hands: self.hands.clone(),
                phase: BidStateKind::SecondRoundFirstPlayer {
                    forbidden_suit: trump_candidate.suit,
                },
            }),
            BidStateKind::SecondRoundFirstPlayer { forbidden_suit } => Some(BidState {
                dealer: self.dealer,
                hands: self.hands.clone(),
                phase: BidStateKind::SecondRoundSecondPlayer { forbidden_suit },
            }),
            BidStateKind::SecondRoundSecondPlayer { forbidden_suit } => Some(BidState {
                dealer: self.dealer,
                hands: self.hands.clone(),
                phase: BidStateKind::SecondRoundThirdPlayer { forbidden_suit },
            }),
            BidStateKind::SecondRoundThirdPlayer { forbidden_suit } => Some(BidState {
                dealer: self.dealer,
                hands: self.hands.clone(),
                phase: BidStateKind::SecondRoundFourthPlayer { forbidden_suit },
            }),
            BidStateKind::SecondRoundFourthPlayer { .. } => Some(BidState {
                dealer: self.dealer,
                hands: self.hands.clone(),
                phase: BidStateKind::NoOneCalled,
            }),
            _ => None,
        }
    }

    pub fn get_trump_candidate(&self) -> Option<CardLogic> {
        match self.phase {
            BidStateKind::FirstRoundFirstPlayer { trump_candidate }
            | BidStateKind::FirstRoundSecondPlayer { trump_candidate }
            | BidStateKind::FirstRoundThirdPlayer { trump_candidate }
            | BidStateKind::FirstRoundFourthPlayer { trump_candidate }
            | BidStateKind::OrderedUp {
                caller: _,
                trump_candidate,
            } => Some(trump_candidate),
            _ => None,
        }
    }

    pub fn discard(&self, card: CardLogic) -> Option<BidState> {
        match self.phase {
            BidStateKind::OrderedUp {
                trump_candidate,
                caller,
            } => {
                todo!()
            }
            BidStateKind::OrderedUpAlone {
                trump_candidate,
                caller,
            } => {
                todo!()
            }
            BidStateKind::OrderedUpDefendedAlone {
                trump_candidate,
                caller,
                defender,
            } => {
                todo!()
            }
            _ => None,
        }
    }
}
