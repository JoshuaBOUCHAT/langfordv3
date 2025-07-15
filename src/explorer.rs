use std::{collections::HashMap, thread};

use crate::langford::{LangfordSate, N};

pub struct Explorer {
    states: HashMap<LangfordSate, u64>,
}
impl Explorer {
    pub fn init_from_start() -> Self {
        Self {
            states: HashMap::from([(LangfordSate::default(), 1)]),
        }
    }

    pub fn init_from_end() -> Self {
        Self {
            states: HashMap::from([(LangfordSate::full_pattern(), 1)]),
        }
    }
    pub fn explore_down(self) -> Self {
        let new_patterns = self
            .states
            .into_iter()
            .flat_map(|(p, nb)| (p.get_child_iter().map(move |p| (p.into_unique(), nb))));
        let mut new_map = HashMap::new();
        for (pattern, nb) in new_patterns {
            if let Some(total_nb) = new_map.get_mut(&pattern) {
                *total_nb += nb;
            } else {
                new_map.insert(pattern, nb);
            }
        }

        Explorer { states: new_map }
    }
    pub fn explore_up(self) -> Self {
        let new_patterns = self
            .states
            .into_iter()
            .flat_map(|(p, nb)| (p.get_parent_iter().map(move |p| (p.into_unique(), nb))));
        let mut new_map = HashMap::new();
        for (pattern, nb) in new_patterns {
            if let Some(total_nb) = new_map.get_mut(&pattern) {
                *total_nb += nb;
            } else {
                new_map.insert(pattern, nb);
            }
        }

        Explorer { states: new_map }
    }
}

pub fn compute_langford() -> u64 {
    let mut start_explorer = Explorer::init_from_start();
    let mut end_explorer = Explorer::init_from_end();

    let end_iter_count = N / 2;
    let start_iter_count = N - end_iter_count;

    let (start_result, end_result) = thread::scope(|s| {
        let end_handle = s.spawn(move || {
            for _ in 0..end_iter_count {
                end_explorer = end_explorer.explore_up();
            }
            println!("end finished !");
            end_explorer
        });

        let start_handle = s.spawn(move || {
            for _ in 0..start_iter_count {
                start_explorer = start_explorer.explore_down();
            }
            start_explorer
        });

        (
            start_handle.join().expect("start thread panicked"),
            end_handle.join().expect("end thread panicked"),
        )
    });

    let mut total_count = 0;
    for (state, nb1) in start_result.states {
        if let Some(nb2) = end_result.states.get(&state) {
            total_count += nb1 * nb2;
        }
    }

    total_count
}
