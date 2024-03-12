use super::*;
use ::hashbrown::hash_map::DefaultHashBuilder;
use ::rand::{ Rng, rngs::OsRng };
use ::std::string::String as StdString;
use ::std::iter::repeat;
use ::std::hash::BuildHasher;

#[test]
fn slices_wrap_iter_hash_and_eq() {
	let hash_builder = hashbrown::hash_map::DefaultHashBuilder::default();

	for _ in 0..1000 {
		// generate vec of random length 0-10, with strings 0-100 chars
		let strs = repeat(0u8)
			.take(OsRng.gen_range(1..20))
			.map(|_| rand_std_string())
			.collect::<Vec<_>>();

		// create instance of SliceHashWrap (joining strings)
		let pool_strs = strs.iter()
			.map(|s| &**s)
			.collect::<String>();
		let pool_strs = Arc::new(SliceHashWrap(pool_strs.into_bytes().into_boxed_slice()));

		// create instance of SlicesWrap
		let mut _slices = strs.iter()
			.map(|s| s.as_bytes())
			.collect::<Vec<_>>();
		let slices = SlicesWrap(&_slices);

		let hash_pool = hash_item(&hash_builder, &pool_strs);
		let hash_slices = hash_item(&hash_builder, &slices);

		// test hash eq
		assert_eq!(hash_pool, hash_slices, "hashes should be equal");
		// test actual eq
		assert!(slices.equivalent(&pool_strs), "pool and slices should be equal");

		// this one is guaranteed by rand_std_string generating 1..100 chars
		// panics when that is changed to 0..100
		// TODO: refactor to make it be able to generate 0..100 without issue
		let last = _slices.last_mut().unwrap();
		let last_str = unsafe { std::str::from_utf8_unchecked(last) };
		*last = &last[..last.len() - last_str.chars().last().unwrap().len_utf8()];

		let slices = SlicesWrap(&_slices);
		let hash_slices = hash_item(&hash_builder, &slices);

		assert_ne!(hash_pool, hash_slices, "hashes should not be eq");
		assert!(!slices.equivalent(&pool_strs), "pool and slices should not be equal");
	}
}

fn rand_std_string() -> StdString {
	let mut vec = vec![' '; OsRng.gen_range(1..100)];
	OsRng.fill(&mut *vec);
	vec.into_iter().collect()
}

fn hash_item<T: Hash>(hash_builder: &DefaultHashBuilder, item: &T) -> u64 {
	let mut hasher = hash_builder.build_hasher();
	item.hash(&mut hasher);
	hasher.finish()
}
