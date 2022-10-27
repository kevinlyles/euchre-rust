use yew::prelude::*;

use crate::{
    bid_state::BidState,
    card::{Card, CardBack},
    player::Player,
};

#[function_component(PlayingSurface)]
pub fn playing_surface(props: &PlayingSurfaceProps) -> Html {
    let class = match props.bid_state.clone() {
        Some(bid_state) => match bid_state.dealer {
            Player::Left => "left",
            Player::Top => "top",
            Player::Right => "right",
            Player::Bottom => "bottom",
        },
        None => "",
    };
    html! {
        <div class={class}>
            {
                match props.bid_state.clone() {
                    Some(bid_state) => match bid_state.get_trump_candidate() {
                        Some(card) => html!{<Card card={card} />},
                        None => html!{<CardBack />},
                    },
                    None => html!{},
                }
            }
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct PlayingSurfaceProps {
    pub bid_state: Option<BidState>,
}
