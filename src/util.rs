use std::error;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path;
pub fn read_to_buff(buff: &mut String) {
    let _ = io::stdin()
        .read_line(buff)
        .expect("Attempting stdio input.");

    if let Some('\n') = buff.chars().next_back() {
        buff.pop();
    }
    if let Some('\r') = buff.chars().next_back() {
        buff.pop();
    }
}

pub fn read_filepath(file_path: String) -> Result<String, io::Error> {
    fs::read_to_string(file_path)
}
