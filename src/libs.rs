pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn inverse_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if !line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn case_insensitive_search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query_lower_val = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_ascii_lowercase().contains(&query_lower_val) {
            results.push(line);
        }
    }
    results
}

pub fn print_help() -> String {
    let mut help = String::new();
    help.push_str("Usage: grep [OPTIONS] <pattern> <files...>\n");
    help.push_str("Options:\n");
    help.push_str("-i                Case-insensitive search\n");
    help.push_str("-n                Print line numbers\n");
    help.push_str("-v                Invert match (exclude lines that match the pattern)\n");
    help.push_str("-r                Recursive directory search\n");
    help.push_str("-c                Enable colored output\n");
    help.push_str("-h, --help        Show help information\n");
    help
}
