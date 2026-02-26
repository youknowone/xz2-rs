pub type uint32_t = u32;
#[no_mangle]
pub unsafe extern "C" fn tuklib_cpucores() -> uint32_t {
    let mut ret: uint32_t = 0 as uint32_t;
    return ret;
}
