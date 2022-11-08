use yew::prelude::*;

use crate::{
    bid_controls::BidControls,
    bid_state::BidState,
    hand::{Hand, HandProps},
    player::Player,
};

#[function_component(PlayerArea)]
pub fn player_area(props: &PlayerAreaProps) -> Html {
    html! {
        <>
            <Hand ..props.hand.clone() />
            {match props.bid_state {
                Some(bid_state) => html! {
                    <BidControls player={props.player} bid_state={bid_state} />
                },
                None => html!{},
            }}
        </>
    }
}

#[derive(PartialEq, Properties)]
pub struct PlayerAreaProps {
    pub player: Player,
    pub hand: HandProps,
    pub bid_state: Option<BidState>,
}
