// todo should we validate lengths of things with limits? im not sure
// maybe we do in serialisation but not deserialisation?
// gonna carry on without it for now
// todo maybe #[cfg_attr(feature = "serde", derive(Deserialize, Serialize))]

use core::ascii::Char;
use core::hint;
use core::ops::Deref;
use core::str::{ self, FromStr };

pub struct Color {
	r: u8,
	g: u8,
	b: u8
}

impl FromStr for Color {
	type Err = ();

	#[inline]
	fn from_str(s: &str) -> Result<Self, ()> {
		// todo use our own hex decode impl maybe?
		if s.len() != 6 || !s.is_ascii() { return Err(()) }


		let [r1, r2, g1, g2, b1, b2] = *s.as_bytes() else {
			// SAFETY: s here is all valid ascii and valid for 6 bytes we just checked
			unsafe { hint::unreachable_unchecked() }
		};

		macro_rules! do_convert {
			($b:ident $b1:ident $b2:ident) => {
				do_convert!($b1);
				do_convert!($b2);
				let $b = ($b1 << 4) | $b2;
			};

			($b:ident) => {
				let $b = match $b {
					b'0'..=b'9' => {
						// SAFETY: we wont overflow
						unsafe { $b.unchecked_sub(b'0') }
					}

					b'a'..=b'f' => {
						// SAFETY: we wont overflow
						unsafe { $b.unchecked_sub(const { b'a' - 10 }) }
					}

					b'A'..=b'F' => {
						// SAFETY: we wont overflow
						unsafe { $b.unchecked_sub(const { b'A' - 10 }) }
					}

					_ => { return Err(()) }
				};
			};
		}

		do_convert!(r r1 r2);
		do_convert!(g g1 g2);
		do_convert!(b b1 b2);

		Ok(Self { r, g, b })
	}
}

// struct DateTime {}

pub struct Id {
	inner: IdInner
}

// is branching better if it avoids a heap allocation? probably?
// // is this helpful or necessary? idk
// #[cfg_attr(target_pointer_width = "16", repr(align(2)))]
// #[cfg_attr(target_pointer_width = "32", repr(align(4)))]
// #[cfg_attr(target_pointer_width = "64", repr(align(8)))]
enum IdInner {
	Id5 { str: [Char; 5] },
	Id6 { str: [Char; 6] }
}

impl Id {
	#[inline]
	pub fn as_str(&self) -> &str {
		match &self.inner {
			IdInner::Id5 { str } => { str.as_str() }
			IdInner::Id6 { str } => { str.as_str() }
		}
	}
}

impl Deref for Id {
	type Target = str;

	#[inline]
	fn deref(&self) -> &str {
		self.as_str()
	}
}

// impl FromStr for Id {
// 	// todo idk
// 	type Err = ();

// 	#[inline]
// 	fn from_str(s: &str) -> Result<Id, ()> {
// 		// todo need to redo this, with stricter checks
// 		if s.len() != 5 || s.len() != 6 { return Err(()) }

// 		let s = s.as_ascii().ok_or_else(|| ())?;
// 		let inner = match s.len() {
// 			5 => {
// 				let ptr = s.as_ptr().cast::<[Char; 5]>();
// 				// SAFETY: checked len is 5, and ptr made from valid reference
// 				unsafe { IdInner::Id5 { str: *ptr } }
// 			}
// 			6 => {
// 				let ptr = s.as_ptr().cast::<[Char; 6]>();
// 				// SAFETY: checked len is 6, and ptr made from valid reference
// 				unsafe { IdInner::Id6 { str: *ptr } }
// 			}

// 			_ => {
// 				// SAFETY: checked `s.len()` is 5 or 6
// 				unsafe { unreachable_unchecked() }
// 			}
// 		};

// 		Ok(Id { inner })
// 	}
// }

#[repr(u8)]
pub enum Privacy {
	Private,
	Public
}

pub struct ProxyTag {
	pub prefix: Option<String>,
	pub suffix: Option<String>
}

#[cfg(feature = "uuid")]
// is this helpful or necessary? idk
#[cfg_attr(target_pointer_width = "16", repr(align(2)))]
#[cfg_attr(target_pointer_width = "32", repr(align(4)))]
#[cfg_attr(target_pointer_width = "64", repr(align(8)))]
pub struct Uuid {
	inner: uuid::Uuid
}






pub struct System {
	id: Id,
	// uuid
	// name
	// description
	// tag
	// pronouns
	// avatar_url
	// banner
	// color
	// created
	// privacy
}

pub struct SystemPrivacy {
	// description
	// pronouns
	// member_list
	// group_list
	// front
	// front_history
}

pub struct Member {
	id: Id,
	// uuid
	// system
	// name
	// display_name
	// color
	// birthday
	// pronouns
	// avatar_url
	// webhook_avatar_url
	// banner
	// description
	// created
	// proxy_tags
	// keep_proxy
	// tts
	// autoproxy_enabled
	// message_count
	// last_message_timestamp
	// privacy
}

pub struct MemberPrivacy {
	// visibility
	// name
	// description
	// birthday
	// pronoun
	// avatar
	// metadata
	// proxy
}

pub struct Group {
	id: Id,
	// uuid
	// system
	// name
	// display_name
	// description
	// icon
	// banner
	// color
	// privacy
}

pub struct GroupPrivacy {
	// visibility
	// name
	// description
	// icon
	// list
	// metadata
}

pub struct Switch {
	// uuid
	// timestamp
	// members
}

pub struct Message {
	// timestamp
	// message_id
	// original_message_id
	// sender_id
	// channel_id
	// guild_id
	// system
	// member
}

pub struct SystemSettings {
	// timezone
	// pings_enabled
	// latch_timeout
	// member_default_private
	// group_default_private
	// show_private_info
	// member_limit
	// group_limit
}

pub struct SystemGuildSettings {
	// guild_id
	// proxying_enabled
	// tag
	// tag_enabled
}

pub struct AutoproxySettings {
	// guild_id
	// channel_id
	// autoproxy_mode
	// last_latch_timestamp
}

pub enum AutoproxyMode {
	Off,
	Front,
	Latch  { member_id: Option<Id> },
	Member { member_id: Id }
}

pub struct MemberGuildSettings {
	// guild_id
	// display_name
	// avatar_url
	// keep_proxy
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn basic_color_parse() {
		let parsed = Color::from_str("af8dc0").unwrap();
		assert_eq!(parsed.r, 0xaf);
		assert_eq!(parsed.g, 0x8d);
		assert_eq!(parsed.b, 0xc0);
	}
}
