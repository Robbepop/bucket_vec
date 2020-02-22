use super::{Bucket, BucketVec};

/// An iterator over the elements of a bucket vector.
#[derive(Debug)]
pub struct Iter<'a, T> {
    /// Buckets iterator.
    buckets: core::slice::Iter<'a, Bucket<T>>,
    /// Front iterator for `next`.
    front_iter: Option<core::slice::Iter<'a, T>>,
    /// Back iterator for `next_back`.
    back_iter: Option<core::slice::Iter<'a, T>>,
    /// Number of elements that are to be yielded by the iterator.
    len: usize,
}

impl<'a, T> Iter<'a, T> {
    /// Creates a new iterator over the bucket vector.
    pub fn new<C>(vec: &'a BucketVec<T, C>) -> Self {
        Self {
            buckets: vec.buckets.iter(),
            front_iter: None,
            back_iter: None,
            len: vec.len(),
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

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
                Some(bucket) => self.front_iter = Some(bucket.iter()),
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<'a, T> DoubleEndedIterator for Iter<'a, T> {
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
                    return self.front_iter.as_mut()?.next_back();
                }
                Some(bucket) => self.back_iter = Some(bucket.iter()),
            }
        }
    }
}

impl<'a, T> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.len
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
                Some(bucket) => self.front_iter = Some(bucket.iter_mut()),
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
                    return self.front_iter.as_mut()?.next_back();
                }
                Some(bucket) => self.back_iter = Some(bucket.iter_mut()),
            }
        }
    }
}

impl<'a, T> ExactSizeIterator for IterMut<'a, T> {
    fn len(&self) -> usize {
        self.len
    }
}
