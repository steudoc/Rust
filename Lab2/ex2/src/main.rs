
mod table;

fn conv(c: char) -> char {
    if let Some(pos) = table::SUBS_I.chars().position(|x| x == c) {
        table::SUBS_O.chars().nth(pos).unwrap_or('-')
    } else if c.is_ascii_alphanumeric() {
        c
    } else {
        '-'
    }
}

fn slugify(s: &str) -> String {
    let mut result = String::new();
    let mut last_was_dash = false;

    for c in s.to_lowercase().chars() {
        let conv_c = conv(c);
        if conv_c == '-' {
            if !last_was_dash {
                result.push('-');
                last_was_dash = true;
            }
        } else {
            result.push(conv_c);
            last_was_dash = false;
        }
    }
    result.trim_matches('-').to_string()
}

#[ cfg( test )]
mod tests {
    use super ::*;

    #[ test ]
    fn test_conv_accented() {
        assert_eq!(conv('é'), 'e');
    }

    #[test]
    fn test_conv_plain() {
        assert_eq!(conv('a'), 'a');
    }

    #[test]
    fn test_conv_invalid() {
        assert_eq!(conv('!'), '-');
    }

    #[test]
    fn test_conv_unknown_accent() {
        assert_eq!(conv(' '), '-');
    }

    #[test]
    fn test_slug_multiword() {
        assert_eq!(slugify("Hello World"), "hello-world");
    }

    #[test]
    fn test_slug_accented() {
        assert_eq!(slugify("Café"), "cafe");
    }

    #[test]
    fn test_slug_empty() {
        assert_eq!(slugify(""), "");
    }

    #[test]
    fn test_slug_consecutive_spaces() {
        assert_eq!(slugify("Hello   World"), "hello-world");
    }

    #[test]
    fn test_slug_consecutive_invalid() {
        assert_eq!(slugify("Hello!!!World"), "hello-world");
    }

    #[test]
    fn test_slug_only_invalid() {
        assert_eq!(slugify("!!!"), "");
    }

    #[test]
    fn test_slug_trailing_space() {
        assert_eq!(slugify("Hello World "), "hello-world");
    }

    #[test]
    fn test_slug_trailing_invalid() {
        assert_eq!(slugify("Hello World!!"), "hello-world");
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run -- <string>");
        std::process::exit(1);
    }
    let input = &args[1];
    let slug = slugify(input);
    println!("{}", slug);
}
