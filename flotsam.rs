use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::process;

fn main() {
    let log_directory = get_log_directory();
    let mut user_counts: HashMap<String, u32> = HashMap::new();
    let mut consecutive_invalid_lines = 0;

    // Recursive function to process log files in the directory
    process_logs_in_directory(&log_directory, &mut user_counts, &mut consecutive_invalid_lines);

    // Generate and sort the TSV log file
    let mut sorted_entries: Vec<_> = user_counts.iter().collect();
    sorted_entries.sort_by(|a, b| a.0.cmp(b.0)); // Sort by username (column 1)

    // Create the timestamp for the log file name
    let timestamp = chrono::Local::now().format("%y%m%d%H%M%S").to_string();
    let log_path = format!("~/flotsam{}.log", timestamp); // Use the appropriate timestamp

    // Write the sorted entries to the TSV log file
    let mut log_file = File::create(log_path).expect("Failed to create log file");
    for (username, count) in sorted_entries {
        let line = format!("{}\t{}\n", username, count);
        log_file
            .write_all(line.as_bytes())
            .expect("Failed to write to log file");
    }
}

fn process_logs_in_directory(
    directory: &str,
    user_counts: &mut HashMap<String, u32>,
    consecutive_invalid_lines: &mut u32,
) {
    if let Ok(entries) = std::fs::read_dir(directory) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                process_log_file(&path, user_counts, consecutive_invalid_lines);
            } else if path.is_dir() {
                let subdir = path.to_string_lossy();
                process_logs_in_directory(&subdir, user_counts, consecutive_invalid_lines);
            }
        }
    }
}

fn process_log_file(
    file_path: &std::path::Path,
    user_counts: &mut HashMap<String, u32>,
    consecutive_invalid_lines: &mut u32,
) {
    let file = File::open(file_path).expect("Failed to open log file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if is_valid_driftwood_format(&line) {
            let fields: Vec<&str> = line.split('\t').collect();
            if fields.len() == 7 {
                let username = fields[4];
                let count = user_counts.entry(username.to_string()).or_insert(0);
                *count += 1;
                *consecutive_invalid_lines = 0; // Reset the counter
            } else {
                *consecutive_invalid_lines += 1;
            }
        } else {
            *consecutive_invalid_lines += 1;
        }

        if *consecutive_invalid_lines >= 3 {
            eprintln!("Error: The log file path may need review.");
            process::exit(1); // Exit with a non-zero code to indicate an error
        }
    }
}

fn is_valid_driftwood_format(line: &str) -> bool {
    let fields: Vec<&str> = line.split('\t').collect();
    fields.len() == 7
}

fn get_log_directory() -> String {
    // Check if a log directory argument was provided
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        return args[1].clone();
    }

    // Prompt the user for the log directory
    println!("Enter the log directory path or press Enter to exit:");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let log_directory = input.trim();

    if log_directory.is_empty() {
        std::process::exit(0); // Exit if the user pressed Enter
    }

    log_directory.to_string()
}
