use game_state::GameState;

mod bid_result;
mod bid_state;
mod card;
mod deck;
mod game_state;
mod hand;
mod hand_state;
mod player;
mod position;
mod rank;
mod rank_with_bowers;
mod suit;
mod trick_state;

fn main() {
    let mut game_state = GameState::create();
    loop {
        match game_state.step() {
            Some(result) => {
                println!("{}", result);
                break;
            }
            None => (),
        }
    }
}
