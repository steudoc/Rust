use  std::{collections::HashMap, fmt::format};

#[derive(Debug, PartialEq)]
enum AlberoError {
    NodeNotFound(String),
    NodeAlreadyExists(String),
    CannotRemoveRoot,
}

impl std::fmt::Display for AlberoError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AlberoError::NodeNotFound(n) => write!(f, "Node {} does not exists!", n),
            AlberoError::NodeAlreadyExists(n) => write!(f, "Node {} already exists!", n),
            AlberoError::CannotRemoveRoot => write!(f, "Cannot remove root!"),
        }
    }
}

struct Albero {
    parents: HashMap<String, String>,
    switches: HashMap<String, bool>,
}

impl Albero {
    pub fn new(root: &str) -> Self {
        let mut parents =  HashMap::new();
        let mut switches = HashMap::new();

        switches.insert(root.to_string(), true);

        Albero { parents, switches }
    }

    // aggiungere un nodo figlio del nodo father
    pub fn add(&mut self, father: &str, node: &str) -> Result<(),AlberoError> {
        if !self.switches.contains_key(father) {
            return Err(AlberoError::NodeNotFound(father.to_string()));
        } else if self.switches.contains_key(node) {
            return Err(AlberoError::NodeAlreadyExists(node.to_string()));
        }

        self.parents.insert(node.to_string(), father.to_string());
        self.switches.insert(node.to_string(), false);

        Ok(())
    }

    // rimuove un nodo e tutti i suoi discendenti
    pub fn remove(&mut self, node: &str) -> Result<(),AlberoError> {
        if !self.parents.contains_key(node) {
            if self.switches.contains_key(node) {
                return Err(AlberoError::CannotRemoveRoot);
            }
            return Err(AlberoError::NodeNotFound(node.to_string()));
        }

        let mut to_remove = vec![node.to_string()];
        let mut i = 0;

        // BFS per ogni noto to_remove, troviamo i suoi figli
        while i < to_remove.len() {
            let current = to_remove[i].clone();
            // cerchiamo tutti i nodi che hanno current come padre
            for (child, parent) in &self.parents {
                if parent == &current {
                    to_remove.push(child.clone());
                }
            }
            i += 1;
        }

        for n in to_remove {
            self.parents.remove(&n);
            self.switches.remove(&n);
        }

        Ok(())
    }

    // commuta l'interruttore del nodo e restituisce il nuovo valore
    pub fn toggle(&mut self, node: &str) -> Result<bool,AlberoError> {
        
        match self.switches.get_mut(node) {
            Some(switch) => {
                *switch = !*switch;
                Ok(*switch)
            },
            None => Err(AlberoError::NodeNotFound(node.to_string()))
        }
    }

    // restituisce se la luce del nodo è accesa
    pub fn peek(&self, node: &str) -> Result<bool,AlberoError> {
        if !self.switches.contains_key(node) {
            return Err(AlberoError::NodeNotFound(node.to_string()));
        }

        let mut current = node.to_string();
        loop {
            match self.switches.get(&current) {
                None => return Err(AlberoError::NodeNotFound(current)),
                Some(false) => return Ok(false),
                Some(true) => {
                    // interruttore on - risaliamo al padre
                    match self.parents.get(&current) {
                        None => return Ok(true),
                        Some(parent) => current = parent.clone()
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_ok() {
        let mut albero = Albero::new("ROOT");
        assert!(albero.add("ROOT", "A").is_ok());
        assert!(albero.add("ROOT", "B").is_ok());
        assert!(albero.add("A", "C").is_ok());
    }

    #[test]
    fn test_add_father_not_found() {
        let mut albero = Albero::new("ROOT");
        let result = albero.add("NONEXISTENT", "A");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AlberoError::NodeNotFound("NONEXISTENT".to_string()));
    }

    #[test]
    fn test_add_node_already_exists() {
        let mut albero = Albero::new("ROOT");
        let mut result = albero.add("ROOT", "A");
        result = albero.add("ROOT", "A");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AlberoError::NodeAlreadyExists("A".to_string()));
    }

    #[test]
    fn test_add_remove() {
        let mut albero = Albero::new("ROOT");
        assert!(albero.add("ROOT", "A").is_ok());
        assert!(albero.add("ROOT", "B").is_ok());
        assert!(albero.add("A", "C").is_ok());
        assert!(albero.add("A", "C").is_err());
        assert!(albero.remove("C").is_ok());
        assert!(albero.add("A", "C").is_ok());
    }

    #[test]
    fn test_toggle_peek() {
        let mut albero = Albero::new("ROOT");
        assert!(albero.add("ROOT", "A").is_ok());
        assert!(albero.toggle("A").is_ok());
        assert!(albero.add("A", "B").is_ok());
        assert!(albero.toggle("B").is_ok());
        assert!(albero.add("B", "C").is_ok());
        assert!(albero.toggle("C").is_ok());
        assert_eq!(albero.peek("C").unwrap(), true);
        let _ = albero.toggle("A");
        assert_eq!(albero.peek("C").unwrap(), false);
    }
}

fn main() {
    let mut tree = Albero::new("root");
    let nodes = [
        ("root", "A"),
        ("A", "B"),
        ("A", "C"),
        ("root", "A"),
        ("root", "D"),
        ("D", "E"),
        ("banana", "republic"),
        ("E", "F"),
    ];

    for (father, node) in nodes {
        match tree.add(father, node) {
            Ok(()) => println!("Node {} successfully added", node),
            Err(err) => println!("{}", err)
        }
    }

}
