# wiwi-wasm

Nicer, higher level APIs for working with JavaScript from Rust (WASM), mostly revolving around builder-style APIs

## MSRV

Rust 1.85, which is, at the time of writing, only available on the beta/nightly release channels.

## API design

If there are any discrepancies between this documentation and the actual API, please file an issue letting us know the name of the item that deviates!

The "base" type of the entire crate is `ExternAny`, representing a reference to any extern (stored in JS) value. The more "primitive" operations, such as `typeof`, are implemented on this type.

Global objects are accessible via top level functions (ex. `Reflect` is available via `reflect()`).
