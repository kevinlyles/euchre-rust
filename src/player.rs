#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Player {
    Left,
    Top,
    Right,
    Bottom,
}

impl Player {
    pub fn index(&self) -> usize {
        match self {
            Self::Left => 0,
            Self::Top => 1,
            Self::Right => 2,
            Self::Bottom => 3,
        }
    }

    pub fn partner(&self) -> Player {
        match self {
            Self::Left => Self::Right,
            Self::Top => Self::Bottom,
            Self::Right => Self::Left,
            Self::Bottom => Self::Top,
        }
    }

    fn next(&self) -> Player {
        match self {
            Self::Left => Self::Top,
            Self::Top => Self::Right,
            Self::Right => Self::Bottom,
            Self::Bottom => Self::Left,
        }
    }

    pub fn next_player(
        &self,
        went_alone: Option<Player>,
        defended_alone: Option<Player>,
    ) -> Player {
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

    #[test_case(Player::Left, None, None => Player::Top)]
    #[test_case(Player::Left, Some(Player::Right), None => Player::Top)]
    #[test_case(Player::Left, Some(Player::Top), None => Player::Top)]
    #[test_case(Player::Left, Some(Player::Bottom), None => Player::Right)]
    #[test_case(Player::Left, Some(Player::Top), Some(Player::Left) => Player::Top)]
    #[test_case(Player::Left, Some(Player::Left), Some(Player::Top) => Player::Top)]
    #[test_case(Player::Left, Some(Player::Bottom), Some(Player::Left) => Player::Bottom)]
    #[test_case(Player::Left, Some(Player::Left), Some(Player::Bottom) => Player::Bottom)]
    #[test_case(Player::Left, None, Some(Player::Bottom) => Player::Top)]
    #[test_case(Player::Top, None, None => Player::Right)]
    #[test_case(Player::Right, None, None => Player::Bottom)]
    #[test_case(Player::Bottom, None, None => Player::Left)]
    fn next_player(
        player: Player,
        went_alone: Option<Player>,
        defended_alone: Option<Player>,
    ) -> Player {
        player.next_player(went_alone, defended_alone)
    }
}
