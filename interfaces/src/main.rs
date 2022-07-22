use std::env;
use std::process::{Command, Output};
use straprs::*;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let cmd = parse_args(&mut args);

    let filename = cmd.leader.as_str();

    let block: Vec<String> = read_backends_file(filename);
    let intf: Interface = build_interface(block).expect("could not parse interface");

    let _ = create_impl_file(filename.to_string());
    let _ = insert_into_file(format!("package {}", cmd.package).as_str(), filename);
    let _ = insert_into_file(intf.build_implementation().as_str(), filename);

    let format_out = format_file(filename.to_string());
    let str_out = String::from_utf8(format_out.stdout).expect("failed to parse output");
    let _ = overwrite_file(str_out.as_str(), filename);

    let imports_out = apply_imports(filename.to_string());
    let str_out_out = String::from_utf8(imports_out.stdout).expect("failed to parse output");
    let _ = overwrite_file(str_out_out.as_str(), filename);
}

struct Cmd {
    leader: String,
    package: String,
}

fn parse_args(args: &mut Vec<String>) -> Cmd {
    let leader: String = String::from(&args[1]);
    let package: String = String::from(&args[2]);
    Cmd {
        leader: leader,
        package: package,
    }
}

fn insert_into_file(data: &str, file: &str) -> Output {
    let dir: String = directory_from_file(file.to_string());
    let a: String = format!("echo '{}' >> {}/backends_impl.go", data, dir);
    let o = Command::new("sh")
        .arg("-c")
        .arg(a.as_str())
        .output()
        .expect("failed to exec");
    o
}

fn overwrite_file(data: &str, file: &str) -> Output {
    let dir: String = directory_from_file(file.to_string());
    let a: String = format!("echo '{}' > {}/backends_impl.go", data, dir);
    let o = Command::new("sh")
        .arg("-c")
        .arg(a.as_str())
        .output()
        .expect("failed to exec");
    o
}

fn create_impl_file(filename: String) -> Output {
    let dir: String = directory_from_file(filename.clone());
    let a: String = format!("touch {}/backends_impl.go", dir);
    println!("{}", a);
    let o = Command::new("sh")
        .arg("-c")
        .arg(a.as_str())
        .output()
        .expect("failed to exec");
    o
}

fn directory_from_file(file: String) -> String {
    let split: Vec<&str> = file.split("/").collect();
    let ancestors = &split[0..split.len() - 1];

    String::from(ancestors.join("/"))
}

fn format_file(file: String) -> Output {
    let dir: String = directory_from_file(file.clone());
    let a: String = format!("gofmt {}/backends_impl.go", dir);

    let o = Command::new("sh")
        .arg("-c")
        .arg(a.as_str())
        .output()
        .expect("failed to exec");
    o
}

fn apply_imports(file: String) -> Output {
    let dir: String = directory_from_file(file.clone());
    let a: String = format!("goimports {}/backends_impl.go", dir);

    let o = Command::new("sh")
        .arg("-c")
        .arg(a.as_str())
        .output()
        .expect("failed to exec");
    o
}
