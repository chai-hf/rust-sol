use crate::problem;
use alloc::vec::Vec;
use core::fmt::{Result, Write};
use toy::{Heap, PushUnsafe, Reader};

problem!(data_structure, double_ended_priority_queue);

#[inline]
pub fn double_ended_priority_queue(mut rd: Reader, wt: &mut impl Write) -> Result {
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
