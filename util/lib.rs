#![feature(test)]
extern crate test;
use std::{
    fmt::{Arguments, Result, Write},
    fs,
    io::{self, Read},
    process::Command,
};
use toy::Reader;

#[inline(never)]
pub fn gendata(dir: &str, problem: &str) {
    let status = Command::new("../problems/generate.py")
        .arg("-p")
        .arg(format!("{dir}/{problem}"))
        .output()
        .unwrap();
    if !status.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&status.stderr));
        panic!();
    }
}

#[inline(never)]
pub fn rmdata(dir: &str, problem: &str) {
    let status = Command::new("../problems/generate.py")
        .arg("-p")
        .arg(format!("{dir}/{problem}"))
        .arg("--clean")
        .output()
        .unwrap();
    if !status.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&status.stderr));
        panic!();
    }
}

#[inline(never)]
pub fn gencode(dir: &str, problem: &str, solution: &str) {
    let input_dir = format!("../problems/{dir}/{problem}/in");
    let mut tests: Vec<String> = Vec::new();
    for entry in fs::read_dir(&input_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().map(|ext| ext == "in").unwrap_or(false)
            && let Some(stem) = path.file_stem().and_then(|s| s.to_str())
        {
            tests.push(stem.into());
        }
    }
    tests.sort();

    let mut contents = format!(
        r#"use sol::{dir}::{solution} as testfn;
const DIR: &str = "{dir}";
const PROBLEM: &str = "{solution}";
"#
    );
    for test in &tests {
        contents.push_str(&format!(
            r#"
#[test]
fn _{test}() {{
    util::test(testfn, DIR, PROBLEM, "{test}");
}}
"#
        ));
    }
    fs::create_dir_all("tests").unwrap();
    let path = format!("tests/{solution}.rs");
    fs::write(&path, contents).unwrap();

    let mut contents = format!(
        r#"#![feature(test)]
extern crate test;
use sol::{dir}::{solution} as testfn;
const DIR: &str = "{dir}";
const PROBLEM: &str = "{solution}";
"#
    );
    for test in &tests {
        contents.push_str(&format!(
            r#"
#[bench]
fn _{test}(b: &mut test::Bencher) {{
    util::bench(testfn, DIR, PROBLEM, "{test}", b);
}}
"#
        ));
    }
    fs::create_dir_all("benches").unwrap();
    let path = format!("benches/{solution}.rs");
    fs::write(&path, contents).unwrap();

    let contents = format!(
        r#"fn main() {{
    util::main(sol::{dir}::{solution});
}}
"#
    );
    fs::create_dir_all("examples").unwrap();
    let path = format!("examples/{solution}.rs");
    fs::write(&path, contents).unwrap();

    let mut contents = format!("cargo b -r -q --example {solution}\n");
    for test in &tests {
        contents.push_str(&format!(
            "time -f %MKB target/release/examples/{solution} \
            {test} < problems/{dir}/{problem}/in/{test}.in\n"
        ));
    }
    let path = format!("../bench_{solution}.sh");
    fs::write(&path, contents).unwrap();

    let mut contents = String::from(
        r#"#![cfg_attr(target_os = "none", no_std)]
#![cfg_attr(target_os = "none", no_main)]
#[cfg(target_os = "none")]
extern crate alloc;

#[cfg(target_os = "none")]
use core::{
    alloc::{GlobalAlloc, Layout},
    fmt::{Result, Write},
    hint,
    panic::PanicInfo,
};
#[cfg(target_os = "none")]
use toy::Reader;

#[panic_handler]
#[cfg(target_os = "none")]
fn panic(_: &PanicInfo) -> ! {
    unsafe { hint::unreachable_unchecked() }
}

#[global_allocator]
#[cfg(target_os = "none")]
static GLOBAL: GlobalAllocImpl = GlobalAllocImpl;

#[cfg(target_os = "none")]
struct GlobalAllocImpl;

#[cfg(target_os = "none")]
unsafe impl GlobalAlloc for GlobalAllocImpl {
    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let ret;
        unsafe {
            core::arch::asm!(
                "syscall",
                in("rax") 9,
                in("rdi") 0,
                in("rsi") layout.size(),
                in("rdx") 3,
                in("r10") 34,
                in("r8") -1,
                in("r9") 0,
                lateout("rax") ret,
                out("rcx") _,
                out("r11") _,
            );
        }
        ret
    }

    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { self.alloc_zeroed(layout) }
    }

    #[inline]
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let _: usize;
        unsafe {
            core::arch::asm!(
                "syscall",
                in("rax") 11,
                in("rdi") ptr,
                in("rsi") layout.size(),
                lateout("rax") _,
                out("rcx") _,
                out("r11") _,
            );
        }
    }
}

#[cfg(target_os = "none")]
struct Stdout;

#[cfg(target_os = "none")]
impl Write for Stdout {
    #[inline]
    fn write_str(&mut self, s: &str) -> Result {
        unsafe {
            core::arch::asm!(
                "syscall",
                in("rax") 1,
                in("rdi") 1,
                in("rsi") s.as_ptr(),
                in("rdx") s.len(),
                lateout("rax") _,
                out("rcx") _,
                out("r11") _,
            );
        }
        Ok(())
    }
}

#[unsafe(no_mangle)]
#[cfg(target_os = "none")]
#[unsafe(link_section = ".text.entry")]
fn _start() {
    let ptr;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 9,
            in("rdi") 0,
            in("rsi") 1 << 30,
            in("rdx") 1,
            in("r10") 2,
            in("r8") 0,
            in("r9") 0,
            lateout("rax") ptr,
            out("rcx") _,
            out("r11") _,
        );
    }
    let rd = Reader::new(ptr);
    testfn(rd, &mut Stdout).unwrap();
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 60,
            in("rdi") 0,
        );
    }
    unsafe { hint::unreachable_unchecked() };
}

#[cfg(not(target_os = "none"))]
fn main() {}
"#,
    );
    contents.push_str(&format!(
        r#"
#[cfg(target_os = "none")]
use sol::{dir}::{solution} as testfn;
"#
    ));
    fs::create_dir_all("src/bin").unwrap();
    let path = format!("src/bin/{solution}.rs");
    fs::write(&path, contents).unwrap();

    let mut contents = format!(
        r#"cargo b -r -q --target x86_64-unknown-none --bin {solution}
"#
    );
    for test in &tests {
        contents.push_str(&format!(
            r#"printf {test}
time -f ' %es %MKB' target/x86_64-unknown-none/release/{solution} \
< problems/{dir}/{problem}/in/{test}.in \
> problems/{dir}/{problem}/out/{test}.res
problems/{dir}/{problem}/checker \
problems/{dir}/{problem}/in/{test}.in \
problems/{dir}/{problem}/out/{test}.res \
problems/{dir}/{problem}/out/{test}.out
rm problems/{dir}/{problem}/out/{test}.res
"#
        ));
    }
    let path = format!("../judge_{solution}.sh");
    fs::write(&path, contents).unwrap();
}

#[inline(never)]
pub fn rmcode(solution: &str) {
    fs::remove_file(format!("tests/{solution}.rs")).unwrap();
    fs::remove_file(format!("benches/{solution}.rs")).unwrap();
    fs::remove_file(format!("examples/{solution}.rs")).unwrap();
    fs::remove_file(format!("../bench_{solution}.sh")).unwrap();
    fs::remove_file(format!("src/bin/{solution}.rs")).unwrap();
    fs::remove_file(format!("../judge_{solution}.sh")).unwrap();
    fs::remove_file(format!("../check_{solution}.sh")).unwrap();
    fs::remove_file(format!("../bundle_{solution}.c")).unwrap();
    fs::remove_file(format!("../bundle_{solution}")).unwrap();
}

#[inline(never)]
pub fn bundle(dir: &str, problem: &str, solution: &str) {
    let input_dir = format!("../problems/{dir}/{problem}/in");
    let mut tests: Vec<String> = Vec::new();
    for entry in fs::read_dir(&input_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().map(|ext| ext == "in").unwrap_or(false)
            && let Some(stem) = path.file_stem().and_then(|s| s.to_str())
        {
            tests.push(stem.into());
        }
    }
    tests.sort();

    let status = Command::new("cargo")
        .arg("b")
        .arg("-r")
        .arg("-q")
        .arg("--target")
        .arg("x86_64-unknown-none")
        .arg("--bin")
        .arg(solution)
        .output()
        .unwrap();
    if !status.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&status.stderr));
        panic!();
    }
    let status = Command::new("objcopy")
        .arg("-S")
        .arg("-O")
        .arg("binary")
        .arg(format!("../target/x86_64-unknown-none/release/{solution}"))
        .arg(format!("../target/{solution}"))
        .output()
        .unwrap();
    if !status.status.success() {
        eprintln!("{}", String::from_utf8_lossy(&status.stderr));
        panic!();
    }

    let mut binary = fs::read(format!("../target/{solution}")).unwrap();
    while binary.len() % 8 != 0 {
        binary.push(0);
    }
    let mut contents = String::from(
        r#"#include <stdint.h>
#include <string.h>
#include <sys/mman.h>
const uint64_t bin[] = {"#,
    );
    for chunk in binary.chunks(8) {
        let mut bytes = [0; 8];
        bytes.copy_from_slice(chunk);
        let val = u64::from_ne_bytes(bytes);
        let dec = &format!("{val},");
        let hex = &format!("0x{val:X},");
        contents.push_str(if dec.len() < hex.len() { dec } else { hex });
    }
    contents.push_str(
        r#"};
int main() {
  void (*func)() =
      mmap((void *)0x100000, 0x100000, PROT_READ | PROT_WRITE | PROT_EXEC,
           MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
  memcpy(func, bin, sizeof(bin));
  func();
}
"#,
    );
    let path = format!("../bundle_{solution}.c");
    fs::write(&path, contents).unwrap();

    let mut contents = format!(
        r#"make bundle_{solution} > /dev/null
"#
    );
    for test in &tests {
        contents.push_str(&format!(
            r#"printf {test}
time -f ' %es %MKB' ./bundle_{solution} \
< problems/{dir}/{problem}/in/{test}.in \
> problems/{dir}/{problem}/out/{test}.res
problems/{dir}/{problem}/checker \
problems/{dir}/{problem}/in/{test}.in \
problems/{dir}/{problem}/out/{test}.res \
problems/{dir}/{problem}/out/{test}.out
rm problems/{dir}/{problem}/out/{test}.res
"#
        ));
    }
    let path = format!("../check_{solution}.sh");
    fs::write(&path, contents).unwrap();
}

#[inline(never)]
pub fn test(
    testfn: impl Fn(Reader, &mut dyn Write) -> Result,
    dir: &str,
    problem: &str,
    test: &str,
) {
    let input_path = format!("../problems/{dir}/{problem}/in/{test}.in");
    let result_path = format!("../problems/{dir}/{problem}/out/{test}.res");
    let output_path = format!("../problems/{dir}/{problem}/out/{test}.out");
    let checker_path = format!("../problems/{dir}/{problem}/checker");
    let input_data = fs::read(&input_path).unwrap();
    let rd = Reader::new(input_data.as_ptr());
    let mut wt = String::new();
    testfn(rd, &mut wt).unwrap();
    fs::write(&result_path, wt).unwrap();
    let status = Command::new(&checker_path)
        .arg(&input_path)
        .arg(&result_path)
        .arg(&output_path)
        .output()
        .unwrap();
    if !status.status.success() {
        eprintln!("stderr: {}", String::from_utf8_lossy(&status.stderr));
        eprintln!("Input:  {input_path}");
        eprintln!("Result: {result_path}");
        eprintln!("Output: {output_path}");
        panic!();
    }
    fs::remove_file(&result_path).unwrap();
}

#[inline(never)]
pub fn bench(
    testfn: impl Fn(Reader, &mut dyn Write) -> Result,
    dir: &str,
    problem: &str,
    test: &str,
    b: &mut test::Bencher,
) {
    let input_path = format!("../problems/{dir}/{problem}/in/{test}.in");
    let input_data = fs::read(&input_path).unwrap();
    let rd = Reader::new(input_data.as_ptr());
    b.iter(|| testfn(rd, test::black_box(&mut Sink)).unwrap());
}

#[inline]
pub fn main(testfn: impl Fn(Reader, &mut dyn Write) -> Result) {
    let mut input_data = Vec::new();
    io::stdin().read_to_end(&mut input_data).unwrap();
    let rd = Reader::new(input_data.as_ptr());
    #[cfg(debug_assertions)]
    {
        let mut wt = String::new();
        testfn(rd, &mut wt).unwrap();
        eprintln!("{wt}");
    }
    #[cfg(not(debug_assertions))]
    {
        use std::{env, time::Instant};
        let test = env::args().nth(1).unwrap_or("-".into());
        let time = Instant::now();
        testfn(rd, test::black_box(&mut Sink)).unwrap();
        let d = time.elapsed();
        let s = d.as_secs();
        let ms = d.subsec_millis();
        let us = d.subsec_micros() % 1000;
        let ns = d.subsec_nanos() % 1000;
        if s > 0 {
            eprint!("{test:<35}{s}{ms:03}.{us:03} ms/iter ");
        } else if ms > 0 {
            eprint!("{test:<35} {ms:3}.{us:03} ms/iter ");
        } else if us > 0 {
            eprint!("{test:<35} {us:3}.{ns:03} us/iter ");
        } else {
            eprint!("{test:<35}     {ns:3} ns/iter ");
        }
    }
}

struct Sink;

impl Write for Sink {
    #[inline]
    fn write_str(&mut self, _: &str) -> Result {
        Ok(())
    }
    #[inline]
    fn write_char(&mut self, _: char) -> Result {
        Ok(())
    }
    #[inline]
    fn write_fmt(&mut self, _: Arguments<'_>) -> Result {
        Ok(())
    }
}
