crate::export_all_submodules! {
	base
	// this doesn't need to be a "base"
	// float_base
	signedness
	array_conversions

	from_lossless
	into_lossless
	from_lossy
	into_lossy

	widening
	narrowing

	count_bits

	add_regular
	add_checked
	// add_unchecked
	// add_strict
	add_overflowing
	// add_saturating
	// add_wrapping
	add_carrying

	sub_regular
	sub_checked
	sub_overflowing
	sub_borrowing

	mul_regular
	mul_checked
	mul_unchecked
	mul_overflowing
	mul_widening

	div_regular
	div_checked
	div_int
	div_float
	div_overflowing

	rem_regular
	rem_checked

	shl_regular
	shl_checked

	shr_regular
	shr_checked

	// pow_regular
	// pow_int
	// pow_float

	neg_regular
	neg_checked

	not_regular

	and_regular

	or_regular

	xor_regular
}

// TODO: ilog/2/10 sum(?) product(?)
// TODO: f16 f128
