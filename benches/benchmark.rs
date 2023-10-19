use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cuv::{unpack, pack, create_lut};

fn create_lut_benchmark(c: &mut Criterion) {
    c.bench_function("create_lut()", |b| {
        b.iter(|| {
            let res = create_lut();
            assert_eq!(res.len(), 0x2000);
        })
    });
}

fn pack_benchmark(c: &mut Criterion) {
    c.bench_function("pack(1,0,0)", |b| {
        b.iter(|| {
            let res = pack(black_box(1.0), black_box(0.0), black_box(0.0));
            assert_eq!(res, 255);
        })
    });
}

fn unpack_benchmark(c: &mut Criterion) {
    let lut = create_lut();
    c.bench_function("unpack(255)", |b| {
        b.iter(|| {
            let res = unpack(black_box(255), &lut);
            assert_eq!(res, [1.0, 0.0, 0.0]);
        })
    });
}

criterion_group!(benches, create_lut_benchmark, pack_benchmark, unpack_benchmark);
criterion_main!(benches);
