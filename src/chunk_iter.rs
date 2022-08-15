use std::{iter::Peekable, ops::Range};

pub struct ChunkIterIndex<P, T, D, V>
where
    P: FnMut(&D) -> V,
    V: Eq,
    T: Iterator<Item = D>,
{
    data: Peekable<T>,
    index: usize,
    predicate: P,
}

impl<P, T, D, V> ChunkIterIndex<P, T, D, V>
where
    P: FnMut(&D) -> V,
    V: Eq,
    T: Iterator<Item = D>,
{
    pub fn new(data: T, predicate: P) -> Self {
        Self {
            data: data.peekable(),
            index: 0,
            predicate,
        }
    }
}

impl<P, T, D, V> Iterator for ChunkIterIndex<P, T, D, V>
where
    P: FnMut(&D) -> V,
    V: Eq,
    T: Iterator<Item = D>,
{
    type Item = (Range<usize>, V);

    fn next(&mut self) -> Option<Self::Item> {
        let start_index = self.index;
        let first = self.data.next()?;
        self.index += 1;

        while self
            .data
            .next_if(|elem| (self.predicate)(&first) == (self.predicate)(elem))
            .is_some()
        {
            self.index += 1;
        }

        Some((start_index..self.index, (self.predicate)(&first)))
    }
}
