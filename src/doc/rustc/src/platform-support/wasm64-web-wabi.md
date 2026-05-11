# `wasm64-web-wabi`

**Tier: 3**

The `wasm64-web-wabi` target is a WebAssembly compilation target for web
environments. It is based on [`wasm64-unknown-unknown`](./wasm64-unknown-unknown.md)
and uses 64-bit linear memories.

This target uses `target_os = "web"` and `target_env = "wabi"`. The default
linker is `wabi-ld`, which wraps `rust-lld`, passes wabi-specific linker
arguments, and produces the final WebAssembly file.

## Target maintainers

[@xxx](https://github.com/xxx)
[@xxx](https://github.com/xxx)
[@xxx](https://github.com/xxx)

## Requirements

This target is cross-compiled. The target includes support for `std`, `core`,
and `alloc`, with a default allocator inherited from the generic WebAssembly
target configuration.

The standard library supports a small set of host facilities:

* `std::io`, for `stdout` and `stderr` only, using `console.log` and
  `console.error`
* `std::time`
* `std::random`
* `std::env::args`, primarily for tests run by `wabi-runner` and other
  binaries, using the `argc` and `argv` passed to `main`

Other operating-system facilities are still unsupported in the same broad sense
as `wasm64-unknown-unknown`. For example, filesystem, networking, process, and
thread APIs should not be assumed to work unless the standard library grows
explicit support for them.

## Building the target

To build this target, add `wasm64-web-wabi` to the `target` list in
`bootstrap.toml`.

## Building Rust programs

This target requires building the standard library with `build-std`:

The SDK used by `WABI_SDK` can be downloaded from the
[`wasm-bindgen/wabi`](https://github.com/wasm-bindgen/wabi).

```sh
$ WABI_SDK=/path/to/wabi-sdk cargo +nightly build -Zbuild-std --target wasm64-web-wabi
```

## Cross-compilation

This target can be cross-compiled from any host with the WebAssembly LLVM
backend, LLD, `wabi-ld`, and `wabi-sdk`.

## Testing

This target is not currently tested in CI for the rust-lang/rust repository.

`libtest` is supported, but tests must be run through `wabi-runner`.

For example:

```sh
$ CARGO_TARGET_WASM64_WEB_WABI_RUNNER=wabi-runner \
    cargo +nightly test -Zbuild-std --target wasm64-web-wabi
```

## Conditionally compiling code

It's recommended to conditionally compile code for this target with:

```text
#[cfg(all(target_family = "wasm", target_os = "web", target_env = "wabi"))]
```

Use `target_arch = "wasm64"` as an additional condition when code must
distinguish this target from `wasm32-web-wabi`.

## Enabled WebAssembly features

The default set of WebAssembly features enabled for compilation is currently the
same as [`wasm64-unknown-unknown`](./wasm64-unknown-unknown.md). See that
target's documentation for more information about the default WebAssembly
feature set and how to tune it.

## Unwinding

This target is compiled with `-Cpanic=unwind` by default. As with other
WebAssembly targets, unwinding relies on WebAssembly exception handling support
in the engine that runs the final module. See the
[`wasm32-unknown-unknown` unwinding documentation](./wasm32-unknown-unknown.md#unwinding)
for background on WebAssembly unwinding and exception handling.

## Cross-compilation toolchains and C code

The default linker is `wabi-ld`, and the target passes
wabi-specific arguments to the linker. C or C++ interoperation uses an ABI
compatible with `wasm64-unknown-unknown`.
