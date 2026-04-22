mod rust;

#[cfg(feature = "custom_allocator")]
mod c;
#[cfg(feature = "custom_allocator")]
mod custom;
#[cfg(not(feature = "custom_allocator"))]
mod rust_only;

#[cfg(feature = "custom_allocator")]
use custom as policy;
#[cfg(not(feature = "custom_allocator"))]
use rust_only as policy;

#[cfg(feature = "custom_allocator")]
pub use c::{c_allocator, c_allocator_ptr, lzma_c_alloc, lzma_c_free};
#[cfg(feature = "custom_allocator")]
pub use custom::allocator_or_c;
pub use rust::rust_allocator;

pub(crate) use policy::{
    internal_alloc_array, internal_alloc_bytes, internal_alloc_object, internal_alloc_zeroed_array,
    internal_free, lzma_alloc_bytes, lzma_alloc_zeroed_bytes, lzma_free_ptr,
};
