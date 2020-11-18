Rust FFI bindings to the `minimp3_ex` library
=============================================

This crate provides raw FFI bindings to the `minimp3_ex` library for reading
MP3 audio files.

* Crate: https://crates.io/crates/minimp3_ex-sys
* Documentation: https://docs.rs/minimp3_ex-sys

Following the `*-sys` package conventions,
the `minimp3_ex-sys` crate does not define higher-level abstractions over
the native `minimp3_ex` library functions.

The `minimp3` project (<https://github.com/lieff/minimp3>) provides
two header-only libraries:

* `minimp3.h` contains only the two functions `mp3dec_init()` and
  `mp3dec_decode_frame()` (and the necessary type definitions).
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


Usage
-----

Add this to your `Cargo.toml`:

```toml
[dependencies]
minimp3_ex-sys = "0.1"
```


Features
--------

The feature `float-output` changes the output data type (`mp3d_sample_t`)
from `i16` to `f32`.

**WARNING:** This feature doesn't behave like typical Cargo features
because it is not additive.
If multiple instances of `minimp3_ex-sys` appear in the dependency tree
and at least one of them has the `float-output` feature enabled,
it will be enabled for all instances (probably leading to compiler errors).
Hopefully, this doesn't happen in practice.

The function `mp3dec_f32_to_s16()` is only available if the `float-output`
feature is enabled.

The functions ending in `_w` are only available on Windows.


Building the `minimp3_ex` library
---------------------------------

When building this crate, the `minimp3_ex` library is automatically built as
well, using the [cc] crate.

[cc]: https://crates.io/crates/cc


Auto-generating the Rust bindings
---------------------------------

The Rust bindings have already been auto-generated with [bindgen]
(using the `bindgen/run-bindgen.sh` script) and are part of this crate
(see `src/bindings.rs`).

[bindgen]: https://crates.io/crates/bindgen


Contributing
------------

If you want to report a problem or suggest an improvement, please go to
<https://github.com/mgeier/minimp3_ex-sys>.
Contributions are always welcome!
