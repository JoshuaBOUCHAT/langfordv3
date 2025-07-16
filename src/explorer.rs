use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::langford::LangfordSate;
use std::collections::BTreeMap;

pub struct Explorer {
    pub states: BTreeMap<LangfordSate, u64>,
}

impl Explorer {
    pub fn init_from_start() -> Self {
        Self {
            states: BTreeMap::from([(LangfordSate::default(), 1)]),
        }
    }

    pub fn init_from_end() -> Self {
        Self {
            states: BTreeMap::from([(LangfordSate::full_pattern(), 1)]),
        }
    }

    pub fn explore_down(self) -> Self {
        let new_patterns = self
            .states
            .into_iter()
            .flat_map(|(p, nb)| p.get_child_iter().map(move |p| (p.into_unique(), nb)));

        let mut new_map = BTreeMap::new();
        for (pattern, nb) in new_patterns {
            *new_map.entry(pattern).or_insert(0) += nb;
        }

        Explorer { states: new_map }
    }

    pub fn explore_up(self) -> Self {
        let new_patterns = self
            .states
            .into_iter()
            .flat_map(|(p, nb)| p.get_parent_iter().map(move |p| (p.into_unique(), nb)));

        let mut new_map = BTreeMap::new();
        for (pattern, nb) in new_patterns {
            *new_map.entry(pattern).or_insert(0) += nb;
        }

        Explorer { states: new_map }
    }
}
