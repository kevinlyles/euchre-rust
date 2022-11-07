use yew::Callback;

use crate::{deck::Deck, hand_state::HandState, player::Player};

#[derive(PartialEq)]
pub struct GameState {
    pub phase: GamePhase,
    pub north_south_score: u8,
    pub east_west_score: u8,
    update: Callback<GameState>,
}

#[derive(PartialEq)]
pub enum GamePhase {
    Initializing,
    Playing { hand_state: HandState },
    Done,
}

impl GameState {
    pub fn create(update: Callback<GameState>) -> GameState {
        let (hands, trump_candidate) = Deck::create_shuffled_deck().deal();
        let state = GameState {
            phase: GamePhase::Initializing,
            north_south_score: 0,
            east_west_score: 0,
            update,
        };
        let update = Box::new(|hand_state| state.update_hand_state(hand_state));
        state.phase = GamePhase::Playing {
            hand_state: HandState::create(Player::Bottom, hands, trump_candidate, update),
        };
        state
    }

    fn update_hand_state(&mut self, hand_state: HandState) -> () {
        match self.phase {
            GamePhase::Initializing => (),
            //TODO: handle hand ending, updating score, etc.
            GamePhase::Playing { hand_state } => self.phase = GamePhase::Playing { hand_state },
            GamePhase::Done => (),
        }
    }
}
