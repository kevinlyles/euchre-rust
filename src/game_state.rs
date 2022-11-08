use crate::{deck::Deck, hand_state::HandState, player::Player};

#[derive(PartialEq)]
pub struct GameState {
    pub phase: GamePhase,
    pub north_south_score: u8,
    pub east_west_score: u8,
}

#[derive(PartialEq)]
pub enum GamePhase {
    Initializing,
    Playing { hand_state: HandState },
    Done,
}

impl GameState {
    pub fn create() -> GameState {
        let (hands, trump_candidate) = Deck::create_shuffled_deck().deal();
        let mut state = GameState {
            phase: GamePhase::Initializing,
            north_south_score: 0,
            east_west_score: 0,
        };
        state.phase = GamePhase::Playing {
            hand_state: HandState::create(Player::Bottom, hands, trump_candidate),
        };
        state
    }

    fn update_hand_state(&mut self, new_hand_state: HandState) -> () {
        match &self.phase {
            GamePhase::Initializing => (),
            GamePhase::Playing { hand_state } => {
                self.phase = GamePhase::Playing {
                    hand_state: new_hand_state,
                };
            }
            GamePhase::Done => (),
        }
    }

    fn finish_hand(&mut self, tricks_taken: [u8; 4]) -> () {
        match &self.phase {
            GamePhase::Initializing => (),
            //TODO: handle hand ending, updating score, etc.
            GamePhase::Playing { hand_state } => (),
            GamePhase::Done => (),
        }
    }
}
