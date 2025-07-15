use crate::langford::{LangfordSate, N};
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

pub fn compute_langford() -> u64 {
    let mut start_explorer = Explorer::init_from_start();
    let mut end_explorer = Explorer::init_from_end();

    let end_iter_count = 7;
    let start_iter_count = 9;

    for _ in 0..end_iter_count {
        end_explorer = end_explorer.explore_up();
    }
    println!("end finished size: {}", end_explorer.states.len());

    for _ in 0..start_iter_count {
        start_explorer = start_explorer.explore_down();
    }
    println!("start finished size: {}", start_explorer.states.len());

    let (start_result, end_result) = (start_explorer, end_explorer);

    let mut total_count = 0;
    for (state, nb1) in start_result.states {
        if let Some(nb2) = end_result.states.get(&state) {
            total_count += nb1 * nb2;
        }
    }

    total_count
}
