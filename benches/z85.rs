use ::criterion::{ black_box, criterion_group, criterion_main, Criterion };
use ::rand::{ Rng, thread_rng };

fn benchmark(c: &mut Criterion) {
	const FIFTY_MIB: usize = 50 * 1024 * 1024;

	let mut rng = thread_rng();
	let mut bytes = vec![0u8; FIFTY_MIB];
	rng.fill(&mut *bytes);
	let bytes = &*bytes;

	let encoded_wiwi = ::wiwi::z85::encode_z85(bytes);
	let encoded_wiwi = encoded_wiwi.as_bytes();

	let encoded_z85 = ::z85::encode(bytes);
	let encoded_z85 = encoded_z85.as_bytes();

	let encoded_hex = ::hex::encode(bytes);
	let encoded_hex = encoded_hex.as_bytes();

	c
		.bench_function("hex::decode 50MiB", |b| b.iter(|| {
			let _: Vec<u8> = ::hex::decode(black_box(encoded_hex)).unwrap();
		}))
		.bench_function("hex::encode 50MiB", |b| b.iter(|| {
			let _: String = ::hex::encode(black_box(bytes));
		}))
		.bench_function("z85::decode 50MiB", |b| b.iter(|| {
			let _: Vec<u8> = ::z85::decode(black_box(encoded_z85)).unwrap();
		}))
		.bench_function("z85::encode 50MiB", |b| b.iter(|| {
			let _: String = ::z85::encode(black_box(bytes));
		}))
		.bench_function("wiwi::z85::decode_z85 50MiB", |b| b.iter(|| {
			let _: Vec<u8> = ::wiwi::z85::decode_z85(black_box(encoded_wiwi)).unwrap();
		}))
		.bench_function("wiwi::z85::encode_z85 50MiB", |b| b.iter(|| {
			let _: String = ::wiwi::z85::encode_z85(black_box(bytes));
		}));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
