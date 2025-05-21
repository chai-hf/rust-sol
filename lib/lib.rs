#![no_std]
extern crate alloc;

mod algebra;
mod fenwick;
mod mint;
mod reader;
mod util;

pub use algebra::*;
pub use fenwick::*;
pub use mint::*;
pub use reader::*;
pub use util::*;

pub trait Difference<S: Group> {
    fn difference(&self, _: S, l: usize, r: usize) -> S::Item;
}

impl<T, S> Difference<S> for [T]
where
    S: Group<Item = T>,
{
    fn difference(&self, _: S, l: usize, r: usize) -> T {
        S::sub(&self[r], &self[l])
    }
}
