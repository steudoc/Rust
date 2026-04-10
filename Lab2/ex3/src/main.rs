
use std::time::SystemTime;

const BSIZE: usize = 20;

#[derive(Debug, Clone)]
pub struct Board {
    boats: [u8; 4],
    data: [[u8; BSIZE]; BSIZE],
}

#[derive(Debug, PartialEq)]
pub enum Error { Overlap, OutOfBounds, BoatCount }

#[derive(Debug)]
pub enum Boat { Vertical(usize), Horizontal(usize) }

enum MyError {
    Simple ( SystemTime ),
    Complex ( SystemTime , String ),
}

pub enum MulErr { Overflow , NegativeNumber }

pub fn mul(a: i32 , b: i32) -> Result <u32, MulErr > {
    if (a < 0 && b > 0) || (a > 0 && b < 0) {
        return Err(MulErr::NegativeNumber);
    } else {
        match a.checked_mul(b) {
            Some(result) => Ok(result as u32),
            None => Err(MulErr::Overflow),
        }
    }
}

fn print_error(e: MyError) {
    match e {
        MyError::Simple ( time ) => eprintln!("Simple error at timestamp {}", time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs()),
        MyError::Complex ( time , msg ) => eprintln!("Complex error at timestamp {}: {}", time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(), msg),
    }
}

fn read_and_write_file(filename: &str) {
    let content = std::fs::read_to_string(filename);
    let mut new_content = String::new();

    match content {
        Ok(text) => {
            for _ in 0..10 {
                new_content.push_str(&text);
            }
            std::fs::write(filename, new_content).expect("Unable to write file");

            println!("File {} has been written 10 times.", filename);
        }
        Err(e) => {
            let error = MyError::Simple(SystemTime::now());
            print_error(error);
            let error = MyError::Complex(SystemTime::now(), e.to_string());
            print_error(error);
        }
    }
}

impl Board {
    /// Crea una board vuota con la disponibilità di navi specificata
    pub fn new(boats: &[u8]) -> Board {
        let mut boats_array = [0; 4];

        for i in 0..4 {
            if i < boats.len() {
                boats_array[i] = boats[i];
            }
        }
        Board {
            boats: boats_array,
            data: [[b' '; BSIZE]; BSIZE],
        }
    }

    /// Crea una board a partire dal contenuto del file (come stringa)
    pub fn from(s: String) -> Board {
        let mut lines = s.lines();
        let mut board = Board::new(&[0; 4]);

        if let Some(boats_line) = lines.next() {
            let boats: Vec<u8> = boats_line
                .split_whitespace()
                .filter_map(|num| num.parse().ok())
                .collect();
            for i in 0..4 {
                if i < boats.len() {
                    board.boats[i] = boats[i];
                }
            }
        }

        for (i, line) in lines.enumerate().take(BSIZE) {
            for (j, c) in line.chars().enumerate().take(BSIZE) {
                board.data[i][j] = c as u8;
            }
        }

        board
    }

    /// Aggiunge la nave; restituisce la nuova Board o un errore
    pub fn add_boat(&mut self, boat: Boat, pos: (usize, usize)) -> Result<&Board, Error> {
        let row = pos.0 - 1;
        let col = pos.1 - 1;

        let (len, is_vertical) = match boat {
            Boat::Vertical(l) => (l, true),
            Boat::Horizontal(l) => (l, false),
        };

        // 1. Controllo disponibilità della nave
        if len < 1 || len > 4 || self.boats[len - 1] == 0 {
            return Err(Error::BoatCount);
        }

        // 2. Controllo limiti (OutOfBounds)
        if is_vertical {
            if row + len > BSIZE || col >= BSIZE {
                return Err(Error::OutOfBounds);
            }
        } else {
            if row >= BSIZE || col + len > BSIZE {
                return Err(Error::OutOfBounds);
            }
        }

        // 3. Controllo sovrapposizione (Overlap)
        for i in 0..len {
            let r = if is_vertical { row + i } else { row };
            let c = if is_vertical { col } else { col + i };

            for dr in -1isize..=1 {
                for dc in -1isize..=1 {
                    let nr = (r as isize + dr) as usize;
                    let nc = (c as isize + dc) as usize;

                    if nr < BSIZE && nc < BSIZE && self.data[nr][nc] != b' ' {
                        return Err(Error::Overlap);
                    }
                }
            }
        }

        // 4. Inserimento della nave
        for i in 0..len {
            let r = if is_vertical { row + i } else { row };
            let c = if is_vertical { col } else { col + i };
            self.data[r][c] = b'B';
        }

        // Aggiorna navi
        self.boats[len-1] -= 1;

        Ok(self)
    }

    /// Converte la board in una stringa salvabile su file
    pub fn to_string(&self) -> String {
        let mut s = format!(
            "{} {} {} {}\n", 
            self.boats[0], self.boats[1], self.boats[2], self.boats[3]
        );

        for row in 0..BSIZE {
            let row_str: String = self.data[row].iter().map(|&b| b as char).collect();
            s.push_str(&row_str);
            s.push('\n');
        }

        s
    }
}

mod test {
    use super::*;

    #[test]
    fn test_new_board() {
        let boats = [1, 2, 3, 4];
        let board = Board::new(&boats);
        assert_eq!(board.boats, boats);
        for row in board.data.iter() {
            for &cell in row.iter() {
                assert_eq!(cell, b' ');
            }
        }
    }

    #[test]
    fn test_add_boat() {
        let boats = [1, 0, 0, 0];
        let mut board = Board::new(&boats);
        let result = board.add_boat(Boat::Horizontal(1), (1, 1));
        assert!(result.is_ok());
        let updated_board = result.unwrap();
        assert_eq!(updated_board.data[0][0], b'B');
        assert_eq!(updated_board.boats[0], 0);
    }

    #[test]
    fn test_add_boat_overlap() {
        let boats = [2, 0, 0, 0];
        let mut board = Board::new(&boats);
        let result = board.add_boat(Boat::Horizontal(1), (1, 1));
        assert!(result.is_ok());
        let result = board.add_boat(Boat::Horizontal(1), (1, 1));
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), Error::Overlap);
    }

    #[test]
    fn test_add_boat_out_of_bounds() {
        let boats = [0, 0, 0, 1];
        let mut board = Board::new(&boats);
        let result = board.add_boat(Boat::Horizontal(4), (18, 18));
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), Error::OutOfBounds);  
    }

    #[test]
    fn test_add_boat_count() {
        let boats = [0, 0, 0, 0];
        let mut board = Board::new(&boats);
        let result = board.add_boat(Boat::Horizontal(1), (1, 1));
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), Error::BoatCount);
    }

}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 4 {
        eprintln!("Usage: cargo run -- <filename.txt> <cmd> <params>");
        std::process::exit(1);
    }

    let filename = &args[1];
    let cmd = &args[2];
    let params = &args[3];

    match cmd.as_str() {
        "new" => {
            let boats: Vec<u8> = params.split(',').filter_map(|s| s.parse().ok()).collect();

            if boats.len() != 4 {
                print_error(MyError::Complex(SystemTime::now(), "Invalid boat configuration".to_string()));
                eprintln!("Usage for 'new': cargo run -- <filename.txt> new <boat1,boat2,boat3,boat4>");
                std::process::exit(1);
            }

            let board = Board::new(&boats);
            std::fs::write(filename, board.to_string()).expect("Unable to write file");
            println!("Board created and saved to {}", filename);
        }
        "add_boat" => {
            let parts: Vec<&str> = params.split(',').collect();

            if parts.len() != 4 {
                print_error(MyError::Complex(SystemTime::now(), "Invalid add_boat parameters".to_string()));
                eprintln!("Usage for 'add_boat': cargo run -- <filename.txt> add_boat <ORIENTATION,length,row,col>");
                std::process::exit(1);
            }

            let is_vert = if parts[0] == "V" { true } else if parts[0] == "H" { false } else {
                print_error(MyError::Complex(SystemTime::now(), "Invalid orientation".to_string()));
                eprintln!("Orientation must be 'V' or 'H'");
                std::process::exit(1);
            };

            let len = parts[1].parse().unwrap_or(0);
            let row = parts[2].parse().unwrap_or(0);
            let col = parts[3].parse().unwrap_or(0);

            let boat = if is_vert { Boat::Vertical(len) } else { Boat::Horizontal(len) };

            let content = std::fs::read_to_string(filename).expect("Unable to read file");
            let mut board = Board::from(content);

            match board.add_boat(boat, (row, col)) {
                Ok(updated_board) => {
                    std::fs::write(filename, updated_board.to_string()).expect("Unable to write file");
                    println!("Boat added successfully to {}", filename);
                }
                Err(e) => {
                    let error = MyError::Complex(SystemTime::now(), format!("Failed to add boat: {:?}", e));
                    print_error(error);
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!("Unknown command: {}", cmd);
            eprintln!("Available commands: new, add_boat");
            std::process::exit(1);
        }
    }
}
