#[cfg(feature = "z85")]
fn main() {
	use ::wiwi::z85::TABLE_ENCODER;

	let mut string = String::new();

	string.push_str("pub const TABLE_DECODER: [Option<u8>; 256] = [");
	for i in 0..=u8::MAX {
		if i & 0b1111 == 0 { string.push_str("\n\t") }

		let char = TABLE_ENCODER.iter()
			.enumerate()
			.find(|(_, b)| **b == i)
			.map(|(i, _)| i);

		if let Some(num) = char {
			string.push_str(&format!("Some(0x{num:02x}), "));
		} else {
			string.push_str("None,       ");
		}
	}

	for _ in 0..",       ".len() {
		string.pop();
	}

	string.push_str("\n];");

	println!("{string}");
}

#[cfg(not(feature = "z85"))]
fn main() {
	eprintln!("Hi, this is the `generate-z85-table-decoder` binary, but you aren't running this binary with the `z85` feature enabled. Try rerunning this with that enabled: `cargo run --features z85");
	::std::process::exit(69);
}
