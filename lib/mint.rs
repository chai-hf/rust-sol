use core::ops::{Add, Mul, Sub};

#[derive(Clone, Copy)]
pub struct Mint<const M: u32> {
    x: u32,
}

impl<const M: u32> Mint<M> {
    #[inline]
    pub const fn new(x: u32) -> Self {
        Mint { x }
    }

    #[inline]
    pub const fn add(self, rhs: Self) -> Self {
        if self.x + rhs.x < M {
            Self::new(self.x + rhs.x)
        } else {
            Self::new(self.x + rhs.x - M)
        }
    }

    #[inline]
    pub const fn sub(self, rhs: Self) -> Self {
        if self.x <= rhs.x {
            Self::new(self.x - rhs.x)
        } else {
            Self::new(self.x + M - rhs.x)
        }
    }

    #[inline]
    pub const fn mul(self, rhs: Self) -> Self {
        Self::new(((self.x as u64) * (rhs.x as u64) % (M as u64)) as u32)
    }

    #[inline]
    pub const fn pow(mut self, mut x: u32) -> Self {
        let mut res = Self::new(1);
        while x > 0 {
            if x % 2 == 1 {
                res = res.mul(self);
            }
            self = self.mul(self);
            x >>= 1;
        }
        res
    }
}

impl<const M: u32> Add for Mint<M> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        self.add(rhs)
    }
}

impl<const M: u32> Sub for Mint<M> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        self.sub(rhs)
    }
}

impl<const M: u32> Mul for Mint<M> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        self.mul(rhs)
    }
}
