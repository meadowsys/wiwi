use crate::num_traits::*;
use super::math;

pub static SRGB_TO_XYZ: [[f64; 3]; 3] = [
	[0.41233895, 0.35762064, 0.18051042],
	[0.2126, 0.7152, 0.0722],
	[0.01932141, 0.11916382, 0.95034478]
];

pub static XYZ_TO_SRGB: [[f64; 3]; 3] = [
	[3.2413774792388685, -1.5376652402851851, -0.49885366846268053],
	[-0.9691452513005321, 1.8758853451067872, 0.04156585616912061],
	[0.05562093689691305, -0.20395524564742123, 1.0571799111220335]
];

pub static WHITE_POINT_D65: [f64; 3] = [95.047, 100.0, 108.883];

#[inline]
pub fn argb_from_rgb(red: u8, green: u8, blue: u8) -> u32 {
	let a = 255u32 << 24;
	let r = red.into_u32() << 16;
	let g = green.into_u32() << 8;
	let b = blue.into_u32();

	a | r | g | b
}

#[inline]
pub fn argb_from_linrgb(linrgb: &[f64; 3]) -> u32 {
	let r = delinearised(linrgb[0]);
	let g = delinearised(linrgb[1]);
	let b = delinearised(linrgb[2]);

	argb_from_rgb(r, g, b)
}

#[inline]
pub fn alpha_from_argb(argb: u32) -> u8 {
	(argb >> 24).into_u8_lossy()
}

#[inline]
pub fn red_from_argb(argb: u32) -> u8 {
	(argb >> 16).into_u8_lossy()
}

#[inline]
pub fn green_from_argb(argb: u32) -> u8 {
	(argb >> 8).into_u8_lossy()
}

#[inline]
pub fn blue_from_argb(argb: u32) -> u8 {
	argb.into_u8_lossy()
}

#[inline]
pub fn is_opaque(argb: u32) -> bool {
	alpha_from_argb(argb) == u8::MAX
}

#[inline]
pub fn argb_from_xyz(x: f64, y: f64, z: f64) -> u32 {
	let r = (XYZ_TO_SRGB[0][0] * x) + (XYZ_TO_SRGB[0][1] * y) + (XYZ_TO_SRGB[0][2] * z);
	let g = (XYZ_TO_SRGB[1][0] * x) + (XYZ_TO_SRGB[1][1] * y) + (XYZ_TO_SRGB[1][2] * z);
	let b = (XYZ_TO_SRGB[2][0] * x) + (XYZ_TO_SRGB[2][1] * y) + (XYZ_TO_SRGB[2][2] * z);

	let r = delinearised(r);
	let g = delinearised(g);
	let b = delinearised(b);

	argb_from_rgb(r, g, b)
}

#[inline]
pub fn xyz_from_argb(argb: u32) -> [f64; 3] {
	let r = linearised(red_from_argb(argb));
	let g = linearised(green_from_argb(argb));
	let b = linearised(blue_from_argb(argb));

	math::matrix_multiply(&[r, g, b], &SRGB_TO_XYZ)
}

#[inline]
pub fn argb_from_lab(l: f64, a: f64, b: f64) -> u32 {
	let fy = (l + 16.0) / 116.0;
	let fx = (a / 500.0) + fy;
	let fz = fy - (b / 200.0);

	let x = lab_inv_f(fx) * WHITE_POINT_D65[0];
	let y = lab_inv_f(fy) * WHITE_POINT_D65[1];
	let z = lab_inv_f(fz) * WHITE_POINT_D65[2];

	argb_from_xyz(x, y, z)
}

#[inline]
pub fn lab_from_argb(argb: u32) -> [f64; 3] {
	let lin_r = linearised(red_from_argb(argb));
	let lin_g = linearised(green_from_argb(argb));
	let lin_b = linearised(blue_from_argb(argb));

	let x = (SRGB_TO_XYZ[0][0] * lin_r) + (SRGB_TO_XYZ[0][1] * lin_g) + (SRGB_TO_XYZ[0][2] * lin_b);
	let y = (SRGB_TO_XYZ[1][0] * lin_r) + (SRGB_TO_XYZ[1][1] * lin_g) + (SRGB_TO_XYZ[1][2] * lin_b);
	let z = (SRGB_TO_XYZ[2][0] * lin_r) + (SRGB_TO_XYZ[2][1] * lin_g) + (SRGB_TO_XYZ[2][2] * lin_b);

	let x = x / WHITE_POINT_D65[0];
	let y = y / WHITE_POINT_D65[1];
	let z = z / WHITE_POINT_D65[2];

	let x = lab_f(x);
	let y = lab_f(y);
	let z = lab_f(z);

	let l = 116.0 * y - 16.0;
	let a = 500.0 * (x - y);
	let b = 200.0 * (y - z);

	[l, a, b]
}

#[inline]
pub fn argb_from_lstar(lstar: f64) -> u32 {
	let y = y_from_lstar(lstar);
	let component = delinearised(y);
	argb_from_rgb(component, component, component)
}

#[inline]
pub fn lstar_from_argb(argb: u32) -> f64 {
	let y = xyz_from_argb(argb)[1];
	(116.0 * lab_f(y / 100.0)) - 16.0
}

#[inline]
pub fn y_from_lstar(lstar: f64) -> f64 {
	100.0 * lab_inv_f((lstar + 16.0) / 116.0)
}

#[inline]
pub fn lstar_from_y(y: f64) -> f64 {
	(lab_f(y / 100.0) * 116.0) - 16.0
}

#[inline]
pub fn linearised(rgb_component: u8) -> f64 {
	let normalised = rgb_component.into_f64() / 255.0;
	if normalised <= 0.040449936 {
		normalised / 12.92 * 100.0
	} else {
		((normalised + 0.055) / 1.055).powf(2.4) * 100.0
	}
}

#[inline]
pub fn delinearised(rgb_component: f64) -> u8 {
	let normalised = rgb_component / 100.0;

	let delinearised = if normalised <= 0.0031308 {
		normalised * 12.92
	} else {
		1.055 * normalised.powf(1.0 / 2.4) - 0.055
	};

	(delinearised * 255.0)
		.round()
		.clamp(0.0, 255.0)
		.into_u8_lossy()
}

#[inline]
pub fn lab_f(t: f64) -> f64 {
	let e = 216.0 / 24389.0;
	let kappa = 24389.0 / 27.0;
	if t > e {
		t.powf(1.0 / 3.0)
	} else {
		((kappa * t) + 16.0) / 116.0
	}
}

#[inline]
pub fn lab_inv_f(ft: f64) -> f64 {
	let e = 216.0 / 24389.0;
	let kappa = 24389.0 / 27.0;
	let ft3 = ft * ft * ft;

	if ft3 > e {
		ft3
	} else {
		((116.0 * ft) - 16.0) / kappa
	}
}
