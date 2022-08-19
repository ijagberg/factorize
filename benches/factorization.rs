use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::distributions::{Distribution, Uniform};
use factorize;

pub fn factorize_benchmark(c: &mut Criterion) {
    c.bench_function("5_trial_division", |b| {
        b.iter(|| factorize::TrialDivision::factor_generic(black_box(5_u128)))
    });
    c.bench_function("11_trial_division", |b| {
        b.iter(|| factorize::TrialDivision::factor_generic(black_box(11_u128)))
    });
    c.bench_function("15_trial_division", |b| {
        b.iter(|| factorize::TrialDivision::factor_generic(black_box(15_u128)))
    });

    c.bench_function("5_brents_rho", |b| {
        b.iter(|| factorize::BrentsRho::factor_generic(black_box(5_u128)))
    });
    c.bench_function("11_brents_rho", |b| {
        b.iter(|| factorize::BrentsRho::factor_generic(black_box(11_u128)))
    });
    c.bench_function("15_brents_rho", |b| {
        b.iter(|| factorize::BrentsRho::factor_generic(black_box(15_u128)))
    });
    c.bench_function("79447834793_trial_division", |b| {
        b.iter(|| factorize::TrialDivision::factor_generic(black_box(79447834793_u128)))
    });
    c.bench_function("79447834793_brents_rho", |b| {
        b.iter(|| factorize::BrentsRho::factor_generic(black_box(79447834793_u128)))
    });
    let range = rand::distributions::Uniform::from(0..1000);
    let mut rng = rand::thread_rng();
    c.bench_function("random_low_number_trial_division", |b| {
        b.iter(|| factorize::TrialDivision::factor_generic(black_box(range.sample(&mut rng))))
    });
}

criterion_group!(benches, factorize_benchmark);
criterion_main!(benches);
