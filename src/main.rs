mod explorer;
mod langford;
use std::time::Instant;

use crate::{explorer::compute_langford, langford::N};

fn main() {
    let now = Instant::now();
    println!(
        "Langford(2,{N}) = {}, computed in {}",
        compute_langford() / 2 / 2,
        now.elapsed().as_secs_f32()
    );
}
