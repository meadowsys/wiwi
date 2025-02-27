use super::Iter;

pub trait IntoIter {
	type Item;
	type Iter: Iter<Item = Self::Item>;

	fn into_wiwi_iter(self) -> Self::Iter;
}

impl<I: Iter> IntoIter for I {
	type Item = I::Item;
	type Iter = Self;

	#[inline]
	fn into_wiwi_iter(self) -> Self {
		self
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn into_iter() {
		let mut iter = vec![1, 2, 3]
			.into_wiwi_iter()
			.into_wiwi_iter()
			.into_wiwi_iter()
			.into_wiwi_iter()
			.into_wiwi_iter()
			.into_wiwi_iter()
			.into_wiwi_iter()
			.into_wiwi_iter()
			.into_wiwi_iter()
			.into_wiwi_iter()
			.into_wiwi_iter()
			.into_wiwi_iter()
			// yes...???
			.into_wiwi_iter();

		assert_eq!(iter.next(), Some(1));
		assert_eq!(iter.next(), Some(2));
		assert_eq!(iter.next(), Some(3));
		assert_eq!(iter.next(), None);
	}
}
