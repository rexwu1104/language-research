use std::{path::Path, fs};

pub fn read_raw(file_name: &str) -> String {
    if is_file(file_name) {
        fs::read_to_string(file_name).expect(&["'{}' not found", file_name].concat())
    } else {
        String::new()
    }
}

fn is_file(file_name: &str) -> bool {
    Path::new(file_name).is_file()
}