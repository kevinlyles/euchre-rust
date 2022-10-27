use crate::card::*;
use yew::{prelude::*, props};

#[function_component(Hand)]
pub fn hand(props: &HandProps) -> Html {
    html! {
        <div>
            {
                for props.hand.cards.iter()
                    .map(|card| html! {if props.visible || cfg!(show_all_cards) {<Card card={*card}/>} else { <CardBack/>}})
            }
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct HandProps {
    pub hand: HandLogic,
    pub visible: bool,
    #[prop_or_default]
    pub callback: Callback<CardLogic>,
}

impl HandProps {
    pub fn without_callback(hand: HandLogic, visible: bool) -> Self {
        props! {
            HandProps {
                hand,
                visible,
            }
        }
    }
}

/*
impl PartialEq for HandProps {
    fn eq(&self, other: &Self) -> bool {
        self.visible.eq(&other.visible)
            && self.callback.eq(&other.callback)
            && self.hand.eq(&other.hand)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(&other)
    }
}
*/

#[derive(Clone, PartialEq)]
pub struct HandLogic {
    pub cards: Vec<CardLogic>,
}
