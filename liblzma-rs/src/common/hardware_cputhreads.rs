use crate::tuklib::tuklib_cpucores::tuklib_cpucores;

#[no_mangle]
pub extern "C" fn lzma_cputhreads() -> u32 {
    return tuklib_cpucores();
}
