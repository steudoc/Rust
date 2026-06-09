mod cli;
mod io;

fn main() {
    let args = match cli::parse_args(std::env::args()) {
        Ok(a) => a,
        Err(e) => {
            eprint!("error: {e}");
            eprintln!("usage: cargo run -- <file.csv> [--head <N>]");
            std::process::exit(1);
        }
    };

    let data = match io::read_csv(&args.file, args.head) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    };

    println!("rows: {}", data.total_rows);
    println!("head ({}):", args.head);
    for line in &data.head {
        println!("{line}");
    }
}
