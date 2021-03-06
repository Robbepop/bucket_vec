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
mod scale;

#[cfg(test)]
mod tests;

use self::bucket::Bucket;
use self::math::FloatExt;
pub use self::{
    config::{BucketVecConfig, DefaultConfig},
    iter::{IntoIter, Iter, IterMut},
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
#[derive(Debug)]
pub struct BucketVec<T, C = DefaultConfig> {
    /// The number of elements stored in the bucket vector.
    len: usize,
    /// The entry vector.
    buckets: Vec<Bucket<T>>,
    /// The config phantom data.
    config: PhantomData<fn() -> C>,
}

impl<T, C> IntoIterator for BucketVec<T, C> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

impl<'a, T, C> IntoIterator for &'a BucketVec<T, C> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}

impl<'a, T, C> IntoIterator for &'a mut BucketVec<T, C> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        IterMut::new(self)
    }
}

impl<T, C> Clone for BucketVec<T, C>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            len: self.len(),
            buckets: self.buckets.clone(),
            config: Default::default(),
        }
    }
}

impl<T, C> PartialEq for BucketVec<T, C>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.iter().zip(other.iter()).all(|(lhs, rhs)| lhs == rhs)
    }
}

impl<T, C> Eq for BucketVec<T, C> where T: Eq {}

impl<T, C> core::cmp::PartialOrd for BucketVec<T, C>
where
    T: core::cmp::PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        for (lhs, rhs) in self.iter().zip(other.iter()) {
            match lhs.partial_cmp(rhs) {
                Some(core::cmp::Ordering::Equal) => (),
                non_eq => return non_eq,
            }
        }
        self.len().partial_cmp(&other.len())
    }
}

impl<T, C> core::cmp::Ord for BucketVec<T, C>
where
    T: core::cmp::Ord,
{
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        for (lhs, rhs) in self.iter().zip(other.iter()) {
            match lhs.cmp(rhs) {
                core::cmp::Ordering::Equal => (),
                non_eq => return non_eq,
            }
        }
        self.len().cmp(&other.len())
    }
}

impl<T, C> core::hash::Hash for BucketVec<T, C>
where
    T: core::hash::Hash,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.len().hash(state);
        for elem in self.iter() {
            elem.hash(state);
        }
    }
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
    pub(crate) fn new(index: usize, reference: &'a mut T) -> Self {
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

    /// Returns an iterator that yields exclusive reference to the elements of the bucket vector.
    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut::new(self)
    }

    /// Returns a shared reference to the first element of the bucket vector.
    pub fn first(&self) -> Option<&T> {
        if self.is_empty() {
            return None
        }
        Some(&self.buckets[0][0])
    }

    /// Returns an exclusive reference to the first element of the bucket vector.
    pub fn first_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            return None
        }
        Some(&mut self.buckets[0][0])
    }

    /// Returns a shared reference to the last element of the bucket vector.
    pub fn last(&self) -> Option<&T> {
        if self.is_empty() {
            return None
        }
        let len_buckets = self.buckets.len();
        let len_entries = self.buckets[len_buckets - 1].len();
        Some(&self.buckets[len_buckets - 1][len_entries - 1])
    }

    /// Returns an exclusive reference to the last element of the bucket vector.
    pub fn last_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            return None
        }
        let len_buckets = self.buckets.len();
        let len_entries = self.buckets[len_buckets - 1].len();
        Some(&mut self.buckets[len_buckets - 1][len_entries - 1])
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
        Some(config::bucket_entry_indices::<C>(index))
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

    /// Pushes a new bucket containing the new value onto the bucket vector.
    fn push_bucket(&mut self, new_value: T) {
        let len_buckets = self.buckets.len();
        let new_capacity = config::bucket_capacity::<C>(len_buckets);
        let mut new_bucket = Bucket::new(new_capacity);
        new_bucket.push(new_value);
        self.buckets.push(new_bucket);
        self.len += 1;
    }

    /// Pushes a new element onto the bucket vector.
    ///
    /// # Note
    ///
    /// This operation will never move other elements, reallocates or otherwise
    /// invalidate pointers of elements contained by the bucket vector.
    pub fn push(&mut self, new_value: T) {
        if let Some(bucket) = self.buckets.last_mut() {
            if bucket.len() < bucket.capacity() {
                bucket.push(new_value);
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
        let len_buckets = self.buckets.len();
        let len_entries = self.buckets[len_buckets - 1].len();
        Access::new(index, &mut self.buckets[len_buckets - 1][len_entries - 1])
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
