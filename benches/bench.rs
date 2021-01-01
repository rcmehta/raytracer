use criterion::{criterion_group, criterion_main, Criterion};

extern crate raytracer;
use raytracer::vec3::*;

// For Vec3::random_on_unit_sphere()

fn random_on_by_gaussian() -> Vec3 {
    loop {
        let vec = Vec3::new(random_gaussian(), random_gaussian(), random_gaussian());
        if vec.length_squared() > 0.0 {
            return vec.unit();
        }
    }
}

fn random_on_by_rejection() -> Vec3 {
    loop {
        let vec = Vec3::random_vector(-1.0, 1.0);
        if vec.length_squared() > 0.0 && vec.length_squared() <= 1.0 {
            return vec.unit();
        }
    }
}

// For Vec3::random_in_unit_disc().
fn random_in_by_gaussian() -> Vec3 {
    loop {
        let vec = Vec3::new(random_gaussian(), random_gaussian(), 0.0);
        let r = random().powf(0.5);
        if vec.length_squared() > 0.0 {
            return vec.unit() * r;
        }
    }
}

fn random_in_by_rejection() -> Vec3 {
    loop {
        let vec = Vec3::new(random_range(-1.0, 1.0), random_range(-1.0, 1.0), 0.0);
        if vec.length_squared() < 1.0 {
            return vec;
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("on_gaussian", |b| b.iter(|| random_on_by_gaussian()));
    c.bench_function("on_rejection", |b| b.iter(|| random_on_by_rejection()));

    c.bench_function("in_gaussian", |b| b.iter(|| random_in_by_gaussian()));
    c.bench_function("in_rejection", |b| b.iter(|| random_in_by_rejection()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

// Type - On, In
// Gaussian ~ 68 ns, 63 ns
// Rejection ~ 104 ns, 46 ns - curse of dimensionality
