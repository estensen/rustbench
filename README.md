# rustbench
Benchmarks for Rust

## Run
```
cargo +nightly bench
...
running 8 tests
test benchmarks::bench_manual_memcpy_1000       ... bench:          23.15 ns/iter (+/- 0.66)
test benchmarks::bench_manual_memcpy_10000      ... bench:         213.52 ns/iter (+/- 2.42)
test benchmarks::bench_manual_memcpy_100000     ... bench:       1,975.71 ns/iter (+/- 42.26)
test benchmarks::bench_manual_memcpy_1000000    ... bench:      21,778.84 ns/iter (+/- 5,661.98)
test benchmarks::bench_optimized_memcpy_1000    ... bench:          15.67 ns/iter (+/- 0.16)
test benchmarks::bench_optimized_memcpy_10000   ... bench:         126.82 ns/iter (+/- 3.84)
test benchmarks::bench_optimized_memcpy_100000  ... bench:       3,048.96 ns/iter (+/- 1,575.80)
test benchmarks::bench_optimized_memcpy_1000000 ... bench:      16,770.28 ns/iter (+/- 394.76)
```
