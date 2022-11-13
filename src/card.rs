use crate::{rank_with_bowers::RankWithBowers, suit::Suit};
use core::fmt;

#[derive(Copy, Clone, PartialEq)]
pub struct CardLogic {
    pub suit: Suit,
    pub rank: RankWithBowers,
}

impl fmt::Display for CardLogic {
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

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(CardLogic {suit: Suit::Clubs, rank: RankWithBowers::RightBower} => "\u{1F0DB}")]
    #[test_case(CardLogic {suit: Suit::Clubs, rank: RankWithBowers::LeftBower} => "\u{1F0AB}")]
    #[test_case(CardLogic {suit: Suit::Clubs, rank: RankWithBowers::Ace} => "\u{1F0D1}")]
    #[test_case(CardLogic {suit: Suit::Clubs, rank: RankWithBowers::King} => "\u{1F0DE}")]
    #[test_case(CardLogic {suit: Suit::Clubs, rank: RankWithBowers::Queen} => "\u{1F0DD}")]
    #[test_case(CardLogic {suit: Suit::Clubs, rank: RankWithBowers::Jack} => "\u{1F0DB}")]
    #[test_case(CardLogic {suit: Suit::Clubs, rank: RankWithBowers::Ten} => "\u{1F0DA}")]
    #[test_case(CardLogic {suit: Suit::Clubs, rank: RankWithBowers::Nine} => "\u{1F0D9}")]
    #[test_case(CardLogic {suit: Suit::Diamonds, rank: RankWithBowers::RightBower} => "\u{1F0CB}")]
    #[test_case(CardLogic {suit: Suit::Diamonds, rank: RankWithBowers::LeftBower} => "\u{1F0BB}")]
    #[test_case(CardLogic {suit: Suit::Diamonds, rank: RankWithBowers::Ace} => "\u{1F0C1}")]
    #[test_case(CardLogic {suit: Suit::Diamonds, rank: RankWithBowers::King} => "\u{1F0CE}")]
    #[test_case(CardLogic {suit: Suit::Diamonds, rank: RankWithBowers::Queen} => "\u{1F0CD}")]
    #[test_case(CardLogic {suit: Suit::Diamonds, rank: RankWithBowers::Jack} => "\u{1F0CB}")]
    #[test_case(CardLogic {suit: Suit::Diamonds, rank: RankWithBowers::Ten} => "\u{1F0CA}")]
    #[test_case(CardLogic {suit: Suit::Diamonds, rank: RankWithBowers::Nine} => "\u{1F0C9}")]
    #[test_case(CardLogic {suit: Suit::Hearts, rank: RankWithBowers::RightBower} => "\u{1F0BB}")]
    #[test_case(CardLogic {suit: Suit::Hearts, rank: RankWithBowers::LeftBower} => "\u{1F0CB}")]
    #[test_case(CardLogic {suit: Suit::Hearts, rank: RankWithBowers::Ace} => "\u{1F0B1}")]
    #[test_case(CardLogic {suit: Suit::Hearts, rank: RankWithBowers::King} => "\u{1F0BE}")]
    #[test_case(CardLogic {suit: Suit::Hearts, rank: RankWithBowers::Queen} => "\u{1F0BD}")]
    #[test_case(CardLogic {suit: Suit::Hearts, rank: RankWithBowers::Jack} => "\u{1F0BB}")]
    #[test_case(CardLogic {suit: Suit::Hearts, rank: RankWithBowers::Ten} => "\u{1F0BA}")]
    #[test_case(CardLogic {suit: Suit::Hearts, rank: RankWithBowers::Nine} => "\u{1F0B9}")]
    #[test_case(CardLogic {suit: Suit::Spades, rank: RankWithBowers::RightBower} => "\u{1F0AB}")]
    #[test_case(CardLogic {suit: Suit::Spades, rank: RankWithBowers::LeftBower} => "\u{1F0DB}")]
    #[test_case(CardLogic {suit: Suit::Spades, rank: RankWithBowers::Ace} => "\u{1F0A1}")]
    #[test_case(CardLogic {suit: Suit::Spades, rank: RankWithBowers::King} => "\u{1F0AE}")]
    #[test_case(CardLogic {suit: Suit::Spades, rank: RankWithBowers::Queen} => "\u{1F0AD}")]
    #[test_case(CardLogic {suit: Suit::Spades, rank: RankWithBowers::Jack} => "\u{1F0AB}")]
    #[test_case(CardLogic {suit: Suit::Spades, rank: RankWithBowers::Ten} => "\u{1F0AA}")]
    #[test_case(CardLogic {suit: Suit::Spades, rank: RankWithBowers::Nine} => "\u{1F0A9}")]
    fn display(card: CardLogic) -> String {
        card.to_string()
    }
}
