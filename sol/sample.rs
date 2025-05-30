use crate::problem;
use core::fmt::{Result, Write};
use toy::Reader;

problem!(sample, aplusb);

#[inline]
pub fn aplusb(mut rd: Reader, wt: &mut dyn Write) -> Result {
    let a = rd.u32();
    let b = rd.u32();
    let sum = a + b;
    writeln!(wt, "{sum}")
}

problem!(sample, many_aplusb);

#[inline]
pub fn many_aplusb(mut rd: Reader, wt: &mut dyn Write) -> Result {
    let n = rd.u26();
    for _ in 0..n {
        let a = rd.u64();
        let b = rd.u64();
        let sum = a + b;
        writeln!(wt, "{sum}")?;
    }
    Ok(())
}
