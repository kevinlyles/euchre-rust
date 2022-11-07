use yew::Callback;

use crate::{bid_result::BidResult, card::CardLogic, hand::HandLogic, player::Player, suit::Suit};

pub struct BidState {
    pub dealer: Player,
    pub hands: [HandLogic; 4],
    pub phase: BidPhase,
    update_callback: Box<dyn FnMut(BidState) -> ()>,
    finished_callback: Box<dyn FnMut(BidResult) -> ()>,
}

impl PartialEq for BidState {
    fn eq(&self, other: &Self) -> bool {
        self.dealer == other.dealer && self.hands == other.hands && self.phase == other.phase
    }
}

#[derive(Clone, PartialEq, Debug)]
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
        caller: Player,
        trump: Suit,
    },
    OrderedUpAlone {
        caller: Player,
        trump: Suit,
    },
    OrderedUpDefendedAlone {
        caller: Player,
        trump: Suit,
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
    pub fn create(
        dealer: Player,
        hands: [HandLogic; 4],
        trump_candidate: CardLogic,
        update_callback: Box<dyn FnMut(BidState) -> ()>,
        finished_callback: Box<dyn FnMut(BidResult) -> ()>,
    ) -> Box<BidState> {
        Box::new(BidState {
            dealer,
            hands,
            phase: BidPhase::FirstRoundFirstPlayer { trump_candidate },
            update_callback,
            finished_callback,
        })
    }

    pub fn get_active_player(&self) -> Player {
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

    pub fn pass(&mut self) -> bool {
        match self.phase {
            BidPhase::FirstRoundFirstPlayer { trump_candidate } => {
                self.phase = BidPhase::FirstRoundSecondPlayer { trump_candidate };
                (self.update_callback)(*self);
                true
            }
            BidPhase::FirstRoundSecondPlayer { trump_candidate } => {
                self.phase = BidPhase::FirstRoundThirdPlayer { trump_candidate };
                (self.update_callback)(*self);
                true
            }
            BidPhase::FirstRoundThirdPlayer { trump_candidate } => {
                self.phase = BidPhase::FirstRoundFourthPlayer { trump_candidate };
                (self.update_callback)(*self);
                true
            }
            BidPhase::FirstRoundFourthPlayer { trump_candidate } => {
                self.phase = BidPhase::SecondRoundFirstPlayer {
                    forbidden_suit: trump_candidate.suit,
                };
                (self.update_callback)(*self);
                true
            }
            BidPhase::SecondRoundFirstPlayer { forbidden_suit } => {
                self.phase = BidPhase::SecondRoundSecondPlayer { forbidden_suit };
                (self.update_callback)(*self);
                true
            }
            BidPhase::SecondRoundSecondPlayer { forbidden_suit } => {
                self.phase = BidPhase::SecondRoundThirdPlayer { forbidden_suit };
                (self.update_callback)(*self);
                true
            }
            BidPhase::SecondRoundThirdPlayer { forbidden_suit } => {
                self.phase = BidPhase::SecondRoundFourthPlayer { forbidden_suit };
                (self.update_callback)(*self);
                true
            }
            BidPhase::SecondRoundFourthPlayer { .. } => {
                self.phase = BidPhase::NoOneCalled;
                (self.update_callback)(*self);
                true
            }
            _ => false,
        }
    }

    pub fn order_it_up(&mut self, alone: bool, defending_alone: Option<Player>) -> bool {
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
                (self.update_callback)(*self);
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
                (self.finished_callback)(BidResult {
                    caller,
                    trump,
                    called_alone: false,
                    defender: None,
                });
                true
            }
            BidPhase::OrderedUpAlone { trump, caller } => {
                BidPhase::CalledAlone { caller, trump };
                (self.finished_callback)(BidResult {
                    caller,
                    trump,
                    called_alone: true,
                    defender: None,
                });
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
                (self.finished_callback)(BidResult {
                    caller,
                    trump,
                    called_alone: true,
                    defender: Some(defender),
                });
                true
            }
            _ => false,
        }
    }

    pub fn call(&mut self, trump: Suit, alone: bool, defending_alone: Option<Player>) -> bool {
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

fn discard_card(mut hands: [HandLogic; 4], dealer: &Player, discard: &CardLogic) -> [HandLogic; 4] {
    hands[dealer.index()].cards.retain(|card| card != discard);
    hands
}

fn get_ordered_up_hands(
    mut hands: [HandLogic; 4],
    dealer: &Player,
    trump_candidate: CardLogic,
) -> [HandLogic; 4] {
    hands[dealer.index()].cards.push(trump_candidate);
    hands
}
