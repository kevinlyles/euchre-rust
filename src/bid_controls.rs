use enum_iterator::IntoEnumIterator;
use yew::prelude::*;

use crate::{
    bid_result::BidResult,
    bid_state::{BidPhase, BidState},
    card::CardLogic,
    hand::Hand,
    player::Player,
    suit::Suit,
};

#[function_component(BidControls)]
pub fn bid_controls(props: &BidControlsProps) -> Html {
    match props.bid_state {
        Some(state) => {
            match state.phase {
                BidPhase::FirstRoundFirstPlayer { .. }
                | BidPhase::FirstRoundSecondPlayer { .. }
                | BidPhase::FirstRoundThirdPlayer { .. }
                | BidPhase::FirstRoundFourthPlayer { .. }
                    if state.get_active_player() == props.player =>
                {
                    let order_up_callback = Callback::from(|_| {
                        //TODO: allow alone and defending alone
                        state.order_it_up(false, None);
                    });
                    let pass_callback = Callback::from(|_| {
                        state.pass();
                    });
                    let label = if state.dealer == props.player {
                        "Pick Up"
                    } else {
                        "Order Up"
                    };
                    html! {
                        <>
                            <button onclick={order_up_callback}>{label }</button>
                            <button onclick={pass_callback}>{"Pass"}</button>
                        </>
                    }
                }
                BidPhase::SecondRoundFirstPlayer { forbidden_suit }
                | BidPhase::SecondRoundSecondPlayer { forbidden_suit }
                | BidPhase::SecondRoundThirdPlayer { forbidden_suit }
                | BidPhase::SecondRoundFourthPlayer { forbidden_suit }
                    if state.get_active_player() == props.player =>
                {
                    let suit_buttons = {
                        Suit::into_enum_iter()
                            .filter(|suit|suit != &forbidden_suit)
                            .map(|suit| {
                                let callback = Callback::from( |_| {
                                    //TODO: allow alone and defending alone
                                    state.call(suit, false, None);
                                });
                                html! {
                                    <span onclick={callback} class={suit.color()}>{suit.to_string()}</span>
                                }
                            })
                    };
                    let pass_button = {
                        let callback = Callback::from(|_| {
                            state.pass();
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
                BidPhase::OrderedUp {
                    trump: _,
                    caller: _,
                }
                | BidPhase::OrderedUpAlone {
                    trump: _,
                    caller: _,
                }
                | BidPhase::OrderedUpDefendedAlone {
                    trump: _,
                    caller: _,
                    defender: _,
                } if props.player == state.dealer => {
                    let hand = state.hands[state.dealer.index()].clone();
                    let callback = Callback::from(|card: CardLogic| {
                        state.discard(card);
                    });
                    html! {
                        <>
                            <span>{"Choose discard:"}</span>
                            <Hand hand={hand.clone()} callback={callback.clone()} visible={true}/>
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
    pub bid_state: Option<Box<BidState>>,
}
