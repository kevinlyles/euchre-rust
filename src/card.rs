use crate::{rank::Rank, rank_with_bowers::RankWithBowers, suit::Suit};
use core::fmt;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct CardBeforeBidding {
    pub suit: Suit,
    pub rank: Rank,
}

impl From<Card> for CardBeforeBidding {
    fn from(value: Card) -> Self {
        CardBeforeBidding {
            suit: if value.rank == RankWithBowers::LeftBower {
                value.suit.other_suit_of_same_color()
            } else {
                value.suit
            },
            rank: value.rank.rank_for_display(),
        }
    }
}

impl FromStr for CardBeforeBidding {
    type Err = String;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        let (rank_name, suit_name) = name.split_at(name.len() - 1);
        match Rank::from_str(rank_name) {
            Ok(rank) => Suit::from_str(suit_name).map(|suit| CardBeforeBidding { rank, suit }),
            Err(error) => Err(error),
        }
    }
}

impl fmt::Display for CardBeforeBidding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let unicode_value =
            self.suit.starting_point_for_unicode_card() + self.rank.offset_for_unicode_card();
        let unicode_char = char::from_u32(unicode_value);
        match unicode_char {
            Some(c) => write!(f, "{}", c),
            _ => write!(f, "{}{}", self.rank, self.suit),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub rank: RankWithBowers,
}

impl fmt::Display for Card {
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

impl Card {
    pub fn update_bowers(cards: Vec<CardBeforeBidding>, &trump: &Suit) -> Vec<Card> {
        cards
            .into_iter()
            .map(|card| match card.rank {
                Rank::Jack if card.suit == trump => Card {
                    suit: trump,
                    rank: RankWithBowers::RightBower,
                },
                Rank::Jack if card.suit == trump.other_suit_of_same_color() => Card {
                    suit: trump,
                    rank: RankWithBowers::LeftBower,
                },
                rank => Card {
                    suit: card.suit,
                    rank: rank.into(),
                },
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(Card {suit: Suit::Clubs, rank: RankWithBowers::RightBower} => "\u{1F0DB}")]
    #[test_case(Card {suit: Suit::Clubs, rank: RankWithBowers::LeftBower} => "\u{1F0AB}")]
    #[test_case(Card {suit: Suit::Clubs, rank: RankWithBowers::Ace} => "\u{1F0D1}")]
    #[test_case(Card {suit: Suit::Clubs, rank: RankWithBowers::King} => "\u{1F0DE}")]
    #[test_case(Card {suit: Suit::Clubs, rank: RankWithBowers::Queen} => "\u{1F0DD}")]
    #[test_case(Card {suit: Suit::Clubs, rank: RankWithBowers::Jack} => "\u{1F0DB}")]
    #[test_case(Card {suit: Suit::Clubs, rank: RankWithBowers::Ten} => "\u{1F0DA}")]
    #[test_case(Card {suit: Suit::Clubs, rank: RankWithBowers::Nine} => "\u{1F0D9}")]
    #[test_case(Card {suit: Suit::Diamonds, rank: RankWithBowers::RightBower} => "\u{1F0CB}")]
    #[test_case(Card {suit: Suit::Diamonds, rank: RankWithBowers::LeftBower} => "\u{1F0BB}")]
    #[test_case(Card {suit: Suit::Diamonds, rank: RankWithBowers::Ace} => "\u{1F0C1}")]
    #[test_case(Card {suit: Suit::Diamonds, rank: RankWithBowers::King} => "\u{1F0CE}")]
    #[test_case(Card {suit: Suit::Diamonds, rank: RankWithBowers::Queen} => "\u{1F0CD}")]
    #[test_case(Card {suit: Suit::Diamonds, rank: RankWithBowers::Jack} => "\u{1F0CB}")]
    #[test_case(Card {suit: Suit::Diamonds, rank: RankWithBowers::Ten} => "\u{1F0CA}")]
    #[test_case(Card {suit: Suit::Diamonds, rank: RankWithBowers::Nine} => "\u{1F0C9}")]
    #[test_case(Card {suit: Suit::Hearts, rank: RankWithBowers::RightBower} => "\u{1F0BB}")]
    #[test_case(Card {suit: Suit::Hearts, rank: RankWithBowers::LeftBower} => "\u{1F0CB}")]
    #[test_case(Card {suit: Suit::Hearts, rank: RankWithBowers::Ace} => "\u{1F0B1}")]
    #[test_case(Card {suit: Suit::Hearts, rank: RankWithBowers::King} => "\u{1F0BE}")]
    #[test_case(Card {suit: Suit::Hearts, rank: RankWithBowers::Queen} => "\u{1F0BD}")]
    #[test_case(Card {suit: Suit::Hearts, rank: RankWithBowers::Jack} => "\u{1F0BB}")]
    #[test_case(Card {suit: Suit::Hearts, rank: RankWithBowers::Ten} => "\u{1F0BA}")]
    #[test_case(Card {suit: Suit::Hearts, rank: RankWithBowers::Nine} => "\u{1F0B9}")]
    #[test_case(Card {suit: Suit::Spades, rank: RankWithBowers::RightBower} => "\u{1F0AB}")]
    #[test_case(Card {suit: Suit::Spades, rank: RankWithBowers::LeftBower} => "\u{1F0DB}")]
    #[test_case(Card {suit: Suit::Spades, rank: RankWithBowers::Ace} => "\u{1F0A1}")]
    #[test_case(Card {suit: Suit::Spades, rank: RankWithBowers::King} => "\u{1F0AE}")]
    #[test_case(Card {suit: Suit::Spades, rank: RankWithBowers::Queen} => "\u{1F0AD}")]
    #[test_case(Card {suit: Suit::Spades, rank: RankWithBowers::Jack} => "\u{1F0AB}")]
    #[test_case(Card {suit: Suit::Spades, rank: RankWithBowers::Ten} => "\u{1F0AA}")]
    #[test_case(Card {suit: Suit::Spades, rank: RankWithBowers::Nine} => "\u{1F0A9}")]
    fn display(card: Card) -> String {
        card.to_string()
    }

    #[test_case("9C" => Ok(CardBeforeBidding {rank:Rank::Nine, suit: Suit::Clubs}))]
    #[test_case("9D" => Ok(CardBeforeBidding {rank:Rank::Nine, suit: Suit::Diamonds}))]
    #[test_case("9H" => Ok(CardBeforeBidding {rank:Rank::Nine, suit: Suit::Hearts}))]
    #[test_case("9S" => Ok(CardBeforeBidding {rank:Rank::Nine, suit: Suit::Spades}))]
    #[test_case("NC" => Ok(CardBeforeBidding {rank:Rank::Nine, suit: Suit::Clubs}))]
    #[test_case("10C" => Ok(CardBeforeBidding {rank:Rank::Ten, suit: Suit::Clubs}))]
    #[test_case("TC" => Ok(CardBeforeBidding {rank:Rank::Ten, suit: Suit::Clubs}))]
    #[test_case("JC" => Ok(CardBeforeBidding {rank:Rank::Jack, suit: Suit::Clubs}))]
    #[test_case("QC" => Ok(CardBeforeBidding {rank:Rank::Queen, suit: Suit::Clubs}))]
    #[test_case("KC" => Ok(CardBeforeBidding {rank:Rank::King, suit: Suit::Clubs}))]
    #[test_case("AC" => Ok(CardBeforeBidding {rank:Rank::Ace, suit: Suit::Clubs}))]
    fn from_str(name: &str) -> Result<CardBeforeBidding, String> {
        CardBeforeBidding::from_str(name)
    }
}
