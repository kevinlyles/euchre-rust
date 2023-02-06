use crate::{position::Position, suit::Suit};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BidResultAll {
    Called {
        trump: Suit,
        caller: Position,
    },
    CalledAlone {
        trump: Suit,
        caller: Position,
    },
    DefendedAlone {
        trump: Suit,
        caller: Position,
        defender: Position,
    },
    NoOneCalled,
}

impl From<BidResultCalled> for BidResultAll {
    fn from(value: BidResultCalled) -> Self {
        match value {
            BidResultCalled::Called { trump, caller } => BidResultAll::Called { trump, caller },
            BidResultCalled::CalledAlone { trump, caller } => {
                BidResultAll::CalledAlone { trump, caller }
            }
            BidResultCalled::DefendedAlone {
                trump,
                caller,
                defender,
            } => BidResultAll::DefendedAlone {
                trump,
                caller,
                defender,
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BidResultCalled {
    Called {
        trump: Suit,
        caller: Position,
    },
    CalledAlone {
        trump: Suit,
        caller: Position,
    },
    DefendedAlone {
        trump: Suit,
        caller: Position,
        defender: Position,
    },
}

impl TryFrom<BidResultAll> for BidResultCalled {
    type Error = &'static str;

    fn try_from(value: BidResultAll) -> Result<Self, Self::Error> {
        match value {
            BidResultAll::Called { trump, caller } => Ok(Self::Called { trump, caller }),
            BidResultAll::CalledAlone { trump, caller } => Ok(Self::CalledAlone { trump, caller }),
            BidResultAll::DefendedAlone {
                trump,
                caller,
                defender,
            } => Ok(Self::DefendedAlone {
                trump,
                caller,
                defender,
            }),
            BidResultAll::NoOneCalled => {
                Err("BidResultCalled only accepts bid results that resulted in a call.")
            }
        }
    }
}

impl BidResultCalled {
    pub fn trump(&self) -> Suit {
        match self {
            Self::Called { trump, .. }
            | Self::CalledAlone { trump, .. }
            | Self::DefendedAlone { trump, .. } => *trump,
        }
    }

    pub fn caller(&self) -> Position {
        match self {
            Self::Called { caller, .. }
            | Self::CalledAlone { caller, .. }
            | Self::DefendedAlone { caller, .. } => *caller,
        }
    }

    pub(crate) fn is_equivalent(&self, value: &Self) -> bool {
        if self == value {
            return true;
        }

        match self {
            Self::CalledAlone {
                trump: trump1,
                caller: caller1,
            }
            | Self::DefendedAlone {
                trump: trump1,
                caller: caller1,
                ..
            } => match value {
                Self::CalledAlone {
                    trump: trump2,
                    caller: caller2,
                }
                | Self::DefendedAlone {
                    trump: trump2,
                    caller: caller2,
                    ..
                } => trump1 == trump2 && caller1 == caller2,
                _ => false,
            },
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    impl BidResultAll {
        pub fn called(trump: &str) -> BidResultAll {
            BidResultAll::Called {
                trump: Suit::from_str(trump).unwrap(),
                caller: Position::South,
            }
        }

        pub fn alone(trump: &str) -> BidResultAll {
            BidResultAll::CalledAlone {
                trump: Suit::from_str(trump).unwrap(),
                caller: Position::South,
            }
        }
    }
}
