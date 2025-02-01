pub trait AcceptableTarget {
	fn as_target(&self) -> &crate::ExternObject;
}
