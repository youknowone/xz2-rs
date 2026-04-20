#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
use libc::c_void;
#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
use libc::size_t;
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
use std::alloc::{Layout, alloc, alloc_zeroed, dealloc};
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
use std::os::raw::c_void;

use crate::{lzma_allocator, lzma_stream};

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
const C_ALLOC_ALIGN: usize = 16;
#[cfg(all(target_family = "wasm", target_os = "unknown"))]
const C_ALLOC_HEADER_SIZE: usize = 16;

#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
unsafe extern "C" {
    fn malloc(__size: size_t) -> *mut c_void;
    #[allow(dead_code)]
    fn calloc(__count: size_t, __size: size_t) -> *mut c_void;
    fn free(__ptr: *mut c_void);
}

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
fn c_alloc_layout(size: usize) -> Option<Layout> {
    let total_size = size.checked_add(C_ALLOC_HEADER_SIZE)?;
    Layout::from_size_align(total_size, C_ALLOC_ALIGN).ok()
}

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
fn malloc(size: size_t) -> *mut c_void {
    let layout = match c_alloc_layout(size as usize) {
        Some(layout) => layout,
        None => return core::ptr::null_mut(),
    };
    let base = unsafe { alloc(layout) };
    if base.is_null() {
        return core::ptr::null_mut();
    }
    unsafe {
        *(base as *mut usize) = layout.size();
        base.add(C_ALLOC_HEADER_SIZE) as *mut c_void
    }
}

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
fn calloc(count: size_t, size: size_t) -> *mut c_void {
    let size = match (count as usize).checked_mul(size as usize) {
        Some(size) => size,
        None => return core::ptr::null_mut(),
    };
    let layout = match c_alloc_layout(size) {
        Some(layout) => layout,
        None => return core::ptr::null_mut(),
    };
    let base = unsafe { alloc_zeroed(layout) };
    if base.is_null() {
        return core::ptr::null_mut();
    }
    unsafe {
        *(base as *mut usize) = layout.size();
        base.add(C_ALLOC_HEADER_SIZE) as *mut c_void
    }
}

#[cfg(all(target_family = "wasm", target_os = "unknown"))]
unsafe fn free(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }
    let base = (ptr as *mut u8).sub(C_ALLOC_HEADER_SIZE);
    let total_size = *(base as *const usize);
    let layout = Layout::from_size_align_unchecked(total_size, C_ALLOC_ALIGN);
    dealloc(base, layout);
}

unsafe fn lzma_c_alloc(_opaque: *mut c_void, nmemb: size_t, size: size_t) -> *mut c_void {
    let size = match (nmemb as usize).checked_mul(size as usize) {
        Some(0) => 1,
        Some(size) => size,
        None => return core::ptr::null_mut(),
    };
    malloc(size as size_t)
}

unsafe fn lzma_c_free(_opaque: *mut c_void, ptr: *mut c_void) {
    free(ptr);
}

#[repr(transparent)]
struct StaticAllocator(lzma_allocator);

unsafe impl Sync for StaticAllocator {}

static C_ALLOCATOR: StaticAllocator = StaticAllocator(lzma_allocator {
    alloc: Some(lzma_c_alloc),
    free: Some(lzma_c_free),
    opaque: core::ptr::null_mut(),
});

fn c_allocator_ptr() -> *const lzma_allocator {
    &raw const C_ALLOCATOR.0
}

pub(crate) fn normalize_c_allocator(allocator: *const lzma_allocator) -> *const lzma_allocator {
    if allocator.is_null() {
        c_allocator_ptr()
    } else {
        allocator
    }
}

pub(crate) unsafe fn normalize_c_stream_allocator(strm: *mut lzma_stream) {
    if !strm.is_null() && (*strm).allocator.is_null() {
        (*strm).allocator = c_allocator_ptr();
    }
}
