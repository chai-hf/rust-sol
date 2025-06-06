use crate::problem;
use alloc::vec::Vec;
use core::{
    fmt::{Result, Write},
    iter,
};
use toy::{Fenwick, Heap, PushUnsafe, Reader};

fn read_prefix_sum(rd: &mut Reader, n: usize) -> Vec<u64> {
    let mut sum: u64 = 0;
    iter::once(0)
        .chain((0..n).map(|_| {
            sum += rd.u32() as u64;
            sum
        }))
        .collect()
}

problem!(data_structure, double_ended_priority_queue);

#[inline]
pub fn double_ended_priority_queue(mut rd: Reader, wt: &mut dyn Write) -> Result {
    let n = rd.u26() as usize;
    let q = rd.u26() as usize;
    let mut vec = Vec::with_capacity(n + q);
    vec.extend((0..n).map(|_| rd.i32()));
    vec.heap_init(n);
    rd.add((n == 0) as usize);
    for _ in 0..q {
        match rd.digit() {
            0 => {
                let x = rd.i32();
                vec.push_unsafe(x);
                let n = vec.len();
                vec.heap_push(n);
            }
            1 => {
                let n = vec.len();
                vec.heap_pop_min(n);
                let ans = vec.pop().unwrap();
                writeln!(wt, "{ans}")?;
            }
            2 => {
                let n = vec.len();
                vec.heap_pop_max(n);
                let ans = vec.pop().unwrap();
                writeln!(wt, "{ans}")?;
            }
            _ => unreachable!(),
        }
    }
    Ok(())
}

problem!(data_structure, static_range_sum);

#[inline]
pub fn static_range_sum(mut rd: Reader, wt: &mut dyn Write) -> Result {
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

problem!(data_structure, point_add_range_sum);

#[inline]
pub fn point_add_range_sum(mut rd: Reader, wt: &mut dyn Write) -> Result {
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
