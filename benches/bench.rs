use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::path::PathBuf;
use std::process::Command;

fn bench_bed_count(c: &mut Criterion) {
    let bin = env!("CARGO_BIN_EXE_rsomics-bed-count");
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // bed-count reads stdin or a positional file; use a small BED from bed-complement golden
    let bed = manifest.parent().unwrap().join("rsomics-bed-complement/tests/golden/input.bed");
    c.bench_function("rsomics-bed-count golden", |b| {
        b.iter(|| {
            let out = Command::new(black_box(bin))
                .arg(bed.to_str().unwrap())
                .output()
                .unwrap();
            assert!(out.status.success());
        });
    });
}

criterion_group!(benches, bench_bed_count);
criterion_main!(benches);
