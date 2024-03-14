//! h

/// h
pub fn h() -> String {
	"h".into()
}

#[cfg(test)]
mod tests {
	#[test]
	pub fn h() {
		assert_eq!(super::h(), "h", "h should be h");
	}
}
