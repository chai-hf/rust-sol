use crate::{problem, solution};
use core::fmt::{Result, Write};
use toy::Read;

problem!(sample, aplusb);

solution!(sample, aplusb, aplusb);

#[inline]
pub fn aplusb(mut rd: Read, wt: &mut dyn Write) -> Result {
    let a = rd.u32();
    let b = rd.u32();
    let sum = a + b;
    writeln!(wt, "{sum}")
}

problem!(sample, many_aplusb);

solution!(sample, many_aplusb, many_aplusb);

#[inline]
pub fn many_aplusb(mut rd: Read, wt: &mut dyn Write) -> Result {
    let n = rd.u26();
    for _ in 0..n {
        let a = rd.u64();
        let b = rd.u64();
        let sum = a + b;
        writeln!(wt, "{sum}")?;
    }
    Ok(())
}
