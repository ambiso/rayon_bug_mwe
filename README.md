# Rayon Bug MWE

This repo demonstrates a potential bug in the rayon rust library.

When running a long-running computation concurrently sometimes less than the number of cores are being used to perform the computation.

Most easily reproducible with many threads:

```
$ RAYON_NUM_THREADS=96 cargo run --release
    Finished release [optimized] target(s) in 0.03s
     Running `target/release/rayon_bug`
Iterating over 96 items
Expecting to run 96 threads
Running: 5
Running: 4
Running: 1
Running: 7
Running: 8
Running: 9
Running: 6
Running: 3
Running: 2
```

Only 9 cores are being "used" to perform the long running computation

The `reproduce.sh` script runs the rust application and checks after a second whether all threads have output a message.

It counts how many times it was successful in reproducing the issue:

```
Reproduced 73 / 100 times
```

The issue does not occur anymore when we increase the item count to `t*t*2`.

Also it doesn't seem to be a deadlock, as the computations terminates eventually.