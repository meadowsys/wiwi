use crate::rust_std::{ alloc, debug_assert };
use crate::rust_std::clone::Clone;
use crate::rust_std::cmp::{ Ord as _, Ordering };
use crate::rust_std::marker::Copy;
use crate::rust_std::mem::transmute;
use crate::rust_std::option::{ Option, Option::Some, Option::None };
use crate::rust_std::ptr::{ self, NonNull };

pub use crate::rust_std::alloc::{ GlobalAlloc, Layout };

/// # Safety
///
/// This trait has the same safety requirements as [`GlobalAlloc`]:
///
/// - an allocator unwinding is UB
/// - [`Layout`]s used must be correct
/// - You cannot rely on allocations actually happening.
pub unsafe trait Allocator {
	fn alloc(&self, layout: Layout) -> Option<NonNull<u8>>;

	#[expect(clippy::missing_safety_doc)]
	unsafe fn dealloc(&self, ptr: NonNull<u8>, layout: Layout);

	#[inline]
	fn alloc_zeroed(&self, layout: Layout) -> Option<NonNull<u8>> {
		let ptr = self.alloc(layout)?;
		// SAFETY: `alloc` returns a valid ptr to a block of memory specified by `layout`
		unsafe { ptr.as_ptr().write_bytes(0, layout.size()) }
		Some(ptr)
	}

	#[expect(clippy::missing_safety_doc)]
	#[inline]
	unsafe fn grow(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Option<NonNull<u8>> {
		debug_assert!(old_layout.size() <= new_layout.size());

		let new_ptr = self.alloc(new_layout)?;

		// SAFETY: caller promises that `ptr` is valid, `old_layout` fits `ptr`,
		// `old_layout` is smaller than  `new_layout`, and `alloc` returns a valid ptr
		unsafe { new_ptr.copy_from_nonoverlapping(ptr, old_layout.size()) }

		// SAFETY: caller promises `ptr` is valid and `old_layout` fits `ptr`
		unsafe { self.dealloc(ptr, old_layout) }

		Some(new_ptr)
	}

	#[expect(clippy::missing_safety_doc)]
	#[inline]
	unsafe fn grow_zeroed(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Option<NonNull<u8>> {
		// SAFETY: caller promises to uphold promises of `grow`
		let new_ptr = unsafe { self.grow(ptr, old_layout, new_layout)? };

		// SAFETY: `grow` returns valid block of memory, and `new_layout` is larger than `old_layout`
		let new_region = unsafe { new_ptr.add(old_layout.size()) };

		// SAFETY: this is the region that's from the end of the old len to the
		// end of the new len, valid for writes
		unsafe { new_region.write_bytes(0, new_layout.size() - old_layout.size()) }

		Some(new_ptr)
	}

	#[expect(clippy::missing_safety_doc)]
	#[inline]
	unsafe fn shrink(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Option<NonNull<u8>> {
		debug_assert!(old_layout.size() >= new_layout.size());
		let new_ptr = self.alloc(new_layout)?;

		// SAFETY: `alloc` returns a valid ptr to `new_layout`, which caller
		// promises is smaller than `old_layout`
		unsafe { new_ptr.copy_from_nonoverlapping(ptr, new_layout.size()) }

		// SAFETY: caller promises that `ptr` fits `old_layout`
		unsafe { self.dealloc(ptr, old_layout) }

		Some(new_ptr)
	}
}

#[derive(Clone, Copy)]
pub struct GlobalAllocAdapter<A> {
	allocator: A
}

// SAFETY: `Allocator` has same invariants as `GlobalAlloc`, so upheld by its' impl
unsafe impl<A: Allocator> GlobalAlloc for GlobalAllocAdapter<A> {
	#[inline]
	unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
		match self.allocator.alloc(layout) {
			Some(ptr) => { ptr.as_ptr() }
			None => { ptr::null_mut() }
		}
	}

	#[inline]
	unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		// SAFETY: caller promises `ptr` points to a valid allocated block of
		// memory, so must be non null
		let ptr = unsafe { NonNull::new_unchecked(ptr) };
		// SAFETY: see above, and additionally caller promises `ptr` was allocated
		// with `layout`
		unsafe { self.allocator.dealloc(ptr, layout) }
	}

	#[inline]
	unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
		match self.allocator.alloc_zeroed(layout) {
			Some(ptr) => { ptr.as_ptr() }
			None => { ptr::null_mut() }
		}
	}

	#[inline]
	unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
		// SAFETY: caller promises `ptr` points to a valid allocated block of
		// memory, so must be non null
		let ptr = unsafe { NonNull::new_unchecked(ptr) };
		// SAFETY: align is checked by previous `Layout` instance's invariants, and
		// caller promises `new_size` doesn't overflow `isize::MAX`
		let new_layout = unsafe { Layout::from_size_align_unchecked(new_size, layout.align()) };

		let inner_opt = match new_size.cmp(&layout.size()) {
			Ordering::Equal => { return ptr.as_ptr() }
			Ordering::Greater => {
				// SAFETY: `cmp` verified `new_size` is greater than `layout`
				// (also `new_layout` is made from `new_size`)
				unsafe { self.allocator.grow(ptr, layout, new_layout) }
			}
			Ordering::Less => {
				// SAFETY: `cmp` verified `new_size` is lesser than `layout`
				// (also `new_layout` is made from `new_size`)
				unsafe { self.allocator.shrink(ptr, layout, new_layout) }
			}
		};

		match inner_opt {
			Some(ptr) => { ptr.as_ptr() }
			None => { ptr::null_mut() }
		}
	}
}

#[derive(Clone, Copy)]
pub struct Global;

// SAFETY:
// - we don't unwind (following rust's global alloc APIs)
// - callers must promise `Layout` passed in is correct
// - we don't rely on allocations happening (and callers
// must promise this too)
unsafe impl Allocator for Global {
	fn alloc(&self, layout: Layout) -> Option<NonNull<u8>> {
		match layout.size() {
			// SAFETY: can transmute from usize to *mut u8 (creating
			// dangling pointer from align, but can't use ptr::dangling)
			0 => unsafe { transmute::<usize, Option<NonNull<u8>>>(layout.align()) }
			// SAFETY: we checked layout is non zero
			_ => { NonNull::new(unsafe { alloc::alloc(layout) }) }
		}
	}

	unsafe fn dealloc(&self, ptr: NonNull<u8>, layout: Layout) {
		match layout.size() {
			0 => { /* noop */ }
			// SAFETY: caller promises to uphold `dealloc` invariants
			_ => unsafe { alloc::dealloc(ptr.as_ptr(), layout) }
		}
	}

	fn alloc_zeroed(&self, layout: Layout) -> Option<NonNull<u8>> {
		match layout.size() {
			// SAFETY: can transmute from usize to *mut u8 (creating
			// dangling pointer from align, but can't use ptr::dangling)
			0 => unsafe { transmute::<usize, Option<NonNull<u8>>>(layout.align()) }
			// SAFETY: we checked layout is non zero
			_ => { NonNull::new(unsafe { alloc::alloc_zeroed(layout) }) }
		}
	}

	unsafe fn grow(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Option<NonNull<u8>> {
		debug_assert!(old_layout.size() <= new_layout.size());

		match (old_layout.size(), new_layout.size()) {
			// both zero sized, just return dangling
			// SAFETY: can transmute from usize to *mut u8 (creating
			// dangling pointer from align, but can't use ptr::dangling)
			(0, 0) => unsafe { transmute::<usize, Option<NonNull<u8>>>(new_layout.align()) }

			// old is zero size, so just allocate new
			(0, _) => { self.alloc(new_layout) }

			// same size, just return unchanged
			(old, new) if old == new => { Some(ptr) }

			// growing
			// SAFETY: caller promises to uphold invariants of `realloc`
			(_, _) => { NonNull::new(unsafe { alloc::realloc(ptr.as_ptr(), old_layout, new_layout.size()) }) }
		}
	}

	unsafe fn shrink(&self, ptr: NonNull<u8>, old_layout: Layout, new_layout: Layout) -> Option<NonNull<u8>> {
		debug_assert!(old_layout.size() >= new_layout.size());

		match (old_layout.size(), new_layout.size()) {
			// both zero sized, just return dangling
			// SAFETY: can transmute from usize to *mut u8 (creating
			// dangling pointer from align, but can't use ptr::dangling)
			(0, 0) => unsafe { transmute::<usize, Option<NonNull<u8>>>(new_layout.align()) }

			// new is zero size, so just deallocate old
			(_, 0) => {
				// SAFETY: caller promises `ptr` is valid and fits `old_layout`
				unsafe { self.dealloc(ptr, old_layout) }

				// SAFETY: can transmute from usize to *mut u8 (creating
				// dangling pointer from align, but can't use ptr::dangling)
				unsafe { transmute::<usize, Option<NonNull<u8>>>(new_layout.align()) }
			}

			// same size, just return unchanged
			(old, new) if old == new => { Some(ptr) }

			// shrinking
			// SAFETY: caller promises to uphold invariants of `realloc`
			(_, _) => { NonNull::new(unsafe { alloc::realloc(ptr.as_ptr(), old_layout, new_layout.size()) }) }
		}
	}
}
