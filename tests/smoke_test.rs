use std::mem::MaybeUninit;

use libc::{c_int, c_void};

use minimp3_ex_sys as ffi;

unsafe extern "C" fn read_cb(_buf: *mut c_void, _size: usize, _user_data: *mut c_void) -> usize {
    0
}

unsafe extern "C" fn seek_cb(_position: u64, _user_data: *mut c_void) -> c_int {
    -1
}

#[test]
fn open_dummy_cb() {
    let mut decoder = MaybeUninit::<ffi::mp3dec_ex_t>::uninit();
    let mut io = ffi::mp3dec_io_t {
        read: Some(read_cb),
        seek: Some(seek_cb),
        read_data: std::ptr::null_mut(),
        seek_data: std::ptr::null_mut(),
    };
    let result =
        unsafe { ffi::mp3dec_ex_open_cb(decoder.as_mut_ptr(), &mut io, ffi::MP3D_SEEK_TO_SAMPLE) };
    assert_eq!(result, ffi::MP3D_E_IOERROR);
    unsafe {
        // Even though there was an IO error, we still have to free the reserved memory:
        ffi::mp3dec_ex_close(decoder.as_mut_ptr());
    }
}
