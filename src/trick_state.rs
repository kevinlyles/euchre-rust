use crate::{
    bid_result::BidResultCalled, card::Card, hand::Hand, player::Player, position::Position,
    suit::Suit,
};

#[derive(Debug)]
pub(crate) struct TrickState {
    pub(crate) bid_result: BidResultCalled,
    pub(crate) leader: Position,
    pub(crate) phase: TrickPhase,
}

#[derive(Debug)]
pub(crate) enum TrickPhase {
    BeforeFirstCard,
    BeforeSecondCard { cards_played: [Card; 1] },
    BeforeThirdCard { cards_played: [Card; 2] },
    BeforeFourthCard { cards_played: [Card; 3] },
    Done { trick_winner: Position },
}

impl TrickState {
    pub(crate) fn create(bid_result: BidResultCalled, leader: Position) -> TrickState {
        TrickState {
            bid_result,
            leader,
            phase: TrickPhase::BeforeFirstCard,
        }
    }

    pub(crate) fn step(
        &mut self,
        players: &mut [impl Player; 4],
        hands: &mut [Hand; 4],
    ) -> Option<Position> {
        match self.phase {
            TrickPhase::BeforeFirstCard => {
                let player = self.leader;
                let hand = &mut hands[player.index()];
                let mut card = players[player.index()].play_card(
                    &hand,
                    &self.bid_result.caller(),
                    &self.bid_result.trump(),
                    None,
                );
                if !hand.cards.contains(&card) {
                    card = hand.cards[0];
                }
                hand.cards.retain(|c| c != &card);
                self.phase = TrickPhase::BeforeSecondCard {
                    cards_played: [card],
                };
                None
            }
            TrickPhase::BeforeSecondCard { cards_played } => {
                let player = &self.leader.next_position_playing(&self.bid_result);
                let card = TrickState::play_card(
                    player,
                    players,
                    hands,
                    &self.bid_result.caller(),
                    &self.bid_result.trump(),
                    &cards_played[0].suit,
                );
                self.phase = TrickPhase::BeforeThirdCard {
                    cards_played: [cards_played[0], card],
                };
                None
            }
            TrickPhase::BeforeThirdCard { cards_played } => {
                let player = &self
                    .leader
                    .next_position_playing(&self.bid_result)
                    .next_position_playing(&self.bid_result);
                if *player == self.leader {
                    self.phase = TrickPhase::Done {
                        trick_winner: TrickState::get_winning_position(
                            &self.bid_result,
                            &self.leader,
                            cards_played.as_slice(),
                        ),
                    };
                    let cards_played = cards_played.into();
                    for player in players {
                        player.trick_end(&self.bid_result, &self.leader, &cards_played);
                    }
                } else {
                    let card = TrickState::play_card(
                        player,
                        players,
                        hands,
                        &self.bid_result.caller(),
                        &self.bid_result.trump(),
                        &cards_played[0].suit,
                    );
                    self.phase = TrickPhase::BeforeFourthCard {
                        cards_played: [cards_played[0], cards_played[1], card],
                    }
                }
                None
            }
            TrickPhase::BeforeFourthCard { cards_played } => {
                let player = &self
                    .leader
                    .next_position_playing(&self.bid_result)
                    .next_position_playing(&self.bid_result)
                    .next_position_playing(&self.bid_result);
                if *player == self.leader {
                    self.phase = TrickPhase::Done {
                        trick_winner: TrickState::get_winning_position(
                            &self.bid_result,
                            &self.leader,
                            cards_played.as_slice(),
                        ),
                    };
                    let cards_played = cards_played.into();
                    for player in players {
                        player.trick_end(&self.bid_result, &self.leader, &cards_played);
                    }
                } else {
                    let card = TrickState::play_card(
                        player,
                        players,
                        hands,
                        &self.bid_result.caller(),
                        &self.bid_result.trump(),
                        &cards_played[0].suit,
                    );
                    let new_cards_played =
                        [cards_played[0], cards_played[1], cards_played[2], card];
                    self.phase = TrickPhase::Done {
                        trick_winner: TrickState::get_winning_position(
                            &self.bid_result,
                            &self.leader,
                            new_cards_played.as_slice(),
                        ),
                    };
                    let cards_played = new_cards_played.into();
                    for player in players {
                        player.trick_end(&self.bid_result, &self.leader, &cards_played);
                    }
                }
                None
            }
            TrickPhase::Done { trick_winner } => Some(trick_winner),
        }
    }

    fn play_card(
        player: &Position,
        players: &mut [impl Player; 4],
        hands: &mut [Hand; 4],
        caller: &Position,
        trump: &Suit,
        suit_led: &Suit,
    ) -> Card {
        let hand = &mut hands[player.index()];
        let mut card = players[player.index()].play_card(&hand, &caller, &trump, None);
        if !hand.cards.contains(&card) {
            card = hand.cards[0]
        }
        if card.suit != *suit_led {
            match hand.cards.iter().filter(|card| card.suit == *trump).next() {
                Some(card_following_suit) => card = *card_following_suit,
                None => (),
            }
        }
        hand.cards.retain(|c| c != &card);
        card
    }

    fn get_winning_position(
        bid_result: &BidResultCalled,
        leader: &Position,
        cards_played: &[Card],
    ) -> Position {
        //TODO: see if we can combine these steps?
        let winning_card = cards_played
            .iter()
            .reduce(|first_card, second_card| {
                if first_card.suit == second_card.suit {
                    if first_card.rank > second_card.rank {
                        first_card
                    } else {
                        second_card
                    }
                } else if second_card.suit == bid_result.trump() {
                    second_card
                } else {
                    first_card
                }
            })
            .unwrap();
        let mut player = *leader;
        let mut iterator = cards_played.iter();
        loop {
            match iterator.next() {
                Some(card) if card == winning_card => break,
                None => panic!(),
                _ => player = player.next_position_playing(bid_result),
            }
        }
        player
    }
}
