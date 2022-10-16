use crate::card::*;
use crate::deck::*;
use crate::hand::*;
use yew::prelude::*;

#[function_component(Euchre)]
pub fn euchre() -> Html {
    let cards = vec![
        CardProps {
            suit: Suit::Clubs,
            rank: RankWithBowers::Ace,
        },
        CardProps {
            suit: Suit::Diamonds,
            rank: RankWithBowers::Ace,
        },
        CardProps {
            suit: Suit::Hearts,
            rank: RankWithBowers::Ace,
        },
    ];
    html! {
    <>
        <div class="playing-surface">
            <div class="player left" />
            <div class="player top" />
            <div class="player right" />
            <div class="player bottom" />
            <div class="center">
                <Hand cards={cards.clone()} visible={true} />
                <br/>
                <Hand cards={cards} visible={false} />
                <br/>
                {Deck::create_shuffled_deck()
                    .cards
                    .iter()
                    .map(|card| html! {<Card ..*card/>})
                    .collect::<Html>()}
            </div>
        </div>
    </>    }
}
