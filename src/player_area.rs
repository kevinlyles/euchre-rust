use yew::prelude::*;

use crate::{
    bid_controls::BidControls,
    bid_state::BidState,
    hand::{Hand, HandProps},
    hand_state::HandState,
    player::Player,
};

#[function_component(PlayerArea)]
pub fn player_area(props: &PlayerAreaProps) -> Html {
    html! {
        <>
            <Hand ..props.hand.clone() />
            <BidControls player={props.player} bid_state={props.bid_state.clone()} />
        </>
    }
}

#[derive(PartialEq, Properties)]
pub struct PlayerAreaProps {
    pub player: Player,
    pub hand: HandProps,
    pub hand_state: UseStateHandle<HandState>,
    pub bid_state: UseStateHandle<Option<BidState>>,
}
