pub mod merge;
pub mod dedup;
// pub mod splice;

/// The trait `UncheckedIterator` is just `Iterator` but the return value is forced to be 'Item'. This means that
/// the iterator must be stopped before the iterator run.
/// The trait itself is unsafe, because the safety of `next_unchecked` depends on `trusted_len`.
/// Note that this trait may be deprecated after the trait `std::iter::TrustedLen` becomes stable.
/// # Safety:
/// `next_unchecked` must be safe to call `self.trusted_len()` times.
pub unsafe trait UncheckedIterator {
    type Item;
    unsafe fn next_unchecked(&mut self) -> Self::Item;
    fn trusted_len(&self) -> usize;
}

pub mod prelude {
    pub use crate::merge::{
        Mergable,
        merge_dyn,
        merge_dyn_by,
        merge_dyn_by_key,
    };
    
    pub use crate::dedup::Dedupable;
}