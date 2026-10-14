struct CoordinateCompression<T> {
    values: Vec<T>,
}

impl<T: Ord> CoordinateCompression<T> {
    fn new(mut values: Vec<T>) -> Self {
        values.sort_unstable();
        values.dedup();
        Self { values }
    }

    fn index(&self, value: &T) -> Option<usize> {
        self.values.binary_search(value).ok()
    }

    fn values(&self) -> &[T] {
        &self.values
    }

    fn len(&self) -> usize {
        self.values.len()
    }
}
