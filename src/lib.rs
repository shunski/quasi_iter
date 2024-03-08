pub mod merge;
pub mod dedup;
// pub mod splice;

/// The trait `UncheckedIterator` is just `Iterator` but the return value is forced to be 'Item'.
/// 'next_unchecked()' thus should be unsafe in general.
pub trait UncheckedIterator {
    type Item;
    fn next_unchecked(&mut self) -> Self::Item;
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