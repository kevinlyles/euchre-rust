use std::fmt::Display;

use crate::{rank::Rank, suit::Suit};
use enum_iterator::IntoEnumIterator;

#[derive(Copy, Clone, Debug, IntoEnumIterator, PartialEq, Eq, PartialOrd, Ord)]
pub enum RankWithBowers {
    RightBower = 16,
    LeftBower = 15,
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
}

impl Display for RankWithBowers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let rank = match self {
            Self::RightBower => "R",
            Self::LeftBower => "L",
            Self::Ace => "A",
            Self::King => "K",
            Self::Queen => "Q",
            Self::Jack => "J",
            Self::Ten => "10",
            Self::Nine => "9",
        };
        write!(f, "{}", rank)
    }
}

impl From<Rank> for RankWithBowers {
    fn from(rank: Rank) -> Self {
        match rank {
            Rank::Ace => Self::Ace,
            Rank::King => Self::King,
            Rank::Queen => Self::Queen,
            Rank::Jack => Self::Jack,
            Rank::Ten => Self::Ten,
            Rank::Nine => Self::Nine,
        }
    }
}

impl RankWithBowers {
    pub fn rank_for_display(&self) -> Rank {
        match self {
            Self::RightBower | Self::LeftBower | Self::Jack => Rank::Jack,
            Self::Ace => Rank::Ace,
            Self::King => Rank::King,
            Self::Queen => Rank::Queen,
            Self::Ten => Rank::Ten,
            Self::Nine => Rank::Nine,
        }
    }

    pub fn suit_for_display(&self, suit: &Suit) -> Suit {
        match self {
            Self::LeftBower => Suit::other_suit_of_same_color(suit),
            _ => *suit,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(RankWithBowers::RightBower => Rank::Jack)]
    #[test_case(RankWithBowers::LeftBower => Rank::Jack)]
    #[test_case(RankWithBowers::Ace => Rank::Ace)]
    #[test_case(RankWithBowers::King => Rank::King)]
    #[test_case(RankWithBowers::Queen => Rank::Queen)]
    #[test_case(RankWithBowers::Jack => Rank::Jack)]
    #[test_case(RankWithBowers::Ten => Rank::Ten)]
    #[test_case(RankWithBowers::Nine => Rank::Nine)]
    fn rank_for_display(rank: RankWithBowers) -> Rank {
        rank.rank_for_display()
    }

    #[test_case(RankWithBowers::RightBower, Suit::Clubs => Suit::Clubs)]
    #[test_case(RankWithBowers::LeftBower, Suit::Clubs => Suit::Spades)]
    #[test_case(RankWithBowers::Ace, Suit::Clubs => Suit::Clubs)]
    #[test_case(RankWithBowers::King, Suit::Clubs => Suit::Clubs)]
    #[test_case(RankWithBowers::Queen, Suit::Clubs => Suit::Clubs)]
    #[test_case(RankWithBowers::Jack, Suit::Clubs => Suit::Clubs)]
    #[test_case(RankWithBowers::Ten, Suit::Clubs => Suit::Clubs)]
    #[test_case(RankWithBowers::Nine, Suit::Clubs => Suit::Clubs)]
    #[test_case(RankWithBowers::RightBower, Suit::Diamonds => Suit::Diamonds)]
    #[test_case(RankWithBowers::LeftBower, Suit::Diamonds => Suit::Hearts)]
    #[test_case(RankWithBowers::Ace, Suit::Diamonds => Suit::Diamonds)]
    #[test_case(RankWithBowers::King, Suit::Diamonds => Suit::Diamonds)]
    #[test_case(RankWithBowers::Queen, Suit::Diamonds => Suit::Diamonds)]
    #[test_case(RankWithBowers::Jack, Suit::Diamonds => Suit::Diamonds)]
    #[test_case(RankWithBowers::Ten, Suit::Diamonds => Suit::Diamonds)]
    #[test_case(RankWithBowers::Nine, Suit::Diamonds => Suit::Diamonds)]
    #[test_case(RankWithBowers::RightBower, Suit::Hearts => Suit::Hearts)]
    #[test_case(RankWithBowers::LeftBower, Suit::Hearts => Suit::Diamonds)]
    #[test_case(RankWithBowers::Ace, Suit::Hearts => Suit::Hearts)]
    #[test_case(RankWithBowers::King, Suit::Hearts => Suit::Hearts)]
    #[test_case(RankWithBowers::Queen, Suit::Hearts => Suit::Hearts)]
    #[test_case(RankWithBowers::Jack, Suit::Hearts => Suit::Hearts)]
    #[test_case(RankWithBowers::Ten, Suit::Hearts => Suit::Hearts)]
    #[test_case(RankWithBowers::Nine, Suit::Hearts => Suit::Hearts)]
    #[test_case(RankWithBowers::RightBower, Suit::Spades => Suit::Spades)]
    #[test_case(RankWithBowers::LeftBower, Suit::Spades => Suit::Clubs)]
    #[test_case(RankWithBowers::Ace, Suit::Spades => Suit::Spades)]
    #[test_case(RankWithBowers::King, Suit::Spades => Suit::Spades)]
    #[test_case(RankWithBowers::Queen, Suit::Spades => Suit::Spades)]
    #[test_case(RankWithBowers::Jack, Suit::Spades => Suit::Spades)]
    #[test_case(RankWithBowers::Ten, Suit::Spades => Suit::Spades)]
    #[test_case(RankWithBowers::Nine, Suit::Spades => Suit::Spades)]
    fn suit_for_display(rank: RankWithBowers, suit: Suit) -> Suit {
        rank.suit_for_display(&suit)
    }
}
