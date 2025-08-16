use std::{
    fmt::Result,
    fs,
    io::{self, Read},
    process::Command,
};
use toy::Reader;

pub fn gen_data(dir: &str, problem: &str) {
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

pub fn rm_data(dir: &str, problem: &str) {
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

pub fn gen_code(dir: &str, problem: &str, solution: &str) {
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
const PROBLEM: &str = "{problem}";
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

    let mut contents = format!(
        r#"set -e
echo "Running {solution} tests"
cargo build --release --quiet --example {solution}
"#
    );
    for test in &tests {
        contents.push_str(&format!(
            r#"printf {test}
time -f ' %es %MKB' target/release/examples/{solution} \
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

pub fn rm_code(solution: &str) {
    fs::remove_file(format!("tests/{solution}.rs")).unwrap();
    fs::remove_file(format!("examples/{solution}.rs")).unwrap();
    fs::remove_file(format!("../judge_{solution}.sh")).unwrap();
}

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

pub fn main(testfn: impl Fn(Reader, &mut String) -> Result) {
    let mut input_data = Vec::new();
    io::stdin().read_to_end(&mut input_data).unwrap();
    let rd = Reader::new(input_data.as_ptr());
    let mut wt = String::new();
    testfn(rd, &mut wt).unwrap();
    println!("{wt}");
}
