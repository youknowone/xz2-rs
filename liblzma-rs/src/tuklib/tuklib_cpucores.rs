pub type uint32_t = u32;

#[no_mangle]
pub unsafe extern "C" fn tuklib_cpucores() -> uint32_t {
    match std::thread::available_parallelism() {
        Ok(n) => n.get() as uint32_t,
        Err(_) => 0,
    }
}
