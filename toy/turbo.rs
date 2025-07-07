pub trait PushUnsafe<T>: Extend<T> + Sized {
    #[inline]
    #[track_caller]
    fn push_unsafe(&mut self, item: T) {
        unsafe { self.extend_one_unchecked(item) }
    }
}

impl<T, V> PushUnsafe<T> for V where V: Extend<T> + Sized {}
