use core::ffi::{c_uchar, c_uint};

// Platform-dependent type aliases
pub type size_t = libc::size_t;
pub type uintptr_t = libc::uintptr_t;

// lzma type aliases
pub type lzma_bool = c_uchar;
pub type lzma_ret = c_uint;
pub type lzma_action = c_uint;
pub type lzma_check = c_uint;
pub type lzma_vli = u64;
pub type lzma_reserved_enum = c_uint;
pub type lzma_mode = c_uint;
pub type lzma_match_finder = c_uint;
pub type lzma_lzma_state = c_uint;
pub type lzma_delta_type = c_uint;
pub type probability = u16;
