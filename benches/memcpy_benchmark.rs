#![feature(test)]
extern crate test;

#[cfg(test)]
mod benchmarks {
    use test::black_box;
    use test::Bencher;

    // Check perf improvement of enabling clippy::manual_memcpy lint
    // https://rust-lang.github.io/rust-clippy/master/index.html#/manual_memcpy?groups=perf
    //
    // Manual memcpy implementation
    fn manual_memcpy(src: &[u8], dst: &mut [u8]) {
        for i in 0..src.len() {
            dst[i + 64] = src[i];
        }
    }

    // Optimized memcpy using `clone_from_slice`
    fn optimized_memcpy(src: &[u8], dst: &mut [u8]) {
        dst[64..(src.len() + 64)].clone_from_slice(&src[..]);
    }

    macro_rules! define_benchmark {
        ($manual_fn:ident, $optimized_fn:ident, $size:expr) => {
            #[bench]
            fn $manual_fn(b: &mut Bencher) {
                let src = vec![1u8; $size];
                let mut dst = vec![0u8; $size + 64];
                b.iter(|| {
                    manual_memcpy(black_box(&src), black_box(&mut dst));
                });
            }

            #[bench]
            fn $optimized_fn(b: &mut Bencher) {
                let src = vec![1u8; $size];
                let mut dst = vec![0u8; $size + 64];
                b.iter(|| {
                    optimized_memcpy(black_box(&src), black_box(&mut dst));
                });
            }
        };
    }
    define_benchmark!(bench_manual_memcpy_1000, bench_optimized_memcpy_1000, 1000);
    define_benchmark!(
        bench_manual_memcpy_10000,
        bench_optimized_memcpy_10000,
        10_000
    );
    define_benchmark!(
        bench_manual_memcpy_100000,
        bench_optimized_memcpy_100000,
        100_000
    );
    define_benchmark!(
        bench_manual_memcpy_1000000,
        bench_optimized_memcpy_1000000,
        1_000_000
    );
}
