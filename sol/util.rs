extern crate std;
extern crate test;
use std::{
    eprintln,
    fmt::{Arguments, Result, Write},
    format, fs,
    io::{self, Read},
    path::Path,
    process::Command,
    string::String,
    vec::Vec,
};
use toy::Reader;

#[macro_export]
macro_rules! problem {
    ($dir:ident, $problem:ident) => {
        #[cfg(test)]
        mod $problem {
            #[test]
            #[ignore]
            fn generate_data() {
                $crate::util::generate_data(stringify!($dir), stringify!($problem));
            }
            #[test]
            #[ignore]
            fn remove_data() {
                $crate::util::remove_data(stringify!($dir), stringify!($problem));
            }
        }
    };
}

#[macro_export]
macro_rules! solution {
    ($dir:ident, $problem:ident,$solution:ident) => {
        #[cfg(test)]
        mod $solution {
            #[test]
            #[ignore]
            fn generate_code() {
                $crate::util::generate_code(
                    stringify!($dir),
                    stringify!($problem),
                    stringify!($solution),
                );
            }
            #[test]
            #[ignore]
            fn remove_code() {
                $crate::util::remove_code(stringify!($solution));
            }
        }
    };
}

#[inline(never)]
pub fn generate_data(dir: &str, problem: &str) {
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
pub fn remove_data(dir: &str, problem: &str) {
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
pub fn generate_code(dir: &str, problem: &str, solution: &str) {
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

    let mut code = String::new();
    writeln!(code, "use sol::{dir}::{solution} as testfn;").unwrap();
    writeln!(code).unwrap();
    writeln!(code, "fn main() {{").unwrap();
    writeln!(code, "    sol::util::main(testfn);").unwrap();
    writeln!(code, "}}").unwrap();
    fs::create_dir_all("src/bin").unwrap();
    let path = format!("src/bin/{solution}.rs");
    fs::write(&path, code).unwrap();

    let mut code = String::new();
    writeln!(code, "use sol::{dir}::{solution} as testfn;").unwrap();
    writeln!(code, "const DIR: &str = \"{dir}\";").unwrap();
    writeln!(code, "const PROBLEM: &str = \"{problem}\";").unwrap();
    for test in &tests {
        writeln!(code).unwrap();
        writeln!(code, "#[test]").unwrap();
        writeln!(code, "fn _{test}() {{").unwrap();
        writeln!(code, "    let test = \"{test}\";").unwrap();
        writeln!(code, "    sol::util::test(testfn, DIR, PROBLEM, test);").unwrap();
        writeln!(code, "}}").unwrap();
    }
    fs::create_dir_all("tests").unwrap();
    let path = format!("tests/{solution}.rs");
    fs::write(&path, code).unwrap();

    let mut code = String::new();
    writeln!(code, "#![feature(test)]").unwrap();
    writeln!(code, "extern crate test;").unwrap();
    writeln!(code, "use sol::{dir}::{solution} as testfn;").unwrap();
    writeln!(code, "const DIR: &str = \"{dir}\";").unwrap();
    writeln!(code, "const PROBLEM: &str = \"{problem}\";").unwrap();
    for test in &tests {
        writeln!(code).unwrap();
        writeln!(code, "#[bench]").unwrap();
        writeln!(code, "fn _{test}(b: &mut test::Bencher) {{").unwrap();
        writeln!(code, "    let test = \"{test}\";").unwrap();
        writeln!(code, "    sol::util::bench(testfn, DIR, PROBLEM, test, b);").unwrap();
        writeln!(code, "}}").unwrap();
    }
    fs::create_dir_all("benches").unwrap();
    let path = format!("benches/{solution}.rs");
    fs::write(&path, code).unwrap();

    let mut code = String::new();
    writeln!(code, "cargo b -r -q --bin {solution}").unwrap();
    for test in &tests {
        write!(code, "\"$@\" target/release/{solution} {test} < ").unwrap();
        writeln!(code, "problems/{dir}/{problem}/in/{test}.in").unwrap();
    }
    let path = format!("../judge_{solution}.sh");
    fs::write(&path, code).unwrap();
}

#[inline(never)]
pub fn remove_code(solution: &str) {
    let main_path = format!("src/bin/{solution}.rs");
    let test_path = format!("tests/{solution}.rs");
    let bench_path = format!("benches/{solution}.rs");
    let judge_path = format!("../judge_{solution}.sh");
    fs::remove_file(&main_path).unwrap();
    fs::remove_file(&test_path).unwrap();
    fs::remove_file(&bench_path).unwrap();
    fs::remove_file(&judge_path).unwrap();
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
            eprintln!("{test:<40} {s}{ms:03}.{us:03},{ns:03} ms/iter");
        } else if ms > 0 {
            eprintln!("{test:<40}  {ms:3}.{us:03},{ns:03} ms/iter");
        } else if us > 0 {
            eprintln!("{test:<40}      {us:3}.{ns:03} us/iter");
        } else {
            eprintln!("{test:<40}          {ns:3} ns/iter");
        }
    }
}

#[inline(never)]
pub fn test(
    testfn: impl Fn(Reader, &mut dyn Write) -> Result,
    dir: &str,
    problem: &str,
    test: &str,
) {
    let input_path = format!("../problems/{dir}/{problem}/in/{test}.in");
    let result_path = format!("../problems/{dir}/{problem}/res/{test}.res");
    let output_path = format!("../problems/{dir}/{problem}/out/{test}.out");
    let checker_path = format!("../problems/{dir}/{problem}/checker");
    let input_data = fs::read(&input_path).unwrap();
    let rd = Reader::new(input_data.as_ptr());
    let mut wt = String::new();
    testfn(rd, &mut wt).unwrap();
    fs::create_dir_all(Path::new(&result_path).parent().unwrap()).unwrap();
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
    b.iter(|| testfn(rd, test::black_box(&mut Sink)));
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
