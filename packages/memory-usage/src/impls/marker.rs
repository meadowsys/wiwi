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
