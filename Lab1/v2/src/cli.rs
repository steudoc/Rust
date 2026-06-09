pub struct Args {
    pub file: String,
    pub head: usize,
}

#[derive(Debug)]
pub enum ArgsError {
    MissingFile,
    UnknownArg(String),
    MissingHeadValue,
    InvalidHeadValue(String),
    NegativeHead(String),
}

impl std::fmt::Display for ArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgsError::MissingFile => write!(f, "missing argument: <file.csv>"),
            ArgsError::UnknownArg(a) => write!(f, "unknown argument: {a}"),
            ArgsError::MissingHeadValue => write!(f, "--head requires a value"),
            ArgsError::InvalidHeadValue(v) => write!(f, "--head: not a number: {v}"),
            ArgsError::NegativeHead(v) => write!(f, "--head: must be >= 0, got {v}"),
        }
    }
}

pub fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Args, ArgsError> {
    args.next(); // skip program's name

    let file = args.next().ok_or(ArgsError::MissingFile)?;
    let mut head = 10usize; // default

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--head" => {
                let val = args.next().ok_or(ArgsError::MissingHeadValue)?;

                let n: i64 = val.parse()
                    .map_err(|_| ArgsError::InvalidHeadValue(val.clone()))?;
                if n < 0 {
                    return Err(ArgsError::NegativeHead(val));
                }
                head = n as usize;
            },
            other => return Err(ArgsError::UnknownArg(other.to_string())),
        }
    }

    Ok(Args { file, head }) 
}