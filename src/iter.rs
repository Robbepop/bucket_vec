use super::{Bucket, BucketVec};

/// An iterator over the elements of a bucket vector.
#[derive(Debug)]
pub struct Iter<'a, T> {
    /// Buckets iterator used by forward iteration.
    start_buckets: core::slice::Iter<'a, Bucket<T>>,
    /// Buckets iterator used by backward iteration.
    end_buckets: core::slice::Iter<'a, Bucket<T>>,
    /// Entries of the start bucket.
    start_entries: core::slice::Iter<'a, T>,
    /// Entries of the end bucket.
    end_entries: core::slice::Iter<'a, T>,
    /// Total iterated elements.
    total_iterated: usize,
    /// The total length of the iterated bucket vector.
    vec_len: usize,
}

impl<'a, T> Iter<'a, T> {
    /// Creates a new iterator over the bucket vector.
    pub fn new<C>(vec: &'a BucketVec<T, C>) -> Self {
        if vec.len() == 0 {
            Self {
                start_buckets: [].iter(),
                end_buckets: [].iter(),
                start_entries: [].iter(),
                end_entries: [].iter(),
                total_iterated: 0,
                vec_len: 0,
            }
        } else {
            Self {
                start_buckets: vec.buckets[1..].into_iter(),
                end_buckets: vec.buckets[..vec.buckets.len() - 1].into_iter(),
                start_entries: vec.buckets[0].iter(),
                end_entries: vec.buckets[vec.buckets.len() - 1].iter(),
                total_iterated: 0,
                vec_len: vec.len(),
            }
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.total_iterated == self.vec_len {
            return None;
        }
        self.total_iterated += 1;
        if let Some(start) = self.start_entries.next() {
            Some(start)
        } else {
            self.start_entries = self.start_buckets.next().unwrap().iter();
            Some(self.start_entries.next().unwrap())
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.total_iterated == self.vec_len {
            return None;
        }
        self.total_iterated += 1;
        if let Some(end) = self.end_entries.next_back() {
            Some(end)
        } else {
            self.end_entries = self.end_buckets.next_back().unwrap().iter();
            Some(self.end_entries.next_back().unwrap())
        }
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.vec_len - self.total_iterated
    }
}
