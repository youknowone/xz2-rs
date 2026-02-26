extern "C" {
    fn lzma_simple_coder_init(
        next: *mut lzma_next_coder,
        allocator: *const lzma_allocator,
        filters: *const lzma_filter_info,
        filter: Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                uint32_t,
                bool,
                *mut uint8_t,
                size_t,
            ) -> size_t,
        >,
        simple_size: size_t,
        unfiltered_max: size_t,
        alignment: uint32_t,
        is_encoder: bool,
    ) -> lzma_ret;
}
pub type __darwin_size_t = usize;
pub type uintptr_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint32_t = u32;
pub type uint64_t = u64;
pub type lzma_ret = ::core::ffi::c_uint;
pub const LZMA_RET_INTERNAL8: lzma_ret = 108;
pub const LZMA_RET_INTERNAL7: lzma_ret = 107;
pub const LZMA_RET_INTERNAL6: lzma_ret = 106;
pub const LZMA_RET_INTERNAL5: lzma_ret = 105;
pub const LZMA_RET_INTERNAL4: lzma_ret = 104;
pub const LZMA_RET_INTERNAL3: lzma_ret = 103;
pub const LZMA_RET_INTERNAL2: lzma_ret = 102;
pub const LZMA_RET_INTERNAL1: lzma_ret = 101;
pub const LZMA_SEEK_NEEDED: lzma_ret = 12;
pub const LZMA_PROG_ERROR: lzma_ret = 11;
pub const LZMA_BUF_ERROR: lzma_ret = 10;
pub const LZMA_DATA_ERROR: lzma_ret = 9;
pub const LZMA_OPTIONS_ERROR: lzma_ret = 8;
pub const LZMA_FORMAT_ERROR: lzma_ret = 7;
pub const LZMA_MEMLIMIT_ERROR: lzma_ret = 6;
pub const LZMA_MEM_ERROR: lzma_ret = 5;
pub const LZMA_GET_CHECK: lzma_ret = 4;
pub const LZMA_UNSUPPORTED_CHECK: lzma_ret = 3;
pub const LZMA_NO_CHECK: lzma_ret = 2;
pub const LZMA_STREAM_END: lzma_ret = 1;
pub const LZMA_OK: lzma_ret = 0;
pub type lzma_action = ::core::ffi::c_uint;
pub const LZMA_FINISH: lzma_action = 3;
pub const LZMA_FULL_BARRIER: lzma_action = 4;
pub const LZMA_FULL_FLUSH: lzma_action = 2;
pub const LZMA_SYNC_FLUSH: lzma_action = 1;
pub const LZMA_RUN: lzma_action = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_allocator {
    pub alloc: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t, size_t) -> *mut ::core::ffi::c_void,
    >,
    pub free:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> ()>,
    pub opaque: *mut ::core::ffi::c_void,
}
pub type lzma_next_coder = lzma_next_coder_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_next_coder_s {
    pub coder: *mut ::core::ffi::c_void,
    pub id: lzma_vli,
    pub init: uintptr_t,
    pub code: lzma_code_function,
    pub end: lzma_end_function,
    pub get_progress:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint64_t, *mut uint64_t) -> ()>,
    pub get_check: Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> lzma_check>,
    pub memconfig: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut uint64_t,
            *mut uint64_t,
            uint64_t,
        ) -> lzma_ret,
    >,
    pub update: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *const lzma_allocator,
            *const lzma_filter,
            *const lzma_filter,
        ) -> lzma_ret,
    >,
    pub set_out_limit:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint64_t, uint64_t) -> lzma_ret>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter {
    pub id: lzma_vli,
    pub options: *mut ::core::ffi::c_void,
}
pub type lzma_vli = uint64_t;
pub type lzma_check = ::core::ffi::c_uint;
pub const LZMA_CHECK_SHA256: lzma_check = 10;
pub const LZMA_CHECK_CRC64: lzma_check = 4;
pub const LZMA_CHECK_CRC32: lzma_check = 1;
pub const LZMA_CHECK_NONE: lzma_check = 0;
pub type lzma_end_function =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const lzma_allocator) -> ()>;
pub type lzma_code_function = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *const lzma_allocator,
        *const uint8_t,
        *mut size_t,
        size_t,
        *mut uint8_t,
        *mut size_t,
        size_t,
        lzma_action,
    ) -> lzma_ret,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct lzma_filter_info_s {
    pub id: lzma_vli,
    pub init: lzma_init_function,
    pub options: *mut ::core::ffi::c_void,
}
pub type lzma_init_function = Option<
    unsafe extern "C" fn(
        *mut lzma_next_coder,
        *const lzma_allocator,
        *const lzma_filter_info,
    ) -> lzma_ret,
>;
pub type lzma_filter_info = lzma_filter_info_s;
pub const __DARWIN_NULL: *mut ::core::ffi::c_void = ::core::ptr::null_mut::<::core::ffi::c_void>();
pub const NULL: *mut ::core::ffi::c_void = __DARWIN_NULL;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn read32be(mut buf: *const uint8_t) -> uint32_t {
    let mut num: uint32_t =
        (*buf.offset(0 as ::core::ffi::c_int as isize) as uint32_t) << 24 as ::core::ffi::c_int;
    num |= (*buf.offset(1 as ::core::ffi::c_int as isize) as uint32_t) << 16 as ::core::ffi::c_int;
    num |= (*buf.offset(2 as ::core::ffi::c_int as isize) as uint32_t) << 8 as ::core::ffi::c_int;
    num |= *buf.offset(3 as ::core::ffi::c_int as isize) as uint32_t;
    return num;
}
#[inline]
unsafe extern "C" fn read32le(mut buf: *const uint8_t) -> uint32_t {
    let mut num: uint32_t = *buf.offset(0 as ::core::ffi::c_int as isize) as uint32_t;
    num |= (*buf.offset(1 as ::core::ffi::c_int as isize) as uint32_t) << 8 as ::core::ffi::c_int;
    num |= (*buf.offset(2 as ::core::ffi::c_int as isize) as uint32_t) << 16 as ::core::ffi::c_int;
    num |= (*buf.offset(3 as ::core::ffi::c_int as isize) as uint32_t) << 24 as ::core::ffi::c_int;
    return num;
}
#[inline]
unsafe extern "C" fn write32be(mut buf: *mut uint8_t, mut num: uint32_t) {
    *buf.offset(0 as ::core::ffi::c_int as isize) = (num >> 24 as ::core::ffi::c_int) as uint8_t;
    *buf.offset(1 as ::core::ffi::c_int as isize) = (num >> 16 as ::core::ffi::c_int) as uint8_t;
    *buf.offset(2 as ::core::ffi::c_int as isize) = (num >> 8 as ::core::ffi::c_int) as uint8_t;
    *buf.offset(3 as ::core::ffi::c_int as isize) = num as uint8_t;
}
#[inline]
unsafe extern "C" fn write32le(mut buf: *mut uint8_t, mut num: uint32_t) {
    *buf.offset(0 as ::core::ffi::c_int as isize) = num as uint8_t;
    *buf.offset(1 as ::core::ffi::c_int as isize) = (num >> 8 as ::core::ffi::c_int) as uint8_t;
    *buf.offset(2 as ::core::ffi::c_int as isize) = (num >> 16 as ::core::ffi::c_int) as uint8_t;
    *buf.offset(3 as ::core::ffi::c_int as isize) = (num >> 24 as ::core::ffi::c_int) as uint8_t;
}
unsafe extern "C" fn riscv_encode(
    mut simple: *mut ::core::ffi::c_void,
    mut now_pos: uint32_t,
    mut is_encoder: bool,
    mut buffer: *mut uint8_t,
    mut size: size_t,
) -> size_t {
    if size < 8 as size_t {
        return 0 as size_t;
    }
    size = size.wrapping_sub(8 as size_t);
    let mut i: size_t = 0;
    let mut current_block_22: u64;
    i = 0 as size_t;
    while i <= size {
        let mut inst: uint32_t = *buffer.offset(i as isize) as uint32_t;
        if inst == 0xef as uint32_t {
            let b1: uint32_t = *buffer.offset(i.wrapping_add(1 as size_t) as isize) as uint32_t;
            if !(b1 & 0xd as uint32_t != 0 as uint32_t) {
                let b2: uint32_t = *buffer.offset(i.wrapping_add(2 as size_t) as isize) as uint32_t;
                let b3: uint32_t = *buffer.offset(i.wrapping_add(3 as size_t) as isize) as uint32_t;
                let pc: uint32_t = now_pos.wrapping_add(i as uint32_t);
                let mut addr: uint32_t = (b1 & 0xf0 as uint32_t) << 8 as ::core::ffi::c_int
                    | (b2 & 0xf as uint32_t) << 16 as ::core::ffi::c_int
                    | (b2 & 0x10 as uint32_t) << 7 as ::core::ffi::c_int
                    | (b2 & 0xe0 as uint32_t) >> 4 as ::core::ffi::c_int
                    | (b3 & 0x7f as uint32_t) << 4 as ::core::ffi::c_int
                    | (b3 & 0x80 as uint32_t) << 13 as ::core::ffi::c_int;
                addr = addr.wrapping_add(pc);
                *buffer.offset(i.wrapping_add(1 as size_t) as isize) = (b1 & 0xf as uint32_t
                    | addr >> 13 as ::core::ffi::c_int & 0xf0 as uint32_t)
                    as uint8_t;
                *buffer.offset(i.wrapping_add(2 as size_t) as isize) =
                    (addr >> 9 as ::core::ffi::c_int) as uint8_t;
                *buffer.offset(i.wrapping_add(3 as size_t) as isize) =
                    (addr >> 1 as ::core::ffi::c_int) as uint8_t;
                i = i.wrapping_add((4 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as size_t);
            }
        } else if inst & 0x7f as uint32_t == 0x17 as uint32_t {
            inst |= (*buffer.offset(i.wrapping_add(1 as size_t) as isize) as uint32_t)
                << 8 as ::core::ffi::c_int;
            inst |= (*buffer.offset(i.wrapping_add(2 as size_t) as isize) as uint32_t)
                << 16 as ::core::ffi::c_int;
            inst |= (*buffer.offset(i.wrapping_add(3 as size_t) as isize) as uint32_t)
                << 24 as ::core::ffi::c_int;
            if inst & 0xe80 as uint32_t != 0 {
                let mut inst2: uint32_t = read32le(
                    buffer
                        .offset(i as isize)
                        .offset(4 as ::core::ffi::c_int as isize),
                );
                if (inst << 8 as ::core::ffi::c_int ^ inst2.wrapping_sub(3 as uint32_t))
                    & 0xf8003 as uint32_t
                    != 0
                {
                    i = i.wrapping_add(
                        (6 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as size_t,
                    );
                    current_block_22 = 12517898123489920830;
                } else {
                    let mut addr_0: uint32_t = inst & 0xfffff000 as uint32_t;
                    addr_0 = addr_0.wrapping_add(
                        (inst2 >> 20 as ::core::ffi::c_int)
                            .wrapping_sub(inst2 >> 19 as ::core::ffi::c_int & 0x1000 as uint32_t),
                    );
                    addr_0 = addr_0.wrapping_add(now_pos.wrapping_add(i as uint32_t));
                    inst = (0x17 as ::core::ffi::c_int
                        | (2 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                        as uint32_t
                        | inst2 << 12 as ::core::ffi::c_int;
                    write32le(buffer.offset(i as isize), inst);
                    write32be(
                        buffer
                            .offset(i as isize)
                            .offset(4 as ::core::ffi::c_int as isize),
                        addr_0,
                    );
                    current_block_22 = 15125582407903384992;
                }
            } else {
                let fake_rs1: uint32_t = inst >> 27 as ::core::ffi::c_int;
                if inst.wrapping_sub(0x3117 as uint32_t) << 18 as ::core::ffi::c_int
                    >= fake_rs1 & 0x1d as uint32_t
                {
                    i = i.wrapping_add(
                        (4 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as size_t,
                    );
                    current_block_22 = 12517898123489920830;
                } else {
                    let fake_addr: uint32_t = read32le(
                        buffer
                            .offset(i as isize)
                            .offset(4 as ::core::ffi::c_int as isize),
                    ) as uint32_t;
                    let fake_inst2: uint32_t =
                        inst >> 12 as ::core::ffi::c_int | fake_addr << 20 as ::core::ffi::c_int;
                    inst = 0x17 as uint32_t
                        | fake_rs1 << 7 as ::core::ffi::c_int
                        | fake_addr & 0xfffff000 as uint32_t;
                    write32le(buffer.offset(i as isize), inst);
                    write32le(
                        buffer
                            .offset(i as isize)
                            .offset(4 as ::core::ffi::c_int as isize),
                        fake_inst2,
                    );
                    current_block_22 = 15125582407903384992;
                }
            }
            match current_block_22 {
                12517898123489920830 => {}
                _ => {
                    i = i.wrapping_add(
                        (8 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as size_t,
                    );
                }
            }
        }
        i = i.wrapping_add(2 as size_t);
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_simple_riscv_encoder_init(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
    mut filters: *const lzma_filter_info,
) -> lzma_ret {
    return lzma_simple_coder_init(
        next,
        allocator,
        filters,
        Some(
            riscv_encode
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    uint32_t,
                    bool,
                    *mut uint8_t,
                    size_t,
                ) -> size_t,
        ),
        0 as size_t,
        8 as size_t,
        2 as uint32_t,
        true_0 != 0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_bcj_riscv_encode(
    mut start_offset: uint32_t,
    mut buf: *mut uint8_t,
    mut size: size_t,
) -> size_t {
    start_offset = (start_offset as ::core::ffi::c_uint & !(1 as ::core::ffi::c_uint)) as uint32_t;
    return riscv_encode(NULL, start_offset, true_0 != 0, buf, size);
}
unsafe extern "C" fn riscv_decode(
    mut simple: *mut ::core::ffi::c_void,
    mut now_pos: uint32_t,
    mut is_encoder: bool,
    mut buffer: *mut uint8_t,
    mut size: size_t,
) -> size_t {
    if size < 8 as size_t {
        return 0 as size_t;
    }
    size = size.wrapping_sub(8 as size_t);
    let mut i: size_t = 0;
    let mut current_block_23: u64;
    i = 0 as size_t;
    while i <= size {
        let mut inst: uint32_t = *buffer.offset(i as isize) as uint32_t;
        if inst == 0xef as uint32_t {
            let b1: uint32_t = *buffer.offset(i.wrapping_add(1 as size_t) as isize) as uint32_t;
            if !(b1 & 0xd as uint32_t != 0 as uint32_t) {
                let b2: uint32_t = *buffer.offset(i.wrapping_add(2 as size_t) as isize) as uint32_t;
                let b3: uint32_t = *buffer.offset(i.wrapping_add(3 as size_t) as isize) as uint32_t;
                let pc: uint32_t = now_pos.wrapping_add(i as uint32_t);
                let mut addr: uint32_t = (b1 & 0xf0 as uint32_t) << 13 as ::core::ffi::c_int
                    | b2 << 9 as ::core::ffi::c_int
                    | b3 << 1 as ::core::ffi::c_int;
                addr = addr.wrapping_sub(pc);
                *buffer.offset(i.wrapping_add(1 as size_t) as isize) = (b1 & 0xf as uint32_t
                    | addr >> 8 as ::core::ffi::c_int & 0xf0 as uint32_t)
                    as uint8_t;
                *buffer.offset(i.wrapping_add(2 as size_t) as isize) =
                    (addr >> 16 as ::core::ffi::c_int & 0xf as uint32_t
                        | addr >> 7 as ::core::ffi::c_int & 0x10 as uint32_t
                        | addr << 4 as ::core::ffi::c_int & 0xe0 as uint32_t)
                        as uint8_t;
                *buffer.offset(i.wrapping_add(3 as size_t) as isize) =
                    (addr >> 4 as ::core::ffi::c_int & 0x7f as uint32_t
                        | addr >> 13 as ::core::ffi::c_int & 0x80 as uint32_t)
                        as uint8_t;
                i = i.wrapping_add((4 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as size_t);
            }
        } else if inst & 0x7f as uint32_t == 0x17 as uint32_t {
            let mut inst2: uint32_t = 0;
            inst |= (*buffer.offset(i.wrapping_add(1 as size_t) as isize) as uint32_t)
                << 8 as ::core::ffi::c_int;
            inst |= (*buffer.offset(i.wrapping_add(2 as size_t) as isize) as uint32_t)
                << 16 as ::core::ffi::c_int;
            inst |= (*buffer.offset(i.wrapping_add(3 as size_t) as isize) as uint32_t)
                << 24 as ::core::ffi::c_int;
            if inst & 0xe80 as uint32_t != 0 {
                inst2 = read32le(
                    buffer
                        .offset(i as isize)
                        .offset(4 as ::core::ffi::c_int as isize),
                );
                if (inst << 8 as ::core::ffi::c_int ^ inst2.wrapping_sub(3 as uint32_t))
                    & 0xf8003 as uint32_t
                    != 0
                {
                    i = i.wrapping_add(
                        (6 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as size_t,
                    );
                    current_block_23 = 12517898123489920830;
                } else {
                    let mut addr_0: uint32_t = inst & 0xfffff000 as uint32_t;
                    addr_0 = addr_0.wrapping_add(inst2 >> 20 as ::core::ffi::c_int);
                    inst = (0x17 as ::core::ffi::c_int
                        | (2 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int)
                        as uint32_t
                        | inst2 << 12 as ::core::ffi::c_int;
                    inst2 = addr_0;
                    current_block_23 = 6669252993407410313;
                }
            } else {
                let inst2_rs1: uint32_t = inst >> 27 as ::core::ffi::c_int;
                if inst.wrapping_sub(0x3117 as uint32_t) << 18 as ::core::ffi::c_int
                    >= inst2_rs1 & 0x1d as uint32_t
                {
                    i = i.wrapping_add(
                        (4 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as size_t,
                    );
                    current_block_23 = 12517898123489920830;
                } else {
                    let mut addr_1: uint32_t = read32be(
                        buffer
                            .offset(i as isize)
                            .offset(4 as ::core::ffi::c_int as isize),
                    );
                    addr_1 = addr_1.wrapping_sub(now_pos.wrapping_add(i as uint32_t));
                    inst2 = inst >> 12 as ::core::ffi::c_int | addr_1 << 20 as ::core::ffi::c_int;
                    inst = 0x17 as uint32_t
                        | inst2_rs1 << 7 as ::core::ffi::c_int
                        | addr_1.wrapping_add(0x800 as uint32_t) & 0xfffff000 as uint32_t;
                    current_block_23 = 6669252993407410313;
                }
            }
            match current_block_23 {
                12517898123489920830 => {}
                _ => {
                    write32le(buffer.offset(i as isize), inst);
                    write32le(
                        buffer
                            .offset(i as isize)
                            .offset(4 as ::core::ffi::c_int as isize),
                        inst2,
                    );
                    i = i.wrapping_add(
                        (8 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as size_t,
                    );
                }
            }
        }
        i = i.wrapping_add(2 as size_t);
    }
    return i;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_simple_riscv_decoder_init(
    mut next: *mut lzma_next_coder,
    mut allocator: *const lzma_allocator,
    mut filters: *const lzma_filter_info,
) -> lzma_ret {
    return lzma_simple_coder_init(
        next,
        allocator,
        filters,
        Some(
            riscv_decode
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    uint32_t,
                    bool,
                    *mut uint8_t,
                    size_t,
                ) -> size_t,
        ),
        0 as size_t,
        8 as size_t,
        2 as uint32_t,
        false_0 != 0,
    );
}
#[no_mangle]
pub unsafe extern "C" fn lzma_bcj_riscv_decode(
    mut start_offset: uint32_t,
    mut buf: *mut uint8_t,
    mut size: size_t,
) -> size_t {
    start_offset = (start_offset as ::core::ffi::c_uint & !(1 as ::core::ffi::c_uint)) as uint32_t;
    return riscv_decode(NULL, start_offset, false_0 != 0, buf, size);
}
