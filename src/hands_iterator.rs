use itertools::{Itertools, Permutations, Unique};
use std::array::IntoIter;

pub fn create() -> Unique<Permutations<IntoIter<CardLocation, 18>>> {
    [
        CardLocation::WestHand,
        CardLocation::WestHand,
        CardLocation::WestHand,
        CardLocation::WestHand,
        CardLocation::WestHand,
        CardLocation::NorthHand,
        CardLocation::NorthHand,
        CardLocation::NorthHand,
        CardLocation::NorthHand,
        CardLocation::NorthHand,
        CardLocation::EastHand,
        CardLocation::EastHand,
        CardLocation::EastHand,
        CardLocation::EastHand,
        CardLocation::EastHand,
        CardLocation::Kitty,
        CardLocation::Kitty,
        CardLocation::Kitty,
    ]
    .into_iter()
    .permutations(18)
    .unique()
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CardLocation {
    WestHand,
    NorthHand,
    EastHand,
    Kitty,
}
