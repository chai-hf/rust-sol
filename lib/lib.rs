#![no_std]
#![feature(portable_simd)]
extern crate alloc;

mod algebra;
mod fenwick;
mod read;
mod util;

pub use algebra::*;
pub use fenwick::*;
pub use read::*;
pub use util::*;
