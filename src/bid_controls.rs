use enum_iterator::IntoEnumIterator;
use yew::prelude::*;

use crate::{
    bid_state::{BidPhase, BidState},
    card::CardLogic,
    hand::Hand,
    player::Player,
    suit::Suit,
};

#[derive(PartialEq, Properties)]
pub struct BidControlsProps {
    pub player: Player,
    pub bid_state: BidState,
}

enum Message {
    Pass,
    OrderUp {
        alone: bool,
        defender: Option<Player>,
    },
    Discard {
        card: CardLogic,
    },
    Call {
        trump: Suit,
        alone: bool,
        defender: Option<Player>,
    },
}

pub struct BidControls;

impl Component for BidControls {
    type Message = Message;

    type Properties = BidControlsProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let state = props.bid_state;
        match state.phase {
            BidPhase::FirstRoundFirstPlayer { .. }
            | BidPhase::FirstRoundSecondPlayer { .. }
            | BidPhase::FirstRoundThirdPlayer { .. }
            | BidPhase::FirstRoundFourthPlayer { .. }
                if state.get_active_player() == props.player =>
            {
                let order_up_callback = ctx.link().callback(|_| Message::OrderUp {
                    alone: false,
                    defender: None,
                });
                let pass_callback = ctx.link().callback(|_| Message::Pass);
                let label = if state.dealer == props.player {
                    "Pick Up"
                } else {
                    "Order Up"
                };
                html! {
                    <>
                        <button onclick={order_up_callback}>{label}</button>
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
                                let callback = ctx.link().callback( |_| Message::Call { trump: suit, alone: false, defender: None });
                                html! {
                                    <span onclick={callback} class={suit.color()}>{suit.to_string()}</span>
                                }
                            })
                };
                let pass_button = {
                    let callback = ctx.link().callback(|_| Message::Pass);
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
                let callback = ctx
                    .link()
                    .callback(|card: CardLogic| Message::Discard { card });
                html! {
                    <>
                        <span>{"Choose discard:"}</span>
                        <Hand hand={hand} callback={callback} visible={true}/>
                    </>
                }
            }
            _ => html! {},
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let bid_state = ctx.props().bid_state;
        match msg {
            Message::Pass => bid_state.pass(),
            Message::OrderUp { alone, defender } => bid_state.order_it_up(alone, defender),
            Message::Discard { card } => bid_state.discard(card),
            Message::Call {
                trump,
                alone,
                defender,
            } => bid_state.call(trump, alone, defender),
        }
    }
}
