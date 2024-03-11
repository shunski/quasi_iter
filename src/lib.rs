pub mod merge;
pub mod dedup;
// pub mod splice;

/// The trait `UncheckedIterator` is just `Iterator` but the return value is forced to be 'Self::Item' instead of `Option<Self::Item>`. 
/// This means that the iterator must be stopped before it runs out of its elements.
/// The trait itself is NOT unsafe, because the unsafe method `next_unchecked` is the only method.
/// Note that this trait is different from `UncheckedIterator` in the standard library (privated trait in `core`) in the sense that `UncheckedIterator` in this crate
/// is supposed by an external condition. Hence, it does not require the struct to have the length information, i.e. the trusted `size_hint` is not required.
/// # Safety:
/// `next_unchecked` must be safe to call `self.trusted_len()` times.
pub trait UncheckedIterator {
    type Item;
    unsafe fn next_unchecked(&mut self) -> Self::Item;
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