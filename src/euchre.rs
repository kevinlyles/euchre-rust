use crate::card::*;
use crate::deck::*;
use crate::hand::*;
use yew::prelude::*;

#[function_component(Euchre)]
pub fn euchre() -> Html {
    let game_state = use_state_eq(|| Deck::create_shuffled_deck().deal());
    html! {
     <div class="playing-surface">
         <div class="player left">
            <Hand ..game_state.hands[0].clone() />
         </div>
         <div class="player top">
            <Hand ..game_state.hands[1].clone() />
         </div>
         <div class="player right">
            <Hand ..game_state.hands[2].clone() />
         </div>
         <div class="player bottom">
            <Hand ..game_state.hands[3].clone() />
         </div>
         <div class="center">
             <Card ..game_state.trump_candidate />
         </div>
     </div>
    }
}
