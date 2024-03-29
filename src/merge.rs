use std::cmp::Ordering;
use std::mem;

pub struct Merge<I,J,O> {
    iter1: I,
    iter2: J,
    curr: Option<O>,
    is_iter1_smaller: bool,
}

pub struct MergeBy<I, J, F, O> {
    iter1: I,
    iter2: J,
    comparator: F,
    curr: Option<O>,
    is_iter1_smaller: bool,
}

pub struct MergeByKey<I, J, F, O> {
    iter1: I,
    iter2: J,
    key: F,
    curr: Option<O>,
    is_iter1_smaller: bool,
}


impl<I,J,O> Iterator for Merge<I,J,O> 
    where 
        I: Iterator<Item = O>,
        J: Iterator<Item = O>,
        O: Ord
{
    type Item = O;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = &self.curr {
            let mut next = if self.is_iter1_smaller {
                self.iter1.next()
            } else {
                self.iter2.next()
            };

            if let Some(y) = &next {
                if Ordering::Less == x.cmp(y) {
                    mem::swap(&mut self.curr, &mut next);
                    self.is_iter1_smaller = !self.is_iter1_smaller;
                }
                next
            } else {
                let mut next = None;
                mem::swap(&mut self.curr, &mut next);
                self.is_iter1_smaller = !self.is_iter1_smaller;
                next
            }
        } else {
            if self.is_iter1_smaller {
                self.iter1.next()
            } else {
                self.iter2.next()
            }
        }
    }
}

impl<I,J,F,O> Iterator for MergeBy<I,J,F,O> 
    where 
        I: Iterator<Item = O>,
        J: Iterator<Item = O>,
        F: FnMut(&O,&O) -> Ordering,
{
    type Item = O;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = &self.curr {
            let mut next = if self.is_iter1_smaller {
                self.iter1.next()
            } else {
                self.iter2.next()
            };

            if let Some(y) = &next {
                if Ordering::Less == (self.comparator)(x, y) {
                    mem::swap(&mut self.curr, &mut next);
                    self.is_iter1_smaller = !self.is_iter1_smaller;
                }
                next
            } else {
                let mut next = None;
                mem::swap(&mut self.curr, &mut next);
                self.is_iter1_smaller = !self.is_iter1_smaller;
                next
            }
        } else {
            if self.is_iter1_smaller {
                self.iter1.next()
            } else {
                self.iter2.next()
            }
        }
    }
}


impl<I,J,F,O,K> Iterator for MergeByKey<I,J,F,O> 
    where 
        I: Iterator<Item = O>,
        J: Iterator<Item = O>,
        F: FnMut(&O) -> K,
        K: Ord,
{
    type Item = O;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(x) = &self.curr {
            let mut next = if self.is_iter1_smaller {
                self.iter1.next()
            } else {
                self.iter2.next()
            };

            if let Some(y) = &next {
                if Ordering::Less == (self.key)(x).cmp(&(self.key)(y)) {
                    mem::swap(&mut self.curr, &mut next);
                    self.is_iter1_smaller = !self.is_iter1_smaller;
                }
                next
            } else {
                let mut next = None;
                mem::swap(&mut self.curr, &mut next);
                self.is_iter1_smaller = !self.is_iter1_smaller;
                next
            }
        } else {
            if self.is_iter1_smaller {
                self.iter1.next()
            } else {
                self.iter2.next()
            }
        }
    }
}

pub trait Mergable<J, O>: Iterator + Sized 
    where J: Iterator<Item = O>,
{
    fn merge<L>(self, other: L) -> Merge<Self,J,O>
        where L: IntoIterator<IntoIter = J>;
        
    fn merge_by<F,L>(self, other: L, comparator: F) -> MergeBy<Self,J,F,O>
        where
            F: FnMut(&O,&O)->Ordering, 
            L: IntoIterator<IntoIter = J>;

    fn merge_by_key<F,L,K>(self, other: L, key: F) -> MergeByKey<Self,J,F,O>
        where 
            F: FnMut(&O)->K, 
            L: IntoIterator<IntoIter = J>;
}

impl<I,J,O> Mergable<J,O> for I 
    where
        I: Iterator<Item = O>,
        J: Iterator<Item = O>,
        O: Ord,
{
    fn merge<L>(self, other: L) -> Merge<Self,J,O> 
        where L: IntoIterator<IntoIter = J>
    {
        let iter1 = self;
        let mut iter2 = other.into_iter();
        let curr = iter2.next();
        Merge{ iter1, iter2, curr, is_iter1_smaller: true}
    }

    fn merge_by<F, L>(self, other: L, comparator: F) -> MergeBy<Self, J, F, O> 
        where 
            F: FnMut(&O,&O)->Ordering, 
            L: IntoIterator<IntoIter = J>,
    {
        let iter1 = self;
        let mut iter2 = other.into_iter();
        let curr = iter2.next();
        MergeBy{ iter1, iter2, comparator, curr, is_iter1_smaller: true}
    }

    fn merge_by_key<F,L,K>(self, other: L, key: F) -> MergeByKey<Self,J,F,O>
            where 
                F: FnMut(&O)->K, 
                L: IntoIterator<IntoIter = J> 
    {
        let iter1 = self;
        let mut iter2 = other.into_iter();
        let curr = iter2.next();
        MergeByKey{ iter1, iter2, key, curr, is_iter1_smaller: true}
    }
}

pub struct MergeDyn<O> {
    data: Vec<(O, Box<dyn Iterator<Item = O>>)>,
}

pub struct MergeDynBy<O, F> {
    data: Vec<(O, Box<dyn Iterator<Item = O>>)>,
    comparator: F,
}

pub struct MergeDynByKey<O, F> {
    data: Vec<(O, Box<dyn Iterator<Item = O>>)>,
    key: F,
}

impl<O> Iterator for MergeDyn<O> 
    where O: Ord
{
    type Item = O;
    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            None
        } else {
            let next = self.data.first_mut().unwrap().1.next();
            
            if let Some(mut item) = next {
                // if the itetator 'self.data[0].1' has some next value, then store it in
                // 'self.data[0].0' and return the old value in there.
                mem::swap(&mut item, &mut self.data.first_mut().unwrap().0);

                // Now sort the self.data.
                let mut i = 0;
                while i<self.data.len()-1 && self.data[i].0 > self.data[i+1].0 {
                    // Note that this function does not panic at 'self.data.len()-1' because 'self.data' is not empty.
                    self.data.swap(i, i+1);
                    i+=1;
                }


                Some(item)
            } else {
                // if the iterator 'self.data[0].1' is done, then remove the iterator
                // and return the value
                Some( self.data.remove(0).0 )
            }
        }
    }
}


impl<O, F> Iterator for MergeDynBy<O, F> 
    where 
        O: Ord,
        F: FnMut(&O, &O) -> Ordering
{
    type Item = O;
    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            None
        } else {
            let next = self.data.first_mut().unwrap().1.next();
            
            if let Some(mut item) = next {
                // if the itetator 'self.data[0].1' has some next value, then store it in
                // 'self.data[0].0' and return the old value in there.
                mem::swap(&mut item, &mut self.data.first_mut().unwrap().0);

                // Now sort the self.data.
                let mut i = 0;
                while i<self.data.len()-1 && (self.comparator)( &self.data[i].0, &self.data[i+1].0 ).is_gt() {
                    // Note that this function does not panic at 'self.data.len()-1' because 'self.data' is not empty.
                    self.data.swap(i, i+1);
                    i+=1;
                }


                Some(item)
            } else {
                // if the iterator 'self.data[0].1' is done, then remove the iterator
                // and return the value
                Some( self.data.remove(0).0 )
            }
        }
    }
}

impl<O, F, K> Iterator for MergeDynByKey<O, F> 
    where 
        K: Ord,
        F: FnMut(&O) -> K
{
    type Item = O;
    fn next(&mut self) -> Option<Self::Item> {
        if self.data.is_empty() {
            None
        } else {
            let next = self.data.first_mut().unwrap().1.next();
            
            if let Some(mut item) = next {
                // if the itetator 'self.data[0].1' has some next value, then store it in
                // 'self.data[0].0' and return the old value in there.
                mem::swap(&mut item, &mut self.data.first_mut().unwrap().0);

                // Now sort the self.data.
                let mut i = 0;
                while i<self.data.len()-1 && (self.key)( &self.data[i].0 ).cmp( &(self.key)(&self.data[i+1].0 )).is_gt() {
                    // Note that this function does not panic at 'self.data.len()-1' because 'self.data' is not empty.
                    self.data.swap(i, i+1);
                    i+=1;
                }


                Some(item)
            } else {
                // if the iterator 'self.data[0].1' is done, then remove the iterator
                // and return the value
                Some( self.data.remove(0).0 )
            }
        }
    }
}

pub fn merge_dyn<T, O>(data: T) -> MergeDyn<O> 
    where 
        T: IntoIterator<Item = Box<dyn Iterator<Item = O>>>,
        O: Ord,
{
    let mut data = data.into_iter()
        .map(|mut x| (x.next(), x))
        .filter(|(item, _)| item != &None )
        .map(|(item, iter)| (item.unwrap(), iter))
        .collect::<Vec<_>>();

    // Sort 'data'
    data.sort_by(|(x,_), (y,_)| x.cmp(y) );
    
    MergeDyn{data}
}


pub fn merge_dyn_by<T, O, F>(data: T, mut comparator: F) -> MergeDynBy<O, F> 
    where 
        T: IntoIterator<Item = Box<dyn Iterator<Item = O>>>,
        O: Ord,
        F: FnMut(&O, &O) -> Ordering
{
    let mut data = data.into_iter()
        .map(|mut x| (x.next(), x))
        .filter(|(item, _)| item != &None )
        .map(|(item, iter)| (item.unwrap(), iter))
        .collect::<Vec<_>>();

    // Sort 'data'
    data.sort_by(|(x,_), (y,_)| comparator(x, y) );
    
    MergeDynBy{data, comparator}
}

pub fn merge_dyn_by_key<T, O, F, K>(data: T, mut key: F) -> MergeDynByKey<O, F> 
    where 
        T: IntoIterator<Item = Box<dyn Iterator<Item = O>>>,
        O: PartialEq,
        K: Ord,
        F: FnMut(&O) -> K
{
    let mut data = data.into_iter()
        .map(|mut x| (x.next(), x))
        .filter(|(item, _)| item != &None )
        .map(|(item, iter)| (item.unwrap(), iter))
        .collect::<Vec<_>>();

    // Sort 'data'
    data.sort_by_key(|(x,_)| key(x) );
    
    MergeDynByKey{data, key}
}


