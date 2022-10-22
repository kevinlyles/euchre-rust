use yew::html::IntoPropValue;

use crate::{card::CardProps, player::Player, suit::Suit};

#[derive(Clone, Copy, PartialEq)]
pub struct BidState {
    pub dealer: Player,
    pub phase: BidStateKind,
}

#[derive(Clone, Copy, PartialEq)]
pub enum BidStateKind {
    FirstRoundFirstPlayer {
        trump_candidate: CardProps,
    },
    FirstRoundSecondPlayer {
        trump_candidate: CardProps,
    },
    FirstRoundThirdPlayer {
        trump_candidate: CardProps,
    },
    FirstRoundFourthPlayer {
        trump_candidate: CardProps,
    },
    OrderedUp {
        trump_candidate: CardProps,
        caller: Player,
    },
    OrderedUpAlone {
        trump_candidate: CardProps,
        caller: Player,
    },
    OrderedUpDefendedAlone {
        trump_candidate: CardProps,
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
    fn get_active_player(&self) -> Player {
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
    pub fn pass(&self) -> Option<BidState> {
        match self.phase {
            BidStateKind::FirstRoundFirstPlayer { trump_candidate } => Some(BidState {
                dealer: self.dealer,
                phase: BidStateKind::FirstRoundSecondPlayer { trump_candidate },
            }),
            BidStateKind::FirstRoundSecondPlayer { trump_candidate } => Some(BidState {
                dealer: self.dealer,
                phase: BidStateKind::FirstRoundThirdPlayer { trump_candidate },
            }),
            BidStateKind::FirstRoundThirdPlayer { trump_candidate } => Some(BidState {
                dealer: self.dealer,
                phase: BidStateKind::FirstRoundFourthPlayer { trump_candidate },
            }),
            BidStateKind::FirstRoundFourthPlayer { trump_candidate } => Some(BidState {
                dealer: self.dealer,
                phase: BidStateKind::SecondRoundFirstPlayer {
                    forbidden_suit: trump_candidate.suit,
                },
            }),
            BidStateKind::SecondRoundFirstPlayer { forbidden_suit } => Some(BidState {
                dealer: self.dealer,
                phase: BidStateKind::SecondRoundSecondPlayer { forbidden_suit },
            }),
            BidStateKind::SecondRoundSecondPlayer { forbidden_suit } => Some(BidState {
                dealer: self.dealer,
                phase: BidStateKind::SecondRoundThirdPlayer { forbidden_suit },
            }),
            BidStateKind::SecondRoundThirdPlayer { forbidden_suit } => Some(BidState {
                dealer: self.dealer,
                phase: BidStateKind::SecondRoundFourthPlayer { forbidden_suit },
            }),
            BidStateKind::SecondRoundFourthPlayer { .. } => Some(BidState {
                dealer: self.dealer,
                phase: BidStateKind::NoOneCalled,
            }),
            _ => None,
        }
    }
}

impl IntoPropValue<Option<CardProps>> for &BidState {
    fn into_prop_value(self) -> Option<CardProps> {
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
}
