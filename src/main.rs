use game_state::GameState;
use player::Player;
use players::minimal::MinimalPlayer;

mod bid_result;
mod bid_state;
mod card;
mod deck;
mod game_state;
mod hand;
mod hand_state;
mod player;
mod players;
mod position;
mod rank;
mod rank_with_bowers;
mod suit;
mod trick_state;

fn main() {
    let players: [Box<dyn Player>; 4] = [
        Box::new(MinimalPlayer {}),
        Box::new(MinimalPlayer {}),
        Box::new(MinimalPlayer {}),
        Box::new(MinimalPlayer {}),
    ];
    let mut game_state = GameState::create(players);
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
