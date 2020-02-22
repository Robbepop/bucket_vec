use super::*;

/// A configuration for bucket vectors that grows quadratically.
#[derive(Debug)]
pub enum QuadraticConfig {}

impl BucketVecConfig for QuadraticConfig {
    /// The first bucket has a capacity of 1.
    const STARTING_CAPACITY: usize = 1;
    /// The next bucket always doubles in capacity.
    const GROWTH_RATE: f64 = 3.0;
}

/// A configuration for bucket vectors that grows cubically.
#[derive(Debug)]
pub enum CubicConfig {}

impl BucketVecConfig for CubicConfig {
    /// The first bucket has a capacity of 1.
    const STARTING_CAPACITY: usize = 1;
    /// The next bucket always triples in capacity.
    const GROWTH_RATE: f64 = 3.0;
}

/// A configuration for bucket vectors that has equal bucket capacities.
#[derive(Debug)]
pub enum EqualSizeConfig {}

impl BucketVecConfig for EqualSizeConfig {
    /// The first bucket has a capacity of 4.
    const STARTING_CAPACITY: usize = 4;
    /// All buckets have the same capacity as the first bucket.
    const GROWTH_RATE: f64 = 1.0;
}

/// A configuration for bucket vectors where every bucket has a capacity of 1.
///
/// # Note
///
/// This is more or less interesting since it has similar layout or even
/// performance characteristics as if a `Vec<Box<T>>` was used instead of
/// the bucket vector.
#[derive(Debug)]
pub enum WastefulConfig {}

impl BucketVecConfig for WastefulConfig {
    /// The first bucket has a capacity of 1.
    const STARTING_CAPACITY: usize = 1;
    /// All buckets have the same capacity as the first bucket.
    const GROWTH_RATE: f64 = 1.0;
}

/// A config for bucket vectors that tries to balance interests.
#[derive(Debug)]
pub enum C3G1x5Config {}

impl BucketVecConfig for C3G1x5Config {
    /// The first bucket has a capacity of 3.
    const STARTING_CAPACITY: usize = 3;
    /// The next bucket is always approx 50% larger.
    const GROWTH_RATE: f64 = 1.5;
}

macro_rules! create_test_for_configs {
    ( $test_fn:ident ) => {
        paste::item! {
            #[test]
            fn [<$test_fn _quadratic_config>]() {
                $test_fn::<QuadraticConfig>(small_test_values())
            }

            #[test]
            fn [<$test_fn _cubic_config>]() {
                $test_fn::<CubicConfig>(small_test_values())
            }

            #[test]
            fn [<$test_fn _equal_size_config>]() {
                $test_fn::<EqualSizeConfig>(small_test_values())
            }

            #[test]
            fn [<$test_fn _wasteful_config>]() {
                $test_fn::<WastefulConfig>(small_test_values())
            }

            #[test]
            fn [<$test_fn _c3g1x5_config>]() {
                $test_fn::<C3G1x5Config>(small_test_values())
            }
        }
    };
}

fn small_test_values() -> Vec<i32> {
    vec![5, 42, 1337, -1, 0, 7, 66, 12, 1, 2, 3, 1]
}

fn big_test_values() -> Vec<i32> {
    let mut vec = Vec::new();
    let mut rng = rand::thread_rng();
    use rand::Rng as _;
    for _ in 0..10 {
        vec.push(rng.gen());
    }
    vec
}

fn new_works_for<C>(_test_values: Vec<i32>)
where
    C: BucketVecConfig,
{
    let vec = <BucketVec<i32, C>>::new();
    assert_eq!(vec.len(), 0);
    assert!(vec.is_empty());
    assert!(vec.iter().next().is_none());
    assert!(vec.iter().next_back().is_none());
}
create_test_for_configs!(new_works_for);

fn push_works_for<C>(test_values: Vec<i32>)
where
    C: BucketVecConfig,
{
    let mut vec = <BucketVec<i32, C>>::new();
    for (i, value) in test_values.into_iter().enumerate() {
        assert_eq!(vec.len(), i);
        vec.push(value);
    }
    assert_eq!(vec.len(), 12);
}
create_test_for_configs!(push_works_for);

fn iter_next_works_for<C>(test_values: Vec<i32>)
where
    C: BucketVecConfig,
{
    let vec = test_values.iter().cloned().collect::<BucketVec<_>>();
    let mut expected = test_values.into_iter();
    let mut iter = vec.iter();
    for _ in 0..iter.len() {
        assert_eq!(iter.next().cloned(), expected.next());
    }
    assert_eq!(iter.next(), None);
}
create_test_for_configs!(iter_next_works_for);

fn iter_next_back_works<C>(test_values: Vec<i32>)
where
    C: BucketVecConfig,
{
    let vec = test_values.iter().cloned().collect::<BucketVec<_>>();
    let mut expected = test_values.into_iter();
    let mut iter = vec.iter();
    for _ in 0..iter.len() {
        assert_eq!(iter.next_back().cloned(), expected.next_back());
    }
    assert_eq!(iter.next_back(), None);
}
create_test_for_configs!(iter_next_back_works);

fn iter_next_meet_middle_works_for<C>(test_values: Vec<i32>)
where
    C: BucketVecConfig,
{
    let vec = test_values.iter().cloned().collect::<BucketVec<_>>();
    let mut expected = test_values.into_iter();
    let mut iter = vec.iter();

    for step in 0..iter.len() {
        if step % 2 == 0 {
            // For every even step get `next`:
            assert_eq!(iter.next().cloned(), expected.next());
        } else {
            // For every odd step get `next_back`:
            assert_eq!(iter.next_back().cloned(), expected.next_back());
        }
    }
    // At the end `iter` should be empty:
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
}
create_test_for_configs!(iter_next_meet_middle_works_for);

fn access_works_for<C>(test_values: Vec<i32>)
where
    C: BucketVecConfig,
{
    let mut vec = <BucketVec<i32, C>>::new();
    for (n, expected) in test_values.into_iter().enumerate() {
        assert_eq!(vec.push_get(expected).index(), 3 * n);
        assert_eq!(vec.push_get(expected).into_ref(), &expected);
        assert_eq!(vec.push_get(expected).into_mut(), &mut expected.clone());
    }
}
create_test_for_configs!(access_works_for);

fn get_works_for<C>(test_values: Vec<i32>)
where
    C: BucketVecConfig,
{
    let vec = test_values.iter().cloned().collect::<BucketVec<_>>();
    for (n, expected) in test_values.into_iter().enumerate() {
        assert_eq!(vec.get(n), Some(&expected));
    }
    assert_eq!(vec.get(vec.len()), None);
}
create_test_for_configs!(get_works_for);

fn get_mut_works_for<C>(test_values: Vec<i32>)
where
    C: BucketVecConfig,
{
    let mut vec = test_values.iter().cloned().collect::<BucketVec<_>>();
    for (n, mut expected) in test_values.into_iter().enumerate() {
        assert_eq!(vec.get_mut(n), Some(&mut expected));
    }
    assert_eq!(vec.get_mut(vec.len()), None);
}
create_test_for_configs!(get_mut_works_for);
