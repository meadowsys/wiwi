use crate::macros::{ impl_memory_usage_static, impl_memory_usage_size_of };
use core::marker::*;

impl_memory_usage_static! {
	{ T: ?Sized }
	{ T: ?Sized }
	PhantomData<T>
	=> size_of::<PhantomData<T>>();
}

impl_memory_usage_size_of! {
	PhantomPinned
}

#[cfg(feature = "nightly")]
impl_memory_usage_static! {
	{ T: ?Sized }
	{ T: ?Sized }
	PhantomCovariant<T>
	=> size_of::<PhantomCovariant<T>>();
}

#[cfg(feature = "nightly")]
impl_memory_usage_static! {
	{ 'h }
	{ 'h }
	PhantomCovariantLifetime<'h>
	=> size_of::<PhantomCovariantLifetime<'h>>();
}

#[cfg(feature = "nightly")]
impl_memory_usage_static! {
	{ T: ?Sized }
	{ T: ?Sized }
	PhantomContravariant<T>
	=> size_of::<PhantomContravariant<T>>();
}

#[cfg(feature = "nightly")]
impl_memory_usage_static! {
	{ 'h }
	{ 'h }
	PhantomContravariantLifetime<'h>
	=> size_of::<PhantomContravariantLifetime<'h>>();
}

#[cfg(feature = "nightly")]
impl_memory_usage_static! {
	{ T: ?Sized }
	{ T: ?Sized }
	PhantomInvariant<T>
	=> size_of::<PhantomInvariant<T>>();
}

#[cfg(feature = "nightly")]
impl_memory_usage_static! {
	{ 'h }
	{ 'h }
	PhantomInvariantLifetime<'h>
	=> size_of::<PhantomInvariantLifetime<'h>>();
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{
		MemoryUsage as _,
		MemoryUsageConst as _,
		MemoryUsageStatic
	};

	#[test]
	fn phantom_data_usage() {
		let phantom = PhantomData::<usize>;

		assert_eq!(phantom.memory_usage(), 0);
		assert_eq!(phantom.memory_usage_const(), 0);
		assert_eq!(<PhantomData<usize>>::MEMORY_USAGE, 0);
	}

	#[cfg(feature = "nightly")]
	macro_rules! phantom_data_variance_usage {
		{ $($test_name:ident($ty:path))* } => {
			$(
				#[test]
				fn $test_name() {
					let phantom: $ty = variance();

					assert_eq!(phantom.memory_usage(), 0);
					assert_eq!(phantom.memory_usage_const(), 0);
					assert_eq!(<$ty>::MEMORY_USAGE, 0);
				}
			)*
		}
	}

	#[cfg(feature = "nightly")]
	phantom_data_variance_usage! {
		phantom_covariant_usage(PhantomCovariant<usize>)
		phantom_covariant_lifetime_usage(PhantomCovariantLifetime<'static>)

		phantom_contravariant_usage(PhantomContravariant<usize>)
		phantom_contravariant_lifetime_usage(PhantomContravariantLifetime<'static>)

		phantom_invariant_usage(PhantomInvariant<usize>)
		phantom_invariant_lifetime_usage(PhantomInvariantLifetime<'static>)
	}
}
