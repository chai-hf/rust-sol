use crate::problem;
use alloc::vec::Vec;
use core::{
    fmt::{Result, Write},
    iter,
};
use toy::Reader;

fn read_prefix_sum(rd: &mut Reader, n: usize) -> Vec<u64> {
    let mut sum: u64 = 0;
    iter::once(0)
        .chain((0..n).map(|_| {
            sum += rd.u32() as u64;
            sum
        }))
        .collect()
}

problem!(data_structure, static_range_sum);

#[inline]
pub fn static_range_sum(mut rd: Reader, wt: &mut impl Write) -> Result {
    let n = rd.u26() as usize;
    let q = rd.u26() as usize;
    let vec = read_prefix_sum(&mut rd, n);
    for _ in 0..q {
        let l = rd.u26() as usize;
        let r = rd.u26() as usize;
        let sum = vec[r] - vec[l];
        writeln!(wt, "{sum}")?;
    }
    Ok(())
}
