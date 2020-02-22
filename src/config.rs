use crate::FloatExt;

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
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DefaultConfig {}

impl BucketVecConfig for DefaultConfig {
    /// The first bucket has a capacity of 4.
    const STARTING_CAPACITY: usize = 4;
    /// The next bucket always doubles in capacity.
    const GROWTH_RATE: f64 = 2.0;
}

/// Returns the total capacity of all buckets up to (and including) the
/// bucket indexed by `index`.
pub fn total_capacity<C>(index: usize) -> usize
where
    C: BucketVecConfig,
{
    let start_capacity = <C as BucketVecConfig>::STARTING_CAPACITY;
    let growth_rate = <C as BucketVecConfig>::GROWTH_RATE;
    if <f64 as FloatExt>::fract(growth_rate).abs() < core::f64::EPSILON {
        let growth_rate = growth_rate as usize;
        start_capacity * (growth_rate.pow(index as u32) - 1) / (growth_rate - 1)
    } else {
        <f64 as FloatExt>::floor(
            start_capacity as f64 * (<f64 as FloatExt>::powi(growth_rate, index as i32) - 1.0)
                / (growth_rate - 1.0),
        ) as usize
    }
}

/// Returns the capacity of the indexed bucket.
pub fn bucket_capacity<C>(index: usize) -> usize
where
    C: BucketVecConfig,
{
    let start_capacity = <C as BucketVecConfig>::STARTING_CAPACITY;
    let growth_rate = <C as BucketVecConfig>::GROWTH_RATE;
    if (growth_rate - 1.0).abs() < core::f64::EPSILON {
        start_capacity
    } else {
        let next_total_capacity = total_capacity::<C>(index + 1);
        let total_capacity = total_capacity::<C>(index);
        next_total_capacity - total_capacity
    }
}

/// Returns the bucket index and its internal entry index for the given
/// bucket vector index into an element.
pub fn bucket_entry_indices<C>(index: usize) -> (usize, usize)
where
    C: BucketVecConfig,
{
    // Calculate bucket index and entry index within the bucket.
    let start_capacity = <C as BucketVecConfig>::STARTING_CAPACITY;
    let growth_rate = <C as BucketVecConfig>::GROWTH_RATE;
    if (growth_rate - 1.0).abs() < core::f64::EPSILON {
        // growth_rate == 1.0:
        // Simple case: All buckets are equally sized.
        let x = index / start_capacity;
        let y = index % start_capacity;
        (x, y)
    } else {
        // growth rate != 1.0:
        // Non-trivial case: Buckets are unequally sized.
        let f_inv = 1.0 + (index + 1) as f64 * (growth_rate - 1.0) / start_capacity as f64;
        let off_x = if (growth_rate - 2.0).abs() < core::f64::EPSILON {
            <f64 as FloatExt>::log2(f_inv)
        } else {
            <f64 as FloatExt>::log(f_inv, growth_rate)
        };
        let x = <f64 as FloatExt>::ceil(off_x) as usize - 1;
        let y = index - total_capacity::<C>(x);
        (x, y)
    }
}
