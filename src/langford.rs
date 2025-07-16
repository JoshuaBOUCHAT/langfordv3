use core::fmt;

use crate::{N, SIZE};

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Default)]
pub struct LangfordState {
    pub state: u64,
}

impl std::ops::Add<u64> for LangfordState {
    type Output = Option<Self>;
    fn add(self, rhs: u64) -> Self::Output {
        if self.state & rhs == 0 {
            Some(LangfordState {
                state: self.state | rhs,
            })
        } else {
            None
        }
    }
}

impl std::ops::Sub<u64> for LangfordState {
    type Output = Option<Self>;
    fn sub(self, mask: u64) -> Self::Output {
        if self.state & mask == mask {
            Some(LangfordState {
                //as mask is compose of only two bit set two one and the if garentie that all bit match then doing self.state & (~mask)  is equivalent
                state: self.state ^ mask,
            })
        } else {
            None
        }
    }
}

impl fmt::Display for LangfordState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Affiche de gauche à droite, bit le plus significatif en premier
        for i in (0..SIZE).rev() {
            let bit = (self.state >> i) & 1;
            write!(f, "{}", bit)?;
        }
        Ok(())
    }
}

const fn get_array() -> [u64; 20] {
    let mut res = [0; 20];
    let mut i = 0;
    while i < 20 {
        res[i] = (1 << (i + 1)) | 1;
        i += 1;
    }
    res
}

const MASK: [u64; 20] = get_array();

impl LangfordState {
    pub fn get_child_iter(self) -> impl Iterator<Item = Self> {
        let x = (self.state.count_ones() / 2) as usize;
        let space = N - x;
        let mask = MASK[space];

        (0..SIZE - space - 1).filter_map(move |i| self + (mask << i))
    }
    pub fn get_parent_iter(self) -> impl Iterator<Item = Self> {
        let x = (self.state.count_ones() / 2) as usize - 1; //we need to take the X of the parrent
        let space = N - x;
        let mask = MASK[space];
        (0..SIZE - space - 1).filter_map(move |i| self - (mask << i))
    }

    pub fn into_unique(self) -> Self {
        let reversed = self.state.reverse_bits() >> (64 - SIZE);
        if reversed > self.state {
            return Self { state: reversed };
        }
        self
    }
    pub fn full_pattern() -> Self {
        Self {
            state: (1 << SIZE) - 1,
        }
    }
}
