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
    BeforeSecondCard { cards_played: [PlayedCard; 1] },
    BeforeThirdCard { cards_played: [PlayedCard; 2] },
    BeforeFourthCard { cards_played: [PlayedCard; 3] },
    Done { trick_winner: Position },
}

#[derive(Clone, Copy, Debug)]
pub struct PlayedCard {
    pub player: Position,
    pub card: Card,
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
                let mut card =
                    players[player.index()].play_card(hand, &self.bid_result, &Vec::new());
                if !hand.cards.contains(&card) {
                    card = hand.cards[0];
                }
                hand.cards.retain(|c| c != &card);
                self.phase = TrickPhase::BeforeSecondCard {
                    cards_played: [PlayedCard { player, card }],
                };
                None
            }
            TrickPhase::BeforeSecondCard { cards_played } => {
                let player = self.leader.next_position_playing(&self.bid_result);
                let card =
                    TrickState::play_card(&player, players, hands, &self.bid_result, &cards_played);
                self.phase = TrickPhase::BeforeThirdCard {
                    cards_played: [cards_played[0], PlayedCard { player, card }],
                };
                None
            }
            TrickPhase::BeforeThirdCard { cards_played } => {
                let player = self
                    .leader
                    .next_position_playing(&self.bid_result)
                    .next_position_playing(&self.bid_result);
                if player == self.leader {
                    self.phase = TrickPhase::Done {
                        trick_winner: TrickState::get_winning_position(
                            &self.bid_result.trump(),
                            cards_played.as_slice(),
                        ),
                    };
                    for player in players {
                        player.trick_end(&self.bid_result, &cards_played);
                    }
                } else {
                    let card = TrickState::play_card(
                        &player,
                        players,
                        hands,
                        &self.bid_result,
                        &cards_played,
                    );
                    self.phase = TrickPhase::BeforeFourthCard {
                        cards_played: [
                            cards_played[0],
                            cards_played[1],
                            PlayedCard { player, card },
                        ],
                    }
                }
                None
            }
            TrickPhase::BeforeFourthCard { cards_played } => {
                let player = self
                    .leader
                    .next_position_playing(&self.bid_result)
                    .next_position_playing(&self.bid_result)
                    .next_position_playing(&self.bid_result);
                if player == self.leader {
                    self.phase = TrickPhase::Done {
                        trick_winner: TrickState::get_winning_position(
                            &self.bid_result.trump(),
                            cards_played.as_slice(),
                        ),
                    };
                    for player in players {
                        player.trick_end(&self.bid_result, &cards_played);
                    }
                } else {
                    let card = TrickState::play_card(
                        &player,
                        players,
                        hands,
                        &self.bid_result,
                        &cards_played,
                    );
                    let new_cards_played = [
                        cards_played[0],
                        cards_played[1],
                        cards_played[2],
                        PlayedCard { player, card },
                    ];
                    self.phase = TrickPhase::Done {
                        trick_winner: TrickState::get_winning_position(
                            &self.bid_result.trump(),
                            new_cards_played.as_slice(),
                        ),
                    };
                    for player in players {
                        player.trick_end(&self.bid_result, &new_cards_played);
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
        bid_result: &BidResultCalled,
        cards_played: &[PlayedCard],
    ) -> Card {
        let hand = &mut hands[player.index()];
        let mut card = players[player.index()].play_card(hand, bid_result, cards_played);
        if !hand.cards.contains(&card) {
            card = hand.cards[0]
        }
        if let Some(led_card) = cards_played.first() {
            if card.suit != led_card.card.suit {
                if let Some(card_following_suit) = hand
                    .cards
                    .iter()
                    .find(|card| card.suit == led_card.card.suit)
                {
                    card = *card_following_suit
                }
            }
        }
        hand.cards.retain(|c| c != &card);
        card
    }

    fn get_winning_position(&trump: &Suit, cards_played: &[PlayedCard]) -> Position {
        cards_played
            .iter()
            .reduce(|first_played_card, second_played_card| {
                if first_played_card.card.suit == second_played_card.card.suit {
                    if first_played_card.card.rank > second_played_card.card.rank {
                        first_played_card
                    } else {
                        second_played_card
                    }
                } else if second_played_card.card.suit == trump {
                    second_played_card
                } else {
                    first_played_card
                }
            })
            .unwrap()
            .player
    }
}
