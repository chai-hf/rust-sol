use core::{
    fmt::{Display, Formatter, Result},
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

#[derive(Clone, Copy, Default)]
pub struct Mint<const M: u32 = 998244353> {
    pub x: u32,
}

impl<const M: u32> Mint<M> {
    #[inline]
    pub fn new(x: u32) -> Self {
        Mint { x }
    }

    #[inline]
    pub fn pow(mut self, mut x: u32) -> Self {
        let mut ans = Self::new(1);
        while x > 0 {
            if x % 2 == 1 {
                ans *= self;
            }
            self *= self;
            x /= 2;
        }
        ans
    }
}

impl<const M: u32> Add for Mint<M> {
    type Output = Self;
    #[inline]
    fn add(self, rhs: Self) -> Self {
        if self.x + rhs.x < M {
            Self::new(self.x + rhs.x)
        } else {
            Self::new(self.x + rhs.x - M)
        }
    }
}

impl<const M: u32> Sub for Mint<M> {
    type Output = Self;
    #[inline]
    fn sub(self, rhs: Self) -> Self {
        if self.x <= rhs.x {
            Self::new(self.x - rhs.x)
        } else {
            Self::new(self.x + M - rhs.x)
        }
    }
}

impl<const M: u32> Mul for Mint<M> {
    type Output = Self;
    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Self::new(((self.x as u64) * (rhs.x as u64) % (M as u64)) as u32)
    }
}

impl<const M: u32> Neg for Mint<M> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self::default() - self
    }
}

impl<const M: u32> AddAssign for Mint<M> {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl<const M: u32> SubAssign for Mint<M> {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl<const M: u32> MulAssign for Mint<M> {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<const M: u32> From<u32> for Mint<M> {
    #[inline]
    fn from(x: u32) -> Self {
        Self::new(x)
    }
}

impl<const M: u32> From<Mint<M>> for u32 {
    #[inline]
    fn from(x: Mint<M>) -> Self {
        x.x
    }
}

impl<const M: u32> Display for Mint<M> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", self.x)
    }
}
