use crate::{deck::Deck, hand_state::HandState, player::Player, position::Position};

pub struct GameState {
    pub players: [Box<dyn Player>; 4],
    pub phase: GamePhase,
    pub north_south_score: u8,
    pub east_west_score: u8,
}

pub enum GamePhase {
    Playing { hand_state: HandState },
    Done,
}

impl GameState {
    pub fn create(players: [Box<dyn Player>; 4]) -> GameState {
        let (hands, trump_candidate) = Deck::create_shuffled_deck().deal();
        GameState {
            players,
            phase: GamePhase::Playing {
                hand_state: HandState::create(Position::South, trump_candidate, hands),
            },
            north_south_score: 0,
            east_west_score: 0,
        }
    }

    pub fn step(&mut self) -> Option<String> {
        match &mut self.phase {
            GamePhase::Playing { ref mut hand_state } => {
                match hand_state.step(&mut self.players) {
                    Some((player, score)) => self.finish_hand(player, score),
                    None => (),
                }
                None
            }
            GamePhase::Done => Some(if self.east_west_score >= 10 {
                format!(
                    "East/West wins! Final score: {0}-{1}",
                    self.east_west_score, self.north_south_score,
                )
            } else {
                format!(
                    "North/South wins! Final score: {0}-{1}",
                    self.north_south_score, self.east_west_score,
                )
            }),
        }
    }

    fn finish_hand(&mut self, player: Position, score: u8) -> () {
        if score > 0 {
            match player {
                Position::South | Position::North => {
                    self.north_south_score += score;
                    log::info!(
                        "North/South scored {} points: {}-{}",
                        score,
                        self.north_south_score,
                        self.east_west_score
                    );
                    if self.north_south_score >= 10 {
                        self.phase = GamePhase::Done;
                    }
                }
                Position::West | Position::East => {
                    self.east_west_score += score;
                    log::info!(
                        "East/West scored {} points: {}-{}",
                        score,
                        self.east_west_score,
                        self.north_south_score
                    );
                    if self.east_west_score >= 10 {
                        self.phase = GamePhase::Done;
                    }
                }
            }
        }
        match &self.phase {
            GamePhase::Playing { hand_state, .. } => {
                let (hands, trump_candidate) = Deck::create_shuffled_deck().deal();
                self.phase = GamePhase::Playing {
                    hand_state: HandState::create(
                        hand_state.dealer.next_position_bidding(),
                        trump_candidate,
                        hands,
                    ),
                }
            }
            GamePhase::Done => (),
        }
    }
}
