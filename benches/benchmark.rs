use criterion::{black_box, criterion_group, criterion_main, Criterion};
use cuv::{create_lut, pack, unpack, CompUnitVec};

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

fn rustified_pack_benchmark(c: &mut Criterion) {
    c.bench_function("rustified pack(1,0,0)", |b| {
        b.iter(|| {
            let vec = CompUnitVec::new(black_box(1.0), black_box(0.0), black_box(0.0));
            assert_eq!(vec.get_u16(), 255);
        })
    });
}

fn rustified_unpack_benchmark(c: &mut Criterion) {
    c.bench_function("rustified unpack(255)", |b| {
        b.iter(|| {
            let vec = CompUnitVec::from_u16(black_box(255));
            assert_eq!(vec.get(), [1.0, 0.0, 0.0]);
        })
    });
}

criterion_group!(
    benches,
    create_lut_benchmark,
    pack_benchmark,
    unpack_benchmark,
    rustified_unpack_benchmark,
    rustified_pack_benchmark,
);
criterion_main!(benches);
