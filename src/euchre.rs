use crate::bid_state::BidStateKind;
use crate::card::*;
use crate::deck::*;
use crate::hand::*;
use crate::hand_state::HandStateKind;
use crate::player::Player;
use crate::playing_surface::PlayingSurface;
use yew::prelude::*;

#[function_component(Euchre)]
pub fn euchre() -> Html {
    let game_state = use_state_eq(|| Deck::create_shuffled_deck().deal(Player::Bottom));
    match &game_state.hand_state.phase {
        HandStateKind::Scoring { tricks_taken: _ } => html! {<div>{"To do!"}</div>},
        HandStateKind::Bidding { hands, bid_state } => {
            html! {
               <div class="table">
                  <div class="player left">
                     <Hand ..hands[0].clone()/>
                  </div>
                  <div class="player top">
                     <Hand ..hands[1].clone()/>
                  </div>
                  <div class="player right">
                     <Hand ..hands[2].clone()/>
                  </div>
                  <div class="player bottom">
                     <Hand ..hands[3].clone()/>
                  </div>
                  <div class="center">
                    <PlayingSurface dealer={bid_state.dealer} trump_candidate={bid_state} />
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
            html! {
               <div class="playing-surface">
                  <div class="player left">
                     <Hand ..hands[0].clone()/>
                  </div>
                  <div class="player top">
                     <Hand ..hands[1].clone()/>
                  </div>
                  <div class="player right">
                     <Hand ..hands[2].clone()/>
                  </div>
                  <div class="player bottom">
                     <Hand ..hands[3].clone()/>
                  </div>
                  <div class="center">
                     {match &game_state.hand_state.phase {
                        HandStateKind::Bidding {hands: _, bid_state} => match bid_state.phase {
                           BidStateKind::FirstRoundFirstPlayer { trump_candidate}
                           | BidStateKind::FirstRoundSecondPlayer { trump_candidate }
                           | BidStateKind::FirstRoundThirdPlayer { trump_candidate }
                           | BidStateKind::FirstRoundFourthPlayer { trump_candidate }
                           | BidStateKind::OrderedUp { caller: _, trump_candidate }=>
                              html!{<Card ..trump_candidate.clone()/>},
                         _=>
                           html!{<CardBack />},
                        },
                         HandStateKind::FirstTrick { hands: _, trump, trick_state: _ } |
                         HandStateKind::SecondTrick { hands: _, trump, trick_state: _, tricks_taken: _ } |
                         HandStateKind::ThirdTrick { hands: _, trump, trick_state: _, tricks_taken: _ } |
                         HandStateKind::FourthTrick { hands: _, trump, trick_state: _, tricks_taken: _ } |
                         HandStateKind::FifthTrick { hands: _, trump, trick_state: _, tricks_taken: _ } => html!{
                           <div>{trump.to_string()}</div>
                         },
                         HandStateKind::Scoring { tricks_taken: _ } => html!{}, }}
                  </div>
               </div>
            }
        }
    }
}
