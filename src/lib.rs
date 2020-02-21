//! # Bucket Vector
//!
//! 100% `unsafe` Rust free!
//!
//! ## Description
//!
//! A vector-like data structure that organizes its elements into a set of buckets
//! of fixed-capacity in order to guarantee that mutations to the bucket vector
//! never moves elements and thus invalidates references to them.
//!
//! This is comparable to a `Vec<Box<T>>` but a lot more efficient.
//!
//! ## Under the Hood
//!
//! The `BucketVec` is really just a vector of `Bucket` instances.
//! Whenever an element is pushed to the `BucketVec` the element is pushed onto
//! the last `Bucket` if it isn't filled, yet.
//! If the last `Bucket` is filled a new `Bucket` is pushed onto the `BucketVec`
//! with a new capacity determined by the used bucket vector configuration.
//!
//! This way the `BucketVec` never moves elements around upon inserting new elements
//! in order to preserve references. When a normal `Vec` is modified it can potentially
//! invalidate references because of reallocation of the internal buffer which
//! might cause severe bugs if references to the internal elements are stored
//! outside the `Vec`. Note that normally Rust prevents such situations so the
//! `BucketVec` is mainly used in the area of `unsafe` Rust where a developer
//! actively decides that they want or need pinned references into another data
//! structure.
//!
//! For the same reasons as stated above the `BucketVec` does not allow to remove
//! or swap elements.
//!
//! ## Example
//!
//! Looking at an example `BucketVec<i32>` with the following configuration:
//!
//! - `start_capacity := 1`
//! - `growth_rate := 2`
//!
//! We have already pushed the elements `A`,.., `K` onto it.
//!
//! ```no_compile
//! [ [A], [B, C], [D, E, F, G], [H, I, J, K, _, _, _, _] ]
//! ```
//!
//! Where `_` refers to a vacant bucket entry.
//!
//! Pushing another `L`,.., `O` onto the same `BucketVec` results in:
//!
//! ```no_compile
//! [ [A], [B, C], [D, E, F, G], [H, I, J, K, L, M, N, O] ]
//! ```
//!
//! So we are full on capacity for all buckets.
//! The next time we push another element onto the `BucketVec` it will create a new `Bucket` with a capacity of `16` since `growth_rate == 2` and our current latest bucket already has a capacity of `8`.
//!
//! ```no_compile
//! [ [A], [B, C], [D, E, F, G], [H, I, J, K, L, M, N, O], [P, 15 x _] ]
//! ```
//!
//! Where `15 x _` denotes 15 consecutive vacant entries.

#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

mod bucket;
mod config;
mod iter;
mod math;

#[cfg(test)]
mod tests;

use self::bucket::Bucket;
use self::math::FloatExt;
pub use self::{
    config::{BucketVecConfig, DefaultConfig},
    iter::Iter,
};
use core::marker::PhantomData;

/// A vector-like data structure that never moves its contained elements.
///
/// This is solved by using internal fixed-capacity buckets instead of boxing
/// all elements in isolation.
///
/// # Formulas
///
/// ## Definitions
///
/// In the following we define
///
/// - `N := START_CAPACITY` and
/// - `a := GROWTH_RATE`
///
/// ## Bucket Capacity
///
/// ### For `a != 1`:
///
/// The total capacity of all buckets until bucket `i` (not including `i`)
/// is expressed as:
///
/// ```no_compile
/// capacity_until(i) := N * (a^i - 1) / (a-1)
/// ```
///
/// The capacity of the `i`th bucket is then calculated by:
///
/// ```no_compile
/// capacity(i) := floor(capacity_until(i+1)) - floor(capacity_until(i))
/// ```
///
/// Where `floor: f64 -> f64` rounds the `f64` down to the next even `f64`
/// for positive `f64`.
///
/// Note that `capacity(i)` is approximately `capacity(i)' := N * a^i`.
///
/// ### For `a == 1`:
///
/// This case is trivial and all buckets are equally sized to have a
/// capacity of `N`.
///
/// ## Accessing Elements by Index
///
/// Accessing the `i`th element of a `BucketVec` can be expressed by the
/// following formulas:
///
/// ### For `a != 1`:
///
/// First we define the inverted capacity function for which
/// `1 == capacity(i) * inv_capacity(i)` forall `i`.
/// ```no_compile
/// inv_capacity(i) = ceil(log(1 + (i + 1) * (a - 1) / N, a)) - 1
/// ```
/// Where `ceil: f64 -> f64` rounds the `f64` up to the next even `f64`
/// for positive `f64`.
///
/// Having this the `bucket_index` and the `entry_index` inside the bucket
/// indexed by `bucket_index` is expressed as:
/// ```no_compile
/// bucket_index(i) = inv_capacity(i)
/// entry_index(i) = i - floor(capacity_until(bucket_index(i)))
/// ```
///
/// ### For `a == 1`:
///
/// This case is very easy and we can simply calculate the `bucket_index` and
/// `entry_index` by:
///
/// ```no_compile
/// bucket_index(i) = i / N
/// entry_index(i) = i % N
/// ```
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BucketVec<T, C = DefaultConfig> {
    /// The number of elements stored in the bucket vector.
    len: usize,
    /// The entry vector.
    buckets: Vec<Bucket<T>>,
    /// The config phantom data.
    config: PhantomData<fn() -> C>,
}

/// Accessor into a recently pushed element.
pub struct Access<'a, T> {
    /// Access by index.
    index: usize,
    /// Access by exclusive reference.
    reference: &'a mut T,
}

impl<'a, T> Access<'a, T> {
    /// Creates a new accessor to the given index and reference.
    pub fn new(index: usize, reference: &'a mut T) -> Self {
        Self { index, reference }
    }

    /// Returns the index of the recently pushed element.
    pub fn index(&self) -> usize {
        self.index
    }

    /// Returns a shared reference to the recently pushed element.
    pub fn into_ref(self) -> &'a T {
        self.reference
    }

    /// Returns an exclusive reference to the recently pushed element.
    pub fn into_mut(self) -> &'a mut T {
        self.reference
    }
}

impl<T> Default for BucketVec<T, DefaultConfig> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, C> BucketVec<T, C> {
    /// Creates a new empty bucket vector.
    ///
    /// # Note
    ///
    /// This does not allocate any heap memory.
    pub fn new() -> Self {
        Self {
            len: 0,
            buckets: Vec::new(),
            config: Default::default(),
        }
    }

    /// Returns the number of elements stored in the bucket vector.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` if the bucket vector is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator that yields shared references to the elements of the bucket vector.
    pub fn iter(&self) -> Iter<T> {
        Iter::new(self)
    }
}

impl<T, C> BucketVec<T, C>
where
    C: BucketVecConfig,
{
    /// Returns the bucket index and its internal entry index for the given
    /// bucket vector index into an element.
    ///
    /// Returns `None` if the index is out of bounds.
    fn bucket_entry_indices(&self, index: usize) -> Option<(usize, usize)> {
        if index >= self.len() {
            return None;
        }
        // Calculate bucket index and entry index within the bucket.
        let start_capacity = <C as BucketVecConfig>::STARTING_CAPACITY;
        let growth_rate = <C as BucketVecConfig>::GROWTH_RATE;
        if (growth_rate - 1.0).abs() < 1e-10 {
            // growth_rate == 1.0:
            // Simple case: All buckets are equally sized.
            let x = index / start_capacity;
            let y = index % start_capacity;
            Some((x, y))
        } else {
            // growth rate != 1.0:
            // Non-trivial case: Buckets are unequally sized.
            let f_inv = 1.0 + (index + 1) as f64 * (growth_rate - 1.0) / start_capacity as f64;
            let off_x = if (growth_rate - 2.0).abs() < 1e-10 {
                <f64 as FloatExt>::log2(f_inv)
            } else {
                <f64 as FloatExt>::log(f_inv, growth_rate)
            };
            let x = <f64 as FloatExt>::ceil(off_x) as usize - 1;
            let y = index - Self::total_capacity(x);
            Some((x, y))
        }
    }

    /// Returns a shared reference to the element at the given index if any.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.bucket_entry_indices(index)
            .and_then(|(x, y)| self.buckets[x].get(y))
    }

    /// Returns an exclusive reference to the element at the given index if any.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.bucket_entry_indices(index)
            .and_then(move |(x, y)| self.buckets[x].get_mut(y))
    }

    /// Returns the total capacity of all buckets up to (and including) the
    /// bucket indexed by `index`.
    fn total_capacity(index: usize) -> usize {
        let start_capacity = <C as BucketVecConfig>::STARTING_CAPACITY;
        let growth_rate = <C as BucketVecConfig>::GROWTH_RATE;
        <f64 as FloatExt>::floor(
            start_capacity as f64 * (growth_rate.powi(index as i32) - 1.0) / (growth_rate - 1.0),
        ) as usize
    }

    /// Returns the capacity of the indexed bucket.
    fn bucket_capacity(index: usize) -> usize {
        let start_capacity = <C as BucketVecConfig>::STARTING_CAPACITY;
        let growth_rate = <C as BucketVecConfig>::GROWTH_RATE;
        if (growth_rate - 1.0).abs() < 1e-10 {
            start_capacity
        } else {
            let next_total_capacity = Self::total_capacity(index + 1);
            let total_capacity = Self::total_capacity(index);
            next_total_capacity - total_capacity
        }
    }

    /// Pushes a new bucket containing the new value onto the bucket vector.
    fn push_bucket(&mut self, new_value: T) {
        let len_buckets = self.buckets.len();
        let new_capacity = Self::bucket_capacity(len_buckets);
        let mut new_entry = Bucket::new(new_capacity);
        new_entry.push(new_value);
        self.buckets.push(new_entry);
        self.len += 1;
    }

    /// Pushes a new element onto the bucket vector.
    ///
    /// # Note
    ///
    /// This operation will never move other elements, reallocates or otherwise
    /// invalidate pointers of elements contained by the bucket vector.
    pub fn push(&mut self, new_value: T) {
        if let Some(entry) = self.buckets.last_mut() {
            if entry.len() < entry.capacity() {
                entry.push(new_value);
                self.len += 1;
                return;
            }
        }
        self.push_bucket(new_value);
    }

    /// Pushes a new element onto the bucket vector and returns access to it.
    ///
    /// # Note
    ///
    /// This operation will never move other elements, reallocates or otherwise
    /// invalidate pointers of elements contained by the bucket vector.
    pub fn push_get(&mut self, new_value: T) -> Access<T> {
        let index = self.len();
        self.push(new_value);
        let ref_mut = self
            .get_mut(index)
            .expect("we just pushed an element so must be Some");
        Access::new(index, ref_mut)
    }
}

impl<T, C> core::iter::FromIterator<T> for BucketVec<T, C>
where
    C: BucketVecConfig,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut vec = Self::new();
        <Self as core::iter::Extend<T>>::extend(&mut vec, iter);
        vec
    }
}

impl<T, C> core::iter::Extend<T> for BucketVec<T, C>
where
    C: BucketVecConfig,
{
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push(item)
        }
    }
}
