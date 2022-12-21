pub struct HandsIterator {
    state: Option<[CardLocation; 18]>,
}

impl HandsIterator {
    pub fn create() -> HandsIterator {
        HandsIterator {
            state: Some([
                CardLocation::WestHand,
                CardLocation::WestHand,
                CardLocation::WestHand,
                CardLocation::WestHand,
                CardLocation::WestHand,
                CardLocation::NorthHand,
                CardLocation::NorthHand,
                CardLocation::NorthHand,
                CardLocation::NorthHand,
                CardLocation::NorthHand,
                CardLocation::EastHand,
                CardLocation::EastHand,
                CardLocation::EastHand,
                CardLocation::EastHand,
                CardLocation::EastHand,
                CardLocation::Kitty,
                CardLocation::Kitty,
                CardLocation::Kitty,
            ]),
        }
    }
}

impl Iterator for HandsIterator {
    type Item = [CardLocation; 18];

    fn next(&mut self) -> Option<Self::Item> {
        if let None = self.state {
            return None;
        }

        let mut state = self.state.unwrap();
        let result = state.clone();

        let mut i = state.len() - 1;
        let mut j = i;
        while i > 0 && state[i - 1] >= state[i] {
            i -= 1;
        }
        if i <= 0 {
            self.state = None;
            return Some(result);
        }

        while state[j] <= state[j - 1] {
            j -= 1;
        }

        state.swap(i - 1, j);

        j = state.len() - 1;
        while i < j {
            state.swap(i, j);
            i += 1;
            j -= 1;
        }

        Some(result)
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CardLocation {
    WestHand,
    NorthHand,
    EastHand,
    Kitty,
}
