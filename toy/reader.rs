#[derive(Clone, Copy)]
pub struct Reader {
    ptr: *const u8,
}

impl Reader {
    fn read_raw_u64(&self) -> u64 {
        unsafe { (self.ptr as *const u64).read_unaligned() ^ 0x3030303030303030 }
    }

    fn read_raw_u128(&self) -> u128 {
        unsafe { (self.ptr as *const u128).read_unaligned() ^ 0x30303030303030303030303030303030 }
    }

    fn all_digits_u64(x: u64) -> bool {
        (x & 0xf0f0f0f0f0f0f0f0) == 0
    }

    fn all_digits_u128(x: u128) -> bool {
        (x & 0xf0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0) == 0
    }

    fn parse_from_u64(mut x: u64) -> u32 {
        x = (x * 10 + (x >> 8)) & 0x00ff00ff00ff00ff;
        x = (x * 100 + (x >> 16)) & 0x0000ffff0000ffff;
        (x * 10000 + (x >> 32)) as u32
    }

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

    fn parse_remaining_u32(&mut self, mut num: u32) -> u32 {
        let mut cur = self.pop();
        while cur < 10 {
            num = num * 10 + cur as u32;
            cur = self.pop();
        }
        num
    }

    fn parse_remaining_u64(&mut self, mut num: u64) -> u64 {
        let mut cur = self.pop();
        while cur < 10 {
            num = num * 10 + cur as u64;
            cur = self.pop();
        }
        num
    }

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
    pub fn new(ptr: *const u8) -> Self {
        Self { ptr }
    }

    /// Advances the pointer by `count` bytes.
    pub fn add(&mut self, count: usize) {
        self.ptr = unsafe { self.ptr.add(count) }
    }

    /// Reads a char and convert it to a number.
    pub fn pop(&mut self) -> u8 {
        let num = unsafe { *self.ptr }.wrapping_sub(b'0');
        self.add(1);
        num
    }

    /// Reads a number from 0 to 9.
    pub fn digit(&mut self) -> u8 {
        let num = unsafe { *self.ptr }.wrapping_sub(b'0');
        self.add(2);
        num
    }

    /// Reads a `u32`.
    pub fn u32(&mut self) -> u32 {
        let x = self.read_raw_u64();
        if Self::all_digits_u64(x) {
            self.add(8);
            self.parse_remaining_u32(Self::parse_from_u64(x))
        } else {
            let d = (x & 0xf0f0f0f0f0f0f0f0).trailing_zeros() / u8::BITS;
            self.add(d as usize + 1);
            Self::parse_from_u64(x << ((8 - d) * u8::BITS))
        }
    }

    /// Reads a `u64`
    pub fn u64(&mut self) -> u64 {
        let x = self.read_raw_u128();
        if Self::all_digits_u128(x) {
            self.add(16);
            self.parse_remaining_u64(Self::parse_from_u128(x))
        } else {
            let d = (x & 0xf0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0).trailing_zeros() / u8::BITS;
            self.add(d as usize + 1);
            Self::parse_from_u128(x << ((16 - d) * u8::BITS))
        }
    }

    /// Reads an `i32`.
    pub fn i32(&mut self) -> i32 {
        if self.neg() {
            self.u32().wrapping_neg() as i32
        } else {
            self.u32() as i32
        }
    }

    /// Reads an `i64`.
    pub fn i64(&mut self) -> i64 {
        if self.neg() {
            self.u64().wrapping_neg() as i64
        } else {
            self.u64() as i64
        }
    }
}
