mod libs;
use colored::Colorize;
use std::env;
use std::fs;
use walkdir::WalkDir;

struct Config {
    pattern: String,
    file_path: Vec<String>,
    arguments: Vec<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);
    process_input(&config);
}

impl Config {
    fn new(args: &[String]) -> Config {
        let mut pattern = String::new();
        let mut file_path: Vec<String> = Vec::new();
        let mut arguments: Vec<String> = Vec::new();

        for (index, value) in args.iter().enumerate() {
            if index == 0 {
                continue;
            } else if index == 1 {
                pattern = value.clone();
            } else {
                match value.as_str() {
                    "-i" => arguments.push(value.clone()),
                    "-n" => arguments.push(value.clone()),
                    "-v" => arguments.push(value.clone()),
                    "-r" => arguments.push(value.clone()),
                    "-f" => arguments.push(value.clone()),
                    "-c" => arguments.push(value.clone()),
                    "-h" | "--help" => arguments.push(value.clone()),
                    _ => file_path.push(value.clone()),
                }
            }
        }
        Config {
            pattern,
            file_path,
            arguments,
        }
    }
}

fn has_opt(config: &Config, flag: &str) -> bool {
    config.arguments.iter().any(|s| s == flag)
}

//goes through the cmd_argument vector and calls functions accordingly
fn process_input(config: &Config) {
    let pattern = &config.pattern;
    let pattern_color = pattern.red();

    let mut files: Vec<String> = Vec::new();

    if has_opt(config, "-r") {
        for root in &config.file_path {
            for file_entry in WalkDir::new(root) {
                let file_entry = file_entry.unwrap();
                if file_entry.file_type().is_file() {
                    files.push(file_entry.path().display().to_string());
                }
            }
        }
    } else {
        files.extend(config.file_path.iter().cloned());
    }

    let option_print_file = has_opt(config, "-f");
    let option_invert = has_opt(config, "-v");
    let option_insensitive = has_opt(config, "-i");
    let option_colored = has_opt(config, "-c");
    let option_line_nums = has_opt(config, "-n");

    for file in files {
        let contents = fs::read_to_string(&file).expect("file read");
        if option_print_file {
            println!("{}", file);
        }

        if option_line_nums {
            for (index, line) in contents.lines().enumerate() {
                let matched = if option_insensitive {
                    let q = pattern.to_ascii_lowercase();
                    let l = line.to_ascii_lowercase();
                    let contains_pat = l.contains(&q);
                    if option_invert {
                        !contains_pat
                    } else {
                        contains_pat
                    }
                } else {
                    let contains_pat = line.contains(pattern);
                    if option_invert {
                        !contains_pat
                    } else {
                        contains_pat
                    }
                };

                if matched {
                    if option_colored {
                        let colored_line = line.replace(pattern, &pattern_color.to_string());
                        println!("{}: {}", index + 1, colored_line);
                    } else {
                        println!("{}: {}", index + 1, line);
                    }
                }
            }
        } else {
            let lines = if option_invert {
                libs::inverse_search(pattern, &contents)
            } else if option_insensitive {
                libs::case_insensitive_search(pattern, &contents)
            } else {
                libs::search(pattern, &contents)
            };
            if option_colored {
                for line in lines {
                    let pattern_color = pattern.red();
                    let colored_line = line.replace(pattern, &pattern_color.to_string());
                    println!("{}", colored_line);
                }
            } else {
                for line in lines {
                    println!("{}", line);
                }
            }
        }
    }
    if has_opt(config, "-h") || has_opt(config, "--help") {
        println!("{}", libs::print_help());
    }
}
