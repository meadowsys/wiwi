use super::*;
use std::convert::{ AsRef, AsMut };

/// Trait implemented on chainers
pub trait ChainHalf
where
	Self: Sized + private::Sealed + Into<Self::NonChain> + AsRef<Self::NonChain> + AsMut<Self::NonChain>,
	Self::NonChain: Into<Self> + NonChainHalf<Chain = Self>
{
	/// This chainer's nonchain, "inner" type
	type NonChain;

	/// Borrows `self` as its nonchain
	#[inline(always)]
	fn as_nonchain(&self) -> &Self::NonChain {
		self.as_ref()
	}

	/// Borrows `self` mutably as its nonchain
	#[inline(always)]
	fn as_nonchain_mut(&mut self) -> &mut Self::NonChain {
		self.as_mut()
	}

	/// Converts `self` back into it's nonchain type
	#[inline(always)]
	fn into_nonchain(self) -> Self::NonChain {
		self.into()
	}

	/// Converts a nonchain type into it's chainer
	#[inline(always)]
	fn from_nonchain(nonchain: Self::NonChain) -> Self {
		nonchain.into()
	}
}

// /// Convert any supported type into its chainer type
// ///
// /// Every type that implements this trait has a "preferred" chain type, declared
// /// by the [`NonChainHalf::Chain`] associated type, and calling [`into_chainer`]
// /// will convert it to that chain type.
// ///
// /// To undo this operation, you can use [`ChainHalf`].
// ///
// /// [`into_chainer`]: NonChainHalf::into_chainer
// ///
// /// # Examples
// ///
// /// ```
// /// # use wiwi::chainer::{ NonChainHalf, VecChain };
// /// let vec: Vec<String> = Vec::new();
// /// let chain: VecChain<String> = vec.into_chainer();
// /// // ...
// /// // do funny things with chaining API c:
// /// ```
/// Trait implemented on the nonchain for chainers
pub trait NonChainHalf
where
	Self: Sized + private::Sealed + Into<Self::Chain>,
	Self::Chain: Into<Self> + ChainHalf<NonChain = Self> + AsRef<Self> + AsMut<Self>
{
	/// This type's chainer wrapper type
	type Chain;

	/// Converts `self` into its associated chain type
	#[inline(always)]
	fn into_chainer(self) -> Self::Chain {
		self.into()
	}

	/// Converts a chainer into it's nonchain type
	#[inline(always)]
	fn from_chainer(chainer: Self::Chain) -> Self {
		chainer.into()
	}
}

/// notouchie
pub(super) mod private {
	/// notouchie
	pub trait Sealed {}
}
