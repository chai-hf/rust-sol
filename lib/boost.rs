use core::{
    hint,
    ops::{Index, IndexMut},
};

#[inline]
#[track_caller]
pub fn unreachable() -> ! {
    unsafe { hint::unreachable_unchecked() }
}

pub trait UnwrapBoost<T> {
    #[track_caller]
    fn unwrap_boost(self) -> T;
}

impl<T> UnwrapBoost<T> for Option<T> {
    #[inline]
    fn unwrap_boost(self) -> T {
        unsafe { self.unwrap_unchecked() }
    }
}

pub struct Flice<'a, T> {
    data: &'a [T],
}

pub struct MutFlice<'a, T> {
    data: &'a mut [T],
}

impl<T> Index<usize> for Flice<'_, T> {
    type Output = T;
    #[inline]
    fn index(&self, index: usize) -> &T {
        unsafe { self.data.get_unchecked(index) }
    }
}

impl<T> Index<usize> for MutFlice<'_, T> {
    type Output = T;
    #[inline]
    fn index(&self, index: usize) -> &T {
        unsafe { self.data.get_unchecked(index) }
    }
}

impl<T> IndexMut<usize> for MutFlice<'_, T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut T {
        unsafe { self.data.get_unchecked_mut(index) }
    }
}

pub trait AsFlice<T> {
    fn as_flice(&self) -> Flice<T>;
    fn as_mut_flice(&mut self) -> MutFlice<T>;
}

impl<T> AsFlice<T> for [T] {
    #[inline]
    fn as_flice(&self) -> Flice<T> {
        Flice { data: self }
    }

    #[inline]
    fn as_mut_flice(&mut self) -> MutFlice<T> {
        MutFlice { data: self }
    }
}
