use libc::{c_int, c_uint, c_ulonglong};

pub type herr_t = c_int;
pub type htri_t = c_int;
pub type hsize_t = c_ulonglong;

extern "C" {
    pub fn H5get_libversion(majnum: *mut c_uint, minnum: *mut c_uint, relnum: *mut c_uint)
                            -> herr_t;
}