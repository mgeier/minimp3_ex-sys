/*!
This crate provides raw FFI bindings to the `minimp3_ex` library for reading
MP3 audio files.

Following the `*-sys` package conventions,
the `minimp3_ex-sys` crate does not define higher-level abstractions over
the native `minimp3_ex` library functions.

The `minimp3` project (<https://github.com/lieff/minimp3>) provides
two header-only libraries:

* `minimp3.h` contains only the two functions [`mp3dec_init()`] and
  [`mp3dec_decode_frame()`] (and the necessary type definitions).
* `minimp3_ex.h` (which includes `minimp3.h`) is a bit less "mini" and contains
  many more functions, including for opening files, seeking, using buffers,
  using callback functions, ...

This crate provides bindings for all functions from both header files.

If you only need the `minimp3.h` functionality, you can also use the
[minimp3-sys] crate.

There are also some higher-level crates based on `minimp3.h`:

* [minimp3] (using [minimp3-sys])
* [rmp3] (using its own bindings)

[minimp3-sys]: https://crates.io/crates/minimp3-sys
[minimp3]: https://crates.io/crates/minimp3
[rmp3]: https://crates.io/crates/rmp3

And of course <https://crates.io/> provides a multitude of further MP3-related
libraries for Rust.

# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
minimp3_ex-sys = "0.1"
```

# Features

The features of this crate don't behave like typical Rust features,
e.g. they are not necessarily additive.

They are merely providing a way to set the `MINIMP3_*` preprocessor definitions
used in `minimp3.h` and `minimp3_ex.h`.

* `float-output`
* `no-simd`
* `only-simd`
* `only-mp3`
* `nonstandard-but-logical`
* `no-stdio`
* `allow-mono-stereo-transition`

The function [`mp3dec_f32_to_s16()`] is only available if the `float-output`
feature is enabled.

The type [`mp3d_sample_t`] changes from `i16` to `f32` when using
`float-output`.

The functions ending in `_w` are only available on Windows.

# Building the `minimp3_ex` library

When building this crate, the `minimp3_ex` library is automatically built as
well, using the [cc] crate.

[cc]: https://crates.io/crates/cc

# Auto-generating the Rust bindings

The Rust bindings have already been auto-generated with [bindgen]
(using the `bindgen/run-bindgen.sh` script) and are part of this crate
(see `src/bindings.rs`).

[bindgen]: https://crates.io/crates/bindgen

# Contributing

If you want to report a problem or suggest an improvement, please go to
<https://github.com/mgeier/minimp3_ex-sys>.
Contributions are always welcome!
*/
#![no_std]
#![allow(non_camel_case_types)]

include!("bindings.rs");

#[cfg(feature = "float-output")]
pub type mp3d_sample_t = f32;

#[cfg(not(feature = "float-output"))]
/// Type for decoded samples.
///
/// If the `float-output` feature is used, this is `f32`.
pub type mp3d_sample_t = i16;
