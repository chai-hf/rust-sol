use std::{
    fmt::Result,
    fs,
    io::{self, Read},
    process::Command,
};
use toy::Reader;

#[inline(never)]
pub fn gen_tests(dir: &str, problem: &str) {
    let status = Command::new("../problems/generate.py")
        .arg("-p")
        .arg(format!("{dir}/{problem}"))
        .output()
        .unwrap();
    if !status.status.success() {
        println!("{}", String::from_utf8_lossy(&status.stderr));
        panic!();
    }
}

#[inline(never)]
pub fn rm_tests(dir: &str, problem: &str) {
    let status = Command::new("../problems/generate.py")
        .arg("-p")
        .arg(format!("{dir}/{problem}"))
        .arg("--clean")
        .output()
        .unwrap();
    if !status.status.success() {
        println!("{}", String::from_utf8_lossy(&status.stderr));
        panic!();
    }
}

#[inline(never)]
pub fn gen_scripts(dir: &str, problem: &str, solution: &str) {
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
        r#"use sol::{dir}::{problem}::{solution} as testfn;
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

    let contents = format!(
        r#"fn main() {{
    util::main(sol::{dir}::{problem}::{solution});
}}
"#
    );
    fs::create_dir_all("examples").unwrap();
    let path = format!("examples/{solution}.rs");
    fs::write(&path, contents).unwrap();

    let mut contents = String::from(
        r#"#![no_std]
#![no_main]
extern crate alloc;

use alloc::string::String;
use core::{
    alloc::{GlobalAlloc, Layout},
    hint,
};
use toy::Reader;

#[panic_handler]
#[cfg(target_os = "none")]
fn panic(_: &core::panic::PanicInfo) -> ! {
    unsafe { hint::unreachable_unchecked() }
}

#[global_allocator]
static GLOBAL: GlobalAllocImpl = GlobalAllocImpl;

struct GlobalAllocImpl;

unsafe impl GlobalAlloc for GlobalAllocImpl {
    #[inline]
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
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
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
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

    #[inline]
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        unsafe { self.alloc(layout) }
    }
}

#[unsafe(no_mangle)]
#[unsafe(link_section = ".text.entry")]
fn _start() {
    let ptr;
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 9,
            in("rdi") 0,
            in("rsi") 0x40000000,
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
    let mut wt = String::with_capacity(0x40000000);
    testfn(rd, &mut wt).unwrap();
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 1,
            in("rdi") 1,
            in("rsi") wt.as_ptr(),
            in("rdx") wt.len(),
            lateout("rax") _,
            out("rcx") _,
            out("r11") _,
        );
    }
    unsafe {
        core::arch::asm!(
            "syscall",
            in("rax") 60,
            in("rdi") 0,
        );
    }
    unsafe { hint::unreachable_unchecked() };
}
"#,
    );
    contents.push_str(&format!(
        r#"
use sol::{dir}::{problem}::{solution} as testfn;
"#
    ));
    fs::create_dir_all("../bin/src/bin").unwrap();
    let path = format!("../bin/src/bin/{solution}.rs");
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
pub fn rm_scripts(solution: &str) {
    fs::remove_file(format!("tests/{solution}.rs")).unwrap();
    fs::remove_file(format!("examples/{solution}.rs")).unwrap();
    fs::remove_file(format!("../bin/src/bin/{solution}.rs")).unwrap();
    fs::remove_file(format!("../judge_{solution}.sh")).unwrap();
    fs::remove_file(format!("../bundle_{solution}.c")).unwrap();
    fs::remove_file(format!("../check_{solution}.sh")).unwrap();
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
        .arg("--workspace")
        .arg("-r")
        .arg("-q")
        .arg("--target")
        .arg("x86_64-unknown-none")
        .arg("--bin")
        .arg(solution)
        .output()
        .unwrap();
    if !status.status.success() {
        println!("{}", String::from_utf8_lossy(&status.stderr));
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
        println!("{}", String::from_utf8_lossy(&status.stderr));
        panic!();
    }

    let mut binary = fs::read(format!("../target/{solution}")).unwrap();
    while !binary.len().is_multiple_of(8) {
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
pub fn test(testfn: impl Fn(Reader, &mut String) -> Result, dir: &str, problem: &str, test: &str) {
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
        println!("stderr: {}", String::from_utf8_lossy(&status.stderr));
        println!("Input:  {input_path}");
        println!("Result: {result_path}");
        println!("Output: {output_path}");
        panic!();
    }
    fs::remove_file(&result_path).unwrap();
}

#[inline]
pub fn main(testfn: impl Fn(Reader, &mut String) -> Result) {
    let mut input_data = Vec::new();
    io::stdin().read_to_end(&mut input_data).unwrap();
    let rd = Reader::new(input_data.as_ptr());
    let mut wt = String::new();
    testfn(rd, &mut wt).unwrap();
    println!("{wt}");
}
