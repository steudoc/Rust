
pub struct Config {
    pub filename: String,
    pub head: usize,
}

pub fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Config, String> {

    let filename = match args.next() {
        Some(args) => args,
        None => return Err(String::from("Missing arguments: <file.csv> is mandatory!")),
    };

    let mut head = 10;

    while let Some(arg) = args.next() {
        if arg == "--head" {
            let head_str = match args.next() {
                Some(val) => val,
                None => return Err(String::from("Missing value for parameter --head")),
            };

            head = match head_str.parse::<usize>() {
                Ok(n) => n,
                Err(_) => return Err(String::from("The Head parameter must be an integer number >= 0")),
            }
        } else {
            return Err(String::from("Not recognised parameter {arg}"));
        }
    }

    Ok(Config { filename, head})
}