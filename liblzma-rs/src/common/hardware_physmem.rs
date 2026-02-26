extern "C" {
    fn tuklib_physmem() -> uint64_t;
}
pub type uint64_t = u64;
#[no_mangle]
pub unsafe extern "C" fn lzma_physmem() -> uint64_t {
    return tuklib_physmem();
}
