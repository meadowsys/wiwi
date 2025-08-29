# Spec

The Spec&trade;

## Root

Each encoded document contains one root object, and that's it. It is forbidden to have trailing bytes that aren't used in deserialisation of the first object found. This restriction can however be relaxed with the `deserialise_lax` function.

To (conformantly) encode multiple objects in the "root" of a document, put them in an array.

## Endianness

All values that are sensitive to endianness, will use the little endian byte order when encoded.

## Markers

- `0x00` - the value `false`
- `0x01` - the value `true`
- `0x02` - the value `none` (similar to `null`, `nil`, `Option::None`, etc)
- `0x03` to `0x07` - IEEE754-2008 floating point types (`f16`, `f32`, `f64`, `f128`, `f256`)
- `0x08` - string
- `0x09` - array
- `0x0a` - record
- `0x0b` - map
- `0x0c` - binary
- `0x0d` - intern
- `0x0e` - 2 byte markers
- `0x0f` - 3 byte markers
- `0x10` to `0x1f` - integer markers (`i8`, `i16`, `i24`, `i32`, `i48`, `i64`, `i96`, `i128`, `u8`, `u16`, `u24`, `u32`, `u48`, `u64`, `u96`, `u128`)
- `0x20` to `0x3f` - negative integers, from -31 to 0
- `0x40` to `0x7f` - positive integers, from 1 to 64
- `0x80` to `0x9f` - string, length 1 to 32
- `0xa0` to `0xaf` - array, length 1 to 16
- `0xb0` to `0xb7` - record, length 1 to 8
- `0xb8` to `0xbf` - map, length 1 to 8
- `0xc0` - char
- `0xc1` to `0xfe` - reference to interned value, ref 0 to 61
- `0xff` - reference to interned value

## `none`/`null`/`nil`/`None`/etc

The value `none` is encoded as `0x02`.

## booleans

The value `false` is encoded as `0x00`, and the value `true` is encoded as `0x01`.

## integers

There are... 18 types of integers: small positive ints, small negative ints, `i8`, `i16`, `i24`, `i32`, `i48`, `i64`, `i96`, `i128`, `u8`, `u16`, `u24`, `u32`, `u48`, `u64`, `u96`, `u128`.

Marker types for `i8`...`i128` are in range `0x10` to `0x17`, and marker types for `u8`...`u128` are in range `0x18` to `0x1f`. For example, marker for i96 is `0x16`, and marker for `u24` is `0x1a`.

If an integer is within -31 to 0, it can be encoded as one marker byte within the range `0x20` to `0x3f`. For example, `0x20` has the value of -31 and `0x3f` has the value of 0.

If an integer is within 1 to 64, it can be encoded as one marker byte within range `0x40` to `0x7f`. For example, `0x40` has the value of 1 and `0x7f` has the value of 64.

If an integer is outside of these ranges: first consider what signedness the integer is, and what the _value_ is. The smallest type that has the correct signedness must be used. For example, an unsigned value of 70,000 should use the type `u24`, while a signed value of 70,000 should use the type `i24`. Then first write the marker for the type, followed by the integer bytes itself.

Note: in v1 there were more number types (one for ever multiple of 8, up to and including 128). We've decided there is probably not much benefit to these extra types for the amount of marker space they hog. With this new data model there are half the amount of integer sizes, half the int markers as before, but still a decent amount more flexibility than just every power of 2 from 8 to 128.

## floats

Floating point values are IEEE754-2008 values (our `f32` and `f64` are identical to Rust's).

`f16` values are encoded by first writing the byte `0x03`, followed by the bytes of the float value itself (2 bytes).

`f32` values are encoded by first writing the byte `0x04`, followed by the bytes of the float value itself (4 bytes).

`f64` values are encoded by first writing the byte `0x05`, followed by the bytes of the float value itself (8 bytes).

`f128` values are encoded by first writing the byte `0x06`, followed by the bytes of the float value itself (16 bytes).

`f256` values are encoded by first writing the byte `0x07`, followed by the bytes of the float value itself (32 bytes).

`f16`, `f128`, and `f256` types are included in the spec, but should not actually be used for now, as support for them is very rare.

## strings

Strings are strictly UTF-8 strings, like Rust's `String`/`str` types.

If the string has a byte length within the range `1..=32` (note, _does not_ include length zero bytes), use a marker within the range from `0x80` to `0x9f` to encode its length. For example, `0x80` means string with length 1, `0x81` length 2, `0x82` length 3, ... `0x9f` length 32.

If the string is either 0 bytes long (ie. empty string), or is length 33 bytes or longer, first encode the marker byte `0x08`, followed by an unsigned variable length integer encoding the byte length of the string.

After writing the marker byte(s), write the string in verbatim (ie. during deserialisation you should be able to zero-copy deserialise it just by doing something like `std::str::from_str(&input[pos..pos + len]`, where `len` is the length of the string, and `pos` is the current position in deserialisation)).

## chars

A char is an integer value representing a unicode codepoint.

A codepoint is any value between `0` and `0x10ffff`, excluding the surrogate ranges (`0xd800` to `0xdfff`).

Since the range of values is small enough to fit in a u24, we do just that. First encode the char marker (`0xc0`), then the codepoint value in the following 3 bytes, for a total of 4 bytes used.

## arrays

Arrays are like a list of serialised values. There are specialised array types available, but this one is the most "generic" array type, able to encode lists of anything that can be encoded at all.

If the array has amount of elements within `1..=16` (note: _does not_ include length zero), a marker within range `0xa0` to `0xaf` is used. For example, `0xa0` encodes an array with length 1, and `0xaf` encodes an array with length 16.

If the array has either zero elements, or 17 or more items, first encode the marker byte `0x09`, follwed by an unsigned variable length integer encoding the length of the array.

Next, encode every element in the array, one element at a time, including everything.

## record

Records are key value mappings, from a string to any encodable value. One key-value pair is one entry.

Use marker bytes `0xb0` to `0xb7` to encode a record with an amount of entries within `1..=8` (note, _does not_ include 0 entries).

If the record has either 0 entries or 9 or more entries, first encode the record marker `0x0a`, followed by an unsigned variable length integer encoding the amount of entries.

Then, encode every key value pair in one at a time. First the key, then the value.

Since the key in this type must be a string, we can just always encode its length using an unsigned variable length integer, skipping the string marker. For strings with length 1 to 32, this has no effect on encoded size, but for strings with length 0 or 33 or more, doing this would save 1 byte per string.

## binary

A binary blob of just, binary data.

We made the design decision that any binary data sent through this encoding method will usually exceed the amount of "inlined" markers we give it for encoding its length (which would have been 16, or maybe 8, something quite small for binary data), and felt those marker values would serve better as part of some other type's range.

First encode the binary marker `0x0c`, followed by an unsigned variable length integer encoding the length in bytes.

Then, write the buffer in without modification. Deserialisation should be able to be done zero-copy the same way strings can be.

## map

A map is like a record (key value mappings), except it accepts arbitrary types for the keys as well as strings.

Use marker bytes `0xb8` to `0xbf` to encode a map with a number of entries within `1..=8` (note, _does not_ include 0 entries).

If the map has either 0 entries, or 9 or more entries, first encode the map marker `0x0b`, then an unsigned variable length integer which encodes the amount of entries.

Then, encode every key value pair in one at a time, first the key, then the value.

Note: since we accept arbitrary types for the keys, a marker must be used for the keys here, unlike records.

## interned values, references, and the value registry

The value registry is a global registry of values, where during serialisation, any serialisable value can be "intern"ed into this global registry by serialisers, which will assign a reference to this value to use in the place where the value would have been. This registry is included in the front of the serialised output (only if there are any entries; ie. if the registry is not used, it won't be included), and during deserialisation, it is deserialised first, then used throughout the rest of the deserialisation process to match references with their actual values. Doing this can save many bytes depending on the data. For example, APIs that return an array of structured objects, where all the keys in each object are the same. would benefit from interning all the keys and using references in place of the keys.

During serialisation, you should keep one global store of interned values. This can be a map or a vec. With a vec, the item's index becomes its' reference value (which implies that values _cannot_ be moved into a different index once a reference has been returned). Using a map of some kind, a seperate increment counter (starting from 0, ie. first reference is 0) should be used to keep track of the references, and the reference value should be stored with each entry.

### encoding the value registry

To encode the value registry, first write the intern marker `0x0d`, _twice_. The reason it's written twice is so that the deserialiser can differentiate between the value registry declaration, and malformed data which is trying to start an interned type without the registry written first.

Next, write the amount of entries inside the value registry, encoded as an unsigned variable length integer.

Then, write all the entries. Every entry should be encoded in full, including its type header, and body, since there is no type known otherwise. This is the same encoding method as encoding the body of an array (after the len).

Each interned entry written should have the same index as its reference value. This means that the value with reference value 0 should be written first, followed by the value with ref 1, then ref 2, and so on.

### encoding references

References should be encoded in place of another value.

If the reference value is between 0 and 63, encode it using a marker in `0xc1` to `0xfe` (ie. ref 0 is `0xc1`, and ref 61 is `0xfe`).

If the reference value is 63 or greater, first write the ref marker byte `0xff`, followed by the reference value as an unsigned variable length integer.

### interning values

Any value can be interned. However, some just aren't worth interning. The first 63 references for values interned take up one byte only, after that it takes one byte in addition to the bytes needed to encode the reference value as an unsigned variable length integer.

Values that might be worth interning include floats, strings, arrays, records, maps, and binary buffers. They are worth interning because they are larger than just one or two bytes to encode. But of course, interning values only works well if the items are repeatedly used throughout the document.

Refs are straight up used in the place where you might otherwise expect a value (with a marker).

### records with interned keys

Since keys in records are always strings and as a result can be encoded without a marker, you cannot use an interned value in its place. For this, a map can be used instead, since keys use a marker too, and so can properly recognise a reference.

### nested documents and value registries

It would be possible to take a seperate encoded document, and just plop it inline where any other value would otherwise be expected. However, it may have its own value registry, with of course its own reference values, which would overlap with the parent documents' references. To support nesting arbitrary documents without reserialisation, we must also support arbitrarily nesting value registries.

If a value registry declaration is found when expecting a value instead, don't error, but parse that value registry. Then use only the newly deserialised registry to only deserialise the next value, discarding the registry afterwards.

Embedding another document is putting arbitrary bytes into the output buffer, which can cause issues if the inserted payload is invalid/malformed/malicious. We believe the worst case with malicious inputs is to corrupt data and potentially return arbitrary data. However, in a strict type system like Rust with a serialiser that doesn't access the outside world like the internet (why would it though?), this shouldn't cause RCE bugs or anything like that. It could in a language like javascript though, if an implementation isn't careful about eg. prototype pollution. A document inserted like this should either come from a trusted source, or validated for correct _structure_. Checking for correct structure would mean interpreting the markers and lengths to know how far to skip forward, but skipping validation that isn't needed for skipping ahead (ex. read the string marker and the length, but skip UTF-8 validation). This ensures the structure is intact and can't cause things to become messy during deserialisation.

If the document you want to embed is untrusted and/or unverified, and verifying it is not feasible for whatever reason, you can put a "foreign document" header just before putting the document in. This header is optional, and you don't need to use it if the document you're putting in is from a trusted source and/or validated to be structurally correct. First write the intern marker `0x0d`, followed by `0x00` (yes its the marker for `false`.. it works since no where else is `false` written just after the intern marker), followed by the length in bytes of the document to be embedded as an unsigned variable length integer. Then, write the document. In deserialisation, if this header is found, hold on to the position and decoded length just before the embedded document starts (ie. just after the encoded length), then try deserialising. If deserialisation fails, you may be able to recover and continue by skipping forwards the saved amount of bytes, past the embedded document, and continuing after that.

## variable length integers

these can be used to encode up to a 16 byte (128 bit) integer

There are two variants to this encoding: unsigned only and signed only. Depending on context, one or the other are used. For example, in a context where a length is expected, the unsigned encoding would be used.

### unsigned variable length integer

- `0x00` to `0xf7` - encodes 0 to 247
- `0xf8` - next byte encodes u8
- `0xf9` - next 2 bytes encodes u16
- `0xfa` - next 3 bytes encodes u24
- `0xfb` - next 4 bytes encodes u32
- `0xfc` - next 6 bytes encodes u48
- `0xfd` - next 8 bytes encodes u64
- `0xfe` - next 12 bytes encodes u96
- `0xff` - next 16 bytes encodes u128

### signed variable length integer

- `0x84` to `0xff` - encodes -124 to -1 (two's compliment)
- `0x00` to `0x7b` - encodes 0 to 123
- `0x7c` - next byte encodes i8
- `0x7d` - next 2 bytes encodes i16
- `0x7e` - next 3 bytes encodes i24
- `0x7f` - next 4 bytes encodes i32
- `0x80` - next 6 bytes encodes i48
- `0x81` - next 8 bytes encodes i64
- `0x82` - next 12 bytes encodes i96
- `0x83` - next 16 bytes encodes i128
