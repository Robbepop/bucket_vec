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
                $test_fn::<QuadraticConfig>()
            }

            #[test]
            fn [<$test_fn _cubic_config>]() {
                $test_fn::<CubicConfig>()
            }

            #[test]
            fn [<$test_fn _equal_size_config>]() {
                $test_fn::<EqualSizeConfig>()
            }

            #[test]
            fn [<$test_fn _wasteful_config>]() {
                $test_fn::<WastefulConfig>()
            }

            #[test]
            fn [<$test_fn _c3g1x5_config>]() {
                $test_fn::<C3G1x5Config>()
            }
        }
    };
}

fn filled_dummy_vec<C>() -> BucketVec<i32, C>
where
    C: BucketVecConfig,
{
    vec![5, 42, 1337, -1, 0, 7, 66, 12, 1, 2, 3, 1].into_iter().collect()
}

fn new_works_for<C>()
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

fn push_works_for<C>()
where
    C: BucketVecConfig,
{
    let mut vec = <BucketVec<i32, C>>::new();
    assert_eq!(vec.len(), 0);
    vec.push(5);
    vec.push(42);
    vec.push(1337);
    vec.push(-1);
    vec.push(0);
    vec.push(7);
    vec.push(66);
    vec.push(12);
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(1);
    assert_eq!(vec.len(), 12);
}
create_test_for_configs!(push_works_for);

fn iter_next_works_for<C>()
where
    C: BucketVecConfig,
{
    let vec = filled_dummy_vec::<C>();
    let mut iter = vec.iter();
    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), Some(&42));
    assert_eq!(iter.next(), Some(&1337));
    assert_eq!(iter.next(), Some(&-1));
    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.next(), Some(&7));
    assert_eq!(iter.next(), Some(&66));
    assert_eq!(iter.next(), Some(&12));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), None);
}
create_test_for_configs!(iter_next_works_for);

fn iter_next_back_works<C>()
where
    C: BucketVecConfig,
{
    let vec = filled_dummy_vec::<C>();
    let mut iter = vec.iter();
    assert_eq!(iter.next_back(), Some(&1));
    assert_eq!(iter.next_back(), Some(&3));
    assert_eq!(iter.next_back(), Some(&2));
    assert_eq!(iter.next_back(), Some(&1));
    assert_eq!(iter.next_back(), Some(&12));
    assert_eq!(iter.next_back(), Some(&66));
    assert_eq!(iter.next_back(), Some(&7));
    assert_eq!(iter.next_back(), Some(&0));
    assert_eq!(iter.next_back(), Some(&-1));
    assert_eq!(iter.next_back(), Some(&1337));
    assert_eq!(iter.next_back(), Some(&42));
    assert_eq!(iter.next_back(), Some(&5));
    assert_eq!(iter.next(), None);
}
create_test_for_configs!(iter_next_back_works);

fn iter_next_meet_middle_works_for<C>()
where
    C: BucketVecConfig,
{
    let vec = filled_dummy_vec::<C>();
    let mut iter = vec.iter();

    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next_back(), Some(&1));
    assert_eq!(iter.next(), Some(&42));
    assert_eq!(iter.next_back(), Some(&3));
    assert_eq!(iter.next(), Some(&1337));
    assert_eq!(iter.next_back(), Some(&2));
    assert_eq!(iter.next(), Some(&-1));
    assert_eq!(iter.next_back(), Some(&1));
    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.next_back(), Some(&12));
    assert_eq!(iter.next(), Some(&7));
    assert_eq!(iter.next_back(), Some(&66));

    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
}
create_test_for_configs!(iter_next_meet_middle_works_for);

fn access_works_for<C>()
where
    C: BucketVecConfig,
{
    let mut vec = <BucketVec<i32, C>>::new();
    assert_eq!(vec.push_get(1).index(), 0);
    assert_eq!(vec.push_get(2).into_ref(), &2);
    assert_eq!(vec.push_get(3).into_mut(), &mut 3);
}
create_test_for_configs!(access_works_for);

fn get_works_for<C>()
where
    C: BucketVecConfig,
{
    let vec = filled_dummy_vec::<C>();
    assert_eq!(vec.get(0), Some(&5));
    assert_eq!(vec.get(1), Some(&42));
    assert_eq!(vec.get(2), Some(&1337));
    assert_eq!(vec.get(3), Some(&-1));
    assert_eq!(vec.get(4), Some(&0));
    assert_eq!(vec.get(5), Some(&7));
    assert_eq!(vec.get(6), Some(&66));
    assert_eq!(vec.get(7), Some(&12));
    assert_eq!(vec.get(8), Some(&1));
    assert_eq!(vec.get(9), Some(&2));
    assert_eq!(vec.get(10), Some(&3));
    assert_eq!(vec.get(11), Some(&1));
    assert_eq!(vec.get(12), None);
}
create_test_for_configs!(get_works_for);

fn get_mut_works_for<C>()
where
    C: BucketVecConfig,
{
    let mut vec = filled_dummy_vec::<C>();
    assert_eq!(vec.get_mut(0), Some(&mut 5));
    assert_eq!(vec.get_mut(1), Some(&mut 42));
    assert_eq!(vec.get_mut(2), Some(&mut 1337));
    assert_eq!(vec.get_mut(3), Some(&mut -1));
    assert_eq!(vec.get_mut(4), Some(&mut 0));
    assert_eq!(vec.get_mut(5), Some(&mut 7));
    assert_eq!(vec.get_mut(6), Some(&mut 66));
    assert_eq!(vec.get_mut(7), Some(&mut 12));
    assert_eq!(vec.get_mut(8), Some(&mut 1));
    assert_eq!(vec.get_mut(9), Some(&mut 2));
    assert_eq!(vec.get_mut(10), Some(&mut 3));
    assert_eq!(vec.get_mut(11), Some(&mut 1));
    assert_eq!(vec.get_mut(12), None);
}
create_test_for_configs!(get_mut_works_for);
