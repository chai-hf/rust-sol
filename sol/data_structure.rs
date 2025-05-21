use crate::{problem, solution};
use alloc::vec::Vec;
use core::{
    fmt::{Result, Write},
    iter,
};
use toy::{Difference, Fenwick, Group, Moniod, Reader, Semigroup};

problem!(data_structure, associative_array);

solution!(data_structure, associative_array, associative_array_);

#[inline]
pub fn associative_array_(mut rd: Reader, wt: &mut dyn Write) -> Result {
    extern crate std;
    use std::collections::HashMap;
    let mut map = HashMap::new();
    let q = rd.u26();
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
            _ => toy::unreachable(),
        }
    }
    Ok(())
}

problem!(data_structure, point_add_range_sum);

solution!(data_structure, point_add_range_sum, point_add_range_sum_);

#[inline]
pub fn point_add_range_sum_(mut rd: Reader, wt: &mut dyn Write) -> Result {
    let n = rd.u26();
    let q = rd.u26();
    let mut tree: Fenwick<G> = Fenwick::new(n as usize);
    for i in 1..=n {
        tree.add(i as usize, &(rd.u32() as i64));
    }
    for _ in 1..=q {
        match rd.digit() {
            0 => {
                let k = rd.u26() as usize + 1;
                let x = rd.u32() as i64;
                tree.add(k, &x);
            }
            1 => {
                let l = rd.u26() as usize;
                let r = rd.u26() as usize;
                let x = tree.range_sum(l, r);
                writeln!(wt, "{x}")?;
            }
            _ => toy::unreachable(),
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

problem!(data_structure, static_range_sum);

solution!(data_structure, static_range_sum, static_range_sum_);

#[inline]
pub fn static_range_sum_(mut rd: Reader, wt: &mut dyn Write) -> Result {
    let n = rd.u26();
    let q = rd.u26();
    let mut sum = 0u64;
    let vec: Vec<_> = iter::once(0)
        .chain((0..n).map(|_| {
            sum += rd.u32() as u64;
            sum
        }))
        .collect();
    for _ in 0..q {
        let l = rd.u26() as usize;
        let r = rd.u26() as usize;
        let sum = vec.difference(G, l, r);
        writeln!(wt, "{sum}")?;
    }
    struct G;
    impl Semigroup for G {
        type Item = u64;
        #[inline]
        fn operate(x: &Self::Item, y: &Self::Item) -> Self::Item {
            x.wrapping_add(*y)
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
            x.wrapping_neg()
        }
    }
    Ok(())
}
