// WARNING: 
// - the lifetimes are not set correctly, you have to set them to make it compile
// - you have also to implemment missing functions and fix the code
// - *** see test test functions in the code for usage examples 

use std::io::Error;
use std::fs::File;
use std::io::Read;
use regex::Regex;
// (1) LineEditor: implement functionality
pub struct LineEditor {
    lines: Vec<String>,
}

impl LineEditor {
    pub fn new(s: String) -> Self {
        Self {
            lines: s.lines().map(|l| l.to_string()).collect(),
        }
    }

    // create a new LineEditor from a file
    pub fn from_file(file_name: &str) -> Result<Self, Error> {
        // open file in read mode
        let mut file = File::open(file_name)?;

        // read file content
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let result = Self::new(contents);
        Ok(result)
    }

    pub fn all_lines(&self) -> Vec<&str> {
        self.lines.iter().map(|l| l.as_str()).collect()
    }

    pub fn replace(&mut self, line: usize, start: usize, end: usize, subst: &str) {
        self.lines[line].replace_range(start..end, subst)
    }
} 

// (2) Match contains the information about the match. Fix the lifetimes
// repl will contain the replacement.
// It is an Option because it may be not set yet or it may be skipped 
struct Match<'a> {
    pub line: usize,
    pub start: usize,
    pub end: usize,
    pub text: &'a str,
    pub repl: Option<String>,
}

// use the crate "regex" to find the pattern and its method find_iter for iterating over the matches
// modify if necessary, this is just an example for using a regex to find a pattern
fn find_example<'a>(lines: &'a Vec<&'a str>, pattern: &'a str) -> Vec<Match<'a>> {
    let mut matches = Vec::new();
    let re = regex::Regex::new(pattern).unwrap();
    for (line_idx, line) in lines.iter().enumerate() {
        for mat in re.find_iter(line) {
            matches.push(Match {
                line: line_idx,
                start: mat.start(),
                end: mat.end(),
                text: &line[mat.start()..mat.end()],
                repl: None,
            });
        }
    }
    matches
}
 
// (3) Fix the lifetimes of the FindReplace struct
// (4) implement the Finder struct
struct FindReplace<'a> {
    lines: Vec<&'a str>,
    pattern: String,
    matches: Vec<Match<'a>>,
}

impl<'a> FindReplace<'a> {
    pub fn new(lines: Vec<&'a str>, pattern: &'a str) -> Self {
        let re = Regex::new(pattern).unwrap();
        let mut matches = Vec::new();

        for (index, line) in lines.iter().enumerate() {
            for mat in re.find_iter(line) {
                matches.push(Match {
                    line: index,
                    start: mat.start(),
                    end: mat.end(),
                    text: &line[mat.start()..mat.end()],
                    repl: None,
                });
            }
        }

        Self{ lines, pattern: pattern.to_string(), matches }
    }

    // return all the matches
    pub fn matches(&self) -> &Vec<Match<'a>> {
        &self.matches
    }

    // apply a function to all matches and allow to accept them and set the repl
    // useful for promptig the user for a replacement
    pub fn apply(&mut self, fun: impl Fn(&mut Match) -> bool) {
        for mat in self.matches.iter_mut() {
            fun(mat);
        }
    }
}
   
//(5) how FindReplace should work together with the LineEditor in order
// to replace the matches in the text
#[test]
fn test_find_replace() {
    let s = "Hello World.\nA second line full of text.";
    let mut editor = LineEditor::new(s.to_string());

    let lines = editor.all_lines();
    let mut finder = FindReplace::new(lines, "ll");

    // find all the matches and accept them 
    finder.apply(|m| {
        println!("{} {} {} {}", m.line, m.start, m.end, m.text);
        m.repl = Some("some repl".to_string());
        true
    });

    /* 
    // now let's replace the matches
    // why this loop won't work?
    for m in finder.matches() {
        // m: Match
        let s = match m.repl {
            Some(s) => s,
            None => "".to_string(),
        };
        editor.replace(m.line, m.start, m.end, &s);
    } */   

    // alternate method: why this one works? 

    let mut subs = Vec::new();
    for m in finder.matches() {
        if let Some(ref repl) = m.repl {
            subs.push((m.line, m.start, m.end, repl.clone()));
        }
    }
    
    for (line, start, end, subst) in subs {
        editor.replace(line, start, end, &subst);
    }

}


// (6) sometimes it's very expensive to find all the matches at once before applying 
// the changes
// we can implement a lazy finder that finds just the next match and returns it
// each call to next() will return the next match
// this is a naive implementation of an Iterarator

#[derive(Debug, Clone, Copy)]
struct FinderPos {
    pub line: usize,
    pub offset: usize,
}

struct LazyFinder<'a> {
    lines: Vec<&'a str>,
    pattern: String,
    pos: Option<FinderPos>,
}

impl<'a> LazyFinder<'a> {
    pub fn new(lines: Vec<&'a str>, pattern: &str) -> Self {
        let finder = FinderPos{ line: 0, offset: 0 };
        Self {
            lines,
            pattern: pattern.to_string(),
            pos: Some(finder),
        }
    }

    pub fn next(&mut self) -> Option<Match<'a>> {
        // remember:
        // return None if there are no more matches
        // return Some(Match) if there is a match
        // each time save the position of the match for the next call
        
        let re = regex::Regex::new(&self.pattern).unwrap();

        let pos = self.pos?;

        for (line_idx, line) in self.lines.iter().enumerate().skip(pos.line) {
            // on starting the line, only consider matches at or after pos.offset
            // on subsequent lines, starts from 0
            let search_from = if line_idx == pos.line {pos.offset} else {0};

            if let Some(mat) = re.find(&line[search_from..]) {
                let abs_start = search_from + mat.start();
                let abs_end = search_from + mat.end();

                self.pos = Some(FinderPos { line: line_idx, offset: abs_end });
                
                return Some(Match {
                    line: line_idx,
                    start: abs_start,
                    end: abs_end,
                    text: &line[abs_start..abs_end],
                    repl: None,
                });
            }
        }
        self.pos = None;
        None
    }
}

// (7) example of how to use the LazyFinder
#[test]
fn test_lazy_finder() {
    let s = "Hello World.\nA second line full of text.";
    let mut editor = LineEditor::new(s.to_string());

    let lines = editor.all_lines();
    let mut finder = LazyFinder::new(lines, "ll");

    // find all the matches and accept them 
    while let Some(m) = finder.next() {
        println!("{} {} {} {}", m.line, m.start, m.end, m.text);
    }
}


// (8) now you have everything you need to implement the real Iterator

struct FindIter<'a> {
    lines: Vec<&'a str>,
    pattern: String,
    pos: Option<FinderPos>,
}

impl<'a> FindIter<'a> {
    pub fn new(lines: Vec<&'a str>, pattern: &str) -> Self {
        Self { 
            lines, 
            pattern: pattern.to_string(),
            pos: Some(FinderPos { line: 0, offset: 0 }), 
        }
    }
}

impl<'a> Iterator for FindIter<'a> {
    type Item = Match<'a>; // <== we inform the Iterator that we return a Match

    fn next(&mut self) -> Option<Self::Item> {
        let re = regex::Regex::new(&self.pattern).unwrap();

        let pos = self.pos?;

        for (line_idx, line) in self.lines.iter().enumerate().skip(pos.line) {
            // on starting the line, only consider matches at or after pos.offset
            // on subsequent lines, starts from 0
            let search_from = if line_idx == pos.line {pos.offset} else {0};

            if let Some(mat) = re.find(&line[search_from..]) {
                let abs_start = search_from + mat.start();
                let abs_end = search_from + mat.end();

                self.pos = Some(FinderPos { line: line_idx, offset: abs_end });

                return Some(Match {
                    line: line_idx,
                    start: abs_start,
                    end: abs_end,
                    text: &line[abs_start..abs_end],
                    repl: None,
                });
            }
        }
        self.pos = None;
        None
    }
}

// (9) test the find iterator
#[test]
fn test_find_iter() {
    let s = "Hello World.\nA second line full of text.";
    let mut editor = LineEditor::new(s.to_string());

    let lines = editor.all_lines();
    let mut finder = FindIter::new(lines, "ll");

    // find all the matches and accept them 
    for m in finder {
        println!("{} {} {} {}", m.line, m.start, m.end, m.text);
    
    }
}

