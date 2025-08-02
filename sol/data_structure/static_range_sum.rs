use crate::problem;
use alloc::vec::Vec;
use core::{
    fmt::{Result, Write},
    iter,
};
use toy::Reader;

problem!(data_structure, static_range_sum);

#[inline]
pub fn static_range_sum(mut rd: Reader, wt: &mut impl Write) -> Result {
    let n = rd.u26() as usize;
    let q = rd.u26() as usize;
    let mut sum = 0;
    let vec = Vec::from_iter(
        iter::once(0).chain(
            iter::repeat_with(|| {
                sum += rd.u32() as u64;
                sum
            })
            .take(n),
        ),
    );
    for _ in 0..q {
        let l = rd.u26() as usize;
        let r = rd.u26() as usize;
        let sum = vec[r] - vec[l];
        writeln!(wt, "{sum}")?;
    }
    Ok(())
}
