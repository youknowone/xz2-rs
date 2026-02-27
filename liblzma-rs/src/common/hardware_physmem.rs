use crate::types::*;
extern "C" {
    fn tuklib_physmem() -> u64;
}
#[no_mangle]
pub unsafe extern "C" fn lzma_physmem() -> u64 {
    return tuklib_physmem();
}
