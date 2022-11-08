use crate::game_state::{GamePhase, GameState};
use crate::hand::HandProps;
use crate::hand_state::HandPhase;
use crate::player::Player;
use crate::player_area::PlayerArea;
use crate::playing_surface::PlayingSurface;
use yew::prelude::*;

#[function_component(Euchre)]
pub fn euchre() -> Html {
    let game_state: UseStateHandle<GameState>;
    let game_state_1: UseStateHandle<GameState>;
    let update = Callback::from(move |new_game_state| {
        game_state_1.set(new_game_state);
    });
    game_state = use_state_eq(|| GameState::create(update));
    game_state_1 = game_state.clone();
    match &game_state.phase {
        GamePhase::Initializing => html! {},
        GamePhase::Playing { hand_state } => match &hand_state.phase {
            HandPhase::Initializing => html! {},
            HandPhase::Scoring { tricks_taken: _ } => html! {<div>{"To do!"}</div>},
            HandPhase::Bidding { bid_state } => {
                html! {
                   <div class="table">
                      <div class="player left">
                        <PlayerArea player={Player::Left}
                            hand={HandProps::without_callback(bid_state.hands[0].clone(), false)}
                            bid_state={Some(*bid_state)} />
                      </div>
                      <div class="player top">
                        <PlayerArea player={Player::Top}
                            hand={HandProps::without_callback(bid_state.hands[1].clone(), false)}
                            bid_state={Some(*bid_state)} />
                      </div>
                      <div class="player right">
                        <PlayerArea player={Player::Right}
                            hand={HandProps::without_callback(bid_state.hands[2].clone(), false)}
                            bid_state={Some(*bid_state)} />
                      </div>
                      <div class="player bottom">
                        <PlayerArea player={Player::Bottom}
                            hand={HandProps::without_callback(bid_state.hands[3].clone(), true)}
                            bid_state={Some(*bid_state)} />
                      </div>
                      <div class="center">
                        <PlayingSurface bid_state={Some(*bid_state)} />
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
                html! {
                   <div class="playing-surface">
                      <div class="player left">
                        <PlayerArea player={Player::Left}
                            hand={HandProps::without_callback(hands[0].clone(), false)}
                            bid_state={None} />
                      </div>
                      <div class="player top">
                        <PlayerArea player={Player::Top}
                            hand={HandProps::without_callback(hands[1].clone(), false)}
                            bid_state={None} />
                      </div>
                      <div class="player right">
                        <PlayerArea player={Player::Right}
                            hand={HandProps::without_callback(hands[2].clone(), false)}
                            bid_state={None} />
                      </div>
                      <div class="player bottom">
                        <PlayerArea player={Player::Bottom}
                            hand={HandProps::without_callback(hands[3].clone(), true)}
                            bid_state={None} />
                      </div>
                      <div class="center">
                        <PlayingSurface bid_state={None}/>
                      </div>
                   </div>
                }
            }
        },
        GamePhase::Done => todo!(),
    }
}
