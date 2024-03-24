/// The trait `SizedIterator` is the iterator with finite known size (or length).
/// The length information of the iterator can be obtained from the method `len`. 
/// Note that `next_unchecked` is safe to call `len` times IN ITS ENTIRE LIFETIME, and the implementor
/// must make sure that 'len' remains constant throughtout its lifetime. This implementation allows faster
/// implementation of the augmented iterator structs (such as `Zip`) or the consumer traits (such as `Fold`).
/// All the other trait methods and the consumer traits trust the size information 
/// from `len` and consume the iterator based on it. Hence all the consumer methods are safe to call given that
/// the implementation of `len` is right, thus the trait itself is marked unsafe.
/// # Safety (on implementing the trait):
/// `next_unchecked` must be safe to call 'len()` times THROUGHOUT ITS LIFETIME. This means that `len`
/// must remain constant throughout its lifetime.
/// # Safety (on `next_unchecked`)
/// Do not call `next_unchecked` more than 'len()' times in its lifetime. Do not call any of the other methods 
/// after `next_unchecked`.
pub unsafe trait SizedIterator {
    type Item;
    unsafe fn next_unchecked(&mut self) -> Self::Item;

    fn len(&self) -> usize;

    fn zip<T>(self, other: T) -> Zip<Self, T> 
        where 
            Self: Sized,
            T: SizedIterator,
    {
        Zip::new(self, other)
    }

    fn fold<B, F>(mut self, init: B, mut f: F) -> B
        where
            Self: Sized,
            F: FnMut(B, Self::Item) -> B
    {
        let mut accum = init;
        for _ in 0..self.len() {
            // Safety:
            // `next_unchecked` is called `self.len()` times.
            unsafe {
                accum = f(accum, self.next_unchecked());
            }
        }
        accum
    }
}


pub struct Zip<T, S> {
    t: T,
    s: S,
}

impl<T, S> Zip<T, S>
    where 
        T: SizedIterator, 
        S: SizedIterator,
{
    fn new(t: T, s: S) -> Self {
        Self { t, s }
    }
}

unsafe impl<T,S> SizedIterator for Zip<T, S> 
    where 
        T: SizedIterator,
        S: SizedIterator
{
    type Item = (T::Item, S::Item);

    #[inline]
    unsafe fn next_unchecked(&mut self) -> Self::Item {
        unsafe{ 
            (self.t.next_unchecked(), self.s.next_unchecked())
        }
    }

    // Safety:
    // Both `self.t.unchecked()` and `self.s.unchecked()` are safe to call at least `t.len()` and `s.len()` times.
    #[inline(always)]
    fn len(&self) -> usize {
        std::cmp::min(self.t.len(), self.s.len())
    }
}

