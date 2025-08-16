use crate::problem;
use core::fmt::{Result, Write};
use toy::Reader;

problem!(sample, aplusb);

pub fn aplusb(mut rd: Reader, wt: &mut impl Write) -> Result {
    let a = rd.u32();
    let b = rd.u32();
    let sum = a + b;
    writeln!(wt, "{sum}")
}
