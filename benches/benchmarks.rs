use criterion::{Criterion, criterion_group, criterion_main};
use jyotish::{Planet, apparent, calendar, moon, nutation, planetary, sun};

fn bench_planet_display(c: &mut Criterion) {
    c.bench_function("planet_display", |b| b.iter(|| Planet::Jupiter.to_string()));
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

fn bench_calendar(c: &mut Criterion) {
    c.bench_function("gregorian_to_jdn", |b| {
        b.iter(|| calendar::gregorian_to_jdn(2024, 3, 25).unwrap())
    });
    c.bench_function("jdn_to_gregorian", |b| {
        b.iter(|| calendar::jdn_to_gregorian(2_460_394))
    });
    c.bench_function("gregorian_to_jd", |b| {
        b.iter(|| calendar::gregorian_to_jd(2024, 3, 25, 14, 30, 0.0).unwrap())
    });
    c.bench_function("gmst_degrees", |b| {
        b.iter(|| calendar::gmst_degrees(2_460_394.5))
    });
    c.bench_function("unix_to_jd", |b| {
        b.iter(|| calendar::unix_to_jd(1_711_324_800))
    });
}

fn bench_sun(c: &mut Criterion) {
    let jd = 2_451_545.0;
    c.bench_function("solar_longitude", |b| b.iter(|| sun::solar_longitude(jd)));
    c.bench_function("solar_position", |b| b.iter(|| sun::solar_position(jd)));
}

fn bench_moon(c: &mut Criterion) {
    let jd = 2_451_545.0;
    c.bench_function("lunar_longitude", |b| b.iter(|| moon::lunar_longitude(jd)));
    c.bench_function("lunar_position", |b| b.iter(|| moon::lunar_position(jd)));
}

fn bench_planetary(c: &mut Criterion) {
    let jd = 2_451_545.0;
    c.bench_function("mars_position", |b| {
        b.iter(|| planetary::compute_position(Planet::Mars, jd).unwrap())
    });
    c.bench_function("all_planet_positions", |b| {
        b.iter(|| planetary::compute_all_positions(jd).unwrap())
    });
}

fn bench_nutation(c: &mut Criterion) {
    let jd = 2_451_545.0;
    c.bench_function("nutation_components", |b| {
        b.iter(|| nutation::nutation_components(jd))
    });
    c.bench_function("true_obliquity", |b| {
        b.iter(|| nutation::true_obliquity(jd))
    });
}

fn bench_vsop87(c: &mut Criterion) {
    let jd = 2_451_545.0;
    c.bench_function("vsop87_earth", |b| {
        b.iter(|| jyotish::vsop87::earth_heliocentric(jd))
    });
    c.bench_function("vsop87_jupiter", |b| {
        b.iter(|| jyotish::vsop87::planet_heliocentric(Planet::Jupiter, jd).unwrap())
    });
}

fn bench_apparent(c: &mut Criterion) {
    let jd = 2_451_545.0;
    c.bench_function("apparent_mars", |b| {
        b.iter(|| apparent::apparent_position(Planet::Mars, jd).unwrap())
    });
    c.bench_function("apparent_sun", |b| b.iter(|| apparent::apparent_sun(jd)));
    c.bench_function("apparent_moon", |b| b.iter(|| apparent::apparent_moon(jd)));
}

fn bench_aberration(c: &mut Criterion) {
    let jd = 2_451_545.0;
    c.bench_function("apply_aberration", |b| {
        b.iter(|| jyotish::aberration::apply_aberration(45.0, 1.5, 280.0, 23.44))
    });
    c.bench_function("light_time_correction", |b| {
        b.iter(|| {
            jyotish::aberration::light_time_correction(
                jd,
                |jd_lt| {
                    let pos = planetary::compute_position(Planet::Mars, jd_lt).unwrap();
                    (pos.longitude_deg, pos.latitude_deg, pos.distance_au)
                },
                jyotish::vsop87::earth_heliocentric,
            )
        })
    });
}

fn bench_houses(c: &mut Criterion) {
    let jd = 2_451_545.0;
    c.bench_function("placidus_houses", |b| {
        b.iter(|| {
            jyotish::house::compute_houses(jyotish::house::HouseSystem::Placidus, jd, 51.5, -0.1)
        })
    });
    c.bench_function("whole_sign_houses", |b| {
        b.iter(|| {
            jyotish::house::compute_houses(jyotish::house::HouseSystem::WholeSign, jd, 51.5, -0.1)
        })
    });
}

fn bench_riseset(c: &mut Criterion) {
    c.bench_function("sun_rise_set", |b| {
        b.iter(|| jyotish::riseset::rise_set_transit(Planet::Sun, 2000, 1, 1, 51.5, -0.1))
    });
}

fn bench_events(c: &mut Criterion) {
    c.bench_function("next_vernal_equinox", |b| {
        b.iter(|| jyotish::event::next_season(jyotish::event::Season::VernalEquinox, 2_451_545.0))
    });
}

criterion_group!(
    benches,
    bench_planet_display,
    bench_planet_serde,
    bench_calendar,
    bench_sun,
    bench_moon,
    bench_planetary,
    bench_nutation,
    bench_vsop87,
    bench_apparent,
    bench_aberration,
    bench_houses,
    bench_riseset,
    bench_events,
);
criterion_main!(benches);
