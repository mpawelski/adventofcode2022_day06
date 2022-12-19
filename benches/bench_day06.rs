// To run benchmark quick:
// cargo bench --bench bench_day06 -- --quick --quiet

use criterion::{criterion_group, criterion_main, Criterion};

use day06::*;

static INPUT: &str = include_str!("../input");

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("part1_input");

    group.bench_function("v1", |b| b.iter(|| run_part1_v1(INPUT).unwrap()));
    group.finish();

    let mut group = c.benchmark_group("part2_input");
    group
        .bench_function("v1", |b| b.iter(|| run_part2_v1_hash_set(INPUT).unwrap()))
        .bench_function("v2", |b| {
            b.iter(|| run_part2_v2_hash_set_reused(INPUT).unwrap())
        })
        .bench_function("v3", |b| b.iter(|| run_part2_v3_no_hashset(INPUT).unwrap()))
        .bench_function("v4", |b| {
            b.iter(|| run_part2_v4_no_hashset_no_bound_check(INPUT).unwrap())
        })
        .bench_function("v5", |b| {
            b.iter(|| run_part2_v5_array_counters(INPUT).unwrap())
        })
        .bench_function("v6", |b| b.iter(|| run_part2_v6_bit_tricks(INPUT).unwrap()))
        .bench_function("v7", |b| {
            b.iter(|| run_part2_v7_bit_tricks_xor(INPUT).unwrap())
        });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

/*
# Command to do benchmarks (pwsh):

# Default target cpu:
cargo bench --bench bench_day06 -- --save-baseline default_target_cpu

# Native target cpu (my is "znver3", based on "rustc --print target-cpus")
$env:RUSTFLAGS='-C target-cpu=native'; cargo bench --bench bench_day06 -- --save-baseline native_target_cpu;  remove-item env:\RUSTFLAGS

# Comparing result with critcmp

critcmp default_target_cpu native_target_cpu

# group             default_target_cpu                     native_target_cpu
# -----             ------------------                     -----------------
# part1_input/v1    1.04    154.0±9.79µs        ? ?/sec    1.00    148.0±2.46µs        ? ?/sec
# part2_input/v1    1.04    448.3±7.21µs        ? ?/sec    1.00    430.9±8.44µs        ? ?/sec
# part2_input/v2    1.12    327.6±7.43µs        ? ?/sec    1.00    293.2±5.83µs        ? ?/sec
# part2_input/v3    1.00     29.2±0.43µs        ? ?/sec    1.09     31.9±0.58µs        ? ?/sec
# part2_input/v4    1.00      7.2±0.14µs        ? ?/sec    3.28     23.5±0.48µs        ? ?/sec
# part2_input/v5    1.03      2.8±0.08µs        ? ?/sec    1.00      2.7±0.06µs        ? ?/sec
# part2_input/v6    1.84      7.9±0.14µs        ? ?/sec    1.00      4.3±0.06µs        ? ?/sec
# part2_input/v7    2.06      3.9±0.11µs        ? ?/sec    1.00  1901.2±49.83ns        ? ?/sec


# Compare benchmarks in "target-cpu=native"
critcmp native_target_cpu -g '^(part2)_' --list

# part2
# -----
# native_target_cpu/_input/v7     1.00   1901.2±49.83ns       ? ?/sec
# native_target_cpu/_input/v5     1.41       2.7±0.06µs       ? ?/sec
# native_target_cpu/_input/v6     2.26       4.3±0.06µs       ? ?/sec
# native_target_cpu/_input/v4    12.34      23.5±0.48µs       ? ?/sec
# native_target_cpu/_input/v3    16.79      31.9±0.58µs       ? ?/sec
# native_target_cpu/_input/v2   154.19     293.2±5.83µs       ? ?/sec
# native_target_cpu/_input/v1   226.63     430.9±8.44µs       ? ?/sec

# Compare benchmarks in "target-cpu=default"
critcmp default_target_cpu -g '^(part2)_' --list

# part2
# -----
# default_target_cpu/_input/v5     1.00       2.8±0.08µs       ? ?/sec
# default_target_cpu/_input/v7     1.42       3.9±0.11µs       ? ?/sec
# default_target_cpu/_input/v4     2.59       7.2±0.14µs       ? ?/sec
# default_target_cpu/_input/v6     2.86       7.9±0.14µs       ? ?/sec
# default_target_cpu/_input/v3    10.54      29.2±0.43µs       ? ?/sec
# default_target_cpu/_input/v2   118.26     327.6±7.43µs       ? ?/sec
# default_target_cpu/_input/v1   161.84     448.3±7.21µs       ? ?/sec

# Compare all benchmarks for part2
critcmp native_target_cpu default_target_cpu -g '^(part2)_' --list

# part2
# -----
# native_target_cpu/_input/v7      1.00   1901.2±49.83ns       ? ?/sec
# native_target_cpu/_input/v5      1.41       2.7±0.06µs       ? ?/sec
# default_target_cpu/_input/v5     1.46       2.8±0.08µs       ? ?/sec
# default_target_cpu/_input/v7     2.06       3.9±0.11µs       ? ?/sec
# native_target_cpu/_input/v6      2.26       4.3±0.06µs       ? ?/sec
# default_target_cpu/_input/v4     3.77       7.2±0.14µs       ? ?/sec
# default_target_cpu/_input/v6     4.16       7.9±0.14µs       ? ?/sec
# native_target_cpu/_input/v4     12.34      23.5±0.48µs       ? ?/sec
# default_target_cpu/_input/v3    15.36      29.2±0.43µs       ? ?/sec
# native_target_cpu/_input/v3     16.79      31.9±0.58µs       ? ?/sec
# native_target_cpu/_input/v2    154.19     293.2±5.83µs       ? ?/sec
# default_target_cpu/_input/v2   172.32     327.6±7.43µs       ? ?/sec
# native_target_cpu/_input/v1    226.63     430.9±8.44µs       ? ?/sec
# default_target_cpu/_input/v1   235.81     448.3±7.21µs       ? ?/sec

# Compare v3 and v4 ("array indexing" vs "iterating over slice")

critcmp default_target_cpu -g '^(part2)_input/v[34]' --list
# part2
# -----
# default_target_cpu/_input/v4     1.00       7.2±0.14µs       ? ?/sec
# default_target_cpu/_input/v3     4.08      29.2±0.43µs       ? ?/sec
#
critcmp native_target_cpu -g '^(part2)_input/v[34]' --list
# part2
# -----
# native_target_cpu/_input/v4     1.00      23.5±0.48µs       ? ?/sec
# native_target_cpu/_input/v3     1.36      31.9±0.58µs       ? ?/sec


Summary:
- v7 "XOR trick" is much faster but only when compiled for with "target-cpu=native" parameter.
- On "target-cpu=default" v5 is the fastests. Faster then any "bits tricks" versions
- critcmp shows ns instead of µs below some number, so look at "ratio" comparison.
- For some reasons v3 and v4 is slower with "target-cpu=native" that "target-cpu=default".
  v4 is much slower (3.3 times). Every other version is slightly faster (no bit tricks)
  or much faster (bit tricks with .count_ones())
- Better to change for loop with array indexing inside to iterating over slice and index (.enumerate() iterator method).
  Array indexing is 4 times slower.
*/
