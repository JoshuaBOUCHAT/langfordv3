mod explorer;
mod langford;
mod states;
use std::{collections::HashMap, panic, time::Instant};

use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub const N: usize = 19;
pub const SIZE: usize = 2 * N;
pub const PATTERN_NB_BYTE: usize = (SIZE + 7) / 8;

use crate::explorer::Explorer;

fn main() {
    let now = Instant::now();
    println!(
        "Langford(2,{N}) = {}, computed in {}",
        compute_langford() / 2 / 2,
        now.elapsed().as_secs_f32()
    );
}
use std::thread;

pub fn compute_langford() -> u64 {
    let start_explorer = Explorer::init_from_start();
    let end_explorer = Explorer::init_from_end();

    let end_iter_count = 7;
    let start_iter_count = 11;

    // Spawn two threads to run the loops concurrently
    /*let end_handle = thread::spawn(move || {
        let mut explorer = end_explorer;
        for _ in 0..end_iter_count {
            explorer = explorer.explore_up();
            println!("end size before compact: {}", explorer.len());
            let now = Instant::now();
            explorer.compact_duplicates();
            println!(
                "end size after compact: {}, time to compact {}",
                explorer.len(),
                now.elapsed().as_secs_f32()
            );
        }
        println!("end finished size: {}", explorer.states.len());
        explorer
    });*/

    let start_handle = thread::spawn(move || {
        let mut explorer = start_explorer;
        for _ in 0..start_iter_count {
            explorer = explorer.explore_down();
            println!("start size  before compact: {}", explorer.len());
            let now = Instant::now();
            explorer.compact_duplicates();
            println!(
                "start size after compact: {}, time to compact {}",
                explorer.len(),
                now.elapsed().as_secs_f32()
            );
        }
        println!("start finished size: {}", explorer.len());
        explorer
    });
    let mut sum = start_handle.join().unwrap().len();
    /*let (start, end) = (start_handle.join().unwrap(), end_handle.join().unwrap());
    let mut map = HashMap::with_capacity(start.states.len());
    for (pattern, nb) in start.states.into_iter() {
        map.insert(pattern, nb);
    }

    for (pattern, nb1) in end.states.into_iter() {
        if let Some(&nb2) = map.get(&pattern) {
            sum += (nb1 * nb2) as u64;
        }
    }*/
    sum as u64
}
