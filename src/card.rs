use crate::{rank_with_bowers::RankWithBowers, suit::Suit};
use core::fmt;

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
    pub fn try_create(name: &str) -> Option<Card> {
        let (rank_name, suit_name) = name.split_at(name.len() - 1);
        match RankWithBowers::try_create(rank_name) {
            Some(rank) => match Suit::try_create(suit_name) {
                Some(suit) => Some(Card { rank, suit }),
                None => None,
            },
            None => None,
        }
    }

    pub fn update_bowers(cards: &mut Vec<Card>, trump: &Suit) -> () {
        for card in cards.iter_mut() {
            if card.rank != RankWithBowers::Jack {
                continue;
            }
            if card.suit == *trump {
                *card = Card {
                    rank: RankWithBowers::RightBower,
                    suit: *trump,
                };
            } else if card.suit.other_suit_of_same_color() == *trump {
                *card = Card {
                    rank: RankWithBowers::LeftBower,
                    suit: *trump,
                };
            }
        }
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

    #[test_case("9C" => Some(Card {rank:RankWithBowers::Nine, suit: Suit::Clubs}))]
    #[test_case("9D" => Some(Card {rank:RankWithBowers::Nine, suit: Suit::Diamonds}))]
    #[test_case("9H" => Some(Card {rank:RankWithBowers::Nine, suit: Suit::Hearts}))]
    #[test_case("9S" => Some(Card {rank:RankWithBowers::Nine, suit: Suit::Spades}))]
    #[test_case("NC" => Some(Card {rank:RankWithBowers::Nine, suit: Suit::Clubs}))]
    #[test_case("10C" => Some(Card {rank:RankWithBowers::Ten, suit: Suit::Clubs}))]
    #[test_case("TC" => Some(Card {rank:RankWithBowers::Ten, suit: Suit::Clubs}))]
    #[test_case("JC" => Some(Card {rank:RankWithBowers::Jack, suit: Suit::Clubs}))]
    #[test_case("QC" => Some(Card {rank:RankWithBowers::Queen, suit: Suit::Clubs}))]
    #[test_case("KC" => Some(Card {rank:RankWithBowers::King, suit: Suit::Clubs}))]
    #[test_case("AC" => Some(Card {rank:RankWithBowers::Ace, suit: Suit::Clubs}))]
    #[test_case("LC" => Some(Card {rank:RankWithBowers::LeftBower, suit: Suit::Clubs}))]
    #[test_case("RC" => Some(Card {rank:RankWithBowers::RightBower, suit: Suit::Clubs}))]
    fn try_create(name: &str) -> Option<Card> {
        Card::try_create(name)
    }
}
