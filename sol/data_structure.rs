use core::fmt::{Result, Write};
use toy::*;

struct S;

impl Semigroup for S {
    type Item = i64;
    fn operate(x: &Self::Item, y: &Self::Item) -> Self::Item {
        x + y
    }
}

impl Moniod for S {
    fn unit() -> Self::Item {
        0
    }
}

impl Group for S {
    fn inverse(x: &Self::Item) -> Self::Item {
        -x
    }
}

pub fn point_add_range_sum(mut rd: Read, wt: &mut dyn Write) -> Result {
    let n = rd.u16();
    let q = rd.u16();
    let mut tree: Fenwick<S> = Fenwick::new(5);
    for i in 1..=n {
        tree.add(i as usize, &(rd.u32() as i64));
    }
    for _ in 1..=q {
        match rd.digit() {
            0 => {
                let k = rd.u16() as usize + 1;
                let x = rd.u32() as i64;
                tree.add(k, &x);
            }
            1 => {
                let l = rd.u16() as usize;
                let r = rd.u16() as usize;
                let x = tree.range_sum(l, r);
                writeln!(wt, "{x}")?;
            }
            _ => unreachable(),
        }
    }
    Ok(())
}
