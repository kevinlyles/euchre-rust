use rayon::{
    iter::{IterBridge, MapWith},
    prelude::{ParallelBridge, ParallelIterator},
};

use crate::{
    bid_result::{BidResultAll, BidResultCalled},
    bid_state::BidState,
    card::CardBeforeBidding,
    deck::Deck,
    hand::{Hand, HandBeforeBidding},
    hands_iterator::{CardLocation, HandsIterator},
    player::Player,
    position::Position,
    suit::Suit,
    trick_state::TrickState,
};

#[derive(Debug)]
pub(crate) struct HandState {
    pub(crate) dealer: Position,
    pub(crate) phase: HandPhase,
}

#[derive(Debug)]
pub(crate) enum HandPhase {
    Bidding {
        bid_state: BidState,
        hands: [HandBeforeBidding; 4],
    },
    FirstTrick {
        bid_result: BidResultCalled,
        hands: [Hand; 4],
        trick_state: TrickState,
    },
    SecondTrick {
        bid_result: BidResultCalled,
        hands: [Hand; 4],
        trick_state: TrickState,
        tricks_taken: [u8; 4],
    },
    ThirdTrick {
        bid_result: BidResultCalled,
        hands: [Hand; 4],
        trick_state: TrickState,
        tricks_taken: [u8; 4],
    },
    FourthTrick {
        bid_result: BidResultCalled,
        hands: [Hand; 4],
        trick_state: TrickState,
        tricks_taken: [u8; 4],
    },
    FifthTrick {
        bid_result: BidResultCalled,
        hands: [Hand; 4],
        trick_state: TrickState,
        tricks_taken: [u8; 4],
    },
    Scoring {
        bid_result: BidResultAll,
        tricks_taken: [u8; 4],
    },
}

impl HandState {
    pub(crate) fn create(
        dealer: Position,
        trump_candidate: CardBeforeBidding,
        hands: [HandBeforeBidding; 4],
    ) -> HandState {
        HandState {
            dealer,
            phase: HandPhase::Bidding {
                hands,
                bid_state: BidState::create(dealer, trump_candidate),
            },
        }
    }

    pub(crate) fn create_with_scenario(
        dealer: Position,
        trump_candidate: CardBeforeBidding,
        my_hand: HandBeforeBidding,
    ) -> MapWith<
        IterBridge<HandsIterator>,
        (
            Position,
            CardBeforeBidding,
            HandBeforeBidding,
            [CardBeforeBidding; 18],
        ),
        impl Fn(
            &mut (
                Position,
                CardBeforeBidding,
                HandBeforeBidding,
                [CardBeforeBidding; 18],
            ),
            [CardLocation; 18],
        ) -> HandState,
    > {
        let mut available_cards = Deck::create_all_cards();
        available_cards.retain(|&card| trump_candidate != card && !my_hand.cards.contains(&card));
        let available_cards = available_cards.try_into().unwrap();
        HandsIterator::create().par_bridge().map_with(
            (dealer, trump_candidate, my_hand, available_cards),
            |(dealer, trump_candidate, my_hand, available_cards), permutation| {
                HandState::create(
                    *dealer,
                    *trump_candidate,
                    HandState::generate_hands(my_hand, available_cards, permutation),
                )
            },
        )
    }

    fn generate_hands(
        my_hand: &HandBeforeBidding,
        available_cards: &[CardBeforeBidding; 18],
        permutation: [CardLocation; 18],
    ) -> [HandBeforeBidding; 4] {
        let mut hands = [
            HandBeforeBidding {
                cards: Vec::with_capacity(6),
            },
            HandBeforeBidding {
                cards: Vec::with_capacity(6),
            },
            my_hand.clone(),
            HandBeforeBidding {
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

    pub(crate) fn step(&mut self, players: &mut [impl Player; 4]) -> Option<(Position, u8)> {
        match &mut self.phase {
            HandPhase::Bidding { bid_state, hands } => {
                if let Some(bid_result) = bid_state.step(players, hands) {
                    self.phase = if let BidResultAll::Called { .. }
                    | BidResultAll::CalledAlone { .. }
                    | BidResultAll::DefendedAlone { .. } = bid_result
                    {
                        let bid_result: BidResultCalled = bid_result.try_into().unwrap();
                        HandPhase::FirstTrick {
                            trick_state: TrickState::create(
                                bid_result.clone(),
                                self.dealer.next_position_playing(&bid_result),
                            ),
                            hands: HandState::update_bowers(hands.clone(), &bid_result.trump()),
                            bid_result,
                        }
                    } else {
                        HandPhase::Scoring {
                            bid_result,
                            tricks_taken: [0; 4],
                        }
                    }
                };
                None
            }
            HandPhase::FirstTrick {
                bid_result,
                hands,
                trick_state,
            } => {
                if let Some(trick_winner) = trick_state.step(players, hands) {
                    let mut tricks_taken = [0; 4];
                    tricks_taken[trick_winner.index()] += 1;
                    self.phase = HandPhase::SecondTrick {
                        bid_result: bid_result.clone(),
                        trick_state: TrickState::create(bid_result.clone(), trick_winner),
                        hands: hands.clone(),
                        tricks_taken,
                    }
                };
                None
            }
            HandPhase::SecondTrick {
                bid_result,
                hands,
                trick_state,
                tricks_taken,
            } => {
                if let Some(trick_winner) = trick_state.step(players, hands) {
                    tricks_taken[trick_winner.index()] += 1;
                    self.phase = HandPhase::ThirdTrick {
                        bid_result: bid_result.clone(),
                        hands: hands.clone(),
                        trick_state: TrickState::create(bid_result.clone(), trick_winner),
                        tricks_taken: *tricks_taken,
                    }
                };
                None
            }
            HandPhase::ThirdTrick {
                bid_result,
                hands,
                trick_state,
                tricks_taken,
            } => {
                if let Some(trick_winner) = trick_state.step(players, hands) {
                    tricks_taken[trick_winner.index()] += 1;
                    self.phase = HandPhase::FourthTrick {
                        bid_result: bid_result.clone(),
                        hands: hands.clone(),
                        trick_state: TrickState::create(bid_result.clone(), trick_winner),
                        tricks_taken: *tricks_taken,
                    }
                };
                None
            }
            HandPhase::FourthTrick {
                bid_result,
                hands,
                trick_state,
                tricks_taken,
            } => {
                if let Some(trick_winner) = trick_state.step(players, hands) {
                    tricks_taken[trick_winner.index()] += 1;
                    self.phase = HandPhase::FifthTrick {
                        bid_result: bid_result.clone(),
                        hands: hands.clone(),
                        trick_state: TrickState::create(bid_result.clone(), trick_winner),
                        tricks_taken: *tricks_taken,
                    }
                };
                None
            }
            HandPhase::FifthTrick {
                bid_result,
                hands,
                trick_state,
                tricks_taken,
            } => {
                if let Some(trick_winner) = trick_state.step(players, hands) {
                    tricks_taken[trick_winner.index()] += 1;
                    self.phase = HandPhase::Scoring {
                        bid_result: bid_result.clone().into(),
                        tricks_taken: *tricks_taken,
                    }
                };
                None
            }
            HandPhase::Scoring {
                bid_result,
                tricks_taken,
            } => Some(HandState::get_score(bid_result, tricks_taken)),
        }
    }

    pub(crate) fn finish_bidding(
        &mut self,
        players: &mut [impl Player; 4],
    ) -> Option<BidResultCalled> {
        loop {
            match &self.phase {
                HandPhase::Bidding { .. } => {
                    if self.step(players).is_some() {
                        return None;
                    }
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

    fn update_bowers(hands: [HandBeforeBidding; 4], trump: &Suit) -> [Hand; 4] {
        hands
            .into_iter()
            .map(|hand| HandBeforeBidding::update_bowers(hand, trump))
            .collect::<Vec<Hand>>()
            .try_into()
            .unwrap()
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
