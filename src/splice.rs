use std::cmp::Ordering;

pub struct Splice<I, J> {
    i: I,
    j: J,
}

pub struct SpliceBy<I, J, F> {
    i: I,
    j: J,
    cmp: F,
}


/// SpliceByKey is an iterator that acts like an intersection of the two input iterators. For the 'union' 
/// counterpart of this iterator, see "MergeNyKey".
/// Note that the caller has to make sure that the two input iterators are sorted by keys. Otherwise, 
/// SpliceByKey shows an undefined behavior, and SpliceByKey DOES NOT perform the internal check.

pub struct SpliceByKey<I, J, F> {
    i: I,
    j: J,
    key: F,
}

impl <I, J, T> Iterator for Splice<I, J> 
    where 
        I: Iterator<Item = T>,
        J: Iterator<Item = T>,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

impl <I, J, F, T> Iterator for SpliceBy<I, J, F> 
    where 
        I: Iterator<Item = T>,
        J: Iterator<Item = T>,
        F: FnMut(&T, &T) -> Ordering,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

impl <I, J, F, T, K> Iterator for SpliceByKey<I, J, F> 
    where
        I: Iterator<Item = T>,
        J: Iterator<Item = T>,
        F: FnMut(T) -> K,
        K: Ord,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next()
    }
}

pub trait SpliceByKeyAble<I, J, F> {
    fn splice_sorted_by_key(self) -> SpliceByKey<I, J, F>;
}
