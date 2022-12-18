use core::fmt;
use enum_iterator::IntoEnumIterator;

#[derive(Copy, Clone, Debug, IntoEnumIterator, PartialEq, Eq)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Suit {
    pub fn try_create(name: &str) -> Option<Suit> {
        match name {
            "C" => Some(Self::Clubs),
            "D" => Some(Self::Diamonds),
            "H" => Some(Self::Hearts),
            "S" => Some(Self::Spades),
            _ => None,
        }
    }

    pub fn other_suit_of_same_color(&self) -> Suit {
        match self {
            Self::Spades => Suit::Clubs,
            Self::Hearts => Suit::Diamonds,
            Self::Diamonds => Suit::Hearts,
            Self::Clubs => Suit::Spades,
        }
    }

    pub fn starting_point_for_unicode_card(&self) -> u32 {
        match self {
            Self::Spades => 0x1F0A0,
            Self::Hearts => 0x1F0B0,
            Self::Diamonds => 0x1F0C0,
            Self::Clubs => 0x1F0D0,
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let suit = match self {
            Self::Spades => "\u{2660}",
            Self::Hearts => "\u{2665}",
            Self::Diamonds => "\u{2666}",
            Self::Clubs => "\u{2663}",
        };
        write!(f, "{}", suit)
    }
}
