use criterion::{criterion_group, criterion_main, Criterion};

extern crate raytracer;
use raytracer::vec3::*;

fn random_by_gaussian() -> Vec3 {
    loop {
        let vec = Vec3::new(random_gaussian(), random_gaussian(), random_gaussian());
        if vec.x() != 0.0 && vec.y() != 0.0 && vec.z() != 0.0 {
            return vec / vec.length();
        }
    }
}

fn random_by_select() -> Vec3 {
    loop {
        let vec = Vec3::random_vector(-1.0, 1.0);
        if vec.length_squared() < 1.0 {
            return vec / vec.length();
        }

    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("gaussian", |b| b.iter(|| random_by_gaussian()));
    c.bench_function("select", |b| b.iter(|| random_by_select()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

// Gaussian - 70ns
// Select - 110ns