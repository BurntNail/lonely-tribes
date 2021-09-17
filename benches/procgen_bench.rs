use criterion::{criterion_group, criterion_main, Criterion};
use lonely_tribes_lib::procedural_generator::ProceduralGenerator;

pub fn crit_benchmark (c: &mut Criterion) {
	c.bench_function("procgen rand", |b| b.iter(|| ProceduralGenerator::new(rand::random()).get()));
}

criterion_group!(benches, crit_benchmark);
criterion_main!(benches);