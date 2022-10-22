use crate::card::*;
use yew::prelude::*;

#[function_component(Hand)]
pub fn hand(hand: &HandProps) -> Html {
    html! {
        <div>
            {
                for hand.cards.iter()
                    .map(|card| html! {if hand.visible || cfg!(show_all_cards) {<Card ..*card/>} else { <CardBack/>}})
            }
        </div>
    }
}

#[derive(Clone, Properties)]
pub struct HandProps {
    pub cards: Vec<CardProps>,
    pub visible: bool,
}

impl PartialEq for HandProps {
    fn eq(&self, other: &Self) -> bool {
        self.cards.iter().all(|card| other.cards.contains(card))
            && other.cards.iter().all(|card| self.cards.contains(card))
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(&other)
    }
}
