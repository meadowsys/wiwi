use crate::num_traits::*;

#[inline]
pub fn signum(val: f64) -> i8 {
	match val {
		..0.0 => { -1 }
		0.0 => { 0 }
		_ => { 1 }
	}
}

#[inline]
pub fn lerp(start: f64, stop: f64, amount: f64) -> f64 {
	((1.0 - amount) * start) + (amount * stop)
}

#[inline]
pub fn sanitise_degrees<I: DivInt + Rem + FromU16Lossless>(degrees: I) -> I {
	let degrees = degrees % FromU16Lossless::from_u16(360);
	// TODO: need < operator
	todo!()
}

#[inline]
pub fn rotation_direction(from: f64, to: f64) -> f64 {
	let increasing_diff = sanitise_degrees(to - from);
	if increasing_diff <= 180.0 { 1.0 } else { -1.0 }
}

#[inline]
pub fn difference_degrees(a: f64, b: f64) -> f64 {
	180.0 - ((a - b).abs() - 180.0).abs()
}

#[inline]
pub fn matrix_multiply(row: &[f64; 3], matrix: &[[f64; 3]; 3]) -> [f64; 3] {
	let a = (row[0] * matrix[0][0]) + (row[1] * matrix[0][1]) + (row[2] * matrix[0][2]);
	let b = (row[0] * matrix[1][0]) + (row[1] * matrix[1][1]) + (row[2] * matrix[1][2]);
	let c = (row[0] * matrix[2][0]) + (row[1] * matrix[2][1]) + (row[2] * matrix[2][2]);

	[a, b, c]
}
