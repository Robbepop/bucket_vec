use bucket_vec::BucketVec;
use criterion::{black_box, criterion_group, criterion_main, BatchSize, BenchmarkId, Criterion};

/// A configuration for bucket vectors that has equal bucket capacities.
#[derive(Debug)]
pub enum EqualSizeConfig {}

impl bucket_vec::BucketVecConfig for EqualSizeConfig {
    const STARTING_CAPACITY: usize = 16;
    const GROWTH_RATE: f64 = 1.0;
}

/// A configuration for bucket vectors that tries to balance out interests.
#[derive(Debug)]
pub enum C5g1x5Config {}

impl bucket_vec::BucketVecConfig for C5g1x5Config {
    const STARTING_CAPACITY: usize = 5;
    const GROWTH_RATE: f64 = 1.5;
}

/// A configuration for bucket vectors that grows quadratically.
#[derive(Debug)]
pub enum QuadraticConfig {}

impl bucket_vec::BucketVecConfig for QuadraticConfig {
    const STARTING_CAPACITY: usize = 4;
    const GROWTH_RATE: f64 = 2.0;
}

const BIG_SAMPLE_SIZE: usize = 10_000;

fn bench_bucket_vec_push(c: &mut Criterion) {
    c.bench_with_input(
        BenchmarkId::new("bucket_vec::push", BIG_SAMPLE_SIZE),
        &BIG_SAMPLE_SIZE,
        |b, &size| {
            let mut vec = BucketVec::<i32, QuadraticConfig>::new();
            b.iter(|| {
                for _ in 0..size {
                    vec.push(0);
                }
            });
        },
    );
}

fn bench_vec_box_push(c: &mut Criterion) {
    c.bench_with_input(
        BenchmarkId::new("vec_box::push", BIG_SAMPLE_SIZE),
        &BIG_SAMPLE_SIZE,
        |b, &size| {
            let mut vec = Vec::<Box<i32>>::new();
            b.iter(|| {
                for _ in 0..size {
                    vec.push(Box::new(0));
                }
            });
        },
    );
}

fn bench_vec_value_push(c: &mut Criterion) {
    c.bench_with_input(
        BenchmarkId::new("vec_value::push", BIG_SAMPLE_SIZE),
        &BIG_SAMPLE_SIZE,
        |b, &size| {
            let mut vec = Vec::<i32>::new();
            b.iter(|| {
                for _ in 0..size {
                    vec.push(0);
                }
            });
        },
    );
}

fn bench_bucket_vec_get(c: &mut Criterion) {
    let vec = (0..BIG_SAMPLE_SIZE)
        .into_iter()
        .map(|value| value as i32)
        .collect::<BucketVec<i32, EqualSizeConfig>>();
    c.bench_with_input(
        BenchmarkId::new("bucket_vec::get", BIG_SAMPLE_SIZE),
        &vec,
        |b, vec| {
            b.iter(|| {
                for i in 0..vec.len() {
                    black_box(vec.get(i).map(|val| *val));
                }
            });
        },
    );
}

fn bench_vec_box_get(c: &mut Criterion) {
    let vec = (0..BIG_SAMPLE_SIZE)
        .into_iter()
        .map(|value| Box::new(value as i32))
        .collect::<Vec<Box<i32>>>();
    c.bench_with_input(
        BenchmarkId::new("vec_box::get", BIG_SAMPLE_SIZE),
        &vec,
        |b, vec| {
            b.iter(|| {
                for i in 0..vec.len() {
                    black_box(vec.get(i).map(|val| **val));
                }
            });
        },
    );
}

fn bench_vec_value_get(c: &mut Criterion) {
    let vec = vec![0; BIG_SAMPLE_SIZE];
    c.bench_with_input(
        BenchmarkId::new("vec_value::get", BIG_SAMPLE_SIZE),
        &vec,
        |b, vec| {
            b.iter(|| {
                for i in 0..vec.len() {
                    black_box(vec.get(i).map(|val| *val));
                }
            });
        },
    );
}

fn bench_bucket_vec_iter(c: &mut Criterion) {
    let vec = (0..BIG_SAMPLE_SIZE)
        .into_iter()
        .map(|value| value as i32)
        .collect::<BucketVec<i32, QuadraticConfig>>();
    c.bench_with_input(
        BenchmarkId::new("bucket_vec::iter", BIG_SAMPLE_SIZE),
        &vec,
        |b, vec| {
            b.iter(|| {
                for i in vec.iter() {
                    black_box(*i);
                }
            });
        },
    );
}

fn bench_vec_box_iter(c: &mut Criterion) {
    let vec = (0..BIG_SAMPLE_SIZE)
        .into_iter()
        .map(|value| Box::new(value as i32))
        .collect::<Vec<Box<i32>>>();
    c.bench_with_input(
        BenchmarkId::new("vec_box::iter", BIG_SAMPLE_SIZE),
        &vec,
        |b, vec| {
            b.iter(|| {
                for i in vec.iter() {
                    black_box(**i);
                }
            });
        },
    );
}

fn bench_vec_value_iter(c: &mut Criterion) {
    let vec = vec![0; BIG_SAMPLE_SIZE];
    c.bench_with_input(
        BenchmarkId::new("vec_value::iter", BIG_SAMPLE_SIZE),
        &vec,
        |b, vec| {
            b.iter(|| {
                for i in vec.iter() {
                    black_box(*i);
                }
            });
        },
    );
}

fn bench_bucket_vec_iter_rev(c: &mut Criterion) {
    let vec = (0..BIG_SAMPLE_SIZE)
        .into_iter()
        .map(|value| value as i32)
        .collect::<BucketVec<i32, QuadraticConfig>>();
    c.bench_with_input(
        BenchmarkId::new("bucket_vec::iter.rev()", BIG_SAMPLE_SIZE),
        &vec,
        |b, vec| {
            b.iter(|| {
                for i in vec.iter().rev() {
                    black_box(*i);
                }
            });
        },
    );
}

fn bench_vec_box_iter_rev(c: &mut Criterion) {
    let vec = (0..BIG_SAMPLE_SIZE)
        .into_iter()
        .map(|value| Box::new(value as i32))
        .collect::<Vec<Box<i32>>>();
    c.bench_with_input(
        BenchmarkId::new("vec_box::iter.rev()", BIG_SAMPLE_SIZE),
        &vec,
        |b, vec| {
            b.iter(|| {
                for i in vec.iter().rev() {
                    black_box(**i);
                }
            });
        },
    );
}

fn bench_vec_value_iter_rev(c: &mut Criterion) {
    let vec = vec![0; BIG_SAMPLE_SIZE];
    c.bench_with_input(
        BenchmarkId::new("vec_value::iter.rev()", BIG_SAMPLE_SIZE),
        &vec,
        |b, vec| {
            b.iter(|| {
                for i in vec.iter().rev() {
                    black_box(*i);
                }
            });
        },
    );
}

fn bench_bucket_vec_iter_mut(c: &mut Criterion) {
    let vec = vec![0; BIG_SAMPLE_SIZE]
        .into_iter()
        .collect::<BucketVec<i32, QuadraticConfig>>();
    c.bench_with_input(
        BenchmarkId::new("bucket_vec::iter_mut", BIG_SAMPLE_SIZE),
        &BIG_SAMPLE_SIZE,
        move |b, _size| {
            b.iter_batched_ref(
                || vec.clone(),
                |vec| {
                    for i in vec.iter_mut() {
                        black_box(*i);
                    }
                },
                BatchSize::SmallInput,
            );
        },
    );
}

fn bench_vec_box_iter_mut(c: &mut Criterion) {
    let vec = (0..BIG_SAMPLE_SIZE)
        .into_iter()
        .map(|value| Box::new(value as i32))
        .collect::<Vec<Box<i32>>>();
    c.bench_with_input(
        BenchmarkId::new("vec_box::iter_mut", BIG_SAMPLE_SIZE),
        &BIG_SAMPLE_SIZE,
        move |b, _size| {
            b.iter_batched_ref(
                || vec.clone(),
                |vec| {
                    for i in vec.iter_mut() {
                        black_box(**i);
                    }
                },
                BatchSize::SmallInput,
            );
        },
    );
}

fn bench_vec_value_iter_mut(c: &mut Criterion) {
    let vec = vec![0; BIG_SAMPLE_SIZE];
    c.bench_with_input(
        BenchmarkId::new("vec_value::iter_mut", BIG_SAMPLE_SIZE),
        &BIG_SAMPLE_SIZE,
        move |b, _size| {
            b.iter_batched_ref(
                || vec.clone(),
                |vec| {
                    for i in vec.iter_mut() {
                        black_box(*i);
                    }
                },
                BatchSize::SmallInput,
            );
        },
    );
}

criterion_group!(
    bench_push,
    bench_bucket_vec_push,
    bench_vec_box_push,
    bench_vec_value_push
);
criterion_group!(
    bench_get,
    bench_bucket_vec_get,
    bench_vec_box_get,
    bench_vec_value_get,
);
criterion_group!(
    bench_iter,
    bench_bucket_vec_iter,
    bench_vec_box_iter,
    bench_vec_value_iter,
);
criterion_group!(
    bench_iter_rev,
    bench_bucket_vec_iter_rev,
    bench_vec_box_iter_rev,
    bench_vec_value_iter_rev,
);
criterion_group!(
    bench_iter_mut,
    bench_bucket_vec_iter_mut,
    bench_vec_box_iter_mut,
    bench_vec_value_iter_mut,
);
criterion_main!(
    bench_push,
    bench_get,
    bench_iter,
    bench_iter_rev,
    bench_iter_mut,
);
