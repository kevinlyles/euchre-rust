use core::fmt;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
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
