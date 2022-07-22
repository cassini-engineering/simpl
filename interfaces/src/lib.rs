use convert_case::{Case, Casing};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

pub struct FunctionSignature {
    name: String,
    ret: String,
}

pub fn parse_func_sig(line: String) -> Option<FunctionSignature> {
    let split: Vec<&str> = line.split(" ").collect();

    if split.len() == 1 {
        return None;
    }

    let split_one = split[0];
    let split_two = split[1];

    let s_split_one: Vec<&str> = split_one.split("").collect();
    let l = s_split_one.len();
    if s_split_one[l - 2] != ")" || s_split_one[l - 3] != "(" {
        return None;
    }
    let f = &s_split_one.as_slice()[1..l - 3];

    let f_as_string = f.join("");

    Some(FunctionSignature {
        name: f_as_string,
        ret: split_two.to_string(),
    })
}

impl FunctionSignature {
    pub fn build_method(&self) -> String {
        let mut q: String = String::from("").to_owned();

        q.push_str("func (b *backends) ");
        q.push_str(&self.name);
        q.push_str("() ");
        q.push_str(&self.ret);
        q.push_str(" { return b.");
        q.push_str(&self.name.to_case(Case::Snake));
        q.push_str("_client } \n\n");

        q
    }
}

pub struct Interface {
    functions: Vec<FunctionSignature>,
}

pub fn build_interface(block: Vec<String>) -> Option<Interface> {
    let length = block.len();
    let mut functions: Vec<FunctionSignature> = vec![];
    for l in 0..length {
        let sig = parse_func_sig(block[l].clone());
        match sig {
            None => continue,
            Some(x) => functions.push(x),
        }
    }

    Some(Interface {
        functions: functions,
    })
}

impl Interface {
    pub fn build_implementation(&self) -> String {
        let mut ret: String = String::from("").to_owned();

        for sig in &self.functions {
            ret.push_str(&sig.build_method())
        }

        ret
    }
}

pub fn read_backends_file(filename: &str) -> Vec<String> {
    let mut res: Vec<String> = vec![];

    if let Ok(lines) = read_lines(filename) {
        for l in lines {
            if let Ok(ip) = l {
                if ip == "}" {
                    break;
                }
                res.push(ip);
            }
        }
    }

    res
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
