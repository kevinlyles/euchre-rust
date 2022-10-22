use enum_iterator::IntoEnumIterator;
use yew::prelude::*;

use crate::{
    bid_state::{BidState, BidStateKind},
    player::Player,
    suit::Suit,
};

#[function_component(BidControls)]
pub fn bid_controls(props: &BidControlsProps) -> Html {
    match *props.bid_state {
        Some(state) => {
            match state.phase {
                BidStateKind::FirstRoundFirstPlayer { .. }
                | BidStateKind::FirstRoundSecondPlayer { .. }
                | BidStateKind::FirstRoundThirdPlayer { .. }
                | BidStateKind::FirstRoundFourthPlayer { .. }
                    if state.get_active_player() == props.player =>
                {
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
                }
                BidStateKind::SecondRoundFirstPlayer { forbidden_suit }
                | BidStateKind::SecondRoundSecondPlayer { forbidden_suit }
                | BidStateKind::SecondRoundThirdPlayer { forbidden_suit }
                | BidStateKind::SecondRoundFourthPlayer { forbidden_suit }
                    if state.get_active_player() == props.player =>
                {
                    let suit_buttons = {
                        Suit::into_enum_iter()
                            .filter(|suit|suit != &forbidden_suit)
                            .map(|suit| {
                                let bid_state = props.bid_state.clone();
                                let state = state.clone();
                                let callback = Callback::from(move |_| {
                                    let new_state;
                                    //TODO: allow alone and defending alone
                                    new_state = state.call(suit, false, None);
                                    match new_state {
                                        Some(_) => bid_state.set(new_state),
                                        None => (),
                                    }
                                });
                                html! {
                                    <span onclick={callback} class={suit.color()}>{suit.to_string()}</span>
                                }
                            })
                    };
                    let pass_button = {
                        let bid_state = props.bid_state.clone();
                        let state = state.clone();
                        let callback = Callback::from(move |_| {
                            let new_state;
                            new_state = state.pass();
                            match new_state {
                                Some(_) => bid_state.set(new_state),
                                None => (),
                            }
                        });
                        html! {
                            <>
                                <button onclick={callback}>{"Pass"}</button>
                            </>
                        }
                    };
                    html! {
                        <>
                            {for suit_buttons}
                            {pass_button}
                        </>
                    }
                }
                _ => html! {},
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
