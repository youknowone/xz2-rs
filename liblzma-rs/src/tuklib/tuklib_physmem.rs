pub type uint64_t = u64;
#[no_mangle]
pub unsafe extern "C" fn tuklib_physmem() -> uint64_t {
    let mut ret: uint64_t = 0 as uint64_t;
    return ret;
}
