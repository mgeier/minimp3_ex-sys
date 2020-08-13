use std::env;

fn main() {
    let mut build = cc::Build::new();

    macro_rules! feature {
        ($feature_name:literal) => {
            if env::var(concat!("CARGO_FEATURE_", $feature_name)).is_ok() {
                build.define(concat!("MINIMP3_", $feature_name), None);
            }
        };
    }

    feature!("FLOAT_OUTPUT");
    feature!("NO_SIMD");
    feature!("ONLY_SIMD");
    feature!("ONLY_MP3");
    feature!("NONSTANDARD_BUT_LOGICAL");
    feature!("NO_STDIO");
    feature!("ALLOW_MONO_STEREO_TRANSITION");

    build
        .file("minimp3_ex.c")
        .include("minimp3")
        .compile("minimp3_ex");
}
