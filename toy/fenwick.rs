use core::ops::{AddAssign, IndexMut, SubAssign};

pub trait Fenwick: IndexMut<usize>
where
    Self::Output: Copy + AddAssign + SubAssign,
{
    #[inline]
    fn fenwick_query(&self, mut l: usize, mut r: usize) -> Self::Output {
        let mut ans = self[0];
        while r > 0 {
            ans += self[r];
            r -= r & r.wrapping_neg();
        }
        while l > 0 {
            ans -= self[l];
            l -= l & l.wrapping_neg();
        }
        ans
    }

    #[inline]
    fn fenwick_add(&mut self, mut k: usize, n: usize, x: Self::Output) {
        while k <= n {
            self[k] += x;
            k += k & k.wrapping_neg();
        }
    }

    #[inline]
    fn fenwick_init(&mut self, n: usize) {
        for i in (1..=n).rev() {
            let x = self[i - (i & i.wrapping_neg())];
            self[i] -= x;
        }
    }
}

impl<T: IndexMut<usize> + ?Sized> Fenwick for T where Self::Output: Copy + AddAssign + SubAssign {}
