pub mod merge;
pub mod dedup;
pub mod splice;

pub mod prelude {
    pub use crate::merge::{
        Mergable,
        merge_dyn,
        merge_dyn_by,
        merge_dyn_by_key,
    };
    
    pub use crate::dedup::Dedupable;
}