use crate::types::*;
extern "C" {
    fn tuklib_cpucores() -> u32;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_cputhreads() -> u32 {
    return tuklib_cpucores();
}
