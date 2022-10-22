use yew::prelude::*;

use crate::{
    bid_state::{BidState, BidStateKind},
    player::Player,
};

#[function_component(BidControls)]
pub fn bid_controls(props: &BidControlsProps) -> Html {
    match *props.bid_state {
        Some(state) => {
            let is_first_round = match state.phase {
                BidStateKind::FirstRoundFirstPlayer { .. }
                    if state.dealer.next_player(None, None) == props.player =>
                {
                    true
                }
                BidStateKind::FirstRoundSecondPlayer { .. }
                    if state.dealer.partner() == props.player =>
                {
                    true
                }
                BidStateKind::FirstRoundThirdPlayer { .. }
                    if state.dealer.partner().next_player(None, None) == props.player =>
                {
                    true
                }
                BidStateKind::FirstRoundFourthPlayer { .. } if state.dealer == props.player => true,
                _ => false,
            };
            if is_first_round {
                let bid_state_1 = props.bid_state.clone();
                let state_1 = state.clone();
                let order_up_callback = Callback::from(move |_| {
                    let new_state;
                    //TODO: allow alone and defending alone
                    new_state = state_1.order_it_up(false, None);
                    match new_state {
                        Some(_) => bid_state_1.set(new_state),
                        None => (),
                    }
                });
                let bid_state_2 = props.bid_state.clone();
                let state_2 = state.clone();
                let pass_callback = Callback::from(move |_| {
                    let new_state;
                    new_state = state_2.pass();
                    match new_state {
                        Some(_) => bid_state_2.set(new_state),
                        None => (),
                    }
                });
                html! {
                    <>
                        <button onclick={order_up_callback}>{"Order Up"}</button>
                        <button onclick={pass_callback}>{"Pass"}</button>
                    </>
                }
            } else {
                html! {}
            }
        }
        None => html! {},
    }
}

#[derive(PartialEq, Properties)]
pub struct BidControlsProps {
    pub player: Player,
    pub bid_state: UseStateHandle<Option<BidState>>,
}
