use crate::{langford::LangfordState, states::States};

pub struct Explorer {
    pub states: States,
}

impl Explorer {
    pub fn init_from_start() -> Self {
        Self {
            states: States::from_first((LangfordState::default(), 1)),
        }
    }

    pub fn init_from_end() -> Self {
        Self {
            states: States::from_first((LangfordState::full_pattern(), 1)),
        }
    }

    pub fn explore_down(self) -> Self {
        let len = self.states.len();

        let new_patterns = self
            .states
            .into_iter()
            .flat_map(|(p, nb)| p.get_child_iter().map(move |p| (p.into_unique(), nb)));

        let mut new_states = States::with_capacity(len);
        //let mut i = 1;

        for new_pattern in new_patterns {
            new_states.push(new_pattern);
            /*if new_states.len() & 1048575 == 0 && new_states.len() >> 20 == i {
                i *= 2;
                new_states.compact_duplicates();
                println!("Compacting down!");
            }*/
        }

        Explorer { states: new_states }
    }

    pub fn explore_up(self) -> Self {
        let len = self.states.len();

        let new_patterns = self
            .states
            .into_iter()
            .flat_map(|(p, nb)| p.get_parent_iter().map(move |p| (p.into_unique(), nb)));

        let mut new_states = States::with_capacity(len);
        //let mut i = 1;

        for new_pattern in new_patterns {
            new_states.push(new_pattern);
            /*if new_states.len() & 1048575 == 0 && new_states.len() >> 20 == i {
                i *= 2;
                new_states.compact_duplicates();
                println!("Compacting up!");
            }*/
        }

        Explorer { states: new_states }
    }
    pub fn len(&self) -> usize {
        self.states.len()
    }
    pub fn compact_duplicates(&mut self) {
        self.states.compact_duplicates();
    }
}
