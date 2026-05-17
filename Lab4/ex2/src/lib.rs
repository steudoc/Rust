

// to warm up: the define step by step an adapter for filtering even numbers

pub mod simple_even_iter {
    // (1) let start with a simple iterator adapter for just one type, "i32"
    // see the adapter pattern example in the pdf "Adapter Pattern..."
    struct EvenIter<I> {
        inner: I, // hint: it's a generic type... here we don't care about bounds yet 
    }

    impl<I> EvenIter<I> {
        fn new(iter: I) -> Self {
            Self{ inner: iter }
        }
    }

    impl<I> Iterator for EvenIter<I> 
    where 
        I: Iterator<Item =  i32>,  // here we need to define the bounds for the generic type
        // T it must be an iterator over i32
    {
        type Item = i32; // <== it will work just for i32

        fn next(&mut self) -> Option<Self::Item> {
            
            loop {
                match self.inner.next() {
                    Some(n) if n % 2 == 0 => return Some(n), // even
                    Some(_) => continue,    // odd
                    None => return None,    // exhausted
                }
            }
        }
    }

    // if EvenIter works the test will compile and pass
    #[test]
    fn test_simple_even_iter() {
        let v = vec![1, 2, 3, 4, 5];
        // why iter() does not work here?
        let it = EvenIter::new(v.into_iter());
        for i in it {
            println!("i: {}", i);
        }
    }
 
    // (2) now let's add the adapter to all Iterator<Item=i32> (adavanced)
    trait AddEvenIter: Iterator 
    where
        Self: Sized
    {
        // add even() to anyone implementing this trait
        // usage: v.into_iter().even() ....
        fn even(self) -> EvenIter<Self>{
            EvenIter::new(self)
        }
    }

    // (3) add here the generic implemention, you can supply it for all the iterators
    impl<I> AddEvenIter for I
    where 
        I: Iterator<Item = i32> + Sized,
    {}

    #[test] 
    fn test_adapter() {
        let v = vec![1,2,3,4,5];
        for i in v.into_iter().even() {
            println!("{}", i);
        }
    }

}

pub mod even_iter {
    // (4) more adavanced: implement for all integer types 
    // => install the external crate "num" to have some Traits identifying all number types
    use num;

    // the generic parameters I and U are already defined for you in the struct deinition
    // (5) write in a comment in plain english the meaning of the generic parameters 
    // and their constraints

    /*
        I = the concrete iterator type (e.g. std::vec::IntoIter<u64>)
            constraint: it must yield items of tipe U
        U = the type of each item the iterator produces (e.g. u64, i32, u8, ...)
            constraint: it must be an integer (num::Integer) so we can call .is_even()
                        it must be Copy so we can return it by value without moving
    */
    struct EvenIter<I, U> 
    where 
        I: Iterator<Item = U> 
    {
        iter: I
    }

    impl<I,U> Iterator for EvenIter<I, U> 
    where 
        U: num::Integer + Copy, 
        I: Iterator<Item = U> 
    {
        type Item = U;

        fn next(&mut self) -> Option<Self::Item> {
            
            loop {
                match self.iter.next() {
                    Some(n) if n.is_even() => return Some(n), // even
                    Some(_) => continue,    // odd
                    None => return None,    // exhausted
                }
            }
        }
        
    }

    // (6) once implemented, the test will compile and pass
    #[test]
    fn test_even_iter() {
        let mut v: Vec<u64> = vec![1, 2, 3, 4, 5];
        let mut it = EvenIter { iter: v.into_iter() };
        for i in it {
            println!("i: {}", i);
        }
    }
}


// finally let's implement the grep command
// (1) install the "walkdir" crate for walking over directories using an iterator
// install also the "regex" crate for regular expressions

use std::io;

use regex::Regex;
use walkdir;

// (2) define the match result
struct Match {
    file: String, 
    line: usize,
    text: String
}

// (3) test walkdir iterator, see how errors are handled
#[test]
fn test_walk_dir() {
    let wdir = walkdir::WalkDir::new("/tmp");
    for entry in wdir.into_iter() {
        match entry {
            Ok(e) => println!("{}", e.path().display()),
            Err(e) => println!("Error: {}", e),
        }
    }
} 

// (3) define the grep adapter for the iterator
// add anything you need implement it
struct GrepIter {
    inner: walkdir::IntoIter,
    pattern: Regex,
    buffer: Vec<(usize, String)>,
    current_file: String,
}

impl GrepIter {
    fn new(iter: walkdir::IntoIter, pattern: &str) -> Self {
        GrepIter { 
            inner: iter,
            pattern: Regex::new(pattern).unwrap(),
            buffer: Vec::new(),
            current_file: String::new(),
        }
    }
}

use std::fs::File;
use std::io::BufRead;

impl Iterator for GrepIter {
    type Item = Result<Match, walkdir::Error>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            // (A) drain any buffered matches from the current file first
            if let Some((line_num, text)) = self.buffer.pop() {
                return Some(Ok(Match { 
                    file: self.current_file.clone(), 
                    line: line_num, 
                    text, 
                }));
            }

            // (B) buffer is empty - advance to the next directory entry
            let entry = match self.inner.next()? {
                Ok(e) => e,
                Err(e) => return Some(Err(e)),
            };

            // (C) skip directories, only process files
            if !entry.file_type().is_file() {
                continue;
            }

            // (D) open the file, skip if unreadable (binary, permissions etc. )
            let file = match File::open(entry.path()) {
                Ok(f) => f,
                Err(_) => continue,
            };

            self.current_file = entry.path().display().to_string();

            // (E) scan lines, buffer every matching one (reserved so pop() is FIFO)
            let mut matches: Vec<(usize, String)> = io::BufReader::new(file)
                .lines()
                .enumerate()
                .filter_map(|(idx, line)| {
                    let line = line.ok()?;
                    self.pattern.is_match(&line)
                    .then(|| (idx + 1, line))
                })
                .collect();

            matches.reverse();
            self.buffer = matches;
        }
    }
}

#[test]
fn test_grep_iter() {
    let wdir = walkdir::WalkDir::new("/tmp");
    let grep_iter = GrepIter::new(wdir.into_iter(), "hello");
    for entry in grep_iter {
        match entry {
            Ok(m) => { println!("File: {}, Line: {}, Text: {}", m.file, m.line, m.text); }
            Err(e) => { println!("Error: {}", e); }
        }
    }
}

// (5) add grep() to IntoIter  (see the first example in EvenIter for i32)

trait Grep {
    fn grep(self, pattern: &str) -> GrepIter;
}

impl Grep for walkdir::IntoIter {
    fn grep(self, pattern: &str) -> GrepIter {
        GrepIter::new(self, pattern)
    }
}

#[test]
fn test_grep() {
    let wdir = walkdir::WalkDir::new("/tmp");
    let grep_iter = wdir.into_iter().grep("hello");
    for entry in grep_iter {
        match entry {
            Ok(m) => { println!("File: {}, Line: {}, Text: {}", m.file, m.line, m.text); }
            Err(e) => { println!("Error: {}", e); }
        }
    }
}


