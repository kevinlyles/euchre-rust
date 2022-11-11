use crate::bid_result::BidResult;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Position {
    Left,
    Top,
    Right,
    Bottom,
}

impl Position {
    pub fn index(&self) -> usize {
        match self {
            Self::Left => 0,
            Self::Top => 1,
            Self::Right => 2,
            Self::Bottom => 3,
        }
    }

    pub fn partner(&self) -> Position {
        match self {
            Self::Left => Self::Right,
            Self::Top => Self::Bottom,
            Self::Right => Self::Left,
            Self::Bottom => Self::Top,
        }
    }

    fn next(&self) -> Position {
        match self {
            Self::Left => Self::Top,
            Self::Top => Self::Right,
            Self::Right => Self::Bottom,
            Self::Bottom => Self::Left,
        }
    }

    pub fn next_position_bidding(&self) -> Position {
        self.next()
    }

    pub fn next_position_playing(&self, bid_result: &BidResult) -> Position {
        let next_position = self.next();
        match bid_result {
            BidResult::CalledAlone { caller, .. } if *caller == next_position.partner() => {
                next_position.next_position_playing(bid_result)
            }
            BidResult::DefendedAlone {
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

    #[test_case(Position::Left => Position::Top)]
    #[test_case(Position::Top => Position::Right)]
    #[test_case(Position::Right => Position::Bottom)]
    #[test_case(Position::Bottom => Position::Left)]
    fn next_position_bidding(player: Position) -> Position {
        player.next_position_bidding()
    }

    #[test_case(Position::Left, BidResult::Called { trump: Suit::Hearts, caller: Position::Top } => Position::Top)]
    #[test_case(Position::Left, BidResult::Called { trump: Suit::Hearts, caller: Position::Right } => Position::Top)]
    #[test_case(Position::Left, BidResult::Called { trump: Suit::Hearts, caller: Position::Bottom } => Position::Top)]
    #[test_case(Position::Left, BidResult::Called { trump: Suit::Hearts, caller: Position::Left } => Position::Top)]
    #[test_case(Position::Left, BidResult::CalledAlone { trump: Suit::Hearts, caller: Position::Right } => Position::Top)]
    #[test_case(Position::Left, BidResult::CalledAlone { trump: Suit::Hearts, caller: Position::Top } => Position::Top)]
    #[test_case(Position::Left, BidResult::CalledAlone { trump: Suit::Hearts, caller: Position::Bottom } => Position::Right)]
    #[test_case(Position::Left, BidResult::DefendedAlone { trump: Suit::Hearts, caller: Position::Top, defender: Position::Left } => Position::Top)]
    #[test_case(Position::Left, BidResult::DefendedAlone { trump: Suit::Hearts, caller: Position::Left, defender: Position::Top } => Position::Top)]
    #[test_case(Position::Left, BidResult::DefendedAlone { trump: Suit::Hearts, caller: Position::Bottom, defender: Position::Left } => Position::Bottom)]
    #[test_case(Position::Left, BidResult::DefendedAlone { trump: Suit::Hearts, caller: Position::Left, defender: Position::Bottom } => Position::Bottom)]
    #[test_case(Position::Top, BidResult::Called { trump: Suit::Hearts, caller: Position::Top } => Position::Right)]
    #[test_case(Position::Right, BidResult::Called { trump: Suit::Hearts, caller: Position::Top } => Position::Bottom)]
    #[test_case(Position::Bottom, BidResult::Called { trump: Suit::Hearts, caller: Position::Top } => Position::Left)]
    fn next_position_playing(player: Position, bid_result: BidResult) -> Position {
        player.next_position_playing(&bid_result)
    }
}
