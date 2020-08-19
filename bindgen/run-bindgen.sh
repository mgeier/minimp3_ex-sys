#!/bin/sh

set -e

cd "$(dirname "$0")"

bindgen wrapper.h -o ../src/bindings.rs \
	--use-core \
	--size_t-is-usize \
	--ctypes-prefix libc \
	--blacklist-type __uint8_t \
	--blacklist-type __int16_t \
	--blacklist-type __uint64_t \
	--whitelist-function "mp3dec_.*" \
	--whitelist-type "mp3dec_.*" \
	--whitelist-var "MINIMP3_MAX_SAMPLES_PER_FRAME" \
	--whitelist-var "MINIMP3_.*_SIZE" \
	--whitelist-var "MP3D_.*" \
	--no-copy ".*" \
	--blacklist-type mp3d_sample_t \
	--

sed -i 's/pub const MP3D_SEEK_TO_\(\w*\): u32/pub const MP3D_SEEK_TO_\1: libc::c_int/' ../src/bindings.rs
sed -i 's/pub const MP3D_DO_NOT_SCAN: u32/pub const MP3D_DO_NOT_SCAN: libc::c_int/' ../src/bindings.rs
sed -i 's/pub const MP3D_E_\(\w*\): i32/pub const MP3D_E_\1: libc::c_int/' ../src/bindings.rs
sed -i 's/pub const MINIMP3_MAX_SAMPLES_PER_FRAME: u32/pub const MINIMP3_MAX_SAMPLES_PER_FRAME: usize/' ../src/bindings.rs
sed -i 's/pub const MINIMP3_\(\w*\)_SIZE: u32/pub const MINIMP3_\1_SIZE: usize/' ../src/bindings.rs
