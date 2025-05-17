use crate::{problem, solution};
use core::fmt::{Result, Write};
use toy::{Fenwick, Group, Moniod, Reader, Semigroup, unreachable};

problem!(data_structure, associative_array);

solution!(data_structure, associative_array, associative_array_naive);

#[inline]
pub fn associative_array_naive(mut rd: Reader, wt: &mut dyn Write) -> Result {
    extern crate std;
    let q = rd.u26();
    let mut map = std::collections::HashMap::new();
    for _ in 0..q {
        match rd.digit() {
            0 => {
                let k = rd.u64();
                let v = rd.u64();
                if v > 0 {
                    map.insert(k, v);
                } else {
                    map.remove(&k);
                }
            }
            1 => {
                let k = rd.u64();
                writeln!(wt, "{}", map.get(&k).unwrap_or(&0))?;
            }
            _ => unreachable(),
        }
    }
    Ok(())
}

problem!(data_structure, point_add_range_sum);

solution!(data_structure, point_add_range_sum, point_add_range_sum);

#[inline]
pub fn point_add_range_sum(mut rd: Reader, wt: &mut dyn Write) -> Result {
    let n = rd.u26();
    let q = rd.u26();
    let mut tree: Fenwick<G> = Fenwick::new(n as usize);
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
    struct G;
    impl Semigroup for G {
        type Item = i64;
        #[inline]
        fn operate(x: &Self::Item, y: &Self::Item) -> Self::Item {
            x + y
        }
    }
    impl Moniod for G {
        #[inline]
        fn unit() -> Self::Item {
            0
        }
    }
    impl Group for G {
        #[inline]
        fn inverse(x: &Self::Item) -> Self::Item {
            -x
        }
    }
    Ok(())
}
