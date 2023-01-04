use crate::{hand::Hand, player::Player, position::Position, rank_with_bowers::RankWithBowers};

#[derive(Clone)]
pub(crate) struct AdvancedPlayer {
    position: Position,
    trump_has_been_lead: bool,
}

impl AdvancedPlayer {
    pub(crate) fn create(position: Position) -> AdvancedPlayer {
        AdvancedPlayer {
            position,
            trump_has_been_lead: false,
        }
    }
}

impl Player for AdvancedPlayer {
    fn should_order_up(
        &mut self,
        hand: &Hand,
        dealer: &Position,
        trump_candidate: &crate::card::Card,
    ) -> bool {
        let trump_cards = hand.cards.iter().filter(|card| {
            card.suit == trump_candidate.suit
                || card.rank == RankWithBowers::Jack
                    && card.suit == trump_candidate.suit.other_suit_of_same_color()
        });
        match trump_cards.count() {
            4 | 5 => true,
            3 if *dealer == self.position || *dealer == self.position.partner() => true,
            _ => false,
        }
    }

    fn call_trump(
        &mut self,
        hand: &Hand,
        _dealer: &Position,
        _turned_down: &crate::card::Card,
    ) -> Option<crate::suit::Suit> {
        if hand
            .cards
            .iter()
            .filter(|card| card.suit == hand.cards[0].suit)
            .count()
            >= 4
        {
            Some(hand.cards[0].suit)
        } else if hand
            .cards
            .iter()
            .filter(|card| card.suit == hand.cards[1].suit)
            .count()
            >= 4
        {
            Some(hand.cards[1].suit)
        } else {
            None
        }
    }

    fn should_order_up_alone(
        &mut self,
        _hand: &Hand,
        _dealer: &Position,
        _trump_candidate: &crate::card::Card,
    ) -> bool {
        false
    }

    fn should_defend_alone_ordered(
        &mut self,
        _hand: &Hand,
        _dealer: &Position,
        _trump_candidate: &crate::card::Card,
    ) -> bool {
        false
    }

    fn should_call_alone(
        &mut self,
        _hand: &Hand,
        _dealer: &Position,
        _trump: &crate::suit::Suit,
        _turned_down: &crate::card::Card,
    ) -> bool {
        false
    }

    fn should_defend_alone_called(
        &mut self,
        _hand: &Hand,
        _dealer: &Position,
        _trump: &crate::suit::Suit,
        _turned_down: &crate::card::Card,
    ) -> bool {
        false
    }

    fn choose_discard(&mut self, hand: &Hand, trump: &crate::suit::Suit) -> crate::card::Card {
        *hand
            .cards
            .iter()
            .filter(|card| card.suit != *trump)
            .nth(0)
            .unwrap_or(&hand.cards[0])
    }

    fn play_card(
        &mut self,
        hand: &Hand,
        _trump: &crate::suit::Suit,
        led: Option<crate::suit::Suit>,
    ) -> crate::card::Card {
        match led {
            Some(suit) => match hand.cards.iter().filter(|card| card.suit == suit).nth(0) {
                Some(card) => *card,
                None => hand.cards[0],
            },
            None => hand.cards[0],
        }
    }
}

mod tests {
    use crate::{
        bid_result::BidResultAll,
        bid_state::BidState,
        card::Card,
        players::{preprogrammed_bidder::PreprogrammedBidder, wrapper::Wrapper},
        suit::Suit,
    };

    use super::*;

    fn test_bidding(
        description: String,
        hand: Hand,
        trump_candidate: Card,
        dealer: Position,
        orders_up: bool,
        discard: Option<Card>,
        calls_suit: Option<Suit>,
        goes_alone: bool,
    ) -> () {
        let am_dealer = dealer == Position::South;

        assert!(
            discard.is_some() || !orders_up || !am_dealer,
            "You must specify a discard if things are orderd up and your'e the dealer"
        );
        if discard.is_some() && orders_up && am_dealer {
            let mut found = hand.cards.contains(&discard.unwrap());
            if !found && orders_up && am_dealer {
                found = trump_candidate == discard.unwrap();
            }
            assert!(
                found,
                "Expected discard is neither in hand nor trump candidate"
            );
        }
        if !orders_up {
            assert!(
                calls_suit.is_none() || calls_suit.unwrap() != trump_candidate.suit,
                "Expected to call trump candidate's suit in round 2"
            );
        }

        let ai: AdvancedPlayer;
        let mut hands = [
            Hand { cards: Vec::new() },
            Hand { cards: Vec::new() },
            hand,
            Hand { cards: Vec::new() },
        ];
        ai = AdvancedPlayer::create(Position::South);
        let mut players = [
            Wrapper::create_single_player(Box::new(PreprogrammedBidder::does_nothing())),
            Wrapper::create_single_player(Box::new(PreprogrammedBidder::does_nothing())),
            Wrapper::create_single_player(Box::new(ai)),
            Wrapper::create_single_player(Box::new(PreprogrammedBidder::does_nothing())),
        ];
        let mut bid = BidState::create(dealer, trump_candidate);
        let bid_result = loop {
            match bid.step(&mut players, &mut hands) {
                Some(bid_result) => {
                    break bid_result;
                }
                None => (),
            }
        };

        if orders_up {
            match bid_result {
                BidResultAll::Called { trump, caller }
                | BidResultAll::CalledAlone { trump, caller } => {
                    assert!(caller == Position::South, "Wrong caller");
                    assert!(trump == trump_candidate.suit, "Wrong suit");
                }
                _ => assert!(false, "Did not order it up"),
            }

            if am_dealer {
                let discard = discard.unwrap();
                assert!(
                    !hands[Position::South.index()].cards.contains(&discard),
                    "Did not discard {}",
                    discard
                );
            }
        }

        if !orders_up {
            if let Some(called_suit) = calls_suit {
                match bid_result {
                    BidResultAll::Called { trump, caller }
                    | BidResultAll::CalledAlone { trump, caller } => {
                        assert!(caller == Position::South, "Wrong caller");
                        assert!(trump == called_suit, "Wrong suit");
                    }
                    _ => assert!(false, "Did not call a suit"),
                }
            }
        }

        if orders_up || calls_suit.is_some() {
            match bid_result {
                BidResultAll::Called { .. } => {
                    assert!(!goes_alone, "Did not go alone")
                }
                BidResultAll::CalledAlone { .. } => assert!(goes_alone, "Went alone"),
                _ => assert!(false, "Nothing was called"),
            }
        }
    }

    /*
    describe("Kevin AI Bidding", function () {
        describe("Smoke tests", function () {
            it("Can be instantiated", function () {
                expect(new KevinAI()).toBeDefined();
            });
        });

        testBidding(
            "All nines and tens, dealer, candidate trump matches",
            [
                new Card(Suit.Spades, Rank.Ten),
                new Card(Suit.Spades, Rank.Nine),
                new Card(Suit.Diamonds, Rank.Nine),
                new Card(Suit.Clubs, Rank.Nine),
                new Card(Suit.Hearts, Rank.Nine),
            ],
            new Card(Suit.Spades, Rank.King),
            Player.South,
            false,
            null,
            null,
            false,
        );

        testBidding(
            "Right nine, off ace, off king, off king, candidate trump does not match",
            [
                new Card(Suit.Spades, Rank.Jack),
                new Card(Suit.Spades, Rank.Nine),
                new Card(Suit.Diamonds, Rank.Ace),
                new Card(Suit.Clubs, Rank.King),
                new Card(Suit.Hearts, Rank.King),
            ],
            new Card(Suit.Diamonds, Rank.King),
            Player.West,
            false,
            null,
            null,
            false,
        );

        testBidding(
            "Right, off ace, off jack ten nine, dealer, candidate trump matches",
            [
                new Card(Suit.Spades, Rank.Jack),
                new Card(Suit.Clubs, Rank.Ace),
                new Card(Suit.Diamonds, Rank.Jack),
                new Card(Suit.Diamonds, Rank.Ten),
                new Card(Suit.Diamonds, Rank.Nine),
            ],
            new Card(Suit.Spades, Rank.Nine),
            Player.South,
            true,
            new Card(Suit.Diamonds, Rank.Nine),
            null,
            false,
        );

        testBidding(
            "Right nine, off ace, off ten nine, dealer, candidate trump matches",
            [
                new Card(Suit.Spades, Rank.Jack),
                new Card(Suit.Spades, Rank.Nine),
                new Card(Suit.Clubs, Rank.Ace),
                new Card(Suit.Diamonds, Rank.Ten),
                new Card(Suit.Diamonds, Rank.Nine),
            ],
            new Card(Suit.Spades, Rank.Ten),
            Player.South,
            true,
            new Card(Suit.Diamonds, Rank.Nine),
            null,
            false,
        );

        testBidding(
            "Right, off ace, off jack ten nine, dealer, candidate trump matches",
            [
                new Card(Suit.Spades, Rank.Jack),
                new Card(Suit.Clubs, Rank.Ace),
                new Card(Suit.Diamonds, Rank.Jack),
                new Card(Suit.Diamonds, Rank.Ten),
                new Card(Suit.Diamonds, Rank.Nine),
            ],
            new Card(Suit.Spades, Rank.Nine),
            Player.South,
            true,
            new Card(Suit.Diamonds, Rank.Nine),
            null,
            false,
        );

        testBidding(
            "Perfect hand, candidate trump does not match",
            [
                new Card(Suit.Spades, Rank.Jack),
                new Card(Suit.Clubs, Rank.Jack),
                new Card(Suit.Spades, Rank.Ace),
                new Card(Suit.Spades, Rank.King),
                new Card(Suit.Spades, Rank.Queen),
            ],
            new Card(Suit.Diamonds, Rank.Ace),
            Player.West,
            false,
            null,
            Suit.Spades,
            true,
        );

        testBidding(
            "Right king queen, two off queens, candidate trump does not match",
            [
                new Card(Suit.Spades, Rank.Jack),
                new Card(Suit.Spades, Rank.King),
                new Card(Suit.Spades, Rank.Queen),
                new Card(Suit.Hearts, Rank.Queen),
                new Card(Suit.Diamonds, Rank.Queen),
            ],
            new Card(Suit.Diamonds, Rank.Ace),
            Player.West,
            false,
            null,
            Suit.Spades,
            false,
        );

        testBidding(
            "Right left king, off king nine, candidate trump does not match",
            [
                new Card(Suit.Spades, Rank.Jack),
                new Card(Suit.Spades, Rank.King),
                new Card(Suit.Clubs, Rank.Jack),
                new Card(Suit.Hearts, Rank.King),
                new Card(Suit.Hearts, Rank.Nine),
            ],
            new Card(Suit.Diamonds, Rank.Ace),
            Player.West,
            false,
            null,
            Suit.Spades,
            true,
        );

        testBidding(
            "Right nine, off ace, off ten nine, candidate trump does not match",
            [
                new Card(Suit.Spades, Rank.Jack),
                new Card(Suit.Spades, Rank.Nine),
                new Card(Suit.Diamonds, Rank.Ace),
                new Card(Suit.Hearts, Rank.Ten),
                new Card(Suit.Hearts, Rank.Nine),
            ],
            new Card(Suit.Diamonds, Rank.King),
            Player.West,
            false,
            null,
            Suit.Spades,
            false,
        );

        testBidding(
            "Right nine, off ace, off king queen, dealer, candidate trump is ten",
            [
                new Card(Suit.Spades, Rank.Jack),
                new Card(Suit.Spades, Rank.Nine),
                new Card(Suit.Diamonds, Rank.Ace),
                new Card(Suit.Hearts, Rank.King),
                new Card(Suit.Hearts, Rank.Queen),
            ],
            new Card(Suit.Spades, Rank.Ten),
            Player.South,
            true,
            new Card(Suit.Hearts, Rank.Queen),
            null,
            false,
        );

        testBidding(
            "Right left, off ace, off king queen, dealer, candidate trump is nine",
            [
                new Card(Suit.Spades, Rank.Jack),
                new Card(Suit.Clubs, Rank.Jack),
                new Card(Suit.Diamonds, Rank.Ace),
                new Card(Suit.Hearts, Rank.King),
                new Card(Suit.Hearts, Rank.Queen),
            ],
            new Card(Suit.Spades, Rank.Ten),
            Player.South,
            true,
            new Card(Suit.Hearts, Rank.Queen),
            null,
            true,
        );

        testBidding(
            "Right nine, off ace, off ten nine, candidate trump matches but goes to opponents",
            [
                new Card(Suit.Spades, Rank.Jack),
                new Card(Suit.Spades, Rank.Nine),
                new Card(Suit.Clubs, Rank.Ace),
                new Card(Suit.Diamonds, Rank.Ten),
                new Card(Suit.Diamonds, Rank.Nine),
            ],
            new Card(Suit.Spades, Rank.Ten),
            Player.West,
            false,
            null,
            null,
            false,
        );

        testBidding(
            "Right nine, off ace, off ten nine, candidate trump matches and goes to partner",
            [
                new Card(Suit.Spades, Rank.Jack),
                new Card(Suit.Spades, Rank.Nine),
                new Card(Suit.Clubs, Rank.Ace),
                new Card(Suit.Diamonds, Rank.Ten),
                new Card(Suit.Diamonds, Rank.Nine),
            ],
            new Card(Suit.Spades, Rank.Ten),
            Player.North,
            true,
            null,
            null,
            false,
        );

        testBidding(
            "Right nine, off ace, off ace, off nine, candidate trump does not match",
            [
                new Card(Suit.Spades, Rank.Jack),
                new Card(Suit.Spades, Rank.Nine),
                new Card(Suit.Clubs, Rank.Ace),
                new Card(Suit.Hearts, Rank.Ace),
                new Card(Suit.Diamonds, Rank.Nine),
            ],
            new Card(Suit.Diamonds, Rank.Ace),
            Player.West,
            false,
            null,
            Suit.Spades,
            false,
        );

        testBidding(
            "Perfect hand, candidate trump matches",
            [
                new Card(Suit.Spades, Rank.Jack),
                new Card(Suit.Spades, Rank.Ace),
                new Card(Suit.Spades, Rank.King),
                new Card(Suit.Spades, Rank.Queen),
                new Card(Suit.Clubs, Rank.Jack),
            ],
            new Card(Suit.Spades, Rank.Ten),
            Player.West,
            true,
            null,
            null,
            true,
        );

        testBidding(
            "Right left ace, off ace king, follow dealer, other suit is better",
            [
                new Card(Suit.Spades, Rank.Jack),
                new Card(Suit.Spades, Rank.Ace),
                new Card(Suit.Clubs, Rank.Jack),
                new Card(Suit.Clubs, Rank.Ace),
                new Card(Suit.Clubs, Rank.King),
            ],
            new Card(Suit.Spades, Rank.King),
            Player.East,
            false,
            null,
            Suit.Clubs,
            true,
        );

        testBidding(
            "Right left ace, off ace king, second after dealer, other suit is better",
            [
                new Card(Suit.Spades, Rank.Jack),
                new Card(Suit.Spades, Rank.Ace),
                new Card(Suit.Clubs, Rank.Jack),
                new Card(Suit.Clubs, Rank.Ace),
                new Card(Suit.Clubs, Rank.King),
            ],
            new Card(Suit.Spades, Rank.King),
            Player.North,
            true,
            null,
            null,
            true,
        );

        testBidding(
            "Right nine, off ace, off king, off king, candidate trump does not match but makes one of the off kings good",
            [
                new Card(Suit.Spades, Rank.Jack),
                new Card(Suit.Spades, Rank.Nine),
                new Card(Suit.Clubs, Rank.Ace),
                new Card(Suit.Diamonds, Rank.King),
                new Card(Suit.Hearts, Rank.King),
            ],
            new Card(Suit.Diamonds, Rank.Ace),
            Player.West,
            false,
            null,
            Suit.Spades,
            false,
        );

        testBidding(
            "King queen ten nine, off nine, candidate trump does not match",
            [
                new Card(Suit.Spades, Rank.King),
                new Card(Suit.Spades, Rank.Queen),
                new Card(Suit.Spades, Rank.Ten),
                new Card(Suit.Spades, Rank.Nine),
                new Card(Suit.Hearts, Rank.Nine),
            ],
            new Card(Suit.Diamonds, Rank.Ace),
            Player.West,
            false,
            null,
            Suit.Spades,
            false,
        );

        testBidding(
            "Perfect hand, candidate trump matches, dealer",
            [
                new Card(Suit.Spades, Rank.Jack),
                new Card(Suit.Clubs, Rank.Jack),
                new Card(Suit.Spades, Rank.Ace),
                new Card(Suit.Spades, Rank.King),
                new Card(Suit.Spades, Rank.Queen),
            ],
            new Card(Suit.Spades, Rank.Ten),
            Player.South,
            true,
            new Card(Suit.Spades, Rank.Ten),
            null,
            true,
        );

        testBidding(
            "Perfect hand after picking it up, candidate trump matches, dealer",
            [
                new Card(Suit.Spades, Rank.Jack),
                new Card(Suit.Clubs, Rank.Jack),
                new Card(Suit.Spades, Rank.Ace),
                new Card(Suit.Spades, Rank.King),
                new Card(Suit.Diamonds, Rank.Ten),
            ],
            new Card(Suit.Spades, Rank.Queen),
            Player.South,
            true,
            new Card(Suit.Diamonds, Rank.Ten),
            null,
            true,
        );

        testBidding(
            "Right left ace, off queen, off ten, candidate trump matches, dealer",
            [
                new Card(Suit.Spades, Rank.Jack),
                new Card(Suit.Clubs, Rank.Jack),
                new Card(Suit.Spades, Rank.Ace),
                new Card(Suit.Diamonds, Rank.Queen),
                new Card(Suit.Hearts, Rank.Ten),
            ],
            new Card(Suit.Spades, Rank.King),
            Player.South,
            true,
            new Card(Suit.Hearts, Rank.Ten),
            null,
            false,
        );

        testBidding(
            "Right left ace, off queen, off ten, candidate trump matches, dealer (other order of offsuits)",
            [
                new Card(Suit.Spades, Rank.Jack),
                new Card(Suit.Clubs, Rank.Jack),
                new Card(Suit.Spades, Rank.Ace),
                new Card(Suit.Hearts, Rank.Queen),
                new Card(Suit.Diamonds, Rank.Ten),
            ],
            new Card(Suit.Spades, Rank.King),
            Player.South,
            true,
            new Card(Suit.Diamonds, Rank.Ten),
            null,
            false,
        );

        testBidding(
            "Right left ace, off king queen, candidate trump matches, dealer",
            [
                new Card(Suit.Spades, Rank.Jack),
                new Card(Suit.Clubs, Rank.Jack),
                new Card(Suit.Spades, Rank.Ace),
                new Card(Suit.Hearts, Rank.King),
                new Card(Suit.Hearts, Rank.Queen),
            ],
            new Card(Suit.Spades, Rank.King),
            Player.South,
            true,
            new Card(Suit.Hearts, Rank.Queen),
            null,
            true,
        );

        testBidding(
            "Right left ace, off king queen, candidate trump matches, dealer (other order of offsuits)",
            [
                new Card(Suit.Spades, Rank.Jack),
                new Card(Suit.Clubs, Rank.Jack),
                new Card(Suit.Spades, Rank.Ace),
                new Card(Suit.Hearts, Rank.Queen),
                new Card(Suit.Hearts, Rank.King),
            ],
            new Card(Suit.Spades, Rank.King),
            Player.South,
            true,
            new Card(Suit.Hearts, Rank.Queen),
            null,
            true,
        );

        testBidding(
            "Right left ace king, off ace, candidate trump matches, dealer",
            [
                new Card(Suit.Spades, Rank.Jack),
                new Card(Suit.Clubs, Rank.Jack),
                new Card(Suit.Spades, Rank.Ace),
                new Card(Suit.Spades, Rank.King),
                new Card(Suit.Hearts, Rank.Ace),
            ],
            new Card(Suit.Spades, Rank.Queen),
            Player.South,
            true,
            new Card(Suit.Hearts, Rank.Ace),
            null,
            true,
        );

        testBidding(
            "Right ace king, off king queen, candidate trump is left, dealer",
            [
                new Card(Suit.Spades, Rank.Jack),
                new Card(Suit.Spades, Rank.Ace),
                new Card(Suit.Spades, Rank.King),
                new Card(Suit.Hearts, Rank.King),
                new Card(Suit.Hearts, Rank.Queen),
            ],
            new Card(Suit.Clubs, Rank.Jack),
            Player.East,
            false,
            null,
            Suit.Spades,
            true,
        );
    });
    */
}
