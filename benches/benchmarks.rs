use criterion::{Criterion, criterion_group, criterion_main};
use jyotish::Planet;

fn bench_planet_display(c: &mut Criterion) {
    c.bench_function("planet_display", |b| {
        b.iter(|| Planet::Jupiter.to_string())
    });
}

fn bench_planet_serde(c: &mut Criterion) {
    let planet = Planet::Mars;
    c.bench_function("planet_serialize", |b| {
        b.iter(|| serde_json::to_string(&planet).unwrap())
    });
    let json = serde_json::to_string(&planet).unwrap();
    c.bench_function("planet_deserialize", |b| {
        b.iter(|| serde_json::from_str::<Planet>(&json).unwrap())
    });
}

criterion_group!(benches, bench_planet_display, bench_planet_serde,);
criterion_main!(benches);
