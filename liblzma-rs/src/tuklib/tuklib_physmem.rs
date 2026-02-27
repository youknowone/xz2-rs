pub type uint64_t = u64;

#[no_mangle]
pub unsafe extern "C" fn tuklib_physmem() -> uint64_t {
    #[cfg(target_os = "macos")]
    {
        let mut memsize: u64 = 0;
        let mut len = core::mem::size_of::<u64>() as libc::size_t;
        let mut mib: [libc::c_int; 2] = [libc::CTL_HW, libc::HW_MEMSIZE];
        if libc::sysctl(
            mib.as_mut_ptr(),
            2,
            &mut memsize as *mut u64 as *mut libc::c_void,
            &mut len,
            core::ptr::null_mut(),
            0,
        ) == 0
        {
            return memsize;
        }
        0
    }

    #[cfg(target_os = "linux")]
    {
        let pages = libc::sysconf(libc::_SC_PHYS_PAGES);
        let page_size = libc::sysconf(libc::_SC_PAGESIZE);
        if pages > 0 && page_size > 0 {
            return (pages as u64).wrapping_mul(page_size as u64);
        }
        0
    }

    #[cfg(target_os = "windows")]
    {
        // On Windows, use GetSystemInfo-based approach or return 0
        // Windows support would need winapi/windows-sys crate
        0
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    {
        0
    }
}
