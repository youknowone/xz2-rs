#![allow(bad_style)]

#[cfg(all(feature = "liblzma-sys", feature = "xz-sys"))]
compile_error!("Enable exactly one backend feature: liblzma-sys or xz-sys");
#[cfg(not(any(feature = "liblzma-sys", feature = "xz-sys")))]
compile_error!("Enable one backend feature: liblzma-sys or xz-sys");

#[cfg(feature = "liblzma-sys")]
use liblzma_sys::*;
#[cfg(feature = "xz-sys")]
use xz_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
