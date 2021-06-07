# env-libvpx-sys

![](https://github.com/astraw/env-libvpx-sys/workflows/Build%20and%20Run/badge.svg)

Rust bindings to libvpx.

**Chose your libvpx version when compiling the final application binary.** When
`env-libvpx-sys` is compiled, cargo features are used to select the version of
libvpx supported. Therefore, you must depend on `env-libvpx-sys` in your
application's `Cargo.toml` file and specify the correct cargo feature for the
version of libvpx you want. There are two ways to do this:

*Method 1: specify libvpx version directly in `Cargo.toml`:*

Example `Cargo.toml` snippet:

```
[dependencies]
# In this example v1.10.x is selected:
env-libvpx-sys = {version="6", features=["v1_10"]}
```

*Method 2: specify libvpx version when invoking cargo*

Example `Cargo.toml` snippet:

```
[dependencies]
env-libvpx-sys = "6"
```

Then, when invoking cargo at the command line:

```
cargo run --features env-libvpx-sys/v1_10
```

This began as a fork of
[libvpx-native-sys](https://crates.io/crates/libvpx-native-sys) with a [fix to
simplify working with Windows](https://github.com/kornelski/rust-vpx/pull/1).
Since then, the focus has been on:

 * providing only the `-sys` layer. VPX header files are wrapped with bindgen
   and the native library is linked. However, no higher-level Rust interface
   is provided. (See the [vpx-encode crate](https://crates.io/crates/vpx-encode) for
   a simple higher-level interface).
 * adding [Continuous Integration tests for Windows, Linux and
   Mac](https://github.com/astraw/env-libvpx-sys/actions).

Includes bundled bindgen-generated FFI wrapper for a few versions of libvpx. You
can also enable `generate` feature of this crate to generate FFI on the fly for
a custom version of libvpx.
