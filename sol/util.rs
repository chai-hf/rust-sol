extern crate std;
extern crate test;
use alloc::{format, string::String, vec::Vec};
use core::fmt::{Result, Write};
use toy::Read;

#[macro_export]
macro_rules! problem {
    ($dir:ident, $problem:ident) => {
        #[test]
        #[ignore]
        #[cfg(test)]
        fn ${concat(gendata_, $problem)}() {
            $crate::util::generate_data(stringify!($dir), stringify!($problem));
        }
        #[test]
        #[ignore]
        #[cfg(test)]
        fn ${concat(rmdata_, $problem)}() {
            $crate::util::remove_data(stringify!($dir), stringify!($problem));
        }
    };
}

#[macro_export]
macro_rules! solution {
    ($dir:ident, $problem:ident,$solution:ident) => {
        #[test]
        #[ignore]
        #[cfg(test)]
        fn ${concat(gentest_, $solution)}() {
            $crate::util::generate_test(
                stringify!($dir),
                stringify!($problem),
                stringify!($solution),
            );
        }
        #[test]
        #[ignore]
        #[cfg(test)]
        fn ${concat(rmtest_, $solution)}() {
            $crate::util::remove_test(stringify!($solution));
        }
    };
}

#[inline(never)]
pub fn generate_data(dir: &str, problem: &str) {
    let status = std::process::Command::new("../problems/generate.py")
        .arg("-p")
        .arg(format!("{dir}/{problem}"))
        .output()
        .expect("Failed to run generate.py");
    if !status.status.success() {
        std::eprintln!("{}", String::from_utf8_lossy(&status.stderr));
        panic!();
    }
}

#[inline(never)]
pub fn remove_data(dir: &str, problem: &str) {
    let status = std::process::Command::new("../problems/generate.py")
        .arg("-p")
        .arg(format!("{dir}/{problem}"))
        .arg("--clean")
        .output()
        .expect("Failed to run generate.py");
    if !status.status.success() {
        std::eprintln!("{}", String::from_utf8_lossy(&status.stderr));
        panic!();
    }
}

#[inline(never)]
pub fn generate_test(dir: &str, problem: &str, solution: &str) {
    let input_dir = format!("../problems/{dir}/{problem}/in");
    let mut tests: Vec<String> = Vec::new();
    for entry in std::fs::read_dir(&input_dir).expect("Failed to read input directory") {
        let entry = entry.expect("Invalid directory entry");
        let path = entry.path();
        if path.extension().map(|ext| ext == "in").unwrap_or(false) {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                tests.push(stem.into());
            }
        }
    }
    tests.sort();
    let test_path = format!("tests/{solution}.rs",);
    let bench_path = format!("benches/{solution}.rs",);
    let mut test_code = format!(
        r#"use sol::{dir}::{solution} as testfn;
const DIR: &str = "{dir}";
const PROBLEM: &str = "{problem}";
"#
    );
    for name in &tests {
        test_code += &format!(
            r#"
#[test]
fn {name}() {{
    let test = "{name}";
    sol::util::test(testfn, DIR, PROBLEM, test);
}}
"#
        );
    }
    let mut bench_code = format!(
        r#"#![feature(test)]
extern crate test;
use sol::{dir}::{solution} as testfn;
const DIR: &str = "{dir}";
const PROBLEM: &str = "{problem}";
"#
    );
    for name in &tests {
        bench_code += &format!(
            r#"
#[bench]
fn {name}(b: &mut test::Bencher) {{
    let test = "{name}";
    sol::util::bench(testfn, DIR, PROBLEM, test, b);
}}
"#
        );
    }
    std::fs::create_dir_all("tests").unwrap();
    std::fs::create_dir_all("benches").unwrap();
    std::fs::write(&test_path, test_code).expect("Failed to write test file");
    std::fs::write(&bench_path, bench_code).expect("Failed to write bench file");
}

#[inline(never)]
pub fn remove_test(solution: &str) {
    let test_path = format!("tests/{solution}.rs",);
    let bench_path = format!("benches/{solution}.rs",);
    std::fs::remove_file(&test_path).expect("Failed to remove test file");
    std::fs::remove_file(&bench_path).expect("Failed to remove bench file");
}

#[inline(never)]
pub fn test(testfn: impl Fn(Read, &mut dyn Write) -> Result, dir: &str, problem: &str, test: &str) {
    let input_path = format!("../problems/{dir}/{problem}/in/{test}.in");
    let result_path = format!("../problems/{dir}/{problem}/res/{test}.res");
    let output_path = format!("../problems/{dir}/{problem}/out/{test}.out");
    let checker_path = format!("../problems/{dir}/{problem}/checker");
    let input_data = std::fs::read(&input_path).expect("Failed to read input file");
    let rd = unsafe { Read::new(input_data.as_ptr()) };
    let mut wt = String::new();
    testfn(rd, &mut wt).unwrap();
    std::fs::create_dir_all(std::path::Path::new(&result_path).parent().unwrap()).unwrap();
    std::fs::write(&result_path, wt).expect("Failed to write result file");
    let status = std::process::Command::new(&checker_path)
        .arg(&input_path)
        .arg(&result_path)
        .arg(&output_path)
        .output()
        .expect("Failed to run checker");
    if !status.status.success() {
        std::eprintln!("stderr: {}", String::from_utf8_lossy(&status.stderr));
        std::eprintln!("Input:  {input_path}");
        std::eprintln!("Result: {result_path}");
        std::eprintln!("Output: {output_path}");
        panic!();
    }
    std::fs::remove_file(&result_path).expect("Failed to remove result file");
}

#[inline(never)]
pub fn bench(
    testfn: impl Fn(Read, &mut dyn Write) -> Result,
    dir: &str,
    problem: &str,
    test: &str,
    b: &mut test::Bencher,
) {
    let input_path = format!("../problems/{dir}/{problem}/in/{test}.in");
    let input_data = std::fs::read(&input_path).expect("Failed to read input file");
    let rd = unsafe { Read::new(input_data.as_ptr()) };
    b.iter(|| testfn(rd, test::black_box(&mut Sink)));
}

struct Sink;

impl Write for Sink {
    fn write_str(&mut self, _: &str) -> core::fmt::Result {
        Ok(())
    }
    fn write_char(&mut self, _: char) -> core::fmt::Result {
        Ok(())
    }
    fn write_fmt(&mut self, _: core::fmt::Arguments<'_>) -> core::fmt::Result {
        Ok(())
    }
}
