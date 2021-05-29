use std::process::Command;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

pub fn health() -> String {
    return pool_info_command("health");
}

pub fn capacity() -> i8 {
    let mut storage_capacity = pool_info_command("capacity");
    // We need to remove trailing % symbol
    storage_capacity.pop();
    return storage_capacity.parse().unwrap_or(0);
}

pub fn io_errors() -> Vec<String> {
    let pool_command = Command::new("zpool")
        .arg("status")
        .output()
        .expect("Unable to check zPool I/O errors");

    let reader = BufReader::new(&*pool_command.stdout);

    return reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| line.find("ONLINE").is_some())
        .filter(|line| is_line_ok(line))
        .map(|line| {
            let chunks:Vec<&str> = line.split_whitespace().collect();

            return "Disk ".to_owned() + &chunks[0] + " is faulty.";
        }).collect();
}

fn is_line_ok(line: &String) -> bool {
    let chunks:Vec<&str> = line.split_whitespace().collect();
    if chunks.len() < 5 {
        return false;
    }

    let read_errors: i8 = FromStr::from_str(&chunks[2]).unwrap();
    let write_errors: i8 = FromStr::from_str(&chunks[3]).unwrap();
    let checksum_errors: i8 = FromStr::from_str(&chunks[4]).unwrap();
    if read_errors > 0 || write_errors > 0 || checksum_errors > 0 {
        return true;
    }

    return false;
}

fn pool_info_command(element: &str) -> String {
    let pool_command = Command::new("zpool")
        .arg("list")
        .arg("-H")
        .arg("-o")
        .arg(element)
        .output()
        .expect("Unable to check zPool status");

    return std::str::from_utf8(&pool_command.stdout).unwrap().trim().to_string();
}