use std::fs::OpenOptions;
use std::io::{Write, Seek, BufRead, BufReader};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    // If not proper argument count, print usage and exit.
    if args.len() != 3 {
        println!("Usage: cargo run <filename> <text>");
        std::process::exit(1);
    }

    // Get the filename from the command line arguments
    let filename = &args[1];
    let content = &args[2];

    // Open the file for read and append, create it if it doesn't exist, handle errors with match
    let file = OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(filename);
    let mut file = match file {
        Ok(file) => file,
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::PermissionDenied => {
                    panic!("Permission denied: {}", error)
                }
                _ => {
                    panic!("Error opening file: {}", error)
                }
            }
        }
    };

    // Append the content to the file
    match file.write(format!("{}\n", content).as_bytes()) {
        Ok(_) => (),
        Err(error) => {
            panic!("Error writing to file: {}", error)
        }
    }

    // Flush the file to ensure all data is written
    match file.flush() {
        Ok(_) => (),
        Err(error) => {
            panic!("Error flushing file: {}", error)
        }
    }

    // Move the file cursor to the beginning of the file
    match file.seek(std::io::SeekFrom::Start(0)) {
        Ok(_) => (),
        Err(error) => {
            panic!("Error seeking in file: {}", error)
        }
    }
    // Read the file line by line and print each line  
    let reader = BufReader::new(file);
    for line in reader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(error) => {
                panic!("Error reading line: {}", error)
            }
        }
    }
}