extern "C" {
    fn lzma_raw_encoder_memusage(filters: *const lzma_filter) -> uint64_t;
    fn lzma_easy_preset(easy: *mut lzma_options_easy, preset: uint32_t) -> bool;
}
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type lzma_reserved_enum = ::core::ffi::c_uint;
pub const LZMA_RESERVED_ENUM: lzma_reserved_enum = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter {
    pub id: lzma_vli,
    pub options: *mut ::core::ffi::c_void,
}
pub type lzma_vli = uint64_t;
pub type lzma_match_finder = ::core::ffi::c_uint;
pub const LZMA_MF_BT4: lzma_match_finder = 20;
pub const LZMA_MF_BT3: lzma_match_finder = 19;
pub const LZMA_MF_BT2: lzma_match_finder = 18;
pub const LZMA_MF_HC4: lzma_match_finder = 4;
pub const LZMA_MF_HC3: lzma_match_finder = 3;
pub type lzma_mode = ::core::ffi::c_uint;
pub const LZMA_MODE_NORMAL: lzma_mode = 2;
pub const LZMA_MODE_FAST: lzma_mode = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_options_lzma {
    pub dict_size: uint32_t,
    pub preset_dict: *const uint8_t,
    pub preset_dict_size: uint32_t,
    pub lc: uint32_t,
    pub lp: uint32_t,
    pub pb: uint32_t,
    pub mode: lzma_mode,
    pub nice_len: uint32_t,
    pub mf: lzma_match_finder,
    pub depth: uint32_t,
    pub ext_flags: uint32_t,
    pub ext_size_low: uint32_t,
    pub ext_size_high: uint32_t,
    pub reserved_int4: uint32_t,
    pub reserved_int5: uint32_t,
    pub reserved_int6: uint32_t,
    pub reserved_int7: uint32_t,
    pub reserved_int8: uint32_t,
    pub reserved_enum1: lzma_reserved_enum,
    pub reserved_enum2: lzma_reserved_enum,
    pub reserved_enum3: lzma_reserved_enum,
    pub reserved_enum4: lzma_reserved_enum,
    pub reserved_ptr1: *mut ::core::ffi::c_void,
    pub reserved_ptr2: *mut ::core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_options_easy {
    pub filters: [lzma_filter; 5],
    pub opt_lzma: lzma_options_lzma,
}
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
#[no_mangle]
pub unsafe extern "C" fn lzma_easy_encoder_memusage(mut preset: uint32_t) -> uint64_t {
    let mut opt_easy: lzma_options_easy = lzma_options_easy {
        filters: [lzma_filter {
            id: 0,
            options: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        }; 5],
        opt_lzma: lzma_options_lzma {
            dict_size: 0,
            preset_dict: ::core::ptr::null::<uint8_t>(),
            preset_dict_size: 0,
            lc: 0,
            lp: 0,
            pb: 0,
            mode: 0 as lzma_mode,
            nice_len: 0,
            mf: 0 as lzma_match_finder,
            depth: 0,
            ext_flags: 0,
            ext_size_low: 0,
            ext_size_high: 0,
            reserved_int4: 0,
            reserved_int5: 0,
            reserved_int6: 0,
            reserved_int7: 0,
            reserved_int8: 0,
            reserved_enum1: LZMA_RESERVED_ENUM,
            reserved_enum2: LZMA_RESERVED_ENUM,
            reserved_enum3: LZMA_RESERVED_ENUM,
            reserved_enum4: LZMA_RESERVED_ENUM,
            reserved_ptr1: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            reserved_ptr2: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        },
    };
    if lzma_easy_preset(&raw mut opt_easy, preset) {
        return UINT32_MAX as uint64_t;
    }
    return lzma_raw_encoder_memusage(&raw mut opt_easy.filters as *mut lzma_filter);
}
