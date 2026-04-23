mod slugify;
mod table;

use crate::slugify::MySlug;

#[ cfg( test )]
mod tests {
    use super ::*;

    #[test]
    fn test_is_slug_uppercase() {
        let string = "Hello";
        assert_eq!(string.is_slug(), false);
    }

    #[test]
    fn test_is_slug_space() {
        let string = "hello world";
        assert_eq!(string.is_slug(), false);
    }

    #[test]
    fn test_is_slug_non_ascii() {
        let string = "hèllo";
        assert_eq!(string.is_slug(), false);
    }

    #[test]
    fn test_is_slug_numeric() {
        let string = "h6llo";
        assert_eq!(string.is_slug(), true);
    }

    #[test]
    fn test_is_slug_dash() {
        let string = "h-llo";
        assert_eq!(string.is_slug(), true);
    }

    #[test]
    fn test_is_slug_double_dash() {
        let string = "h--llo";
        assert_eq!(string.is_slug(), false);
    }

    #[test]
    fn test_is_slug_dash_last() {
        let string = "hello-";
        assert_eq!(string.is_slug(), false);
    }

    #[test]
    fn test_is_slug_string() {
        let s1 = String::from("Hello String");
        assert_eq!(s1.is_slug(), false);
    }

    #[test]
    fn test_is_slug_str() {
        let s2 = "hello-slice";
        assert_eq!(s2.is_slug(), true);
    }

    #[test]
    fn test_to_slug_string() {
        let s1 = String::from("Hello String");
        assert_eq!(s1.to_slug(), "hello-string");
    }

    #[test]
    fn test_to_slug_str() {
        let s2 = "hello-slice";
        assert_eq!(s2.to_slug(), "hello-slice");
    }
}

fn main() {
    let s1 = String::from("Hello String");
    let s2 = "hello-slice";

    println!("{}", s1.is_slug()); // false
    println!("{}", s2.is_slug()); // true

    let s3: String = s1.to_slug();
    let s4: String = s2.to_slug();
    println !("s3:{} s4:{} ", s3 , s4); // s3:hello-string s4:hello-slice
}
