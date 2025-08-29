use crate::to_maybeuninit::ToMaybeUninit as _;
use std::mem::{ self, ManuallyDrop, MaybeUninit };
use std::ptr;
use super::{ ArrayRefChain, ArrayMutChain, SliceRefChain, SliceMutChain };

#[repr(transparent)]
pub struct ArrayChain<T, const N: usize> {
	inner: [T; N]
}

impl<T, const N: usize> ArrayChain<T, N> {
	pub fn new_uninit() -> ArrayChain<MaybeUninit<T>, N> {
		unsafe {
			MaybeUninit::<[MaybeUninit<T>; N]>::uninit()
				.assume_init()
				.into()
		}
	}

	pub fn new_zeroed() -> ArrayChain<MaybeUninit<T>, N> {
		unsafe {
			MaybeUninit::<[MaybeUninit<T>; N]>::zeroed()
				.assume_init()
				.into()
		}
	}
}

impl<T, const N: usize> ArrayChain<T, N> {
	pub fn into_inner(self) -> [T; N] {
		self.inner
	}

	pub fn nonchain_array(&self) -> &[T; N] {
		&self.inner
	}

	pub fn nonchain_array_mut(&mut self) -> &mut [T; N] {
		&mut self.inner
	}

	pub fn nonchain_array_chainer_ref(&self) -> ArrayRefChain<T, N> {
		(&self.inner).into()
	}

	pub fn nonchain_array_chainer_mut(&mut self) -> ArrayMutChain<T, N> {
		(&mut self.inner).into()
	}

	pub fn nonchain_slice(&self) -> &[T] {
		&self.inner
	}

	pub fn nonchain_slice_mut(&mut self) -> &mut [T] {
		&mut self.inner
	}

	pub fn nonchain_slice_chainer_ref(&self) -> SliceRefChain<T> {
		(&self.inner as &[T]).into()
	}

	pub fn nonchain_slice_chainer_mut(&mut self) -> SliceMutChain<T> {
		(&mut self.inner as &mut [T]).into()
	}
}

impl<T, const N: usize> ArrayChain<T, N> {
	pub fn len(self, out: &mut usize) -> Self {
		self.len_uninit(unsafe { out.to_maybeuninit_drop() })
	}

	pub fn len_uninit(self, out: &mut MaybeUninit<usize>) -> Self {
		out.write(N);
		self
	}

	pub fn is_empty(self, out: &mut bool) -> Self {
		self.is_empty_uninit(unsafe { out.to_maybeuninit_drop() })
	}

	pub fn is_empty_uninit(mut self, out: &mut MaybeUninit<bool>) -> Self {
		out.write(N == 0);
		self
	}

	pub fn first<F>(self, f: F) -> Self
	where
		F: FnOnce(Option<&T>)
	{
		f(self.inner.first());
		self
	}

	pub fn first_mut<F>(mut self, f: F) -> Self
	where
		F: FnOnce(Option<&mut T>)
	{
		f(self.inner.first_mut());
		self
	}

	// TODO: map, try map
	// TODO: as_slice/mut
	// TODO: each_ref
	// TODO: each_mut
	// TODO: split_array_ref
	// TODO: split_array_mut
	// TODO: rsplit_array_ref
	// TODO: rsplit_array_mut
}

impl<const N: usize> ArrayChain<u8, N> {
	// as_ascii/unchecked
}

impl<T, const N: usize> ArrayChain<MaybeUninit<T>, N> {
	pub unsafe fn assume_init(self) -> ArrayChain<T, N> {
		// TODO: this is subpar (its copying), but I can't find a better way to do it?
		// all ways to do it seem to be unstable (transmute is too dumb, transmute_unchecked
		// is unstable and likely won't ever be stable, MaybeUninit::array_assume_init
		// is unstable (it uses transmute_unchecked internally))
		let me = ManuallyDrop::new(self);
		ptr::read(&me.inner as *const [MaybeUninit<T>; N] as *const [T; N]).into()
	}

	// transpose
}

// o.0
// cannot use generic params in expressions (yet?), so cannot make return type
// impl<T, const N: usize, const N2: usize> ArrayChain<[T; N2], N> {
// 	pub fn flatten(self) -> ArrayChain<T, { N * N2 }> {}
// }

impl<T, const N: usize> From<[T; N]> for ArrayChain<T, N> {
	fn from(value: [T; N]) -> Self {
		Self { inner: value }
	}
}
