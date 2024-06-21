pub mod node;
pub mod tree;
pub mod taxa;
pub mod iter;

pub mod prelude{
    #[doc(no_inline)]
    pub use crate::tree::simple_rtree::*;
    #[doc(no_inline)]
    pub use crate::node::simple_rnode::*;
    #[doc(no_inline)]
    pub use crate::iter::node_iter::*;
}