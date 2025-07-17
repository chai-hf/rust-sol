use crate::problem;
use alloc::vec::Vec;
use core::{
    fmt::{Result, Write},
    iter,
};
use toy::{Fenwick, Reader};

fn read_prefix_sum(rd: &mut Reader, n: usize) -> Vec<u64> {
    let mut sum: u64 = 0;
    iter::once(0)
        .chain((0..n).map(|_| {
            sum += rd.u32() as u64;
            sum
        }))
        .collect()
}

problem!(data_structure, point_add_range_sum);

#[inline]
pub fn point_add_range_sum(mut rd: Reader, wt: &mut impl Write) -> Result {
    let n = rd.u26() as usize;
    let q = rd.u26() as usize;
    let mut vec = read_prefix_sum(&mut rd, n);
    vec.fenwick_init(n);
    for _ in 0..q {
        match rd.digit() {
            0 => {
                let k = rd.u26() as usize + 1;
                let x = rd.u32() as u64;
                vec.fenwick_add(k, n, x);
            }
            1 => {
                let l = rd.u26() as usize;
                let r = rd.u26() as usize;
                let sum = vec.fenwick_query(l, r);
                writeln!(wt, "{sum}")?;
            }
            _ => unreachable!(),
        }
    }
    Ok(())
}
