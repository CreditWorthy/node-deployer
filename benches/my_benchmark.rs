#![allow(unused)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use simple_nav::engine::Engine;
use simple_nav::graph::LatLon;

pub fn criterion_benchmark(c: &mut Criterion) {
    let engine = Engine::build("./data/delaware-latest.osm.pbf").unwrap();
    let orig = LatLon {
        lon: -75.057298,
        lat: 38.537473,
    };

    let dest = LatLon {
        lon: -75.124117,
        lat: 38.731088,
    };

    c.bench_function("fib 20", |b| {
        b.iter(|| {
            let result = engine.routing(black_box(orig), black_box(dest));
            result.unwrap();
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
