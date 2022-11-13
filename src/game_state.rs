use crate::{
    deck::Deck, hand::HandLogic, hand_state::HandState, player::Player, position::Position,
};

pub struct GameState {
    pub players: [Box<dyn Player>; 4],
    pub phase: GamePhase,
    pub north_south_score: u8,
    pub east_west_score: u8,
}

pub enum GamePhase {
    Playing {
        hand_state: HandState,
        //TODO: decide where this should actually live
        hands: [HandLogic; 4],
    },
    Done,
}

impl GameState {
    pub fn create(players: [Box<dyn Player>; 4]) -> GameState {
        let (hands, trump_candidate) = Deck::create_shuffled_deck().deal();
        GameState {
            players,
            phase: GamePhase::Playing {
                hand_state: HandState::create(Position::Bottom, trump_candidate),
                hands,
            },
            north_south_score: 0,
            east_west_score: 0,
        }
    }

    pub fn step(&mut self) -> Option<String> {
        match &mut self.phase {
            GamePhase::Playing {
                ref mut hand_state,
                ref mut hands,
            } => {
                match hand_state.step(&mut self.players, hands) {
                    Some((player, score)) => self.update_score(player, score),
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

    fn update_score(&mut self, player: Position, score: u8) -> () {
        todo!("update scores")
    }
}
