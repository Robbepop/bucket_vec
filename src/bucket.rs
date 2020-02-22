
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

/// An fixed capacity bucket within the bucket vector.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Bucket<T> {
    /// The entries of this bucket.
    entries: Vec<T>,
}

impl<T> Bucket<T> {
    /// Creates a new emtpy bucket with a fixed capacity.
    ///
    /// # Note
    ///
    /// This does not allocate any heap memory.
    pub fn new(capacity: usize) -> Self {
        Self {
            entries: Vec::with_capacity(capacity),
        }
    }

    /// Returns the current length of the entry.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns the fixed capacity of the entry.
    pub fn capacity(&self) -> usize {
        self.entries.capacity()
    }

    /// Returns `true` if the entry is empty.
    #[allow(dead_code)]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a shared reference to the element at the given index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub fn get(&self, index: usize) -> Option<&T> {
        self.entries.get(index)
    }

    /// Returns an exclusive reference to the element at the given index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        self.entries.get_mut(index)
    }

    /// Pushes a new value into the fixed capacity entry.
    ///
    /// # Panics
    ///
    /// If the entry is already at its capacity.
    /// Note that this panic should never happen since the entry is only ever
    /// accessed by its outer bucket vector that checks before pushing.
    pub fn push(&mut self, new_value: T) {
        if self.len() == self.capacity() {
            panic!("entry is already filled to capacity")
        }
        self.entries.push(new_value);
    }

    /// Returns an iterator over the entries of the bucket.
    pub fn iter(&self) -> core::slice::Iter<T> {
        self.entries.iter()
    }
}

impl<T> core::ops::Index<usize> for Bucket<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("index out of bounds")
    }
}

impl<T> core::ops::IndexMut<usize> for Bucket<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("index out of bounds")
    }
}
