extern "C" {
    fn tuklib_cpucores() -> uint32_t;
}
pub type uint32_t = u32;
#[no_mangle]
pub unsafe extern "C" fn lzma_cputhreads() -> uint32_t {
    return tuklib_cpucores();
}
