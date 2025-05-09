use crate::*;
use alloc::vec::Vec;

pub struct Fenwick<S: Moniod> {
    data: Vec<S::Item>,
}

impl<S: Moniod> Fenwick<S> {
    pub fn new(size: usize) -> Self {
        Fenwick {
            data: (0..=size).map(|_| S::unit()).collect(),
        }
    }

    pub fn add(&mut self, mut k: usize, x: &S::Item) {
        debug_assert!(k > 0);
        while k < self.data.len() {
            unsafe {
                *self.data.get_unchecked_mut(k) = S::operate(self.data.get_unchecked(k), x);
            }
            k += k & k.wrapping_neg();
        }
    }

    pub fn sum(&self, mut k: usize) -> S::Item {
        debug_assert!(k < self.data.len());
        let mut res = S::unit();
        while k != 0 {
            res = S::operate(unsafe { self.data.get_unchecked(k) }, &res);
            k -= k & k.wrapping_neg();
        }
        res
    }
}

impl<S: Group> Fenwick<S> {
    pub fn range_sum(&self, l: usize, r: usize) -> S::Item {
        let l = self.sum(l);
        let r = self.sum(r);
        S::operate(&S::inverse(&l), &r)
    }
}
