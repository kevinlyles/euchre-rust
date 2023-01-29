pub(crate) struct HandsIterator {
    state: Option<[CardLocation; 18]>,
}

impl HandsIterator {
    pub(crate) fn create() -> HandsIterator {
        HandsIterator {
            state: Some([
                CardLocation::West,
                CardLocation::West,
                CardLocation::West,
                CardLocation::West,
                CardLocation::West,
                CardLocation::North,
                CardLocation::North,
                CardLocation::North,
                CardLocation::North,
                CardLocation::North,
                CardLocation::East,
                CardLocation::East,
                CardLocation::East,
                CardLocation::East,
                CardLocation::East,
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
        let mut state = self.state?;
        let result = state;

        let mut i = state.len() - 1;
        let mut j = i;
        while i > 0 && state[i - 1] >= state[i] {
            i -= 1;
        }
        if i == 0 {
            self.state = None;
            return Some(result);
        }

        while state[j] <= state[i - 1] {
            j -= 1;
        }

        state.swap(i - 1, j);

        j = state.len() - 1;
        while i < j {
            state.swap(i, j);
            i += 1;
            j -= 1;
        }

        self.state = Some(state);

        Some(result)
    }
}

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub(crate) enum CardLocation {
    West,
    North,
    East,
    Kitty,
}
