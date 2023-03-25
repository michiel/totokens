use std::path::PathBuf;
use walkdir::WalkDir;

pub fn list_files(path: &PathBuf) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            files.push(entry.path().to_owned());
        }
    }
    files
}

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn get_ignorelist(path: &PathBuf) -> Vec<regex::Regex> {
    let mut regex_list = Vec::new();

    if let Ok(file) = File::open(path) {
        for line in BufReader::new(file).lines() {
            let pattern = line.expect("Failed to read line");
            let pattern = pattern.trim();

            // Skip blank lines and comments
            if pattern.is_empty() || pattern.starts_with('#') {
                continue;
            }

            // Escape special characters in the pattern
            let pattern = regex::escape(pattern);

            // Replace gitignore-style wildcards with regex syntax
            let pattern = pattern.replace("\\*", ".*");
            let pattern = pattern.replace("\\?", ".");

            let regex = Regex::new(&format!("^{}$", pattern)).expect("Invalid regex");
            regex_list.push(regex);
        }
    }

    regex_list
}

pub fn filter_strings(strings: Vec<String>, regexes: Vec<Regex>) -> Vec<String> {
    let mut filtered_strings = Vec::new();

    for string in strings {
        let mut matched = false;
        for regex in &regexes {
            if regex.is_match(&string) {
                matched = true;
                break;
            }
        }
        if !matched {
            filtered_strings.push(string);
        }
    }

    filtered_strings
}

pub fn filter_paths(paths: Vec<PathBuf>, regexes: Vec<Regex>) -> Vec<PathBuf> {
    let mut filtered_paths = Vec::new();

    for path in paths {
        let mut matched = false;
        for regex in &regexes {
            if regex.is_match(path.to_str().unwrap_or("")) {
                matched = true;
                break;
            }
        }
        if !matched {
            filtered_paths.push(path);
        }
    }

    filtered_paths
}

use std::fs;

pub fn concat_file_contents_with_separator(paths: &Vec<PathBuf>) -> String {
    let s: String = paths
        .into_iter()
        .filter_map(|path| {
            if let Ok(contents) = fs::read_to_string(&path) {
                Some(format!("---- {}\n{}\n", path.to_string_lossy(), contents))
            } else {
                None
            }
        })
        .collect();
    s
}

#[cfg(test)]
mod test {
    use crate::util::filter_paths;
    use crate::util::filter_strings;
    use regex::Regex;
    use std::path::PathBuf;

    #[test]
    fn test_filter_strings() {
        let strings = vec![
            "foo.txt".to_string(),
            "bar.rs".to_string(),
            "baz.py".to_string(),
        ];

        let regexes = vec![Regex::new("\\.rs$").unwrap(), Regex::new("\\.py$").unwrap()];

        let filtered_strings = filter_strings(strings, regexes);
        assert_eq!(filtered_strings, ["foo.txt".to_string()]);
    }

    #[test]
    fn test_filter_paths() {
        let paths = vec![
            PathBuf::from("/path/to/file.txt"),
            PathBuf::from("/path/to/another_file.rs"),
            PathBuf::from("/path/to/yet_another_file.py"),
        ];

        let regexes = vec![Regex::new("\\.rs$").unwrap(), Regex::new("\\.py$").unwrap()];

        let filtered_paths = filter_paths(paths, regexes);

        assert_eq!(filtered_paths, [PathBuf::from("/path/to/file.txt")]);
    }
}
