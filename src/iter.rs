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

/// An iterator over the elements of a bucket vector.
#[derive(Debug)]
pub struct IterMut<'a, T> {
    /// Buckets iterator used by forward iteration.
    buckets: core::slice::IterMut<'a, Bucket<T>>,
    /// Front iterator for `next`.
    front_iter: Option<core::slice::IterMut<'a, T>>,
    /// Back iterator for `next_back`.
    back_iter: Option<core::slice::IterMut<'a, T>>,
    /// Number of elements that are to be yielded by the iterator.
    len: usize,
}

impl<'a, T> IterMut<'a, T> {
    /// Creates a new iterator over the bucket vector.
    pub fn new<C>(vec: &'a mut BucketVec<T, C>) -> Self {
        let len = vec.len();
        Self {
            buckets: vec.buckets.iter_mut(),
            front_iter: None,
            back_iter: None,
            len,
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut front_iter) = self.front_iter {
                if let front @ Some(_) = front_iter.next() {
                    self.len -= 1;
                    return front;
                }
            }
            match self.buckets.next() {
                None => {
                    self.len -= 1;
                    return self.back_iter.as_mut()?.next();
                }
                Some(inner) => self.front_iter = Some(inner.iter_mut()),
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<'a, T> DoubleEndedIterator for IterMut<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(ref mut back_iter) = self.back_iter {
                if let back @ Some(_) = back_iter.next_back() {
                    self.len -= 1;
                    return back;
                }
            }
            match self.buckets.next_back() {
                None => {
                    self.len -= 1;
                    return self.front_iter.as_mut()?.next();
                }
                Some(inner) => self.back_iter = Some(inner.iter_mut()),
            }
        }
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}
