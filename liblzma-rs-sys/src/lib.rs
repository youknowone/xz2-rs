//! liblzma-sys compatible API layer backed by pure Rust liblzma-rs
//!
//! Re-exports symbols from liblzma-rs with the same names and signatures
//! as liblzma-sys, enabling drop-in replacement.
//!
//! Because c2rust generates per-file type definitions, this layer provides
//! canonical types and thin wrapper functions that cast between structurally
//! identical `#[repr(C)]` types.

#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_imports,
    clippy::all,
)]

use core::ffi::{c_uchar, c_uint, c_void};

// === Canonical type aliases (all c_uint, transparent across modules) ===
pub type lzma_ret = c_uint;
pub type lzma_action = c_uint;
pub type lzma_check = c_uint;
pub type lzma_mode = c_uint;
pub type lzma_match_finder = c_uint;
pub type lzma_bool = c_uchar;
pub type lzma_vli = u64;

// === Canonical struct re-exports (from common::common) ===
pub use liblzma_rs::common::common::lzma_stream;
pub use liblzma_rs::common::common::lzma_allocator;
pub use liblzma_rs::common::filter_common::lzma_options_lzma;
pub use liblzma_rs::delta::delta_common::lzma_filter;
pub use liblzma_rs::common::index_decoder::lzma_index;
pub use liblzma_rs::common::stream_flags_decoder::lzma_stream_flags;

// === Return codes ===
pub use liblzma_rs::common::common::{
    LZMA_OK, LZMA_STREAM_END, LZMA_NO_CHECK, LZMA_UNSUPPORTED_CHECK,
    LZMA_GET_CHECK, LZMA_MEM_ERROR, LZMA_MEMLIMIT_ERROR, LZMA_FORMAT_ERROR,
    LZMA_OPTIONS_ERROR, LZMA_DATA_ERROR, LZMA_BUF_ERROR, LZMA_PROG_ERROR,
};

// === Actions ===
pub use liblzma_rs::common::common::{
    LZMA_RUN, LZMA_SYNC_FLUSH, LZMA_FULL_FLUSH, LZMA_FULL_BARRIER, LZMA_FINISH,
};

// === Check types ===
pub use liblzma_rs::check::check::{
    LZMA_CHECK_NONE, LZMA_CHECK_CRC32, LZMA_CHECK_CRC64, LZMA_CHECK_SHA256,
};

// === Modes ===
pub use liblzma_rs::common::filter_common::{
    LZMA_MODE_FAST, LZMA_MODE_NORMAL,
};

// === Match finders ===
pub use liblzma_rs::common::filter_common::{
    LZMA_MF_HC3, LZMA_MF_HC4, LZMA_MF_BT2, LZMA_MF_BT3, LZMA_MF_BT4,
};

// === Presets ===
pub use liblzma_rs::common::string_conversion::{LZMA_PRESET_DEFAULT, LZMA_PRESET_EXTREME};
pub use liblzma_rs::lzma::lzma_encoder_presets::LZMA_PRESET_LEVEL_MASK;

// === Flags ===
pub use liblzma_rs::common::auto_decoder::{
    LZMA_CONCATENATED, LZMA_TELL_NO_CHECK, LZMA_TELL_UNSUPPORTED_CHECK,
    LZMA_TELL_ANY_CHECK,
};

// === VLI ===
pub use liblzma_rs::common::common::LZMA_VLI_UNKNOWN;

// === Stream header size ===
pub use liblzma_rs::common::index::LZMA_STREAM_HEADER_SIZE;

// === Filter IDs ===
pub use liblzma_rs::common::filter_decoder::{
    LZMA_FILTER_X86, LZMA_FILTER_POWERPC, LZMA_FILTER_IA64,
    LZMA_FILTER_ARM, LZMA_FILTER_ARMTHUMB, LZMA_FILTER_SPARC,
    LZMA_FILTER_ARM64, LZMA_FILTER_RISCV,
    LZMA_FILTER_LZMA1, LZMA_FILTER_LZMA2,
    LZMA_FILTER_DELTA,
};

// =========================================================================
// Functions
// =========================================================================
//
// Functions defined in common::common use the canonical lzma_stream type,
// so they can be re-exported directly. Functions in other modules use
// module-local struct definitions that are structurally identical (#[repr(C)])
// but nominally different, so we provide thin wrappers with pointer casts.

// --- Direct re-exports (canonical types) ---
pub use liblzma_rs::common::common::{lzma_code, lzma_end, lzma_memlimit_get, lzma_memlimit_set};

// --- Wrapper functions (module-local types → canonical types) ---

#[inline]
pub unsafe fn lzma_easy_encoder(
    strm: *mut lzma_stream,
    preset: u32,
    check: lzma_check,
) -> lzma_ret {
    liblzma_rs::common::easy_encoder::lzma_easy_encoder(strm.cast(), preset, check)
}

#[inline]
pub unsafe fn lzma_alone_encoder(
    strm: *mut lzma_stream,
    options: *const lzma_options_lzma,
) -> lzma_ret {
    liblzma_rs::common::alone_encoder::lzma_alone_encoder(strm.cast(), options.cast())
}

#[inline]
pub unsafe fn lzma_stream_encoder(
    strm: *mut lzma_stream,
    filters: *const lzma_filter,
    check: lzma_check,
) -> lzma_ret {
    liblzma_rs::common::stream_encoder::lzma_stream_encoder(strm.cast(), filters.cast(), check)
}

#[inline]
pub unsafe fn lzma_stream_decoder(
    strm: *mut lzma_stream,
    memlimit: u64,
    flags: u32,
) -> lzma_ret {
    liblzma_rs::common::stream_decoder::lzma_stream_decoder(strm.cast(), memlimit, flags)
}

#[inline]
pub unsafe fn lzma_alone_decoder(
    strm: *mut lzma_stream,
    memlimit: u64,
) -> lzma_ret {
    liblzma_rs::common::alone_decoder::lzma_alone_decoder(strm.cast(), memlimit)
}

#[inline]
pub unsafe fn lzma_auto_decoder(
    strm: *mut lzma_stream,
    memlimit: u64,
    flags: u32,
) -> lzma_ret {
    liblzma_rs::common::auto_decoder::lzma_auto_decoder(strm.cast(), memlimit, flags)
}

#[inline]
pub unsafe fn lzma_lzip_decoder(
    strm: *mut lzma_stream,
    memlimit: u64,
    flags: u32,
) -> lzma_ret {
    liblzma_rs::common::lzip_decoder::lzma_lzip_decoder(strm.cast(), memlimit, flags)
}

#[inline]
pub unsafe fn lzma_raw_encoder(
    strm: *mut lzma_stream,
    filters: *const lzma_filter,
) -> lzma_ret {
    liblzma_rs::common::filter_encoder::lzma_raw_encoder(strm.cast(), filters.cast())
}

#[inline]
pub unsafe fn lzma_raw_decoder(
    strm: *mut lzma_stream,
    filters: *const lzma_filter,
) -> lzma_ret {
    liblzma_rs::common::filter_decoder::lzma_raw_decoder(strm.cast(), filters.cast())
}

#[inline]
pub unsafe fn lzma_mt_block_size(
    filters: *const lzma_filter,
) -> u64 {
    liblzma_rs::common::filter_encoder::lzma_mt_block_size(filters.cast())
}

#[inline]
pub unsafe fn lzma_properties_decode(
    filter: *mut lzma_filter,
    allocator: *const lzma_allocator,
    props: *const u8,
    props_size: usize,
) -> lzma_ret {
    liblzma_rs::common::filter_decoder::lzma_properties_decode(
        filter.cast(), allocator.cast(), props, props_size,
    )
}

#[inline]
pub unsafe fn lzma_check_is_supported(check: lzma_check) -> lzma_bool {
    liblzma_rs::check::check::lzma_check_is_supported(check)
}

#[inline]
pub unsafe fn lzma_mf_is_supported(mf: lzma_match_finder) -> lzma_bool {
    liblzma_rs::lz::lz_encoder::lzma_mf_is_supported(mf)
}

#[inline]
pub unsafe fn lzma_lzma_preset(
    options: *mut lzma_options_lzma,
    preset: u32,
) -> lzma_bool {
    liblzma_rs::lzma::lzma_encoder_presets::lzma_lzma_preset(options.cast(), preset)
}

#[inline]
pub unsafe fn lzma_stream_footer_decode(
    options: *mut lzma_stream_flags,
    input: *const u8,
) -> lzma_ret {
    liblzma_rs::common::stream_flags_decoder::lzma_stream_footer_decode(options.cast(), input)
}

#[inline]
pub unsafe fn lzma_index_buffer_decode(
    i: *mut *mut lzma_index,
    memlimit: *mut u64,
    allocator: *const lzma_allocator,
    input: *const u8,
    in_pos: *mut usize,
    in_size: usize,
) -> lzma_ret {
    liblzma_rs::common::index_decoder::lzma_index_buffer_decode(
        i.cast(), memlimit, allocator.cast(), input, in_pos, in_size,
    )
}

#[inline]
pub unsafe fn lzma_index_uncompressed_size(i: *const lzma_index) -> lzma_vli {
    liblzma_rs::common::index::lzma_index_uncompressed_size(i.cast())
}

#[inline]
pub unsafe fn lzma_index_end(i: *mut lzma_index, allocator: *const lzma_allocator) {
    liblzma_rs::common::index::lzma_index_end(i.cast(), allocator.cast())
}

// === Multithreaded (stubs - _mt files not yet transpiled) ===

/// lzma_mt struct matching liblzma-sys layout
#[repr(C)]
pub struct lzma_mt {
    pub flags: u32,
    pub threads: u32,
    pub block_size: u64,
    pub timeout: u32,
    pub preset: u32,
    pub filters: *const lzma_filter,
    pub check: lzma_check,
    _reserved_enum1: u32,
    _reserved_enum2: u32,
    _reserved_enum3: u32,
    _reserved_int1: u32,
    _reserved_int2: u32,
    _reserved_int3: u32,
    _reserved_int4: u32,
    _reserved_int5: u64,
    _reserved_int6: u64,
    _reserved_int7: u64,
    _reserved_int8: u64,
    _reserved_ptr1: *mut c_void,
    _reserved_ptr2: *mut c_void,
    _reserved_ptr3: *mut c_void,
    _reserved_ptr4: *mut c_void,
}

pub unsafe fn lzma_stream_encoder_mt(
    _strm: *mut lzma_stream,
    _options: *const lzma_mt,
) -> lzma_ret {
    unimplemented!("lzma_stream_encoder_mt not yet available in liblzma-rs")
}

pub unsafe fn lzma_stream_decoder_mt(
    _strm: *mut lzma_stream,
    _options: *const lzma_mt,
) -> lzma_ret {
    unimplemented!("lzma_stream_decoder_mt not yet available in liblzma-rs")
}

pub unsafe fn lzma_stream_encoder_mt_memusage(
    _options: *const lzma_mt,
) -> u64 {
    unimplemented!("lzma_stream_encoder_mt_memusage not yet available in liblzma-rs")
}
