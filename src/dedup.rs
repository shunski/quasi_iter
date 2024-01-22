use std::mem;

pub struct Dedup<T, I> {
    prev: Option<T>,
    iter: I,
}


impl<T, I> Iterator for Dedup<T, I> 
    where 
        I: Iterator<Item = T>,
        T: PartialEq,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        if self.prev.is_none() {
            return None;
        }

        while let Some(val) = self.iter.next() {
            if self.prev.as_ref().unwrap() != &val {
                let mut out = Some(val);
                mem::swap( &mut self.prev, &mut out );
                return out;
            }
        }
        let mut out = None;
        mem::swap(&mut self.prev, &mut out);
        out
    }
}

pub trait Dedupable<T,I> {
    fn dedup(self) -> Dedup<T,I>;
}

impl<T, I> Dedupable<T,I> for I
    where I: Iterator<Item = T>,
{
    fn dedup(mut self) -> Dedup<T,I> {

        // let 'prev' be the first value if the iterator returns 'Some()'.
        // Otherwise, we let it be 'None'.
        let next = self.next();
        let prev = if next.is_some() {
            next
        } else  {
            None
        };

        Dedup { prev, iter: self }
    }
}