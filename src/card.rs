use core::fmt;
use enum_iterator::IntoEnumIterator;
use yew::prelude::*;

use crate::rank_with_bowers::RankWithBowers;

#[function_component(Card)]
pub fn card(card: &CardProps) -> Html {
    html! {
        <span style={format!("color:{}; font-size: xx-large;", card.suit.color())}>{card}</span>
    }
}

#[function_component(CardBack)]
pub fn card_back() -> Html {
    html! {
        <span style="color: blue; font-size: xx-large;">{"\u{1F0A0}"}</span>
    }
}

#[derive(Copy, Clone, Properties, PartialEq)]
pub struct CardProps {
    pub suit: Suit,
    pub rank: RankWithBowers,
}

impl fmt::Display for CardProps {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unicode_value = self
            .rank
            .suit_for_display(&self.suit)
            .starting_point_for_unicode_card()
            + self.rank.rank_for_display().offset_for_unicode_card();
        let unicode_char = char::from_u32(unicode_value);
        match unicode_char {
            Some(c) => write!(f, "{}", c),
            _ => write!(
                f,
                "{}{}",
                self.rank.rank_for_display(),
                self.rank.suit_for_display(&self.suit)
            ),
        }
    }
}

#[derive(Copy, Clone, Debug, IntoEnumIterator, PartialEq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    pub fn other_suit_of_same_color(&self) -> Suit {
        match self {
            Self::Clubs => Suit::Spades,
            Self::Diamonds => Suit::Hearts,
            Self::Hearts => Suit::Diamonds,
            Self::Spades => Suit::Clubs,
        }
    }

    pub fn starting_point_for_unicode_card(&self) -> u32 {
        match self {
            Self::Clubs => 0x1F0D0,
            Self::Diamonds => 0x1F0C0,
            Self::Hearts => 0x1F0B0,
            Self::Spades => 0x1F0A0,
        }
    }

    pub fn color(&self) -> String {
        match self {
            Self::Clubs | Self::Spades => String::from("black"),
            Self::Diamonds | Self::Hearts => String::from("red"),
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suit = match self {
            Self::Clubs => "\u{2663}",
            Self::Diamonds => "\u{2666}",
            Self::Hearts => "\u{2665}",
            Self::Spades => "\u{2660}",
        };
        write!(f, "{}", suit)
    }
}
