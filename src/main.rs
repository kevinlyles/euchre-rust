use core::fmt;
use yew::prelude::*;

fn main() {
    yew::start_app::<App>();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Card suit={Suit::Clubs} rank={RankWithBowers::RightBower} />
            <Card suit={Suit::Clubs} rank={RankWithBowers::LeftBower} />
            <Card suit={Suit::Clubs} rank={RankWithBowers::Ace} />
            <br/>
            <Card suit={Suit::Diamonds} rank={RankWithBowers::RightBower} />
            <Card suit={Suit::Diamonds} rank={RankWithBowers::LeftBower} />
            <Card suit={Suit::Diamonds} rank={RankWithBowers::Ace} />
            <br/>
            <Card suit={Suit::Hearts} rank={RankWithBowers::RightBower} />
            <Card suit={Suit::Hearts} rank={RankWithBowers::LeftBower} />
            <Card suit={Suit::Hearts} rank={RankWithBowers::Ace} />
            <br/>
            <Card suit={Suit::Spades} rank={RankWithBowers::RightBower} />
            <Card suit={Suit::Spades} rank={RankWithBowers::LeftBower} />
            <Card suit={Suit::Spades} rank={RankWithBowers::King} />
            <Card suit={Suit::Spades} rank={RankWithBowers::Queen} />
            <Card suit={Suit::Spades} rank={RankWithBowers::Jack} />
            <Card suit={Suit::Spades} rank={RankWithBowers::Ten} />
            <Card suit={Suit::Spades} rank={RankWithBowers::Nine} />
            <br/>
            <CardBack />
        </>
    }
}

#[function_component(Card)]
fn card(card: &CardProps) -> Html {
    html! {
        <span style={format!("color:{}; font-size: xx-large;", card.suit.color())}>{card}</span>
    }
}

#[function_component(CardBack)]
fn card_back() -> Html {
    html! {
        <span style="color: blue; font-size: xx-large;">{"\u{1F0A0}"}</span>
    }
}

#[derive(Properties, PartialEq)]
struct CardProps {
    suit: Suit,
    rank: RankWithBowers,
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

#[derive(Clone, PartialEq)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl Suit {
    fn other_suit_of_same_color(&self) -> Suit {
        match self {
            Self::Clubs => Suit::Spades,
            Self::Diamonds => Suit::Hearts,
            Self::Hearts => Suit::Diamonds,
            Self::Spades => Suit::Clubs,
        }
    }

    fn starting_point_for_unicode_card(&self) -> u32 {
        match self {
            Self::Clubs => 0x1F0D0,
            Self::Diamonds => 0x1F0C0,
            Self::Hearts => 0x1F0B0,
            Self::Spades => 0x1F0A0,
        }
    }

    fn color(&self) -> String {
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

#[derive(Clone, PartialEq)]
enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
}

impl Rank {
    fn offset_for_unicode_card(&self) -> u32 {
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

#[derive(Clone, PartialEq)]
enum RankWithBowers {
    RightBower,
    LeftBower,
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
}

impl RankWithBowers {
    fn suit_for_display(&self, suit: &Suit) -> Suit {
        match self {
            Self::LeftBower => Suit::other_suit_of_same_color(suit),
            _ => suit.clone(),
        }
    }

    fn rank_for_display(&self) -> Rank {
        match self {
            Self::RightBower | Self::LeftBower | Self::Jack => Rank::Jack,
            Self::Ace => Rank::Ace,
            Self::King => Rank::King,
            Self::Queen => Rank::Queen,
            Self::Ten => Rank::Ten,
            Self::Nine => Rank::Nine,
        }
    }
}
