use std::{iter::repeat, sync::Mutex, thread::sleep, time::Duration};

use rayon::{current_num_threads, iter::{ParallelBridge, ParallelIterator}};

fn main() {
    let t = current_num_threads();
    let n = t;
    println!("Iterating over {} items", n);
    println!("Expecting to run {} threads", t);

    let iter = repeat(()).take(n).par_bridge();

    let concurrent = Mutex::new(0);

    let r: u32 = iter.map(|_| {
        {
            let mut c = concurrent.lock().unwrap();
            *c += 1;
            println!("Running: {}", c);
        }
        // Some very long running computation
        sleep(Duration::from_secs(10));
        1
    }).sum();
    println!("Result: {}", r);
}
