
#![allow(unused)]

use simple_nav::graph::LatLon;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simple_nav::engine::Engine;

pub fn criterion_benchmark(c: &mut Criterion) {
    let engine = Engine::build("./data/delaware-latest.osm.pbf").unwrap();
    let orig = LatLon{
        lat : -75.057298,
        lon : 38.537473,
    };

    let dest = LatLon{
        lat : -75.124117,
        lon : 38.731088,
    };

    c.bench_function("fib 20", |b| b.iter(|| {let result = engine.routing(black_box(orig), black_box(dest)); result.unwrap(); }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
