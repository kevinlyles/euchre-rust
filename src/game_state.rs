use crate::{deck::Deck, hand_state::HandState, position::Position};

#[derive(PartialEq)]
pub struct GameState {
    pub phase: GamePhase,
    pub north_south_score: u8,
    pub east_west_score: u8,
}

#[derive(PartialEq)]
pub enum GamePhase {
    Playing { hand_state: HandState },
    Done,
}

impl GameState {
    pub fn create() -> GameState {
        let (hands, trump_candidate) = Deck::create_shuffled_deck().deal();
        GameState {
            phase: GamePhase::Playing {
                hand_state: HandState::create(Position::Bottom, hands, trump_candidate),
            },
            north_south_score: 0,
            east_west_score: 0,
        }
    }

    pub fn step(&mut self) -> Option<String> {
        match &mut self.phase {
            GamePhase::Playing { ref mut hand_state } => {
                match hand_state.step() {
                    Some(tricks_taken) => self.update_score(tricks_taken),
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

    fn update_score(&mut self, tricks_taken: [u8; 4]) -> () {
        todo!("update scores")
    }
}
