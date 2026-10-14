use crate::macros::impl_memory_usage_size_of;

impl_memory_usage_size_of! {
	u8
	u16
	u32
	u64
	u128
	usize
	i8
	i16
	i32
	i64
	i128
	isize
	f32
	f64
}

#[cfg(feature = "nightly")]
impl_memory_usage_size_of! {
	f16
	f128
}
