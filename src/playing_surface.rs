use yew::prelude::*;

use crate::{
    card::{Card, CardBack, CardProps},
    player::Player,
};

#[function_component(PlayingSurface)]
pub fn playing_surface(props: &PlayingSurfaceProps) -> Html {
    let class = match props.dealer {
        Player::Left => "left",
        Player::Top => "top",
        Player::Right => "right",
        Player::Bottom => "bottom",
    };
    html! {
        <div class={class}>
            {match props.trump_candidate {
                Some(card) =>html!{<Card ..card />},
                None => html!{<CardBack />},
            }}
        </div>
    }
}

#[derive(PartialEq, Properties)]
pub struct PlayingSurfaceProps {
    pub dealer: Player,
    pub trump_candidate: Option<CardProps>,
}
