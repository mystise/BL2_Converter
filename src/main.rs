use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::io::{BufRead, BufReader};

fn print_help() {
    println!("-h : help");
    println!("-f : files to read, comma separated list");
}

enum HotfixType {
    OnDemand(Option<String>),
    Level(Option<String>),
    Patch
}

fn main() {
    let mut files = None;
    for arg in env::args() {
        let arg = arg.split('=').collect::<Vec<&str>>();
        match arg[0] {
            "-h" => { // Help
                print_help();
                return;
            },
            "-f" => { // Files
                files = Some(arg[1].split(',').map(|x| (Path::new(x).to_path_buf(), File::open(x).expect(&format!("Can't find file: {}", x)))).collect::<Vec<(PathBuf, File)>>());
            },
            _ => {
            }
        }
    }
    if let None = files {
        print_help();
        return;
    }
    let files = files.unwrap();
    if files.len() == 0 {
        print_help();
        return;
    }

    let mut keys = Vec::new();
    let mut values = Vec::new();

    let mut on_demand_index = 1;
    let mut level_index = 1;
    let mut patch_index = 1;

    let mut current_type = HotfixType::Patch;

    for file in files {
        let prefix = file.0.file_name().expect("File name not found").to_str().expect("File name not unicode").splitn(2, ".").next().expect("File has no name");
        let file = BufReader::new(file.1);

        for line in file.lines().filter_map(|result| result.ok()) {
            match line.chars().nth(0) {
                Some('#') | Some('\n') | Some('\r') | None => {
                    continue;
                },
                _ => {
                }
            }
            let mut parts = line.splitn(2, " ");
            let command = parts.next().expect(&format!("Syntax error: {}", line)).to_lowercase();
            let data = parts.next().expect(&format!("Syntax error: {}", line));
            match command.as_str() {
                "set" => {
                    let mut parts = data.splitn(3, " ");
                    match &current_type {
                        &HotfixType::Level(ref x) => {
                            keys.push(format!("SparkLevelPatchEntry-{}{}", prefix, level_index));
                            values.push(format!("{},{},{},,{}", x.clone().unwrap_or("".to_string()), parts.next().expect(&format!("Syntax error: {}", line)), parts.next().expect(&format!("Syntax error: {}", line)), parts.next().expect(&format!("Syntax error: {}", line))));
                            level_index += 1;
                        },
                        &HotfixType::OnDemand(ref x) => {
                            keys.push(format!("SparkOnDemandPatchEntry-{}{}", prefix, on_demand_index));
                            values.push(format!("{},{},{},,{}", x.clone().unwrap_or("".to_string()), parts.next().expect(&format!("Syntax error: {}", line)), parts.next().expect(&format!("Syntax error: {}", line)), parts.next().expect(&format!("Syntax error: {}", line))));
                            on_demand_index += 1;
                        },
                        &HotfixType::Patch => {
                            keys.push(format!("SparkPatchEntry-{}{}", prefix, patch_index));
                            values.push(format!("{},{},,{}", parts.next().expect(&format!("Syntax error: {}", line)), parts.next().expect(&format!("Syntax error: {}", line)), parts.next().expect(&format!("Syntax error: {}", line))));
                            patch_index += 1;
                        }
                    }
                },
                "set_cmp" => {
                    let mut parts = data.splitn(4, " ");
                    match &current_type {
                        &HotfixType::Level(ref x) => {
                            keys.push(format!("SparkLevelPatchEntry-{}{}", prefix, level_index));
                            values.push(format!("{},{},{},{},{}", x.clone().unwrap_or("".to_string()), parts.next().expect(&format!("Syntax error: {}", line)), parts.next().expect(&format!("Syntax error: {}", line)), parts.next().expect(&format!("Syntax error: {}", line)), parts.next().expect(&format!("Syntax error: {}", line))));
                            level_index += 1;
                        },
                        &HotfixType::OnDemand(ref x) => {
                            keys.push(format!("SparkOnDemandPatchEntry-{}{}", prefix, on_demand_index));
                            values.push(format!("{},{},{},{},{}", x.clone().unwrap_or("".to_string()), parts.next().expect(&format!("Syntax error: {}", line)), parts.next().expect(&format!("Syntax error: {}", line)), parts.next().expect(&format!("Syntax error: {}", line)), parts.next().expect(&format!("Syntax error: {}", line))));
                            on_demand_index += 1;
                        },
                        &HotfixType::Patch => {
                            keys.push(format!("SparkPatchEntry-{}{}", prefix, patch_index));
                            values.push(format!("{},{},{},{}", parts.next().expect(&format!("Syntax error: {}", line)), parts.next().expect(&format!("Syntax error: {}", line)), parts.next().expect(&format!("Syntax error: {}", line)), parts.next().expect(&format!("Syntax error: {}", line))));
                            patch_index += 1;
                        }
                    }
                },
                "start" => {
                    // Level, OnDemand, and Patch
                    let mut parts = data.splitn(2, " ");
                    let command = parts.next().expect(&format!("Syntax error: {}", line)).to_lowercase();
                    match command.as_str() {
                        "ondemand" => {
                            let package = parts.next().expect(&format!("Syntax error: {}", line));
                            current_type = HotfixType::OnDemand(if package.to_lowercase() == "none" {
                                None
                            } else {
                                Some(package.to_string())
                            });
                        },
                        "level" => {
                            let package = parts.next().expect(&format!("Syntax error: {}", line));
                            current_type = HotfixType::Level(if package.to_lowercase() == "none" {
                                None
                            } else {
                                Some(package.to_string())
                            });
                        },
                        "patch" => {
                            current_type = HotfixType::Patch;
                        },
                        _ => {
                        }
                    }
                },
                _ => {
                }
            }
        }
    }
    print!("set Transient.SparkServiceConfiguration_6 Keys (");
    for i in 0..keys.len() {
        if i != 0 {
            print!(",");
        }
        print!("\"{}\"", keys[i]);
    }
    println!(")\r");
    println!("\r");
    println!("\r");
    print!("set Transient.SparkServiceConfiguration_6 Values (");
    for i in 0..values.len() {
        if i != 0 {
            print!(",");
        }
        print!("\"{}\"", values[i]);
    }
    println!(")\r");
}
