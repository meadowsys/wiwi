# Spec&trade;

## Data model

- none (ie. null)
- bool
- (signed) integers of varying size (i8, i16, i24, i32, i48, i64, i96, and arbitrary length bigint)
- floating point numbers (f16, f32, f64, f128, f256 are specified, but realistically only f32 and f64 will be actually used)
- char (Unicode codepoint)
- string, strictly UTF-8
- array
- map
- binary (byte array)

## High level binary layout

The serialised format may begin with a "magic number" and a version to help identify it. This can be ommitted if it is clear to the receiving end what the format of the data is. This is signaled by the byte `0xf6`, followed by "wi" and 0 byte (bytes `0x77`, `0x69`, `0x00`).

Following that, there may be a body length / checksum value. See below for details.

Next, there may be a value registry. Signal the start of it with the byte `0xfa`, followed by a variable length unsigned integer, <!-- TODO: link to the below? --> followed by the items in the registry.

Then, there must be a serialised value. There _must_ only be one. If you want multiple root objects, use an array. <!-- TODO: link also -->

## `none`

A `none` value, also known as `null`, `nil`, or `None`. This is encoded with the byte `0xe9`.

## bool values

`true` is encoded with `0xeb`, and false is encoded with `0xea`.

## int values

## floats

## char

## string

## array

## map

## binary

## Marker bytes

- 00000000 (`0x00`) - \[value reference 0\]
- 00000001 (`0x01`) - \[value reference 1\]
- 00000010 (`0x02`) - \[value reference 2\]
- 00000011 (`0x03`) - \[value reference 3\]
- 00000100 (`0x04`) - \[value reference 4\]
- 00000101 (`0x05`) - \[value reference 5\]
- 00000110 (`0x06`) - \[value reference 6\]
- 00000111 (`0x07`) - \[value reference 7\]
- 00001000 (`0x08`) - \[value reference 8\]
- 00001001 (`0x09`) - \[value reference 9\]
- 00001010 (`0x0a`) - \[value reference 10\]
- 00001011 (`0x0b`) - \[value reference 11\]
- 00001100 (`0x0c`) - \[value reference 12\]
- 00001101 (`0x0d`) - \[value reference 13\]
- 00001110 (`0x0e`) - \[value reference 14\]
- 00001111 (`0x0f`) - \[value reference 15\]
- 00010000 (`0x10`) - \[value reference 16\]
- 00010001 (`0x11`) - \[value reference 17\]
- 00010010 (`0x12`) - \[value reference 18\]
- 00010011 (`0x13`) - \[value reference 19\]
- 00010100 (`0x14`) - \[value reference 20\]
- 00010101 (`0x15`) - \[value reference 21\]
- 00010110 (`0x16`) - \[value reference 22\]
- 00010111 (`0x17`) - \[value reference 23\]
- 00011000 (`0x18`) - \[value reference 24\]
- 00011001 (`0x19`) - \[value reference 25\]
- 00011010 (`0x1a`) - \[value reference 26\]
- 00011011 (`0x1b`) - \[value reference 27\]
- 00011100 (`0x1c`) - \[value reference 28\]
- 00011101 (`0x1d`) - \[value reference 29\]
- 00011110 (`0x1e`) - \[value reference 30\]
- 00011111 (`0x1f`) - \[value reference 31\]
- 00100000 (`0x20`) - \[value reference 32\]
- 00100001 (`0x21`) - \[value reference 33\]
- 00100010 (`0x22`) - \[value reference 34\]
- 00100011 (`0x23`) - \[value reference 35\]
- 00100100 (`0x24`) - \[value reference 36\]
- 00100101 (`0x25`) - \[value reference 37\]
- 00100110 (`0x26`) - \[value reference 38\]
- 00100111 (`0x27`) - \[value reference 39\]
- 00101000 (`0x28`) - \[value reference 40\]
- 00101001 (`0x29`) - \[value reference 41\]
- 00101010 (`0x2a`) - \[value reference 42\]
- 00101011 (`0x2b`) - \[value reference 43\]
- 00101100 (`0x2c`) - \[value reference 44\]
- 00101101 (`0x2d`) - \[value reference 45\]
- 00101110 (`0x2e`) - \[value reference 46\]
- 00101111 (`0x2f`) - \[value reference 47\]
- 00110000 (`0x30`) - \[value reference 48\]
- 00110001 (`0x31`) - \[value reference 49\]
- 00110010 (`0x32`) - \[value reference 50\]
- 00110011 (`0x33`) - \[value reference 51\]
- 00110100 (`0x34`) - \[value reference 52\]
- 00110101 (`0x35`) - \[value reference 53\]
- 00110110 (`0x36`) - \[value reference 54\]
- 00110111 (`0x37`) - \[value reference 55\]
- 00111000 (`0x38`) - \[value reference 56\]
- 00111001 (`0x39`) - \[value reference 57\]
- 00111010 (`0x3a`) - \[value reference 58\]
- 00111011 (`0x3b`) - \[value reference 59\]
- 00111100 (`0x3c`) - \[value reference 60\]
- 00111101 (`0x3d`) - \[value reference 61\]
- 00111110 (`0x3e`) - \[value reference 62\]
- 00111111 (`0x3f`) - \[value reference 63\]
- 01000000 (`0x40`) - \[int 0\]
- 01000001 (`0x41`) - \[int 1\]
- 01000010 (`0x42`) - \[int 2\]
- 01000011 (`0x43`) - \[int 3\]
- 01000100 (`0x44`) - \[int 4\]
- 01000101 (`0x45`) - \[int 5\]
- 01000110 (`0x46`) - \[int 6\]
- 01000111 (`0x47`) - \[int 7\]
- 01001000 (`0x48`) - \[int 8\]
- 01001001 (`0x49`) - \[int 9\]
- 01001010 (`0x4a`) - \[int 10\]
- 01001011 (`0x4b`) - \[int 11\]
- 01001100 (`0x4c`) - \[int 12\]
- 01001101 (`0x4d`) - \[int 13\]
- 01001110 (`0x4e`) - \[int 14\]
- 01001111 (`0x4f`) - \[int 15\]
- 01010000 (`0x50`) - \[int 16\]
- 01010001 (`0x51`) - \[int 17\]
- 01010010 (`0x52`) - \[int 18\]
- 01010011 (`0x53`) - \[int 19\]
- 01010100 (`0x54`) - \[int 20\]
- 01010101 (`0x55`) - \[int 21\]
- 01010110 (`0x56`) - \[int 22\]
- 01010111 (`0x57`) - \[int 23\]
- 01011000 (`0x58`) - \[int 24\]
- 01011001 (`0x59`) - \[int 25\]
- 01011010 (`0x5a`) - \[int 26\]
- 01011011 (`0x5b`) - \[int 27\]
- 01011100 (`0x5c`) - \[int 28\]
- 01011101 (`0x5d`) - \[int 29\]
- 01011110 (`0x5e`) - \[int 30\]
- 01011111 (`0x5f`) - \[int 31\]
- 01100000 (`0x60`) - \[int -32\]
- 01100001 (`0x61`) - \[int -31\]
- 01100010 (`0x62`) - \[int -30\]
- 01100011 (`0x63`) - \[int -29\]
- 01100100 (`0x64`) - \[int -28\]
- 01100101 (`0x65`) - \[int -27\]
- 01100110 (`0x66`) - \[int -26\]
- 01100111 (`0x67`) - \[int -25\]
- 01101000 (`0x68`) - \[int -24\]
- 01101001 (`0x69`) - \[int -23\]
- 01101010 (`0x6a`) - \[int -22\]
- 01101011 (`0x6b`) - \[int -21\]
- 01101100 (`0x6c`) - \[int -20\]
- 01101101 (`0x6d`) - \[int -19\]
- 01101110 (`0x6e`) - \[int -18\]
- 01101111 (`0x6f`) - \[int -17\]
- 01110000 (`0x70`) - \[int -16\]
- 01110001 (`0x71`) - \[int -15\]
- 01110010 (`0x72`) - \[int -14\]
- 01110011 (`0x73`) - \[int -13\]
- 01110100 (`0x74`) - \[int -12\]
- 01110101 (`0x75`) - \[int -11\]
- 01110110 (`0x76`) - \[int -10\]
- 01110111 (`0x77`) - \[int -9\]
- 01111000 (`0x78`) - \[int -8\]
- 01111001 (`0x79`) - \[int -7\]
- 01111010 (`0x7a`) - \[int -6\]
- 01111011 (`0x7b`) - \[int -5\]
- 01111100 (`0x7c`) - \[int -4\]
- 01111101 (`0x7d`) - \[int -3\]
- 01111110 (`0x7e`) - \[int -2\]
- 01111111 (`0x7f`) - \[int -1\]
- 10000000 (`0x80`) - \[string 0\]
- 10000001 (`0x81`) - \[string 1\]
- 10000010 (`0x82`) - \[string 2\]
- 10000011 (`0x83`) - \[string 3\]
- 10000100 (`0x84`) - \[string 4\]
- 10000101 (`0x85`) - \[string 5\]
- 10000110 (`0x86`) - \[string 6\]
- 10000111 (`0x87`) - \[string 7\]
- 10001000 (`0x88`) - \[string 8\]
- 10001001 (`0x89`) - \[string 9\]
- 10001010 (`0x8a`) - \[string 10\]
- 10001011 (`0x8b`) - \[string 11\]
- 10001100 (`0x8c`) - \[string 12\]
- 10001101 (`0x8d`) - \[string 13\]
- 10001110 (`0x8e`) - \[string 14\]
- 10001111 (`0x8f`) - \[string 15\]
- 10010000 (`0x90`) - \[string 16\]
- 10010001 (`0x91`) - \[string 17\]
- 10010010 (`0x92`) - \[string 18\]
- 10010011 (`0x93`) - \[string 19\]
- 10010100 (`0x94`) - \[string 20\]
- 10010101 (`0x95`) - \[string 21\]
- 10010110 (`0x96`) - \[string 22\]
- 10010111 (`0x97`) - \[string 23\]
- 10011000 (`0x98`) - \[array 0\]
- 10011001 (`0x99`) - \[array 1\]
- 10011010 (`0x9a`) - \[array 2\]
- 10011011 (`0x9b`) - \[array 3\]
- 10011100 (`0x9c`) - \[array 4\]
- 10011101 (`0x9d`) - \[array 5\]
- 10011110 (`0x9e`) - \[array 6\]
- 10011111 (`0x9f`) - \[array 7\]
- 10100000 (`0xa0`) - \[array 8\]
- 10100001 (`0xa1`) - \[array 9\]
- 10100010 (`0xa2`) - \[array 10\]
- 10100011 (`0xa3`) - \[array 11\]
- 10100100 (`0xa4`) - \[array 12\]
- 10100101 (`0xa5`) - \[array 13\]
- 10100110 (`0xa6`) - \[array 14\]
- 10100111 (`0xa7`) - \[array 15\]
- 10101000 (`0xa8`) - \[array 16\]
- 10101001 (`0xa9`) - \[array 17\]
- 10101010 (`0xaa`) - \[array 18\]
- 10101011 (`0xab`) - \[array 19\]
- 10101100 (`0xac`) - \[array 20\]
- 10101101 (`0xad`) - \[array 21\]
- 10101110 (`0xae`) - \[array 22\]
- 10101111 (`0xaf`) - \[array 23\]
- 10110000 (`0xb0`) - \[map 0\]
- 10110001 (`0xb1`) - \[map 1\]
- 10110010 (`0xb2`) - \[map 2\]
- 10110011 (`0xb3`) - \[map 3\]
- 10110100 (`0xb4`) - \[map 4\]
- 10110101 (`0xb5`) - \[map 5\]
- 10110110 (`0xb6`) - \[map 6\]
- 10110111 (`0xb7`) - \[map 7\]
- 10111000 (`0xb8`) - \[map 8\]
- 10111001 (`0xb9`) - \[map 9\]
- 10111010 (`0xba`) - \[map 10\]
- 10111011 (`0xbb`) - \[map 11\]
- 10111100 (`0xbc`) - \[map 12\]
- 10111101 (`0xbd`) - \[map 13\]
- 10111110 (`0xbe`) - \[map 14\]
- 10111111 (`0xbf`) - \[map 15\]
- 11000000 (`0xc0`) - \[map 16\]
- 11000001 (`0xc1`) - \[map 17\]
- 11000010 (`0xc2`) - \[map 18\]
- 11000011 (`0xc3`) - \[map 19\]
- 11000100 (`0xc4`) - \[map 20\]
- 11000101 (`0xc5`) - \[map 21\]
- 11000110 (`0xc6`) - \[map 22\]
- 11000111 (`0xc7`) - \[map 23\]
- 11001000 (`0xc8`) - \[binary 0\]
- 11001001 (`0xc9`) - \[binary 1\]
- 11001010 (`0xca`) - \[binary 2\]
- 11001011 (`0xcb`) - \[binary 3\]
- 11001100 (`0xcc`) - \[binary 4\]
- 11001101 (`0xcd`) - \[binary 5\]
- 11001110 (`0xce`) - \[binary 6\]
- 11001111 (`0xcf`) - \[binary 7\]
- 11010000 (`0xd0`) - \[binary 8\]
- 11010001 (`0xd1`) - \[binary 9\]
- 11010010 (`0xd2`) - \[binary 10\]
- 11010011 (`0xd3`) - \[binary 11\]
- 11010100 (`0xd4`) - \[binary 12\]
- 11010101 (`0xd5`) - \[binary 13\]
- 11010110 (`0xd6`) - \[binary 14\]
- 11010111 (`0xd7`) - \[binary 15\]
- 11011000 (`0xd8`) - \[binary 16\]
- 11011001 (`0xd9`) - \[binary 17\]
- 11011010 (`0xda`) - \[binary 18\]
- 11011011 (`0xdb`) - \[binary 19\]
- 11011100 (`0xdc`) - \[binary 20\]
- 11011101 (`0xdd`) - \[binary 21\]
- 11011110 (`0xde`) - \[binary 22\]
- 11011111 (`0xdf`) - \[binary 23\]
- 11100000 (`0xe0`) - i8
- 11100001 (`0xe1`) - i16
- 11100010 (`0xe2`) - i24
- 11100011 (`0xe3`) - i32
- 11100100 (`0xe4`) - i48
- 11100101 (`0xe5`) - i64
- 11100110 (`0xe6`) - i96
- 11100111 (`0xe7`) - arbitrary length bigint (unsigned)
- 11101000 (`0xe8`) - arbitrary length bigint (signed)
- 11101001 (`0xe9`) - none
- 11101010 (`0xea`) - false
- 11101011 (`0xeb`) - true
- 11101100 (`0xec`) - f16
- 11101101 (`0xed`) - f32
- 11101110 (`0xee`) - f64
- 11101111 (`0xef`) - f128
- 11110000 (`0xf0`) - f256
- 11110001 (`0xf1`) - string (utf-8)
- 11110010 (`0xf2`) - array
- 11110011 (`0xf3`) - map
- 11110100 (`0xf4`) - binary
- 11110101 (`0xf5`) - char (24 bit (3 byte) unicode value)  -----------
- 11110110 (`0xf6`) - begin magic number sequence
- 11110111 (`0xf7`) - body length
- 11111000 (`0xf8`) - body length + checksum (sha3-512)
- 11111001 (`0xf9`) - body length + arbitrary length checksum (shake256)
- 11111010 (`0xfa`) - value registry
- 11111011 (`0xfb`) - value reference
- 11111100 (`0xfc`) - unassigned 1
- 11111101 (`0xfd`) - unassigned 2
- 11111110 (`0xfe`) - reserved 2 byte markers (additional 256 markers)
- 11111111 (`0xff`) - reserved 3 byte markers (additional 65536 markers)
