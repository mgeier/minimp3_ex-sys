//! Raw FFI bindings to the [`minimp3_ex`] library for reading MP3 audio files.
//!
//! [`minimp3_ex`]: https://github.com/lieff/minimp3/blob/master/minimp3_ex.h
//!
//! The feature `float-output` changes the output data type ([`mp3d_sample_t`])
//! from `i16` to `f32`.
//!
//! The function [`mp3dec_f32_to_s16()`] is only available if the `float-output`
//! feature is enabled.
//!
//! The functions ending in `_w` are only available on Windows.

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
