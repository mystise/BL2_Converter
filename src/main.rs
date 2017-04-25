#[macro_use]
extern crate clap;

use std::fs::{self, File};
use std::path::Path;
use std::io::{BufRead, BufReader, Write};

#[derive(Clone)]
enum ParsedLine {
    Set(String, String, String), // Object, path, value
    SetCmp(String, String, String, String), // Object, path, compare value, value
    Start(HotfixType)
}

#[derive(Clone)]
enum HotfixType {
    OnDemand(Option<String>),
    Level(Option<String>),
    Patch
}

fn main() {
    let matches = clap_app!(BL2_Converter =>
                            (version: "1.1")
                            (about: "Converts from *.hotfix files to executable BL2 console commands")
                            (@arg FILE: ... "Files to convert, if empty will take all files named '*.hotfix' in the current folder")
                            (@arg OUTPUT: -o --output +takes_value "File to output to, defaults to 'hotfix_output.txt'")
    ).get_matches();

    let mut files = Vec::new();
    if let Some(input_files) = matches.values_of("FILE") {
        for filename in input_files {
            files.push((Path::new(filename).to_path_buf(), File::open(filename).expect(&format!("Can't find file: {}", filename))));
        }
    } else {
        for dir_entry in fs::read_dir(".").unwrap().map(|x| x.unwrap()) {
            if let Some(extension) = dir_entry.path().extension() {
                if extension.to_str().unwrap() == "hotfix" {
                    files.push((dir_entry.path(), File::open(dir_entry.path()).expect(&format!("Can't find file: {}", dir_entry.path().file_name().unwrap().to_str().unwrap()))));
                }
            }
        }
    }

    let mut output = {
        if let Some(output) = matches.value_of("OUTPUT") {
            File::create(output)
        } else {
            File::create("hotfix_output.txt")
        }
    }.unwrap();

    let mut lines = Vec::new();

    for file in files {
        let prefix = file.0.file_name().expect("File name not found").to_str().expect("File name not unicode").splitn(2, ".").next().expect("File has no name").to_string();
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
                    lines.push((prefix.clone(), ParsedLine::Set(parts.next().expect(&format!("Syntax error: {}", line)).to_string(), parts.next().expect(&format!("Syntax error: {}", line)).to_string(), parts.next().expect(&format!("Syntax error: {}", line)).to_string())));
                },
                "set_cmp" => {
                    let mut parts = data.splitn(4, " ");
                    lines.push((prefix.clone(), ParsedLine::SetCmp(parts.next().expect(&format!("Syntax error: {}", line)).to_string(), parts.next().expect(&format!("Syntax error: {}", line)).to_string(), parts.next().expect(&format!("Syntax error: {}", line)).to_string(), parts.next().expect(&format!("Syntax error: {}", line)).to_string())));
                },
                "start" => {
                    // Level, OnDemand, and Patch
                    let mut parts = data.splitn(2, " ");
                    let command = parts.next().expect(&format!("Syntax error: {}", line)).to_lowercase();
                    let mut hotfix_type = HotfixType::Patch;
                    match command.as_str() {
                        "ondemand" => {
                            let package = parts.next().expect(&format!("Syntax error: {}", line));
                            hotfix_type = HotfixType::OnDemand(if package.to_lowercase() == "none" {
                                None
                            } else {
                                Some(package.to_string())
                            });
                        },
                        "level" => {
                            let package = parts.next().expect(&format!("Syntax error: {}", line));
                            hotfix_type = HotfixType::Level(if package.to_lowercase() == "none" {
                                None
                            } else {
                                Some(package.to_string())
                            });
                        },
                        _ => {
                        }
                    }
                    lines.push(("".to_string(), ParsedLine::Start(hotfix_type)))
                },
                _ => {
                }
            }
        }
    }

    let mut keys = Vec::new();
    let mut values = Vec::new();

    let mut current_type = HotfixType::Patch;

    let mut on_demand_index = 1;
    let mut level_index = 1;
    let mut patch_index = 1;

    for parsed_line in lines {
        match parsed_line {
            (prefix, ParsedLine::Set(obj, path, value)) => {
                match &current_type {
                    &HotfixType::Level(ref x) => {
                        keys.push(format!("SparkLevelPatchEntry-{}{}", prefix, level_index));
                        values.push(format!("{},{},{},,{}", x.clone().unwrap_or("".to_string()), obj, path, value));
                        level_index += 1;
                    },
                    &HotfixType::OnDemand(ref x) => {
                        keys.push(format!("SparkOnDemandPatchEntry-{}{}", prefix, on_demand_index));
                        values.push(format!("{},{},{},,{}", x.clone().unwrap_or("".to_string()), obj, path, value));
                        on_demand_index += 1;
                    },
                    &HotfixType::Patch => {
                        keys.push(format!("SparkPatchEntry-{}{}", prefix, patch_index));
                        values.push(format!("{},{},,{}", obj, path, value));
                        patch_index += 1;
                    }
                }
            },
            (prefix, ParsedLine::SetCmp(obj, path, cmp_value, value)) => {
                match &current_type {
                    &HotfixType::Level(ref x) => {
                        keys.push(format!("SparkLevelPatchEntry-{}{}", prefix, level_index));
                        values.push(format!("{},{},{},{},{}", x.clone().unwrap_or("".to_string()), obj, path, cmp_value, value));
                        level_index += 1;
                    },
                    &HotfixType::OnDemand(ref x) => {
                        keys.push(format!("SparkOnDemandPatchEntry-{}{}", prefix, on_demand_index));
                        values.push(format!("{},{},{},{},{}", x.clone().unwrap_or("".to_string()), obj, path, cmp_value, value));
                        on_demand_index += 1;
                    },
                    &HotfixType::Patch => {
                        keys.push(format!("SparkPatchEntry-{}{}", prefix, patch_index));
                        values.push(format!("{},{},{},{}", obj, path, cmp_value, value));
                        patch_index += 1;
                    }
                }
            },
            (_, ParsedLine::Start(hotfix_type)) => {
                current_type = hotfix_type;
            }
        }
    }

    write!(output, "set Transient.SparkServiceConfiguration_6 Keys (").unwrap();
    for i in 0..keys.len() {
        if i != 0 {
            write!(output, ",").unwrap();
        }
        write!(output, "\"{}\"", keys[i]).unwrap();
    }
    writeln!(output, ")").unwrap();
    writeln!(output, "").unwrap();
    write!(output, "set Transient.SparkServiceConfiguration_6 Values (").unwrap();
    for i in 0..values.len() {
        if i != 0 {
            write!(output, ",").unwrap();
        }
        write!(output, "\"{}\"", values[i]).unwrap();
    }
    writeln!(output, ")").unwrap();
}
