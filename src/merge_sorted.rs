use std::cmp::Ordering;
use std::mem;

pub struct MergeSorted<I,J,O> {
    iter1: I,
    iter2: J,
    curr: Option<O>,
    is_iter1_smaller: bool,
}

pub struct MergeSortedBy<I, J, F, O> {
    iter1: I,
    iter2: J,
    comparator: F,
    curr: Option<O>,
    is_iter1_smaller: bool,
}

pub struct MergeSortedByKey<I, J, F, O> {
    iter1: I,
    iter2: J,
    key: F,
    curr: Option<O>,
    is_iter1_smaller: bool,
}


impl<I,J,O> Iterator for MergeSorted<I,J,O> 
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

impl<I,J,F,O> Iterator for MergeSortedBy<I,J,F,O> 
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


impl<I,J,F,O,K> Iterator for MergeSortedByKey<I,J,F,O> 
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

pub fn merge_sorted<I,J,O>(iter1: impl IntoIterator<IntoIter = I>, iter2: impl IntoIterator<IntoIter = J>) -> MergeSorted<I,J,O> 
    where 
        I: Iterator<Item = O>,
        J: Iterator<Item = O>,
        O: Ord
{
    let iter1 = iter1.into_iter();
    let mut iter2 = iter2.into_iter();
    let curr = iter2.next();
    MergeSorted{ iter1, iter2, curr, is_iter1_smaller: true}
}

pub fn merge_sorted_by<I,J,F,O>(iter1: impl IntoIterator<IntoIter = I>, iter2: impl IntoIterator<IntoIter = J>, comparator: F) -> MergeSortedBy<I,J,F,O> 
    where 
        I: Iterator<Item = O>,
        J: Iterator<Item = O>,
        F: FnMut(&O,&O) -> Ordering,
{
    let iter1 = iter1.into_iter();
    let mut iter2 = iter2.into_iter();
    let curr = iter2.next();
    MergeSortedBy{ iter1, iter2, comparator, curr, is_iter1_smaller: true}
}

pub fn merge_sorted_by_key<I,J,F,O,K>(iter1: impl IntoIterator<IntoIter = I>, iter2: impl IntoIterator<IntoIter = J>, key: F) -> MergeSortedByKey<I,J,F,O> 
    where 
        I: Iterator<Item = O>,
        J: Iterator<Item = O>,
        F: FnMut(&O) -> K,
        K: Ord
{
    let iter1 = iter1.into_iter();
    let mut iter2 = iter2.into_iter();
    let curr = iter2.next();
    MergeSortedByKey{ iter1, iter2, key, curr, is_iter1_smaller: true}
}

pub struct MergeSortedDyn<O> {
    data: Vec<(O, Box<dyn Iterator<Item = O>>)>,
}

impl<O> Iterator for MergeSortedDyn<O> 
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

pub fn merge_sorted_dyn<T, O>(data: T) -> MergeSortedDyn<O> 
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
    
    MergeSortedDyn{data}
}

impl<O> MergeSortedDyn<O> 
    where O: Ord
{
    pub fn merge(self, other: Self) -> Self {
        let data = merge_sorted_by(
            self.data.into_iter(),
            other.data.into_iter(),
            |(v1,_),(v2,_)| v1.cmp(v2)
        ).collect();
        Self { data }
    } 
}


