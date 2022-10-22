use yew::prelude::*;

use crate::{
    bid_state::{BidState, BidStateKind},
    player::Player,
};

#[function_component(BidControls)]
pub fn bid_controls(props: &BidControlsProps) -> Html {
    match props.bid_state {
        Some(bid_state) => {
            let is_first_round = match bid_state.phase {
                BidStateKind::FirstRoundFirstPlayer { .. }
                    if bid_state.dealer.next_player(None, None) == props.player =>
                {
                    true
                }
                BidStateKind::FirstRoundSecondPlayer { .. }
                    if bid_state.dealer.partner() == props.player =>
                {
                    true
                }
                BidStateKind::FirstRoundThirdPlayer { .. }
                    if bid_state.dealer.partner().next_player(None, None) == props.player =>
                {
                    true
                }
                BidStateKind::FirstRoundFourthPlayer { .. } if bid_state.dealer == props.player => {
                    true
                }
                _ => false,
            };
            if is_first_round {
                html! {
                    <>
                        <button>{"Order Up"}</button>
                        <button>{"Pass"}</button>
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
    pub bid_state: Option<BidState>,
}
