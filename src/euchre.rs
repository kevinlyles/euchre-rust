use crate::deck::*;
use crate::hand_state::HandStateKind;
use crate::player::Player;
use crate::player_area::PlayerArea;
use crate::playing_surface::PlayingSurface;
use yew::prelude::*;

#[function_component(Euchre)]
pub fn euchre() -> Html {
    let game_state = use_state_eq(|| Deck::create_shuffled_deck().deal(Player::Bottom));
    let hand_state = use_state_eq(|| game_state.hand_state.clone());
    match &game_state.hand_state.phase {
        HandStateKind::Scoring { tricks_taken: _ } => html! {<div>{"To do!"}</div>},
        HandStateKind::Bidding { hands, bid_state } => {
            let bid_state_handle = use_state_eq(|| Some(*bid_state));
            html! {
               <div class="table">
                  <div class="player left">
                    <PlayerArea player={Player::Left} hand={hands[0].clone()} bid_state={bid_state_handle.clone()} hand_state={hand_state.clone()} />
                  </div>
                  <div class="player top">
                    <PlayerArea player={Player::Top} hand={hands[1].clone()} bid_state={bid_state_handle.clone()} hand_state={hand_state.clone()}/>
                  </div>
                  <div class="player right">
                    <PlayerArea player={Player::Right} hand={hands[2].clone()} bid_state={bid_state_handle.clone()} hand_state={hand_state.clone()}/>
                  </div>
                  <div class="player bottom">
                    <PlayerArea player={Player::Bottom} hand={hands[3].clone()} bid_state={bid_state_handle.clone()} hand_state={hand_state}/>
                  </div>
                  <div class="center">
                    <PlayingSurface bid_state={*bid_state_handle} />
                  </div>
               </div>
            }
        }
        HandStateKind::FirstTrick {
            hands,
            trump: _,
            trick_state: _,
        }
        | HandStateKind::SecondTrick {
            hands,
            trump: _,
            trick_state: _,
            tricks_taken: _,
        }
        | HandStateKind::ThirdTrick {
            hands,
            trump: _,
            trick_state: _,
            tricks_taken: _,
        }
        | HandStateKind::FourthTrick {
            hands,
            trump: _,
            trick_state: _,
            tricks_taken: _,
        }
        | HandStateKind::FifthTrick {
            hands,
            trump: _,
            trick_state: _,
            tricks_taken: _,
        } => {
            let bid_state = use_state_eq(|| None);
            html! {
               <div class="playing-surface">
                  <div class="player left">
                    <PlayerArea player={Player::Left} hand={hands[0].clone()} bid_state={bid_state.clone()} hand_state={hand_state.clone()} />
                  </div>
                  <div class="player top">
                    <PlayerArea player={Player::Top} hand={hands[1].clone()} bid_state={bid_state.clone()} hand_state={hand_state.clone()} />
                  </div>
                  <div class="player right">
                    <PlayerArea player={Player::Right} hand={hands[2].clone()} bid_state={bid_state.clone()} hand_state={hand_state.clone()} />
                  </div>
                  <div class="player bottom">
                    <PlayerArea player={Player::Bottom} hand={hands[3].clone()} bid_state={bid_state.clone()} hand_state={hand_state} />
                  </div>
                  <div class="center">
                    <PlayingSurface bid_state={*bid_state}/>
                  </div>
               </div>
            }
        }
    }
}
