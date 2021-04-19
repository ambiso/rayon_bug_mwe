# Rayon Bug MWE

This repo demonstrates a potential bug in the rayon rust library.

When running a long running computation sometimes less than the number of cores are being used to perform the computation.

Most easily reproducible on a system with many cores (>= 96).

But sometimes (~30%) also works on a system with 8 cores:

```
$ cargo run --release
    Finished release [optimized] target(s) in 0.05s
     Running `target/release/rayon_bug`
Running: 1
Running: 2
Running: 3
Running: 4
Running: 5
Running: 6
Running: 7
```

Only 7 cores are being "used" to perform the long running computation