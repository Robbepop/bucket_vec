/// Basic configs of a bucket vector.
pub trait BucketVecConfig {
    /// The capacity of the first entry of the bucket vector.
    ///
    /// This value must be larger than or equal to `1`.
    const STARTING_CAPACITY: usize;
    /// The rate with which the buckets are extended in their capacity.
    ///
    /// This value must be larger than or equal to `1`.
    /// Bigger values increase the growth acceleration upon pushing elements.
    /// A value of `1` renders all buckets equally sized.
    const GROWTH_RATE: f64;
}

/// The default configuration for bucket vectors.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DefaultConfig {}

impl BucketVecConfig for DefaultConfig {
    /// The first bucket has a capacity of 4.
    const STARTING_CAPACITY: usize = 4;
    /// The next bucket always doubles in capacity.
    const GROWTH_RATE: f64 = 2.0;
}
