use crate::problem;
use core::fmt::{Result, Write};
use toy::Reader;

problem!(sample, many_aplusb);

pub fn many_aplusb(mut rd: Reader, wt: &mut impl Write) -> Result {
    let n = rd.u32();
    for _ in 0..n {
        let a = rd.u64();
        let b = rd.u64();
        let sum = a + b;
        writeln!(wt, "{sum}")?;
    }
    Ok(())
}
