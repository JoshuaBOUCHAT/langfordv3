mod explorer;
mod langford;
use std::time::Instant;

use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub const N: usize = 19;
pub const SIZE: usize = 2 * N;

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
    let mut start_explorer = Explorer::init_from_start();
    let mut end_explorer = Explorer::init_from_end();

    let end_iter_count = 8;
    let start_iter_count = 11;

    // Spawn two threads to run the loops concurrently
    let end_handle = thread::spawn(move || {
        let mut explorer = end_explorer;
        for _ in 0..end_iter_count {
            explorer = explorer.explore_up();
        }
        println!("end finished size: {}", explorer.states.len());
        explorer
    });

    let start_handle = thread::spawn(move || {
        let mut explorer = start_explorer;
        for _ in 0..start_iter_count {
            explorer = explorer.explore_down();
        }
        println!("start finished size: {}", explorer.states.len());
        explorer
    });

    // Wait for both threads to finish and get results
    let end_explorer = end_handle.join().unwrap();
    let start_explorer = start_handle.join().unwrap();

    let max = start_explorer
        .states
        .iter()
        .map(|(_, nb)| nb)
        .max()
        .unwrap();
    println!("start max len: {max}");

    let max = end_explorer.states.iter().map(|(_, nb)| nb).max().unwrap();
    println!("end max len: {max}");

    start_explorer
        .states
        .into_par_iter()
        .map(|(state, nb1)| {
            if let Some(nb2) = end_explorer.states.get(&state) {
                nb1 * nb2
            } else {
                0
            }
        })
        .sum()
}
