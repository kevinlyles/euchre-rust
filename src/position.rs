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

    pub fn next_player(
        &self,
        went_alone: Option<Position>,
        defended_alone: Option<Position>,
    ) -> Position {
        let next_player = self.next();
        match went_alone {
            Some(next) if next == next_player.partner() => {
                next_player.next_player(went_alone, defended_alone)
            }
            Some(_) => match defended_alone {
                Some(next) if next == next_player.partner() => {
                    next_player.next_player(went_alone, defended_alone)
                }
                _ => next_player,
            },
            _ => next_player,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(Position::Left, None, None => Position::Top)]
    #[test_case(Position::Left, Some(Position::Right), None => Position::Top)]
    #[test_case(Position::Left, Some(Position::Top), None => Position::Top)]
    #[test_case(Position::Left, Some(Position::Bottom), None => Position::Right)]
    #[test_case(Position::Left, Some(Position::Top), Some(Position::Left) => Position::Top)]
    #[test_case(Position::Left, Some(Position::Left), Some(Position::Top) => Position::Top)]
    #[test_case(Position::Left, Some(Position::Bottom), Some(Position::Left) => Position::Bottom)]
    #[test_case(Position::Left, Some(Position::Left), Some(Position::Bottom) => Position::Bottom)]
    #[test_case(Position::Left, None, Some(Position::Bottom) => Position::Top)]
    #[test_case(Position::Top, None, None => Position::Right)]
    #[test_case(Position::Right, None, None => Position::Bottom)]
    #[test_case(Position::Bottom, None, None => Position::Left)]
    fn next_player(
        player: Position,
        went_alone: Option<Position>,
        defended_alone: Option<Position>,
    ) -> Position {
        player.next_player(went_alone, defended_alone)
    }
}
