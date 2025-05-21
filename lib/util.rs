#[inline]
pub const fn unreachable() -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
