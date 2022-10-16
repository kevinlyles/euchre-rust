use crate::euchre::*;

mod card;
mod deck;
mod euchre;
mod hand;

fn main() {
    yew::start_app::<Euchre>();
}
