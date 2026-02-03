pub use chain::{ Chain, ChainInner };

mod chain;

// todo remove the below lmao

struct Test;

#[allow(
	dead_code,
	clippy::allow_attributes,
	reason = "testing only (todo remove me)"
)]
impl Test {
	fn fn_plain() {}
	fn fn_args(arg1: usize, arg2: &str, arg3: (usize, u8)) {
		let _ = (arg1, arg2, arg3);
	}
	fn fn_self(self) {}
	fn fn_self_ref(&self) {}
	fn fn_self_mut(&mut self) {}
	const fn fn_const() -> Self { Self }
	async fn fn_async() {}
	unsafe fn fn_unsafe() {}
	extern "C" fn fn_extern_c() {}
	fn fn_generics<T1, T2>(t1: T1, t2: T2) {
		let _ = (t1, t2);
	}
	fn fn_output() -> Self { Self }
	fn fn_output2() -> usize { 0 }
}

#[wiwi_macro_proc::chain_fn]
impl Chain<Test> {
	fn fn_plain();
	fn fn_args(arg1: usize, arg2: &str, arg3: (usize, u8));
	// fn fn_self(self);
	// fn fn_self_ref(&self);
	// fn fn_self_mut(&mut self);
	const fn fn_const() -> Self;
	async fn fn_async();
	unsafe fn fn_unsafe();
	extern "C" fn fn_extern_c();
	// fn fn_generics<T1, T2>(t1: T1, t2: T2);
	fn fn_output() -> Self;
	// fn fn_output2() -> usize;
}
