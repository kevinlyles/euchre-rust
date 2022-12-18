use crate::{card::Card, deck::Deck, hand::Hand, position::Position};
use itertools::{Itertools, Permutations, Unique};
use std::array::IntoIter;

pub struct HandsIterator {
    my_hand: Hand,
    available_cards: [Card; 18],
    iterator: Unique<Permutations<IntoIter<CardLocation, 18>>>,
}

impl HandsIterator {
    pub fn create(my_hand: Hand, trump_candidate: Card) -> HandsIterator {
        let mut available_cards = Deck::create_all_cards();
        available_cards.retain(|&card| card != trump_candidate && !my_hand.cards.contains(&card));
        assert!(available_cards.len() == 18, "Wrong number of cards!");
        HandsIterator {
            my_hand,
            available_cards: available_cards.try_into().unwrap(),
            iterator: [
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
            .permutations(15)
            .unique(),
        }
    }
}

impl Iterator for HandsIterator {
    type Item = [Hand; 4];

    fn next(&mut self) -> Option<Self::Item> {
        fn generate_hands(
            my_hand: &Hand,
            available_cards: &[Card; 18],
            permutation: Vec<CardLocation>,
        ) -> [Hand; 4] {
            let mut hands = [
                my_hand.clone(),
                Hand {
                    cards: Vec::with_capacity(6),
                },
                Hand {
                    cards: Vec::with_capacity(6),
                },
                Hand {
                    cards: Vec::with_capacity(6),
                },
            ];
            for (&location, &card) in permutation.iter().zip(available_cards) {
                match location {
                    CardLocation::WestHand => hands[Position::West.index()].cards.push(card),
                    CardLocation::NorthHand => hands[Position::North.index()].cards.push(card),
                    CardLocation::EastHand => hands[Position::East.index()].cards.push(card),
                    CardLocation::Kitty => (),
                }
            }
            hands
        }

        match self.iterator.next() {
            Some(permutation) => Some(generate_hands(
                &self.my_hand,
                &self.available_cards,
                permutation,
            )),
            None => None,
        }
    }
}

#[derive(Copy, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum CardLocation {
    WestHand,
    NorthHand,
    EastHand,
    Kitty,
}

impl CardLocation {
    pub fn next(&self) -> CardLocation {
        match self {
            Self::WestHand => Self::NorthHand,
            Self::NorthHand => Self::EastHand,
            Self::EastHand => Self::Kitty,
            Self::Kitty => Self::WestHand,
        }
    }
}
