use criterion::{Criterion, Throughput, criterion_group, criterion_main};
use rsomics_bed_count::count;
use std::io::Cursor;

fn make_fixture(n: usize) -> String {
    (0..n)
        .map(|i| format!("chr1\t{}\t{}\n", i * 100, i * 100 + 100))
        .collect()
}

fn bench_count(c: &mut Criterion) {
    let fixture = make_fixture(100_000);
    let mut group = c.benchmark_group("bed-count");
    group.throughput(Throughput::Elements(100_000));
    group.bench_function("count_100k", |b| {
        b.iter(|| count(Cursor::new(fixture.as_str())).unwrap());
    });
    group.finish();
}

criterion_group!(benches, bench_count);
criterion_main!(benches);
