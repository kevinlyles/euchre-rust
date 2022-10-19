use crate::euchre::*;

mod card;
mod deck;
mod euchre;
mod game_state;
mod hand;
mod rank;
mod rank_with_bowers;

fn main() {
    yew::start_app::<Euchre>();
}
