fn main() {
	// // we'll encode both upper and lower into the same table
	// use wiwi::hex::{ TABLE_ENCODER_LOWER, TABLE_ENCODER_UPPER };

	// let mut string = String::new();

	// string.push_str("pub const TABLE_DECODER: [Option<u8>; 256] = [");
	// for i in 0..=u8::MAX {
	// 	if i & 0b1111 == 0 { string.push_str("\n\t") }

	// 	let char_lower = TABLE_ENCODER_LOWER.iter()
	// 		.enumerate()
	// 		.find(|(_, b)| **b == i)
	// 		.map(|(i, _)| i);
	// 	let char_upper = TABLE_ENCODER_UPPER.iter()
	// 		.enumerate()
	// 		.find(|(_, b)| **b == i)
	// 		.map(|(i, _)| i);

	// 	if let Some(num) = char_lower {
	// 		string.push_str(&format!("Some(0x{num:02x}), "));
	// 	} else if let Some(num) = char_upper {
	// 		string.push_str(&format!("Some(0x{num:02x}), "));
	// 	} else {
	// 		string.push_str("None,       ");
	// 	}
	// }

	// for _ in 0..",       ".len() {
	// 	string.pop();
	// }

	// string.push_str("\n];");

	// println!("{string}");
}
