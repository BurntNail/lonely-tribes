use criterion::{criterion_group, criterion_main, Criterion};
use lonely_tribes_generation::{sprite_stuff::Room, procedural_generator::ProceduralGenerator};

pub fn procgen_benchmark(c: &mut Criterion) {
    c.bench_function("procgen rand", |b| {
        b.iter(|| ProceduralGenerator::new(rand::random()).get())
    });
}
pub fn room_benchmark(c: &mut Criterion) {
    c.bench_function("normal roomgen bench", |b| {
        b.iter(|| Room::new("lvl-01.png".to_string()))
    });
}
pub fn room_pg_benchmark(c: &mut Criterion) {
    c.bench_function("procgen roomgen bench", |b| {
        b.iter(|| Room::proc_gen(rand::random()))
    });
}

criterion_group!(procgen, procgen_benchmark, room_pg_benchmark);
criterion_group!(normal, room_benchmark);
criterion_main!(procgen, normal);
