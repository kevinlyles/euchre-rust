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
}
