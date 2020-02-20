use super::*;

fn filled_dummy_vec<C>() -> BucketVec<i32, C>
where
    C: BucketVecConfig,
{
    let mut vec = BucketVec::new();
    vec.push(5);
    vec.push(42);
    vec.push(1337);
    vec.push(-1);
    vec.push(0);
    vec.push(7);
    vec.push(66);
    vec.push(12);
    vec
}

#[test]
fn new_works() {
    let vec = <BucketVec<i32>>::default();
    assert_eq!(vec.len(), 0);
    assert!(vec.is_empty());
    assert!(vec.iter().next().is_none());
    assert!(vec.iter().next_back().is_none());
}

#[test]
fn push_works() {
    let mut vec = BucketVec::default();
    assert_eq!(vec.len(), 0);
    vec.push(5);
    vec.push(42);
    vec.push(1337);
    vec.push(-1);
    vec.push(0);
    vec.push(7);
    vec.push(66);
    vec.push(12);
    assert_eq!(vec.len(), 8);
}

#[test]
fn iter_next_works() {
    let vec = filled_dummy_vec::<DefaultConfig>();
    let mut iter = vec.iter();
    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next(), Some(&42));
    assert_eq!(iter.next(), Some(&1337));
    assert_eq!(iter.next(), Some(&-1));
    assert_eq!(iter.next(), Some(&0));
    assert_eq!(iter.next(), Some(&7));
    assert_eq!(iter.next(), Some(&66));
    assert_eq!(iter.next(), Some(&12));
    dbg!(&iter);
    assert_eq!(iter.next(), None);
}

#[test]
fn iter_next_back_works() {
    let vec = filled_dummy_vec::<DefaultConfig>();
    let mut iter = vec.iter();
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

#[test]
fn iter_next_meet_middle() {
    let vec = filled_dummy_vec::<DefaultConfig>();
    let mut iter = vec.iter();
    assert_eq!(iter.next(), Some(&5));
    assert_eq!(iter.next_back(), Some(&12));
    assert_eq!(iter.next(), Some(&42));
    assert_eq!(iter.next(), Some(&1337));
    assert_eq!(iter.next_back(), Some(&66));
    assert_eq!(iter.next_back(), Some(&7));
    assert_eq!(iter.next(), Some(&-1));
    assert_eq!(iter.next_back(), Some(&0));
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next_back(), None);
}

#[test]
fn access_works() {
    let mut vec = BucketVec::default();
    dbg!(&vec);
    assert_eq!(vec.push_get(1).index(), 0);
    // assert_eq!(vec.push_get(2).into_ref(), &2);
    // assert_eq!(vec.push_get(3).into_mut(), &mut 3);
}

fn get_works_for_config<C>()
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
    assert_eq!(vec.get(8), None);
}

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

#[test]
fn get_works_for_quadratic_config() {
    get_works_for_config::<QuadraticConfig>()
}

#[test]
fn get_works_for_cubic_config() {
    get_works_for_config::<CubicConfig>()
}

#[test]
fn get_works_for_equal_size_config() {
    get_works_for_config::<EqualSizeConfig>()
}

#[test]
fn get_works_for_wasteful_config() {
    get_works_for_config::<WastefulConfig>()
}

#[test]
fn get_works_for_c3g1x5_config() {
    get_works_for_config::<C3G1x5Config>()
}
