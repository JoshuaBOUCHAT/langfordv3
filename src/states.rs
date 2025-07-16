use std::time::Instant;

use crate::{langford::LangfordState, PATTERN_NB_BYTE};
use co_sort::co_sort;
use co_sort::Permutation;

pub struct States {
    patterns: Vec<LangfordState>,
    nbs: Vec<u16>,
}
impl States {
    pub fn push(&mut self, item: (LangfordState, u16)) {
        self.patterns.push(item.0);
        self.nbs.push(item.1);
    }
    pub fn compact_duplicates(&mut self) {
        self.sort_in_place();
        compact_duplicates(&mut self.patterns, &mut self.nbs);
    }
    fn sort_in_place(&mut self) {
        println!("start sorting");
        let now = Instant::now();
        //co_sort!(patterns, nbs);
        radix_sort_dual(self.patterns.as_mut_slice(), self.nbs.as_mut_slice());
        println!("finish sorting in {}s", now.elapsed().as_secs_f32());
    }
    pub fn from_first(item: (LangfordState, u16)) -> Self {
        Self {
            patterns: vec![item.0],
            nbs: vec![item.1],
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            patterns: Vec::with_capacity(capacity),
            nbs: Vec::with_capacity(capacity),
        }
    }
    pub fn len(&self) -> usize {
        assert_eq!(self.patterns.len(), self.nbs.len());
        self.patterns.len()
    }
}

impl IntoIterator for States {
    type IntoIter = StatesIterator;
    type Item = (LangfordState, u16);
    fn into_iter(self) -> Self::IntoIter {
        StatesIterator {
            index: 0,
            states: self,
        }
    }
}

pub struct StatesIterator {
    states: States,
    index: usize,
}
impl Iterator for StatesIterator {
    type Item = (LangfordState, u16);
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.states.nbs.len() {
            let ret = Some((
                self.states.patterns[self.index],
                self.states.nbs[self.index],
            ));
            self.index += 1;
            ret
        } else {
            None
        }
    }
}

fn compact_duplicates(patterns: &mut Vec<LangfordState>, nbs: &mut Vec<u16>) {
    assert_eq!(patterns.len(), nbs.len());

    let mut write_idx = 0;
    let mut i = 0;

    while i < patterns.len() {
        let current = patterns[i];
        let mut sum = nbs[i];
        i += 1;

        while i < patterns.len() && patterns[i] == current {
            sum += nbs[i];
            i += 1;
        }

        patterns[write_idx] = current;
        nbs[write_idx] = sum;
        write_idx += 1;
    }

    patterns.truncate(write_idx);
    nbs.truncate(write_idx);
}
pub fn radix_sort_dual(keys: &mut [LangfordState], values: &mut [u16]) {
    assert_eq!(keys.len(), values.len());
    if keys.len() <= 1 {
        return;
    }

    const NB_BUCKETS: usize = 256;
    const N_ROUNDS: usize = 8; // 8 rounds * 8 bits = 64 bits
    const BUCKET_SIZE_HINT: usize = 16;

    let n = keys.len();

    // Buckets pour keys et values
    let mut buckets_keys: Vec<Vec<LangfordState>> = Vec::with_capacity(NB_BUCKETS);
    let mut buckets_values: Vec<Vec<u16>> = Vec::with_capacity(NB_BUCKETS);
    for _ in 0..NB_BUCKETS {
        buckets_keys.push(Vec::with_capacity(BUCKET_SIZE_HINT));
        buckets_values.push(Vec::with_capacity(BUCKET_SIZE_HINT));
    }

    // Première passe : remplir les buckets à partir de keys et values
    for i in 0..n {
        let byte = ((keys[i].state >> (8 * (N_ROUNDS - 1))) & 0xFF) as usize;
        buckets_keys[byte].push(keys[i].clone());
        buckets_values[byte].push(values[i]);
    }

    // Pour les autres rounds, on alterne entre deux ensembles de buckets
    for round in (0..N_ROUNDS - 1).rev() {
        // Nouvelle structure de buckets (vider les anciennes)
        let mut new_buckets_keys: Vec<Vec<LangfordState>> = Vec::with_capacity(NB_BUCKETS);
        let mut new_buckets_values: Vec<Vec<u16>> = Vec::with_capacity(NB_BUCKETS);
        for _ in 0..NB_BUCKETS {
            new_buckets_keys.push(Vec::with_capacity(BUCKET_SIZE_HINT));
            new_buckets_values.push(Vec::with_capacity(BUCKET_SIZE_HINT));
        }

        // Parcourir tous les buckets actuels et redistribuer en fonction du byte suivant
        for bucket_i in 0..NB_BUCKETS {
            for i in 0..buckets_keys[bucket_i].len() {
                let byte = ((buckets_keys[bucket_i][i].state >> (8 * round)) & 0xFF) as usize;
                new_buckets_keys[byte].push(buckets_keys[bucket_i][i].clone());
                new_buckets_values[byte].push(buckets_values[bucket_i][i]);
            }
        }

        buckets_keys = new_buckets_keys;
        buckets_values = new_buckets_values;
    }

    // Recomposer les slices triées
    let mut pos = 0;
    for bucket_i in 0..NB_BUCKETS {
        let len_bucket = buckets_keys[bucket_i].len();
        keys[pos..pos + len_bucket].clone_from_slice(&buckets_keys[bucket_i]);
        values[pos..pos + len_bucket].clone_from_slice(&buckets_values[bucket_i]);
        pos += len_bucket;
    }

    assert_eq!(pos, n);
}
