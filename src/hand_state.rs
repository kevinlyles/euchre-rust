use rayon::{
    iter::{IterBridge, MapWith},
    prelude::{ParallelBridge, ParallelIterator},
};

use crate::{
    bid_result::{BidResultAll, BidResultCalled},
    bid_state::BidState,
    card::Card,
    deck::Deck,
    hand::Hand,
    hands_iterator::{CardLocation, HandsIterator},
    player::Player,
    position::Position,
    suit::Suit,
    trick_state::TrickState,
};

#[derive(Debug)]
pub struct HandState {
    pub dealer: Position,
    hands: [Hand; 4],
    pub phase: HandPhase,
}

#[derive(Debug)]
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
    pub fn create(dealer: Position, trump_candidate: Card, hands: [Hand; 4]) -> HandState {
        HandState {
            dealer,
            hands,
            phase: HandPhase::Bidding {
                bid_state: BidState::create(dealer, trump_candidate),
            },
        }
    }

    pub fn create_with_scenario(
        dealer: Position,
        trump_candidate: Card,
        my_hand: Hand,
    ) -> MapWith<
        IterBridge<HandsIterator>,
        (Position, Card, Hand, [Card; 18]),
        impl Fn(&mut (Position, Card, Hand, [Card; 18]), [CardLocation; 18]) -> HandState,
    > {
        let my_hand = my_hand.clone();
        let mut available_cards = Deck::create_all_cards();
        available_cards.retain(|&card| card != trump_candidate && !my_hand.cards.contains(&card));
        let available_cards = available_cards.try_into().unwrap();
        HandsIterator::create().par_bridge().map_with(
            (dealer, trump_candidate, my_hand, available_cards),
            |(dealer, trump_candidate, my_hand, available_cards), permutation| {
                HandState::create(
                    *dealer,
                    *trump_candidate,
                    HandState::generate_hands(my_hand, &available_cards, permutation),
                )
            },
        )
    }

    fn generate_hands(
        my_hand: &Hand,
        available_cards: &[Card; 18],
        permutation: [CardLocation; 18],
    ) -> [Hand; 4] {
        let mut hands = [
            Hand {
                cards: Vec::with_capacity(6),
            },
            Hand {
                cards: Vec::with_capacity(6),
            },
            my_hand.clone(),
            Hand {
                cards: Vec::with_capacity(6),
            },
        ];
        for (&location, &card) in permutation.iter().zip(available_cards) {
            match location {
                CardLocation::West => hands[Position::West.index()].cards.push(card),
                CardLocation::North => hands[Position::North.index()].cards.push(card),
                CardLocation::East => hands[Position::East.index()].cards.push(card),
                CardLocation::Kitty => (),
            }
        }
        hands
    }

    pub fn step(&mut self, players: &mut [impl Player; 4]) -> Option<(Position, u8)> {
        match &mut self.phase {
            HandPhase::Bidding { bid_state } => {
                match bid_state.step(players, &mut self.hands) {
                    Some(bid_result) => {
                        self.phase = match bid_result {
                            BidResultAll::Called { .. }
                            | BidResultAll::CalledAlone { .. }
                            | BidResultAll::DefendedAlone { .. } => {
                                let bid_result: BidResultCalled = bid_result.try_into().unwrap();
                                HandState::update_bowers(&mut self.hands, &bid_result.trump());
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
                match trick_state.step(players, &mut self.hands) {
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
                match trick_state.step(players, &mut self.hands) {
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
                match trick_state.step(players, &mut self.hands) {
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
                match trick_state.step(players, &mut self.hands) {
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
                match trick_state.step(players, &mut self.hands) {
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

    pub fn finish_bidding(&mut self, players: &mut [impl Player; 4]) -> Option<BidResultCalled> {
        loop {
            match &self.phase {
                HandPhase::Bidding { .. } => {
                    self.step(players);
                }
                HandPhase::FirstTrick { bid_result, .. }
                | HandPhase::SecondTrick { bid_result, .. }
                | HandPhase::ThirdTrick { bid_result, .. }
                | HandPhase::FourthTrick { bid_result, .. }
                | HandPhase::FifthTrick { bid_result, .. } => {
                    return Some(bid_result.clone());
                }
                HandPhase::Scoring { .. } => {
                    return None;
                }
            }
        }
    }

    fn update_bowers(hands: &mut [Hand; 4], trump: &Suit) -> () {
        for hand in hands.iter_mut() {
            Hand::update_bowers(hand, trump);
        }
    }

    fn get_score(bid_result: &BidResultAll, tricks_taken: &[u8; 4]) -> (Position, u8) {
        match bid_result {
            BidResultAll::Called { caller, .. } => {
                let caller_tricks =
                    tricks_taken[caller.index()] + tricks_taken[caller.partner().index()];
                if caller_tricks >= 3 {
                    if caller_tricks >= 5 {
                        (*caller, 2)
                    } else {
                        (*caller, 1)
                    }
                } else {
                    (caller.next_position_bidding(), 2)
                }
            }
            BidResultAll::CalledAlone { caller, .. } => {
                let caller_tricks = tricks_taken[caller.index()];
                if caller_tricks >= 3 {
                    if caller_tricks >= 5 {
                        (*caller, 4)
                    } else {
                        (*caller, 1)
                    }
                } else {
                    (caller.next_position_bidding(), 2)
                }
            }
            BidResultAll::DefendedAlone {
                caller, defender, ..
            } => {
                let caller_tricks = tricks_taken[caller.index()];
                if caller_tricks >= 3 {
                    if caller_tricks >= 5 {
                        (*caller, 4)
                    } else {
                        (*caller, 1)
                    }
                } else {
                    (*defender, 4)
                }
            }
            BidResultAll::NoOneCalled => (Position::South, 0),
        }
    }
}
