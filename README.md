# wiwi

A lil lib containing misc utilities, and Stuff™. Contains some useful things, contains some silly things.

All exposed features are gated behind features, none of which are enabled by default.

## Features

<!-- make sure to check Cargo.toml and workflow files too -->

- **`clock-timer-2`** - An interval tracking clock, yielding ticks at specified intervals and doing so for a specified duration. **Requires an async runtime**
- **`debounce`** - Delay calling a function until a certain time period has passed since the last time it was called. **Requires an async runtime**
- **`h`** - h
- **`lazy-wrap`** - Wrapper around an initialisation function to lazily initialise a value on first access (can be used in statics)
- **`string-pool`** - Global immutable string pool and String type
- **`z85`** - An efficient implementation of [ZeroMQ]'s [z85] format, a format to represent binary data as printable ASCII text. Think base64, but more efficient in encoded size. This implementation is not fully to spec, as it handles padding text to the correct text where the spec says the application code should handle it. (A fully spec compliant version soon&trade;?)

### Async runtime features

Only one can be enabled at a time. utility implementations for **`tokio`** will be prioritised over other runtimes, if/when they are added.

- **`tokio`** - Currently the only available runtime.

### Feature configuration features

These don't change API usage, only some compile time behaviour under the hood.

- **`debounce-dyn-fn`** - Wraps functions into a `Box<dyn Fn>`, to use dynamic dispatch and avoid monomorphisation binary size cost

[zeromq]: https://zeromq.org
[z85]: https://rfc.zeromq.org/spec/32
