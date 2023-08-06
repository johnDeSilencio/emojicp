extern crate rand;

use std::time::Duration;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use emojicp::constants::RAW_PAIRS;
use emojicp::search::search_exact;
use rand::{seq::SliceRandom, thread_rng};

// search_exact_benchmark creates a vector of all emojis
// names, shuffles those names, iterates through that list,
// and searches for the corresponding emoji. This benchmark
// takes ~2 minutes to run locally
fn search_exact_benchmark(c: &mut Criterion) {
    let mut emoji_names: Vec<&str> = RAW_PAIRS.iter().map(|pair| pair.0).collect();
    emoji_names.shuffle(&mut thread_rng());

    let mut group = c.benchmark_group("Search for all emojis");
    group.measurement_time(Duration::from_secs(120));
    group.bench_function("search_exact", |b| {
        b.iter(|| {
            for name in emoji_names.iter() {
                let _ = black_box(search_exact(String::from(*name)));
            }
        })
    });
}

criterion_group!(benches, search_exact_benchmark);
criterion_main!(benches);
