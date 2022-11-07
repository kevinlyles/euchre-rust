use std::ops::Deref;

use crate::bid_state::{BidState, BidPhase};
use crate::deck::*;
use crate::game_state::GameState;
use crate::hand::HandProps;
use crate::hand_state::{HandState, HandPhase};
use crate::player::Player;
use crate::player_area::PlayerArea;
use crate::playing_surface::PlayingSurface;
use yew::prelude::*;

#[function_component(Euchre)]
pub fn euchre() -> Html {
    let (hands, trump_candidate) = Deck::create_shuffled_deck().deal();
    let bid_state = use_state_eq(|| BidState {
        dealer: Player::Bottom,
        hands,
        phase: BidPhase::FirstRoundFirstPlayer { trump_candidate },
    });
    let game_state = use_state_eq(|| GameState {
        hand_state: HandState {
            dealer: Player::Bottom,
            phase: HandPhase::Bidding { bid_state },
        },
    });
    let hand_state = use_state_eq(|| game_state.hand_state.clone());
    match &game_state.hand_state.phase {
        HandPhase::Scoring { tricks_taken: _ } => html! {<div>{"To do!"}</div>},
        HandPhase::Bidding { bid_state } => {
            let game_state_1 = game_state.clone();
            let hand_state_1 = hand_state.clone();
            let done_bidding_callback =
                Callback::from(move |_| match hand_state_1.finish_bidding() {
                    Some(new_state) => {
                        game_state_1.set(GameState {
                            hand_state: new_state.clone(),
                        });
                        hand_state_1.set(new_state);
                        panic!("Made it to callback")
                    }
                    None => (),
                });
            html! {
               <div class="table">
                  <div class="player left">
                    <PlayerArea player={Player::Left}
                        hand={HandProps::without_callback(bid_state.hands[0].clone(), false)}
                        bid_state={Some(bid_state.clone())}
                        hand_state={hand_state.clone()}
                        done_bidding_callback={done_bidding_callback.clone()} />
                  </div>
                  <div class="player top">
                    <PlayerArea player={Player::Top}
                        hand={HandProps::without_callback(bid_state.hands[1].clone(), false)}
                        bid_state={Some(bid_state.clone())}
                        hand_state={hand_state.clone()}
                        done_bidding_callback={done_bidding_callback.clone()} />
                  </div>
                  <div class="player right">
                    <PlayerArea player={Player::Right}
                        hand={HandProps::without_callback(bid_state.hands[2].clone(), false)}
                        bid_state={Some(bid_state.clone())}
                        hand_state={hand_state.clone()}
                        done_bidding_callback={done_bidding_callback.clone()} />
                  </div>
                  <div class="player bottom">
                    <PlayerArea player={Player::Bottom}
                        hand={HandProps::without_callback(bid_state.hands[3].clone(), true)}
                        bid_state={Some(bid_state.clone())}
                        hand_state={hand_state}
                        done_bidding_callback={done_bidding_callback} />
                  </div>
                  <div class="center">
                    <PlayingSurface bid_state={Some(bid_state.deref().clone())} />
                  </div>
               </div>
            }
        }
        HandPhase::FirstTrick {
            bid_result: _,
            hands,
            trick_state: _,
        }
        | HandPhase::SecondTrick {
            bid_result: _,
            hands,
            trick_state: _,
            tricks_taken: _,
        }
        | HandPhase::ThirdTrick {
            bid_result: _,
            hands,
            trick_state: _,
            tricks_taken: _,
        }
        | HandPhase::FourthTrick {
            bid_result: _,
            hands,
            trick_state: _,
            tricks_taken: _,
        }
        | HandPhase::FifthTrick {
            bid_result: _,
            hands,
            trick_state: _,
            tricks_taken: _,
        } => {
            let done_bidding_callback = Callback::default();
            html! {
               <div class="playing-surface">
                  <div class="player left">
                    <PlayerArea player={Player::Left}
                        hand={HandProps::without_callback(hands[0].clone(), false)}
                        bid_state={None}
                        hand_state={hand_state.clone()}
                        done_bidding_callback={done_bidding_callback.clone()} />
                  </div>
                  <div class="player top">
                    <PlayerArea player={Player::Top}
                        hand={HandProps::without_callback(hands[1].clone(), false)}
                        bid_state={None}
                        hand_state={hand_state.clone()}
                        done_bidding_callback={done_bidding_callback.clone()} />
                  </div>
                  <div class="player right">
                    <PlayerArea player={Player::Right}
                        hand={HandProps::without_callback(hands[2].clone(), false)}
                        bid_state={None}
                        hand_state={hand_state.clone()}
                        done_bidding_callback={done_bidding_callback.clone()} />
                  </div>
                  <div class="player bottom">
                    <PlayerArea player={Player::Bottom}
                        hand={HandProps::without_callback(hands[3].clone(), true)}
                        bid_state={None}
                        hand_state={hand_state}
                        done_bidding_callback={done_bidding_callback} />
                  </div>
                  <div class="center">
                    <PlayingSurface bid_state={None}/>
                  </div>
               </div>
            }
        }
    }
}
