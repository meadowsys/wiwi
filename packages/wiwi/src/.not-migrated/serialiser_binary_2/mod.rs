// macro_rules! use_ok {
// 	($result:expr) => {
// 		match $result {
// 			Ok(val) => { val }
// 			Err(err) => { return Err(err) }
// 		}
// 	};
//
// 	($result:expr, #err $err:ident => $err_op:expr) => {
// 		match $result {
// 			Ok(val) => { val }
// 			Err($err) => { return $err_op }
// 		}
// 	};
//
// 	($result:expr, $val:ident => $op:expr) => {
// 		match $result {
// 			Ok($val) => { $op }
// 			Err(err) => { return Err(err) }
// 		}
// 	};
//
// 	($result:expr, $val:ident => $op:expr, #err $err:ident => $err_op:expr) => {
// 		match $result {
// 			Ok($val) => { $op }
// 			Err($err) => { return $err_op }
// 		}
// 	};
// }
// use use_ok;

// macro_rules! use_some {
// 	($option:expr) => {
// 		match $option {
// 			Some(val) => { val }
// 			None => { return None }
// 		}
// 	};
//
// 	($option:expr, #none => $none_op:expr) => {
// 		match $option {
// 			Some(val) => { val }
// 			None => { return $none_op }
// 		}
// 	};
//
// 	($option:expr, $val:ident => $op:expr) => {
// 		match $option {
// 			Some($val) => { $op }
// 			None => { return None }
// 		}
// 	};
//
// 	($option:expr, $val:ident => $op:expr, #none => $none_op:expr) => {
// 		match $option {
// 			Some($val) => { $op }
// 			None => { return $none_op }
// 		}
// 	};
// }
// use use_some;
