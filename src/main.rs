use crate::euchre::*;

mod bid_result;
mod bid_state;
mod card;
mod deck;
mod euchre;
mod game_state;
mod hand;
mod hand_state;
mod player;
mod rank;
mod rank_with_bowers;
mod suit;
mod trick_state;

fn main() {
    yew::start_app::<Euchre>();
}
