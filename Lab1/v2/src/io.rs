use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub enum IoError {
    NotFound(String),
    PermissionDenied(String),
    ReadError(String),
}

impl std::fmt::Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IoError::NotFound(p) => write!(f, "file not found: {p}"),
            IoError::PermissionDenied(p) => write!(f, "permission denied: {p}"),
            IoError::ReadError(p) => write!(f, "read error: {p}"),
        }
    }
}

pub struct CsvData {
    pub total_rows: usize,
    pub head: Vec<String>,
}

pub fn read_csv(path: &str, head_n: usize) -> Result<CsvData, IoError> {
    let file = File::open(path).map_err(|e| match e.kind() {
        io::ErrorKind::NotFound => IoError::NotFound(path.to_string()),
        io::ErrorKind::PermissionDenied => IoError::PermissionDenied(path.to_string()),
        _ => IoError::ReadError(path.to_string()),
    })?;

    let reader = BufReader::new(file);
    let mut total_rows = 0usize;
    let mut head = Vec::new();

    for line in reader.lines() {
        let line = line.map_err(|_| IoError::ReadError(path.to_string()))?;
        if total_rows < head_n {
            head.push(line);
        }
        total_rows += 1;
    }

    Ok(CsvData { total_rows, head })
}