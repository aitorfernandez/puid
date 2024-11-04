use criterion::{black_box, criterion_group, criterion_main, Criterion};
use puid::Puid;

fn bench_puid_creation(c: &mut Criterion) {
    c.bench_function("create puid", |b| {
        b.iter(|| {
            let id = Puid::builder().prefix("test").unwrap().build().unwrap();
            black_box(id);
        });
    });
}

criterion_group!(benches, bench_puid_creation);
criterion_main!(benches);
