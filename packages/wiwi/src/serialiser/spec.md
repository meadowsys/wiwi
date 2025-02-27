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

## Marker bytes

## High level binary layout

The serialised format may begin with a "magic number" and a version to help identify it. This can be ommitted if it is clear to the receiving end what the format of the data is. This is signaled by the byte `0xf5`, followed by "wi" and 0 byte (bytes `0x77`, `0x69`, `0x00`).

Following that, there may be a body length / checksum value. See below for details.

Next, there may be a value registry. Signal the start of it with the byte `0xfa`, followed by a variable length unsigned integer, <!-- TODO: link to the below? --> followed by the items in the registry.

Then, there must be a serialised value. There _must_ only be one. If you want multiple root objects, use an array. <!-- TODO: link also -->

## `none`

A `none` value, also known as `null`, `nil`, or `None`. This is encoded with the byte `0xea`.

## bool values

`true` is encoded with `0xe9`, and false is encoded with `0xe8`.

## int values

## floats

## char

## string

## array

## map

## binary
