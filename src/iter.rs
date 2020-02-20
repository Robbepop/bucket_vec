use super::BucketVec;

/// An iterator over the elements of a bucket vector.
#[derive(Debug)]
pub struct Iter<'a, T, C> {
    /// The iterated over bucket vector.
    vec: &'a BucketVec<T, C>,
    /// The index of the starting entry.
    start_entry: usize,
    /// The index of the ending entry.
    end_entry: usize,
    /// The index of the start element of the starting entry.
    start: usize,
    /// The index of the end element of the ending entry.
    end: usize,
    /// Total iterated elements.
    total_iterated: usize,
}

impl<'a, T, C> Iter<'a, T, C> {
    /// Creates a new iterator over the bucket vector.
    pub fn new(vec: &'a BucketVec<T, C>) -> Self {
        Self {
            vec,
            start_entry: 0,
            end_entry: vec.buckets.len(),
            start: 0,
            end: vec.buckets.last().map(|entry| entry.len()).unwrap_or(0),
            total_iterated: 0,
        }
    }
}

impl<'a, T, C> Iterator for Iter<'a, T, C> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.total_iterated == self.vec.len() {
            return None;
        }
        let next = &self.vec.buckets[self.start_entry][self.start];
        self.start += 1;
        if self.start == self.vec.buckets[self.start_entry].capacity() {
            self.start = 0;
            self.start_entry += 1;
        }
        self.total_iterated += 1;
        Some(next)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }
}

impl<'a, T, C> DoubleEndedIterator for Iter<'a, T, C> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.total_iterated == self.vec.len() {
            return None;
        }
        if self.end == 0 {
            self.end_entry = self.end_entry.saturating_sub(1);
            self.end = self.vec.buckets[self.end_entry.saturating_sub(1)].len();
        }
        self.end = self.end.saturating_sub(1);
        self.total_iterated += 1;
        Some(&self.vec.buckets[self.end_entry.saturating_sub(1)][self.end])
    }
}

impl<'a, T, C> ExactSizeIterator for Iter<'a, T, C> {
    fn len(&self) -> usize {
        self.vec.len() - self.total_iterated
    }
}

/// An iterator over the elements of a bucket vector.
///
/// # Note
///
/// Cannot implement `Iterator` trait without [generic associated types]
/// (https://www.google.com/search?hl=en&q=rustlang%20GAT).
#[derive(Debug)]
pub struct IterMut<'a, T: 'a, C> {
    /// The iterated over bucket vector.
    vec: &'a mut BucketVec<T, C>,
    /// The index of the starting entry.
    start_entry: usize,
    /// The index of the ending entry.
    end_entry: usize,
    /// The index of the start element of the starting entry.
    start: usize,
    /// The index of the end element of the ending entry.
    end: usize,
    /// Total iterated elements.
    total_iterated: usize,
}

impl<'a, T, C> IterMut<'a, T, C> {
    /// Creates a new iterator over the bucket vector.
    pub fn new(vec: &'a mut BucketVec<T, C>) -> Self {
        let len_buckets = vec.buckets.len();
        let last_bucket_capacity = vec.buckets.last().map(|entry| entry.len()).unwrap_or(0);
        Self {
            vec,
            start_entry: 0,
            end_entry: len_buckets,
            start: 0,
            end: last_bucket_capacity,
            total_iterated: 0,
        }
    }

    /// Yields the next front element of the iterator.
    pub fn next<'b>(&'b mut self) -> Option<&'a mut T>
    where
        'b: 'a,
    {
        if self.total_iterated == self.vec.len() {
            return None;
        }
        let start_capacity = self.vec.buckets[self.start_entry].capacity();
        let next = &mut self.vec.buckets[self.start_entry][self.start];
        self.start += 1;
        if self.start == start_capacity {
            self.start = 0;
            self.start_entry += 1;
        }
        self.total_iterated += 1;
        Some(next)
    }

    /// Yields the next back element of the iterator.
    pub fn next_back<'b>(&'b mut self) -> Option<&'a mut T>
    where
        'b: 'a,
    {
        if self.total_iterated == self.vec.len() {
            return None;
        }
        if self.end == 0 {
            self.end_entry = self.end_entry.saturating_sub(1);
            self.end = self.vec.buckets[self.end_entry.saturating_sub(1)].len();
        }
        self.end = self.end.saturating_sub(1);
        self.total_iterated += 1;
        Some(&mut self.vec.buckets[self.end_entry.saturating_sub(1)][self.end])
    }

    /// Returns the number of to-be-yielded elements of the iterator.
    pub fn len(&self) -> usize {
        self.vec.len() - self.total_iterated
    }
}
