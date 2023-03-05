mod version1;
mod version2;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

pub fn compare_performances(c: &mut Criterion) {
    let mut group = c.benchmark_group("conversions");
    for i in [0, 1000].iter() {
        let arg = i.to_string();
        group.bench_with_input(
            BenchmarkId::new("version1_old", arg.clone()),
            &arg,
            |b, i| b.iter(|| version1::num_word_conv(i)),
        );
        group.bench_with_input(
            BenchmarkId::new("version2_new", arg.clone()),
            &arg,
            |b, i| b.iter(|| version2::num_word_conv(i)),
        );
    }
    group.finish();
}

criterion_group!(benches, compare_performances);
criterion_main!(benches);
