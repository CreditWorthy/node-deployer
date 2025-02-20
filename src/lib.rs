use engine::Engine;
use graph::LatLon;

pub mod engine;
pub mod graph;
pub mod parser;
pub mod server;

#[test]
fn benchtest() {
    let engine = Engine::build("./data/delaware-latest.osm.pbf").unwrap();
    let orig = LatLon {
        lat: -75.057298,
        lon: 38.537473,
    };

    let dest = LatLon {
        lat: -75.124117,
        lon: 38.731088,
    };

    let start_time = std::time::Instant::now();
    for i in 0..100000 {
        let result = engine.routing(orig, dest);
        result.unwrap();
    }
    println!("==== time cost: {:?}", start_time.elapsed());

    // 5 s / 100_000 = 5 ms / 100 = 50 micro seconds, ns is nanoseconds
}
