use core::{cmp::Ordering, mem, ops::IndexMut};

pub trait Heap: IndexMut<usize>
where
    Self::Output: Copy + Ord,
{
    #[inline]
    fn ord(k: usize) -> Ordering {
        if (k + 1).leading_zeros() % 2 == 1 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }

    #[inline]
    fn sift_down(&mut self, mut k: usize, n: usize) {
        let o = Self::ord(k);
        let mut value = self[k];
        loop {
            let l = 2 * k + 1;
            let r = 2 * k + 2;
            let ll = 2 * l + 1;
            let lr = 2 * l + 2;
            let rl = 2 * r + 1;
            let rr = 2 * r + 2;
            let m = if rr < n {
                let l = if self[ll].cmp(&self[lr]) == o { ll } else { lr };
                let r = if self[rl].cmp(&self[rr]) == o { rl } else { rr };
                if self[l].cmp(&self[r]) == o { l } else { r }
            } else if rl < n {
                let l = if self[ll].cmp(&self[lr]) == o { ll } else { lr };
                let r = rl;
                if self[l].cmp(&self[r]) == o { l } else { r }
            } else if lr < n {
                let l = if self[ll].cmp(&self[lr]) == o { ll } else { lr };
                if self[l].cmp(&self[r]) == o { l } else { r }
            } else if ll < n {
                let l = ll;
                if self[l].cmp(&self[r]) == o { l } else { r }
            } else if r < n {
                if self[l].cmp(&self[r]) == o { l } else { r }
            } else if l < n {
                l
            } else {
                break;
            };
            if (value.cmp(&self[m])) == o {
                break;
            }
            self[k] = self[m];
            k = m;
            let o = Self::ord(k);
            if self[(k - 1) / 2].cmp(&value) == o {
                mem::swap(&mut self[(k - 1) / 2], &mut value);
            }
        }
        self[k] = value;
    }

    #[inline]
    fn sift_up(&mut self, mut k: usize) {
        let value = self[k];
        let o = Self::ord(k);
        if k > 0 && self[(k - 1) / 2].cmp(&value) == o {
            self[k] = self[(k - 1) / 2];
            k = (k - 1) / 2;
        }
        let o = Self::ord(k);
        while k > 2 && value.cmp(&self[(k + 1) / 4 - 1]) == o {
            self[k] = self[(k + 1) / 4 - 1];
            k = (k + 1) / 4 - 1;
        }
        self[k] = value;
    }

    #[inline]
    fn heap_init(&mut self, n: usize) {
        for k in (0..n / 2).rev() {
            self.sift_down(k, n);
        }
    }

    #[inline]
    fn heap_pop_max(&mut self, n: usize) {
        if n > 2 {
            let k = if self[1] > self[2] { 1 } else { 2 };
            let value = self[k];
            self[k] = self[n - 1];
            self[n - 1] = value;
            self.sift_down(k, n - 1);
        }
    }

    #[inline]
    fn heap_pop_min(&mut self, n: usize) {
        if n > 0 {
            let value = self[0];
            self[0] = self[n - 1];
            self[n - 1] = value;
            self.sift_down(0, n - 1);
        }
    }

    #[inline]
    fn heap_push(&mut self, n: usize) {
        self.sift_up(n - 1);
    }
}

impl<T: IndexMut<usize> + ?Sized> Heap for T where Self::Output: Copy + Ord {}
