use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use emojicp::constants::RAW_PAIRS;
use emojicp::search::search_exact;

fn search_exact_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_exact");
    for pair in RAW_PAIRS.iter() {
        let description = String::from(pair.0);
        //group.throughput(criterion::Throughput::Elements(description));
        group.bench_with_input(
            BenchmarkId::from_parameter(description),
            pair,
            |b, &description| {
                b.iter(|| search_exact(String::from(description.0)));
            },
        );
    }
    group.finish();
}

criterion_group!(benches, search_exact_benchmark);
criterion_main!(benches);
