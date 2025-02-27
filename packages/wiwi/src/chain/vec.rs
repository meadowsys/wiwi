use crate::prelude::*;
use super::{ chain_fn, AsChainInner, ChainInner as _, OutputStorage };

super::decl_chain! {
	generics_decl: [T]
	generics_decl_struct_def: [T]
	generics: [T]
	chain: VecChain
	inner: Vec<T>
}

/// Creates a [`VecChain`] containing the arguments
///
/// Usage is same as [`vec!`], except it returns [`VecChain`] instead of [`Vec`].
///
/// # Examples
///
/// ```
/// # use wiwi::chain::{ VecChain, ChainInner, vec_chain };
/// let chain = vec![0u8; 32].into_chain();
/// let chain = vec_chain![0u8; 32];
/// ```
#[macro_export]
macro_rules! vec_chain {
	[$($tt:tt)*] => { $crate::chain::VecChain::from(vec![$($tt)*]) }
}
pub use vec_chain;

impl<T> VecChain<T> {
	/// Creates a new vector chain without allocating any capacity
	///
	/// It will not allocate until it needs to, either by pushing an element,
	/// calling the [`reserve`](Self::reserve) function to explicitly request
	/// allocation, or something else.
	///
	/// # Examples
	///
	/// ```
	/// # use wiwi::chain::VecChain;
	/// // a chain thingie! yay!...
	/// let chain = VecChain::new();
	/// # let chain: VecChain<String> = chain;
	/// ```
	#[inline]
	pub fn new() -> Self {
		Vec::new().into_chain()
	}

	/// # Safety
	///
	/// You must uphold all safety requirements that [`Vec::from_raw_parts`] has.
	#[inline]
	pub unsafe fn from_raw_parts(ptr: *mut T, length: usize, capacity: usize) -> Self {
		// SAFETY: caller promises to uphold safety requirements of `Vec::from_raw_parts`
		let vec = unsafe { Vec::from_raw_parts(ptr, length, capacity) };
		vec.into_chain()
	}

	/// Creates a new vec chain, and preallocate some memory
	///
	/// The amount of memory allocated will be _at least_ enough to hold `capacity`
	/// elements without reallocating. No allocation will happen if the provided
	/// capacity is zero, or if `T` is a ZST.
	///
	/// There is NO GUARANTEE that this function will allocate an exact amount
	/// of memory, so do not rely on this for soundness. If knowing the actual
	/// allocated capacity is important, always do so using the
	/// [`capacity`](Self::capacity) function.
	///
	/// If the element type (ie. `T`) is a ZST, the vec chain will never
	/// allocate, and will always have a capacity of `usize::MAX` bytes.
	///
	/// # Panics
	///
	/// Panics if the new capacity exceeds `isize::MAX` _bytes_ (not elements,
	/// bytes). This is the same behaviour of [`Vec::with_capacity`].
	///
	/// # Examples
	///
	/// ```
	/// # use wiwi::chain::VecChain;
	/// # let mut len = 0;
	/// # let mut initial_capacity = 0;
	/// # let mut capacity = 0;
	/// let chain = VecChain::new_with_capacity(10)
	///    // chaining methods to get the len and capacity of the vec chain
	///    .len(&mut len)
	///    .capacity(&mut initial_capacity);
	///
	/// // The vector chain contains zero elements, and at least room for 10 more
	/// assert_eq!(len, 0);
	/// assert!(initial_capacity >= 10);
	///
	/// // These are all done without reallocating
	/// let chain = (0..10)
	///    .fold(chain, |chain, i| chain.push(i))
	///    .len(&mut len)
	///    .capacity(&mut capacity);
	///
	/// assert_eq!(len, 10);
	/// assert_eq!(capacity, initial_capacity);
	///
	/// // Now however, pushing another element can make the vector reallocate
	/// let chain = chain
	///    .push(11)
	///    .len(&mut len)
	///    .capacity(&mut capacity);
	///
	/// assert_eq!(len, 11);
	/// assert!(capacity >= 11);
	///
	/// # let mut capacity1 = 0;
	/// # let mut capacity2 = 0;
	/// // ZSTs never allocate and always have a capacity of `usize::MAX`
	/// let chain1 = VecChain::<()>::new()
	///    .capacity(&mut capacity1);
	/// let chain2 = VecChain::<()>::new_with_capacity(10)
	///    .capacity(&mut capacity2);
	///
	/// assert_eq!(capacity1, usize::MAX);
	/// assert_eq!(capacity2, usize::MAX);
	/// ```
	#[inline]
	pub fn new_with_capacity(capacity: usize) -> Self {
		Vec::with_capacity(capacity).into_chain()
	}
}

impl<T> VecChain<T> {
	chain_fn! {
		/// Takes and moves all elements from another `Vec` or `VecChain`
		/// into `self`, leaving it empty.
		///
		/// # Examples
		///
		/// TODO
		append[I](inner, other: &mut I) where {
			I: AsChainInner<Vec<T>>
		} => inner.append(other.as_inner_mut())
	}

	chain_fn! {
		binary_search[O](inner, x: &T, out: O) where {
			T: Ord,
			O: OutputStorage<Result<usize, usize>>
		} => {
			// SAFETY: we always write once to `out`
			unsafe { out.store(inner.binary_search(x)) }
		}
	}

	chain_fn! {
		binary_search_by[O, F](inner, f: F, out: &mut Result<usize, usize>) where {
			F: FnMut(&T) -> cmp::Ordering,
			O: OutputStorage<Result<usize, usize>>
		} => {
			// SAFETY: we always write once to `out`
			unsafe { out.store(inner.binary_search_by(f)) }
		}
	}

	chain_fn! {
		binary_search_by_key[B, O, F](inner, b: &B, f: F, out: O) where {
			B: Ord,
			F: FnMut(&T) -> B,
			O: OutputStorage<Result<usize, usize>>
		} => {
			// SAFETY: we always write once to `out`
			unsafe { out.store(inner.binary_search_by_key(b, f)) }
		}
	}

	chain_fn! {
		capacity[O](inner, out: O) where {
			O: OutputStorage<usize>
		} => {
			// SAFETY: we always write once to `out`
			unsafe { out.store(inner.capacity()) }
		}
	}

	chain_fn! {
		clear(inner)
			=> inner.clear()
	}

	chain_fn! {
		clone_from_slice(inner, src: &[T]) where {
			T: Clone
		} => inner.clone_from_slice(src)
	}

	chain_fn! {
		copy_from_slice(inner, src: &[T]) where {
			T: Copy
		} => inner.copy_from_slice(src)
	}

	chain_fn! {
		contains[O](inner, x: &T, out: O) where {
			T: PartialEq,
			O: OutputStorage<bool>
		} => {
			// SAFETY: we always write once to `out`
			unsafe { out.store(inner.contains(x)) }
		}
	}

	chain_fn! {
		dedup(inner) where {
			T: PartialOrd
		} => inner.dedup()
	}

	chain_fn! {
		dedup_by[F](inner, same_bucket: F) where {
			F: FnMut(&mut T, &mut T) -> bool
		} => inner.dedup_by(same_bucket)
	}

	chain_fn! {
		dedup_by_key[K, F](inner, key: F) where {
			F: FnMut(&mut T) -> K,
			K: PartialEq
		} => inner.dedup_by_key(key)
	}

	chain_fn! {
		ends_with[O](inner, needle: &[T], out: O) where {
			T: PartialEq,
			O: OutputStorage<bool>
		} => {
			// SAFETY: we always write once to `out`
			unsafe { out.store(inner.ends_with(needle)) }
		}
	}

	chain_fn! {
		fill(inner, value: T) where {
			T: Clone
		} => inner.fill(value)
	}

	chain_fn! {
		fill_with[F](inner, f: F) where {
			F: FnMut() -> T
		} => inner.fill_with(f)
	}

	chain_fn! {
		insert(inner, index: usize, element: T)
			=> inner.insert(index, element)
	}

	chain_fn! {
		len[O](inner, out: O) where {
			O: OutputStorage<usize>
		} => {
			// SAFETY: we always write once to `out`
			unsafe { out.store(inner.len()) }
		}
	}

	chain_fn! {
		push(inner, value: T)
			=> inner.push(value)
	}

	chain_fn! {
		remove[O](inner, index: usize, out: O) where {
			O: OutputStorage<T>
		} => {
			// SAFETY: we always write once to `out`
			unsafe { out.store(inner.remove(index)) }
		}
	}

	chain_fn! {
		reserve(inner, additional: usize)
			=> inner.reserve(additional)
	}

	chain_fn! {
		reserve_exact(inner, additional: usize)
			=> inner.reserve_exact(additional)
	}

	chain_fn! {
		/// # Safety
		///
		/// `new_len` must be less than or equal to `capacity`, and
		/// the first `new_len` elements must be initialised/
		unsafe set_len(inner, new_len: usize)
			// SAFETY: caller promises that `new_len <= capacity` and
			// `..new_len` elements are initialised
			=> unsafe { inner.set_len(new_len) }
	}

	chain_fn! {
		sort(inner) where {
			T: Ord
		} => inner.sort()
	}

	chain_fn! {
		sort_by[F](inner, compare: F) where {
			F: FnMut(&T, &T) -> cmp::Ordering
		} => inner.sort_by(compare)
	}
}

/*
Methods
allocator
as_mut_ptr
as_mut_slice
as_ptr
as_slice
capacity
clear
dedup
dedup_by
dedup_by_key
drain
extend_from_slice
extend_from_within
extract_if
from_raw_parts
from_raw_parts_in
insert
into_boxed_slice
into_flattened
into_raw_parts
into_raw_parts_with_alloc
is_empty
leak
len
new
new_in
pop
pop_if
push
push_within_capacity
remove
reserve
reserve_exact
resize
resize_with
retain
retain_mut
set_len
shrink_to
shrink_to_fit
spare_capacity_mut
splice
split_at_spare_mut
split_off
swap_remove
truncate
try_reserve
try_reserve_exact
try_with_capacity
try_with_capacity_in
with_capacity
with_capacity_in
Methods from Deref<Target=[T]>
align_to
align_to_mut
array_chunks
array_chunks_mut
array_windows
as_ascii
as_ascii_unchecked
as_bytes
as_chunks
as_chunks_mut
as_chunks_unchecked
as_chunks_unchecked_mut
as_flattened
as_flattened_mut
as_mut_ptr
as_mut_ptr_range
as_ptr
as_ptr_range
as_rchunks
as_rchunks_mut
as_simd
as_simd_mut
as_str
binary_search
binary_search_by
binary_search_by_key
chunk_by
chunk_by_mut
chunks
chunks_exact
chunks_exact_mut
chunks_mut
clone_from_slice
concat
connect
contains
copy_from_slice
copy_within
elem_offset
ends_with
eq_ignore_ascii_case
escape_ascii
fill
fill_with
first
first_chunk
first_chunk_mut
first_mut
get
get_many_mut
get_many_unchecked_mut
get_mut
get_unchecked
get_unchecked_mut
is_ascii
is_empty
is_sorted
is_sorted_by
is_sorted_by_key
iter
iter_mut
join
last
last_chunk
last_chunk_mut
last_mut
len
make_ascii_lowercase
make_ascii_uppercase
partition_dedup
partition_dedup_by
partition_dedup_by_key
partition_point
rchunks
rchunks_exact
rchunks_exact_mut
rchunks_mut
repeat
reverse
rotate_left
rotate_right
rsplit
rsplit_mut
rsplit_once
rsplitn
rsplitn_mut
select_nth_unstable
select_nth_unstable_by
select_nth_unstable_by_key
sort
sort_by
sort_by_cached_key
sort_by_key
sort_floats
sort_floats
sort_unstable
sort_unstable_by
sort_unstable_by_key
split
split_at
split_at_checked
split_at_mut
split_at_mut_checked
split_at_mut_unchecked
split_at_unchecked
split_first
split_first_chunk
split_first_chunk_mut
split_first_mut
split_inclusive
split_inclusive_mut
split_last
split_last_chunk
split_last_chunk_mut
split_last_mut
split_mut
split_once
splitn
splitn_mut
starts_with
strip_prefix
strip_suffix
subslice_range
swap
swap_unchecked
swap_with_slice
take
take_first
take_first_mut
take_last
take_last_mut
take_mut
to_ascii_lowercase
to_ascii_uppercase
to_vec
to_vec_in
trim_ascii
trim_ascii_end
trim_ascii_start
utf8_chunks
windows

Trait Implementations
AsMut<Vec<T, A>>
AsMut<[T]>
AsRef<Vec<T, A>>
AsRef<[T]>
Borrow<[T]>
BorrowMut<[T]>
Clone
Debug
Default
Deref
DerefMut
DerefPure
Drop
Eq
Extend<&'a T>
Extend<T>
From<&'a Vec<T>>
From<&[T; N]>
From<&[T]>
From<&mut [T; N]>
From<&mut [T]>
From<&str>
From<BinaryHeap<T, A>>
From<Box<[T], A>>
From<CString>
From<Cow<'a, [T]>>
From<String>
From<Vec<NonZero<u8>>>
From<Vec<T, A>>
From<Vec<T, A>>
From<Vec<T, A>>
From<Vec<T, A>>
From<Vec<T, A>>
From<Vec<T>>
From<VecDeque<T, A>>
From<[T; N]>
FromIterator<T>
Hash
Index<I>
IndexMut<I>
IntoIterator
IntoIterator
IntoIterator
Ord
PartialEq<&[U; N]>
PartialEq<&[U]>
PartialEq<&mut [U]>
PartialEq<Vec<U, A2>>
PartialEq<Vec<U, A>>
PartialEq<Vec<U, A>>
PartialEq<Vec<U, A>>
PartialEq<Vec<U, A>>
PartialEq<Vec<U, A>>
PartialEq<[U; N]>
PartialEq<[U]>
PartialOrd<Vec<T, A2>>
TryFrom<Vec<T, A>>
TryFrom<Vec<T>>
Write
Auto Trait Implementations
Freeze
RefUnwindSafe
Send
Sync
Unpin
UnwindSafe
Blanket Implementations
Any
Borrow<T>
BorrowMut<T>
From<T>
Into<U>
ToOwned
TryFrom<U>
TryInto<U>
In std::vec

Structs
Drain
ExtractIf
IntoIter
Splice
Vec
*/
