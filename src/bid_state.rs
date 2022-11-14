use crate::{
    bid_result::BidResultAll, card::CardLogic, hand::HandLogic, player::Player, position::Position,
};

pub struct BidState {
    pub dealer: Position,
    pub phase: BidPhase,
}

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
        card_ordered: CardLogic,
    },
    OrderedUpAlone {
        caller: Position,
        card_ordered: CardLogic,
    },
    OrderedUpDefendedAlone {
        caller: Position,
        card_ordered: CardLogic,
        defender: Position,
    },
    SecondRoundFirstPlayer {
        turned_down: CardLogic,
    },
    SecondRoundSecondPlayer {
        turned_down: CardLogic,
    },
    SecondRoundThirdPlayer {
        turned_down: CardLogic,
    },
    SecondRoundFourthPlayer {
        turned_down: CardLogic,
    },
    Done {
        bid_result: BidResultAll,
    },
}

impl BidState {
    pub fn create(dealer: Position, trump_candidate: CardLogic) -> BidState {
        BidState {
            dealer,
            phase: BidPhase::FirstRoundFirstPlayer { trump_candidate },
        }
    }

    pub fn step(
        &mut self,
        players: &mut [Box<dyn Player>; 4],
        hands: &mut [HandLogic; 4],
    ) -> Option<BidResultAll> {
        match &mut self.phase {
            BidPhase::FirstRoundFirstPlayer {
                ref trump_candidate,
            } => {
                let bidder = self.dealer.next_position_bidding();
                self.phase =
                    match BidState::order_up(&self.dealer, bidder, trump_candidate, players, hands)
                    {
                        Some(phase) => phase,
                        None => BidPhase::FirstRoundSecondPlayer {
                            trump_candidate: trump_candidate.clone(),
                        },
                    };
                None
            }
            BidPhase::FirstRoundSecondPlayer { trump_candidate } => {
                let bidder = self.dealer.partner();
                self.phase =
                    match BidState::order_up(&self.dealer, bidder, trump_candidate, players, hands)
                    {
                        Some(phase) => phase,
                        None => BidPhase::FirstRoundThirdPlayer {
                            trump_candidate: trump_candidate.clone(),
                        },
                    };
                None
            }
            BidPhase::FirstRoundThirdPlayer { trump_candidate } => {
                let bidder = self.dealer.partner().next_position_bidding();
                self.phase =
                    match BidState::order_up(&self.dealer, bidder, trump_candidate, players, hands)
                    {
                        Some(phase) => phase,
                        None => BidPhase::FirstRoundFourthPlayer {
                            trump_candidate: trump_candidate.clone(),
                        },
                    };
                None
            }
            BidPhase::FirstRoundFourthPlayer { trump_candidate } => {
                let bidder = self.dealer;
                self.phase =
                    match BidState::order_up(&self.dealer, bidder, trump_candidate, players, hands)
                    {
                        Some(phase) => phase,
                        None => BidPhase::SecondRoundFirstPlayer {
                            turned_down: *trump_candidate,
                        },
                    };
                None
            }
            BidPhase::OrderedUp {
                caller,
                card_ordered,
            } => {
                let hand = &mut hands[self.dealer.index()];
                hand.cards.push(*card_ordered);
                let discard =
                    players[self.dealer.index()].choose_discard(&hand, &card_ordered.suit);
                hand.cards.retain(|card| *card != discard);
                self.phase = BidPhase::Done {
                    bid_result: BidResultAll::Called {
                        trump: card_ordered.suit,
                        caller: *caller,
                    },
                };
                None
            }
            BidPhase::OrderedUpAlone {
                caller,
                card_ordered,
            } => {
                let hand = &mut hands[self.dealer.index()];
                hand.cards.push(*card_ordered);
                let discard =
                    players[self.dealer.index()].choose_discard(&hand, &card_ordered.suit);
                hand.cards.retain(|card| *card != discard);
                self.phase = BidPhase::Done {
                    bid_result: BidResultAll::CalledAlone {
                        trump: card_ordered.suit,
                        caller: *caller,
                    },
                };
                None
            }
            BidPhase::OrderedUpDefendedAlone {
                caller,
                card_ordered,
                defender,
            } => {
                let hand = &mut hands[self.dealer.index()];
                hand.cards.push(*card_ordered);
                let discard =
                    players[self.dealer.index()].choose_discard(&hand, &card_ordered.suit);
                hand.cards.retain(|card| *card != discard);
                self.phase = BidPhase::Done {
                    bid_result: BidResultAll::DefendedAlone {
                        trump: card_ordered.suit,
                        caller: *caller,
                        defender: *defender,
                    },
                };
                None
            }
            BidPhase::SecondRoundFirstPlayer { ref turned_down } => {
                let bidder = self.dealer.next_position_bidding();
                self.phase = match BidState::call(&self.dealer, bidder, players, hands, turned_down)
                {
                    Some(bid_result) => BidPhase::Done { bid_result },
                    _ => BidPhase::SecondRoundSecondPlayer {
                        turned_down: *turned_down,
                    },
                };
                None
            }
            BidPhase::SecondRoundSecondPlayer { ref turned_down } => {
                let bidder = self.dealer.partner();
                self.phase = match BidState::call(&self.dealer, bidder, players, hands, turned_down)
                {
                    Some(bid_result) => BidPhase::Done { bid_result },
                    _ => BidPhase::SecondRoundThirdPlayer {
                        turned_down: *turned_down,
                    },
                };
                None
            }
            BidPhase::SecondRoundThirdPlayer { ref turned_down } => {
                let bidder = self.dealer.partner().next_position_bidding();
                self.phase = match BidState::call(&self.dealer, bidder, players, hands, turned_down)
                {
                    Some(bid_result) => BidPhase::Done { bid_result },
                    _ => BidPhase::SecondRoundFourthPlayer {
                        turned_down: *turned_down,
                    },
                };
                None
            }
            BidPhase::SecondRoundFourthPlayer { ref turned_down } => {
                let bidder = self.dealer;
                self.phase = match BidState::call(&self.dealer, bidder, players, hands, turned_down)
                {
                    Some(bid_result) => BidPhase::Done { bid_result },
                    _ => {
                        log::info!("{}", "No one called a trump suit");
                        BidPhase::Done {
                            bid_result: BidResultAll::NoOneCalled,
                        }
                    }
                };
                None
            }
            BidPhase::Done { bid_result } => Some(bid_result.clone()),
        }
    }

    fn order_up(
        dealer: &Position,
        bidder: Position,
        trump_candidate: &CardLogic,
        players: &mut [Box<dyn Player>; 4],
        hands: &[HandLogic; 4],
    ) -> Option<BidPhase> {
        let bidder_index = bidder.index();
        if !players[bidder_index].should_order_up(&hands[bidder_index], dealer, trump_candidate) {
            return None;
        }
        log::info!("{:?} ordered up {}", bidder, trump_candidate);
        if !players[bidder_index].should_order_up_alone(
            &hands[bidder_index],
            dealer,
            &trump_candidate,
        ) {
            return Some(BidPhase::OrderedUp {
                caller: bidder,
                card_ordered: *trump_candidate,
            });
        }
        let defender = bidder.next_position_bidding();
        let defender_index = defender.index();
        if players[defender_index].should_defend_alone_ordered(
            &hands[defender_index],
            dealer,
            &trump_candidate,
        ) {
            Some(BidPhase::OrderedUpDefendedAlone {
                caller: bidder,
                card_ordered: *trump_candidate,
                defender,
            })
        } else {
            let defender = defender.partner();
            let defender_index = defender.index();
            if players[defender_index].should_defend_alone_ordered(
                &hands[defender_index],
                dealer,
                trump_candidate,
            ) {
                Some(BidPhase::OrderedUpDefendedAlone {
                    caller: bidder,
                    card_ordered: *trump_candidate,
                    defender,
                })
            } else {
                Some(BidPhase::OrderedUpAlone {
                    caller: bidder,
                    card_ordered: *trump_candidate,
                })
            }
        }
    }

    fn call(
        dealer: &Position,
        bidder: Position,
        players: &mut [Box<dyn Player>; 4],
        hands: &[HandLogic; 4],
        turned_down: &CardLogic,
    ) -> Option<BidResultAll> {
        match players[bidder.index()].call_trump(&hands[bidder.index()], &dealer, turned_down) {
            Some(trump) if trump != turned_down.suit => {
                log::info!("{:?} called {}", bidder, trump);
                if !players[bidder.index()].should_call_alone(
                    &hands[bidder.index()],
                    &dealer,
                    &trump,
                    &turned_down,
                ) {
                    return Some(BidResultAll::Called {
                        trump,
                        caller: bidder,
                    });
                }
                let defender = bidder.next_position_bidding();
                if players[defender.index()].should_defend_alone_called(
                    &hands[defender.index()],
                    &dealer,
                    &trump,
                    &turned_down,
                ) {
                    Some(BidResultAll::DefendedAlone {
                        trump,
                        caller: bidder,
                        defender,
                    })
                } else {
                    let defender = defender.partner();
                    if players[defender.index()].should_defend_alone_called(
                        &hands[defender.index()],
                        &dealer,
                        &trump,
                        &turned_down,
                    ) {
                        Some(BidResultAll::DefendedAlone {
                            trump,
                            caller: bidder,
                            defender,
                        })
                    } else {
                        Some(BidResultAll::CalledAlone {
                            trump,
                            caller: bidder,
                        })
                    }
                }
            }
            _ => None,
        };
        None
    }
}
