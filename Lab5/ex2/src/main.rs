use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
pub enum FSError {
    NotFound(String),
    NotADirectory(String),
    AlreadyExists(String),
    NotAFile(String),
    InvalidPath(String),
    PermissionDenied(String),  // per from_disk
}

impl std::fmt::Display for FSError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FSError::NotFound(p)       => write!(f, "not found: {p}"),
            FSError::NotADirectory(p)  => write!(f, "not a directory: {p}"),
            FSError::AlreadyExists(p)  => write!(f, "already exists: {p}"),
            FSError::NotAFile(p)       => write!(f, "not a file: {p}"),
            FSError::InvalidPath(p)    => write!(f, "invalid path: {p}"),
            FSError::PermissionDenied(p) => write!(f, "permission denied: {p}"),
        }
    }
}

type FSLink = Rc<RefCell<FSItem>>;
type FSWeakLink = Weak<RefCell<FSItem>>;

pub struct Directory {
    pub name: String,
    pub children: Vec<FSLink>,
    pub parent: Option<FSWeakLink>,
}

pub struct File {
    pub name: String,
    pub content: Vec<u8>,
    pub parent: Option<FSWeakLink>,
}

pub struct Link {
    pub target: String,
    pub parent: Option<FSWeakLink>,
}

pub enum FSItem {
    Directory(Directory), // nome , figli , metadati , padre
    File(File), // nome , metadati , padre
    SymLink(Link), // path a cui punta , padre
}

pub struct FileSystem {
    root: FSLink,
    current: FSLink,
}

impl FileSystem {
    pub fn new() -> Self {
        let root = Rc::new(RefCell::new(FSItem::Directory(Directory { 
            name: "/".to_string(), 
            children: vec![], 
            parent: None,
        })));

        FileSystem { 
            current: Rc::clone(&root), 
            root, 
        }
    }

    pub fn from_disk(path: &str) -> Result<Self, FSError> {
        let root_path = std::path::Path::new(path);

        if !root_path.exists() {
            return Err(FSError::NotFound(path.to_string()));
        }
        if !root_path.is_dir() {
            return Err(FSError::NotADirectory(path.to_string()));
        }

        let root = Self::build_tree(root_path, None)?;
        Ok(FileSystem { 
            current: Rc::clone(&root),
            root,
        })
    }

    pub fn build_tree(path: &std::path::Path, parent: Option<FSWeakLink>) -> Result<Rc<RefCell<FSItem>>, FSError> {
        let name = path
            .file_name()
            .unwrap_or(std::ffi::OsStr::new("/"))
            .to_string_lossy()
            .to_string();

        let meta = std::fs::symlink_metadata(path).map_err( |_| FSError::PermissionDenied(name.clone()))?;

        if meta.file_type().is_symlink() {
            let target = std::fs::read_link(path)
                .map_err( |_| FSError::PermissionDenied(name.clone()))?
                .to_string_lossy()
                .to_string();

            return Ok(Rc::new(RefCell::new(FSItem::SymLink(Link { target, parent, }))));
        }

        if meta.is_file() {
            let content = std::fs::read(path).map_err(|_| FSError::PermissionDenied(name.clone()))?;

            return Ok(Rc::new(RefCell::new(FSItem::File(File { 
                name, 
                content, 
                parent, 
            }))));
        }

        let dir_node = Rc::new(RefCell::new(FSItem::Directory(Directory { 
            name: name.clone(), 
            children: vec![], 
            parent, 
        })));

        let weak = Some(Rc::downgrade(&dir_node));

        let entries = std::fs::read_dir(path).map_err(|_| FSError::PermissionDenied(name.clone()))?;
        for entry in entries {
            let entry = entry.map_err(|_| FSError::PermissionDenied(name.clone()))?;

            match Self::build_tree(&entry.path(), weak.clone()) {
                Ok(child) => {
                    if let FSItem::Directory(d) = &mut *dir_node.borrow_mut() {
                        d.children.push(child);
                    }
                },
                Err(e) => {
                    // entry non leggibile, warning ma si continua
                    eprint!("warning: {e}");
                }
            }
        }
        Ok(dir_node)
    }

    pub fn change_dir(&mut self, path: String) -> Result<Self, FSError> {
        
    }
}

fn main() {
    println!("Hello, world!");
}
