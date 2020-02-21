use bucket_vec::BucketVec;
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

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
            black_box(vec);
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
            black_box(vec);
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
            black_box(vec);
        },
    );
}

fn bench_bucket_vec_get(c: &mut Criterion) {
    let vec = vec![0; BIG_SAMPLE_SIZE]
        .into_iter()
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
            black_box(vec);
        },
    );
}

fn bench_vec_box_get(c: &mut Criterion) {
    let vec = vec![0; BIG_SAMPLE_SIZE]
        .into_iter()
        .map(|value| Box::new(value))
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
            black_box(vec);
        },
    );
}

fn bench_vec_value_get(c: &mut Criterion) {
    let vec = vec![0; BIG_SAMPLE_SIZE].into_iter().collect::<Vec<i32>>();
    c.bench_with_input(
        BenchmarkId::new("vec_value::get", BIG_SAMPLE_SIZE),
        &vec,
        |b, vec| {
            b.iter(|| {
                for i in 0..vec.len() {
                    black_box(vec.get(i).map(|val| *val));
                }
            });
            black_box(vec);
        },
    );
}

fn bench_bucket_vec_iter(c: &mut Criterion) {
    let vec = vec![0; BIG_SAMPLE_SIZE]
        .into_iter()
        .collect::<BucketVec<i32, EqualSizeConfig>>();
    c.bench_with_input(
        BenchmarkId::new("bucket_vec::iter", BIG_SAMPLE_SIZE),
        &vec,
        |b, vec| {
            b.iter(|| {
                for i in vec.iter() {
                    black_box(*i);
                }
            });
            black_box(vec);
        },
    );
}

fn bench_vec_box_iter(c: &mut Criterion) {
    let vec = vec![0; BIG_SAMPLE_SIZE]
        .into_iter()
        .map(|value| Box::new(value))
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
            black_box(vec);
        },
    );
}

fn bench_vec_value_iter(c: &mut Criterion) {
    let vec = vec![0; BIG_SAMPLE_SIZE]
        .into_iter()
        .collect::<Vec<i32>>();
    c.bench_with_input(
        BenchmarkId::new("vec_value::iter", BIG_SAMPLE_SIZE),
        &vec,
        |b, vec| {
            b.iter(|| {
                for i in vec.iter() {
                    black_box(*i);
                }
            });
            black_box(vec);
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
criterion_main!(
    bench_push,
    bench_get,
    bench_iter,
);