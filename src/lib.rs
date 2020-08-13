/*!
# FFI bindings to the minimp3_ex C library

*/
#![no_std]
#![allow(non_camel_case_types)]

include!("bindings.rs");

#[cfg(feature = "float-output")]
pub type mp3d_sample_t = f32;

#[cfg(not(feature = "float-output"))]
/// Type for decoded samples.
/// If the `float-output` feature is used, this is `f32`.
pub type mp3d_sample_t = i16;
