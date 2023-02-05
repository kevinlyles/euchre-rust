use core::fmt;
use enum_iterator::IntoEnumIterator;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq, IntoEnumIterator, PartialOrd, Ord)]
pub enum Rank {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
}

impl Rank {
    pub fn offset_for_unicode_card(&self) -> u32 {
        match self {
            Self::Ace => 0x1,
            Self::King => 0xE,
            Self::Queen => 0xD,
            Self::Jack => 0xB,
            Self::Ten => 0xA,
            Self::Nine => 0x9,
        }
    }
}

impl FromStr for Rank {
    type Err = String;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        match name {
            "A" => Ok(Self::Ace),
            "K" => Ok(Self::King),
            "Q" => Ok(Self::Queen),
            "J" => Ok(Self::Jack),
            "10" | "T" => Ok(Self::Ten),
            "9" | "N" => Ok(Self::Nine),
            _ => Err(format!("Invalid rank: {}", name)),
        }
    }
}

impl fmt::Display for Rank {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rank = match self {
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
