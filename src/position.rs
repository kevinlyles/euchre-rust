use crate::bid_result::BidResultCalled;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Position {
    West,
    North,
    East,
    South,
}

impl Position {
    pub fn index(&self) -> usize {
        match self {
            Self::North => 0,
            Self::East => 1,
            Self::South => 2,
            Self::West => 3,
        }
    }

    pub fn partner(&self) -> Position {
        match self {
            Self::West => Self::East,
            Self::North => Self::South,
            Self::East => Self::West,
            Self::South => Self::North,
        }
    }

    fn next(&self) -> Position {
        match self {
            Self::West => Self::North,
            Self::North => Self::East,
            Self::East => Self::South,
            Self::South => Self::West,
        }
    }

    pub fn next_position_bidding(&self) -> Position {
        self.next()
    }

    pub fn next_position_playing(&self, bid_result: &BidResultCalled) -> Position {
        let next_position = self.next();
        match bid_result {
            BidResultCalled::CalledAlone { caller, .. } if *caller == next_position.partner() => {
                next_position.next_position_playing(bid_result)
            }
            BidResultCalled::DefendedAlone {
                caller, defender, ..
            } if *caller == next_position.partner() || *defender == next_position.partner() => {
                next_position.next_position_playing(bid_result)
            }
            _ => next_position,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::suit::Suit;
    use test_case::test_case;

    #[test_case(Position::West => Position::North)]
    #[test_case(Position::North => Position::East)]
    #[test_case(Position::East => Position::South)]
    #[test_case(Position::South => Position::West)]
    fn next_position_bidding(player: Position) -> Position {
        player.next_position_bidding()
    }

    #[test_case(Position::West, BidResultCalled::Called { trump: Suit::Hearts, caller: Position::North } => Position::North)]
    #[test_case(Position::West, BidResultCalled::Called { trump: Suit::Hearts, caller: Position::East } => Position::North)]
    #[test_case(Position::West, BidResultCalled::Called { trump: Suit::Hearts, caller: Position::South } => Position::North)]
    #[test_case(Position::West, BidResultCalled::Called { trump: Suit::Hearts, caller: Position::West } => Position::North)]
    #[test_case(Position::West, BidResultCalled::CalledAlone { trump: Suit::Hearts, caller: Position::East } => Position::North)]
    #[test_case(Position::West, BidResultCalled::CalledAlone { trump: Suit::Hearts, caller: Position::North } => Position::North)]
    #[test_case(Position::West, BidResultCalled::CalledAlone { trump: Suit::Hearts, caller: Position::South } => Position::East)]
    #[test_case(Position::West, BidResultCalled::DefendedAlone { trump: Suit::Hearts, caller: Position::North, defender: Position::West } => Position::North)]
    #[test_case(Position::West, BidResultCalled::DefendedAlone { trump: Suit::Hearts, caller: Position::West, defender: Position::North } => Position::North)]
    #[test_case(Position::West, BidResultCalled::DefendedAlone { trump: Suit::Hearts, caller: Position::South, defender: Position::West } => Position::South)]
    #[test_case(Position::West, BidResultCalled::DefendedAlone { trump: Suit::Hearts, caller: Position::West, defender: Position::South } => Position::South)]
    #[test_case(Position::North, BidResultCalled::Called { trump: Suit::Hearts, caller: Position::North } => Position::East)]
    #[test_case(Position::East, BidResultCalled::Called { trump: Suit::Hearts, caller: Position::North } => Position::South)]
    #[test_case(Position::South, BidResultCalled::Called { trump: Suit::Hearts, caller: Position::North } => Position::West)]
    fn next_position_playing(player: Position, bid_result: BidResultCalled) -> Position {
        player.next_position_playing(&bid_result)
    }
}
