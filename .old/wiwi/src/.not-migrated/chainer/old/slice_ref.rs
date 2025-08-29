use crate::to_maybeuninit::ToMaybeUninit as _;
use std::mem::{ self, MaybeUninit };
use std::slice;
use super::VecChain;

#[repr(transparent)]
pub struct SliceRefChain<'h, T> {
	inner: &'h [T]
}

impl<'h, T> SliceRefChain<'h, T> {
	pub fn into_inner(self) -> &'h [T] {
		self.inner
	}

	pub fn into_vec_chain(self) -> VecChain<T>
	where
		T: Clone
	{
		self.inner.to_vec().into()
	}

	pub fn nonchain_slice(&self) -> &[T] {
		self.inner
	}
}

// TODO: to_vec_chain_in (alloc)

impl<'h, T> SliceRefChain<'h, T> {
	pub fn first<CB>(self, cb: CB) -> Self
	where
		CB: FnOnce(Option<&T>)
	{
		cb(self.inner.first());
		self
	}

	pub fn last<CB>(self, cb: CB) -> Self
	where
		CB: FnOnce(Option<&T>)
	{
		cb(self.inner.last());
		self
	}

	pub fn is_empty(self, out: &mut bool) -> Self {
		self.is_empty_uninit(unsafe { out.to_maybeuninit_drop() })
	}

	pub fn is_empty_uninit(self, out: &mut MaybeUninit<bool>) -> Self {
		out.write(self.inner.is_empty());
		self
	}

	pub fn len(self, out: &mut usize) -> Self {
		self.len_uninit(unsafe { out.to_maybeuninit_drop() })
	}

	pub fn len_uninit(self, out: &mut MaybeUninit<usize>) -> Self {
		out.write(self.inner.len());
		self
	}

	// TODO: utf8_chunks
	// TODO: is_ascii
	// TODO: as_ascii/unchecked
	// TODO: eq_ignore_ascii_case
	// TODO: make_ascii_uppercase/lowercase
	// TODO: escape_ascii
	// TODO: trim_ascii
	// TODO: trim_ascii_start/end
	// TODO: s o r t   f l o a t s

	// TODO: len
	// TODO: is_empty
	// TODO: first/mut
	// TODO: last/mut
	// TODO: split_first/mut
	// TODO: split_last/mut
	// TODO: first_chunk/mut
	// TODO: split_first_chunk/mut
	// TODO: last_chunk/mut
	// TODO: split_last_chunk/mut
	// TODO: get/mut
	// TODO: get_unchecked/mut
	// TODO: as_ptr
	// TODO: as_ptr_mut
	// TODO: as_ptr_range
	// TODO: as_ptr_range_mut
	// TODO: swap/unchecked
	// TODO: reverse
	// TODO: iter/mut
	// TODO: windows
	// TODO: chunks/mut
	// TODO: chunks_exact/mut
	// TODO: as_chunks/unchecked/mut
	// TODO: as_rchunks/unchecked/mut
	// TODO: array_chunks/mut
	// TODO: array_windows
	// TODO: rchunks/mut
	// TODO: rchunks_exact/mut
	// TODO: chunk_by/mut
	// TODO: split_at/mut
	// TODO: split_at_unchecked/mut
	// TODO: split_at_checked/mut
	// TODO: split/mut
	// TODO: split_inclusive/mut
	// TODO: rsplit/mut
	// TODO: splitn/mut
	// TODO: rsplitn/mut
	// TODO: split_once/mut
	// TODO: rsplit_once/mut
	// TODO: contains
	// TODO: starts/ends_with
	// TODO: strip_prefix/suffix
	// TODO: binary_search/by/key
	// TODO: sort_unstable/by/key
	// TODO: select_nth_unstable/by/key
	// TODO: partition_dedup/by/key
	// TODO: rotate_left/right
	// TODO: fill/with
	// TODO: clone/copy_from_slice
	// TODO: copy_within
	// TODO: swap_with_slice
	// TODO: align_to/mut
	// TODO: as_simd/mut
	// TODO: is_sorted/by/key
	// TODO: partition_point
	// TODO: take/mut
	// TODO: take_first/mut
	// TODO: take_last/mut
	// TODO: get_many_unchecked_mut
	// TODO: get_many_mut
	// TODO: as_str
	// TODO: as_bytes
	// TODO: to_ascii_uppercase/lowercase
	// TODO: into_vec (for box chainer)
	// TODO: repeat
	// TODO: concat
	// TODO: join
}

impl<'h, T, const N: usize> SliceRefChain<'h, [T; N]> {
	pub fn flatten(self) -> SliceRefChain<'h, T> {
		// taken from std's flatten fn
		// TODO: use SizedTypeProperties or slice `flatten`, whichever gets stabilised first
		let len = if mem::size_of::<T>() == 0 {
			self.inner.len()
				.checked_mul(N)
				.expect("slice len overflow")
		} else {
			// TODO: wait until 1.79 when this is stabilised
			// unsafe { self.inner.len().unchecked_mul(N) }

			self.inner.len() * N
		};

		let ptr = self.inner as *const [[T; N]] as *const T;
		unsafe { slice::from_raw_parts(ptr, len).into() }
	}
}

impl<'h, T> From<&'h [T]> for SliceRefChain<'h, T> {
	fn from(value: &'h [T]) -> Self {
		Self { inner: value }
	}
}
