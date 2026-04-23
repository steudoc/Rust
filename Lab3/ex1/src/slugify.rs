
use crate::table;

pub trait MySlug {
    fn is_slug(&self) -> bool;
    fn to_slug(&self) -> String;
}

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

impl <T> MySlug for T
where
    T: AsRef<str>,
{
    fn is_slug(&self) -> bool {
        let s = self.as_ref();
        let mut last_was_dash = false;

        for c in s.chars() {
            if c == '-'{    // if c is a dash
                if last_was_dash {
                    return false;
                } else {
                    last_was_dash = true;
                }        
            } else {    // if c is not a dash
                if !c.is_ascii_alphanumeric() {   // if c is a special character return false
                    return false;
                }
                if c.is_ascii_uppercase() { // if c is an alphabetic character and is uppercase, ther return false
                    return false;
                }

                last_was_dash = false;
            }      
        }

        if s.chars().last() == Some('-') {
            return false;
        }
        true
    }

    fn to_slug(&self) -> String {

        let s = self.as_ref();

        if s.is_slug() {
            s.to_string()
        } else {
            slugify(s)
        }
    }
}