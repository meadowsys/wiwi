//! Chaining APIs for common Rust types
//!
//! Temp status: the structs, [`IntoChainer`] impls, constructor / conversion
//! functions, don't count in there

// TODO: remove these progress type comments when chainers are closer to done
// TODO: move these comments to own module and add basic description for each one
mod into_chainer;
pub use into_chainer::IntoChainer;
// pub mod traits;
mod with_vars;

mod vec;
/// status: significant functionality implemented, fully usable for most tasks
pub use vec::VecChain;
mod vec_ref;
/// status: not started
pub use vec_ref::VecRefChain;
mod vec_mut;
/// status: not started
pub use vec_mut::VecMutChain;

mod slice_box;
/// status: not started
pub use slice_box::SliceBoxChain;
mod slice_ref;
/// status: started a bit
pub use slice_ref::SliceRefChain;
mod slice_mut;
/// status: started a bit
pub use slice_mut::SliceMutChain;

mod array;
/// status: started a _tiny_ bit
pub use array::ArrayChain;
mod array_ref;
/// status: not started
pub use array_ref::ArrayRefChain;
mod array_mut;
/// status: not started
pub use array_mut::ArrayMutChain;

mod string;
/// status: a bit of basic functionality
pub use string::StringChain;
mod string_ref;
/// status: not started
pub use string_ref::StringRefChain;
mod string_mut;
/// status: not started
pub use string_mut::StringMutChain;
mod str_box;
/// status: not started
pub use str_box::StrBoxChain;
mod str_ref;
/// status: not started
pub use str_ref::StrRefChain;
mod str_mut;
/// status: not started
pub use str_mut::StrMutChain;

mod vec_deque;
/// status: not started
pub use vec_deque::VecDequeChain;
mod linked_list;
/// status: not started
pub use linked_list::LinkedListChain;

mod hash_map;
/// status: not started
pub use hash_map::HashMapChain;
mod hash_set;
/// status: not started
pub use hash_set::HashSetChain;

mod btree_map;
/// status: not started
pub use btree_map::BTreeMapChain;
mod btree_set;
/// status: not started
pub use btree_set::BTreeSetChain;

mod binary_heap;
/// status: started a bit
pub use binary_heap::BinaryHeapChain;

#[cfg(feature = "bitstream-unstable")]
mod bitstream;
#[cfg_attr(docsrs, doc(cfg(feature = "bitstream-unstable")))]
#[cfg(feature = "bitstream-unstable")]
/// status: need to implement the underlying bit stream encoder first, lol
pub use bitstream::BitstreamEncoderChain;
