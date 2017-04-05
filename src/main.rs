use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

fn print_help() {
    println!("-h : help");
    println!("-p : package (first column)");
    println!("-f : file to read");
    println!("-s : start value for keys");
}

fn main() {
    let mut package = None;
    let mut start = 0;
    let mut file = None;
    for arg in env::args() {
        let arg = arg.split('=').collect::<Vec<&str>>();
        match arg[0] {
            "-h" => { // Help
                print_help();
                process::exit(0);
            },
            "-p" => { // Package
                package = Some(arg[1].to_string());
            },
            "-f" => { // File
                file = Some(File::open(arg[1]).unwrap());
            },
            "-s" => { // Start value
                start = arg[1].parse().unwrap();
            },
            _ => {
            }
        }
    }
    if let None = file {
        print_help();
        process::exit(0);
    }
    if let None = package {
        print_help();
        process::exit(0);
    }
    let package = package.unwrap();
    let file = BufReader::new(file.unwrap());
    let mut count = 0;
    for line in file.lines().filter_map(|result| result.ok()) {
        match line.chars().nth(0) {
            Some('#') | Some('\n') | Some('\r') | None => {
                continue;
            },
            _ => {
            }
        }
        count += 1;
        let mut parts = line.splitn(4, " ");
        let _ = parts.next().unwrap(); // 'set'
        let path = parts.next().unwrap();
        let key = parts.next().unwrap();
        let val = parts.next().unwrap();
        print!(",\"{},{},{},,{}\"", package, path, key, val);
    }
    println!("");
    println!("");
    for i in 0..count {
        print!(",\"SparkOnDemandPatchEntry{}\"", i + start);
    }
    println!("");
}
