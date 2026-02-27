use crate::tuklib::tuklib_physmem::tuklib_physmem;

#[no_mangle]
pub extern "C" fn lzma_physmem() -> u64 {
    return tuklib_physmem();
}
