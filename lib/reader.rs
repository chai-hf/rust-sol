#[derive(Clone, Copy)]
pub struct Reader {
    ptr: *const u8,
}

impl Reader {
    #[inline]
    fn read_raw_u32(&self) -> u32 {
        unsafe { (self.ptr as *const u32).read_unaligned() ^ 0x30303030 }
    }

    #[inline]
    fn read_raw_u64(&self) -> u64 {
        unsafe { (self.ptr as *const u64).read_unaligned() ^ 0x3030303030303030 }
    }

    #[inline]
    fn read_raw_u128(&self) -> u128 {
        unsafe { (self.ptr as *const u128).read_unaligned() ^ 0x30303030303030303030303030303030 }
    }

    #[inline]
    fn all_digits_u32(x: u32) -> bool {
        (x & 0xf0f0f0f0) == 0
    }

    #[inline]
    fn all_digits_u64(x: u64) -> bool {
        (x & 0xf0f0f0f0f0f0f0f0) == 0
    }

    #[inline]
    fn all_digits_u128(x: u128) -> bool {
        (x & 0xf0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0) == 0
    }

    #[inline]
    fn parse_from_u32(mut x: u32) -> u32 {
        x = (x * 10 + (x >> 8)) & 0x00ff00ff;
        (x * 100 + (x >> 16)) & 0x0000ffff
    }

    #[inline]
    fn parse_from_u64(mut x: u64) -> u32 {
        x = (x * 10 + (x >> 8)) & 0x00ff00ff00ff00ff;
        x = (x * 100 + (x >> 16)) & 0x0000ffff0000ffff;
        (x * 10000 + (x >> 32)) as u32
    }

    #[inline]
    fn parse_from_u128(x: u128) -> u64 {
        let mut l = x as u64;
        let mut h = (x >> 64) as u64;
        l = (l * 10 + (l >> 8)) & 0x00ff00ff00ff00ff;
        h = (h * 10 + (h >> 8)) & 0x00ff00ff00ff00ff;
        l = (l * 100 + (l >> 16)) & 0x0000ffff0000ffff;
        h = (h * 100 + (h >> 16)) & 0x0000ffff0000ffff;
        l = (l * 10000 + (l >> 32)) & 0x00000000ffffffff;
        h = (h * 10000 + (h >> 32)) & 0x00000000ffffffff;
        l * 100000000 + h
    }

    #[inline]
    fn parse_remaining_u32(&mut self, mut num: u32) -> u32 {
        let mut cur = self.pop();
        while cur < 10 {
            num = num * 10 + cur as u32;
            cur = self.pop();
        }
        num
    }

    #[inline]
    fn parse_remaining_u64(&mut self, mut num: u64) -> u64 {
        let mut cur = self.pop();
        while cur < 10 {
            num = num * 10 + cur as u64;
            cur = self.pop();
        }
        num
    }

    #[inline]
    fn pop(&mut self) -> u8 {
        let num = unsafe { *self.ptr }.wrapping_sub(b'0');
        self.add(1);
        num
    }

    #[inline]
    fn neg(&mut self) -> bool {
        if unsafe { *self.ptr } == b'-' {
            self.add(1);
            true
        } else {
            false
        }
    }

    /// Creates a new instance from a raw pointer.
    ///
    #[inline]
    pub fn new(ptr: *const u8) -> Self {
        Self { ptr }
    }

    /// Returns the underlying raw pointer.
    #[inline]
    pub fn ptr(&self) -> *const u8 {
        self.ptr
    }

    /// Advances the pointer by `count` bytes.
    #[inline]
    pub fn add(&mut self, count: usize) {
        self.ptr = unsafe { self.ptr.add(count) }
    }

    /// Reads a number from 0 to 9.
    #[inline]
    pub fn digit(&mut self) -> u8 {
        let num = self.pop();
        self.add(1);
        num
    }

    /// Reads a `u32` likely with 3 digits or fewer.
    #[inline]
    pub fn u8(&mut self) -> u32 {
        let num = self.pop() as u32;
        self.parse_remaining_u32(num)
    }

    /// Reads a `u32` likely with 5 digits or fewer.
    #[inline]
    pub fn u16(&mut self) -> u32 {
        let x: u32 = self.read_raw_u32();
        if Self::all_digits_u32(x) {
            self.add(4);
            self.parse_remaining_u32(Self::parse_from_u32(x))
        } else {
            self.u8()
        }
    }

    /// Reads a `u32` with 8 digits or fewer.
    #[inline]
    pub fn u26(&mut self) -> u32 {
        let x = self.read_raw_u64();
        let d = (x & 0xf0f0f0f0f0f0f0f0).trailing_zeros() / u8::BITS;
        self.add(d as usize + 1);
        Self::parse_from_u64(x << ((8 - d) * u8::BITS))
    }

    /// Reads a `u32`.
    #[inline]
    pub fn u32(&mut self) -> u32 {
        let x = self.read_raw_u64();
        if Self::all_digits_u64(x) {
            self.add(8);
            self.parse_remaining_u32(Self::parse_from_u64(x))
        } else {
            self.u26()
        }
    }

    /// Reads a `u64` with 16 digits or fewer.
    #[inline]
    pub fn u53(&mut self) -> u64 {
        let x = self.read_raw_u128();
        let d = (x & 0xf0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0).trailing_zeros() / u8::BITS;
        self.add(d as usize + 1);
        Self::parse_from_u128(x << ((16 - d) * u8::BITS))
    }

    /// Reads a `u64`
    #[inline]
    pub fn u64(&mut self) -> u64 {
        let x = self.read_raw_u128();
        if Self::all_digits_u128(x) {
            self.add(16);
            self.parse_remaining_u64(Self::parse_from_u128(x))
        } else {
            self.u53()
        }
    }

    /// Reads an `i32` likely with 3 digits or fewer.
    #[inline]
    pub fn i8(&mut self) -> i32 {
        if self.neg() {
            self.u8().wrapping_neg() as i32
        } else {
            self.u8() as i32
        }
    }

    /// Reads an `i32` likely with 5 digits or fewer.
    #[inline]
    pub fn i16(&mut self) -> i32 {
        if self.neg() {
            self.u16().wrapping_neg() as i32
        } else {
            self.u16() as i32
        }
    }

    /// Reads an `i32` with 8 digits or fewer.
    #[inline]
    pub fn i26(&mut self) -> i32 {
        if self.neg() {
            self.u26().wrapping_neg() as i32
        } else {
            self.u26() as i32
        }
    }

    /// Reads an `i32`.
    #[inline]
    pub fn i32(&mut self) -> i32 {
        if self.neg() {
            self.u32().wrapping_neg() as i32
        } else {
            self.u32() as i32
        }
    }

    /// Reads an `i64` with 16 digits or fewer.
    #[inline]
    pub fn i53(&mut self) -> i64 {
        if self.neg() {
            self.u53().wrapping_neg() as i64
        } else {
            self.u53() as i64
        }
    }

    /// Reads an `i64`.
    #[inline]
    pub fn i64(&mut self) -> i64 {
        if self.neg() {
            self.u64().wrapping_neg() as i64
        } else {
            self.u64() as i64
        }
    }
}
