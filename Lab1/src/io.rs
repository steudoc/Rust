use crate::cli::Config;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn process_csv(config: &Config) -> Result<(), String> {
    let file = File::open(&config.filename).map_err(|e| format!("Error opening file '{}': {}", &config.filename, e))?;
    let reader = BufReader::new(file);

    let mut total_rows = 0;
    let mut head_lines = Vec::new();

    for line_result in reader.lines() {
        let line = line_result.map_err(|e| format!("Error reading line '{}': {}", total_rows+1, e))?;

        total_rows += 1;

        if total_rows <= config.head {
            head_lines.push(line);
        }
    }

    println!("rows: {}", total_rows);
    println!("head: {}", config.head);
    for line in head_lines {
        println!("{}", line);
    }

    Ok(())
}