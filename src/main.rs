use std::{iter::repeat, sync::Mutex, thread::sleep, time::Duration};

use rayon::{current_num_threads, iter::{ParallelBridge, ParallelIterator}};

fn main() {
    let n = current_num_threads()*3;
    let iter = repeat(()).take(n).par_bridge();
    let concurrent = Mutex::new(0);
    let _: u32 = iter.map(|_| {
        {
            let mut c = concurrent.lock().unwrap();
            *c += 1;
            println!("Running: {}", c);
        }
        // Some very long running computation
        sleep(Duration::from_secs(3600));
        1
    }).sum();
}
