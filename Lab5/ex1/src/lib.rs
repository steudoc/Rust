#![allow(warnings)]

pub mod mem_inspect {
    use crate::{List1, List2};


    // dump object info:
    // size, address, bytes
    pub fn dump_object<T>(obj: &T) {
        let ptr = obj as *const T as *const u8;
        let _size = size_of::<T>();
        let _ptr = ptr as usize;
        println!("Object size: {_size}; address: {_ptr:x}");

        dump_memory(ptr, _size);
    }

    // dump memory info
    pub fn dump_memory(start: *const u8, size: usize) {
        let bytes = unsafe { std::slice::from_raw_parts(start, size) };

        println!("Bytes:");
        for (i, byte) in bytes.iter().enumerate() {
            print!("{:02x} ", byte);
            if i % 8 == 7 {
                println!();
            }
        }
        println!()
    }

    #[test]
    fn dump_object_example() {
        let s = "hello".to_string();
        dump_object(&s);

        let b = Box::new(s);
        // before running try to answer:
        // 1. what is the size of b?
        // 2. what is the content of b?
        dump_object(&b);

        // how to the the pointer of the wrapped object?
        let ptr = b.as_ref() as *const String as *const u8;
        println!("Pointer: {ptr:?}");

        assert!(true);
    }
}


pub mod List1 {
    use std::mem;


    pub enum Node<T> {
        Cons(T, Box<Node<T>>),
        Nil,
    }

    pub struct List<T> {
        head: Node<T>,
    }

    impl<T> List<T> {
        pub fn new() -> Self {
            Self { head: Node::Nil }
        }

        // insert a new element at the beginning of the list
        // you may encouter a problem with the borrow checker while trying to move self.head to a new variable
        // problem:
        // 1. you need to build a new list Node (elem: elem, self.head)
        // 2. but you can't move self.head, because self.head would be undefined
        // 3. you can't copy it either, because Box can't be copied
        // solution: use mem::replace to move the value of self.head into a new variable and replace it with Nil
        // 4. let self.head point to the new created node
        pub fn push(&mut self, elem: T) {
            let old_head = mem::replace(&mut self.head, Node::Nil);
            self.head = Node::Cons(elem, Box::new(old_head));
        }

        // pop the first element of the list and return it
        fn pop(&mut self) -> Option<T> {
            match mem::replace(&mut self.head, Node::Nil) {
                Node::Nil => None,
                Node::Cons(elem, next) => {
                    self.head = *next;
                    Some(elem)
                }
            }
        }

        // return a referece to the first element of the list
        pub fn peek(&self) -> Option<&T> {
            match &self.head {
                Node::Nil => None,
                Node::Cons(elem, _) => Some(elem),
            }
        }

        // uncomment after having implemented the ListIter struct
        // return an interator over the list values
        fn iter(&self) -> ListIter<T> {
            ListIter { next: &self.head, }
        }

        // take the first n elements of the list and return a new list with them
        pub fn take(&mut self, n: usize) -> List<T> {
            let mut buf = Vec::new();

            for _ in 0..n {
                match self.pop() {
                    Some(elem) => buf.push(elem),
                    None => break,
                }
            }

            let mut new_list = List::new();
            for elem in buf.into_iter().rev() {
                new_list.push(elem);
            }

            new_list
        }
    }

    pub struct ListIter<'a, T> {
        // implement the iterator trait for ListIter
        next: &'a Node<T>,
    }
    
    impl<'a, T> Iterator for ListIter<'a, T> {
        type Item = &'a T;
    
        fn next(&mut self) -> Option<Self::Item> {
            match self.next {
                Node::Nil => None,
                Node::Cons(elem, next_node) => {
                    self.next = next_node;
                    Some(elem)
                }
            }
        }
    }

    // something that may be useful for the iterator implementation:
    // let a = Some(T);
    // let b = &a;
    // match b { Some(i) => ... } // here i is a reference to T
}

pub mod List2 {

    pub struct Node<T> {
        elem: T,
        next: NodeLink<T>,
    }

    type NodeLink<T> = Option<Box<Node<T>>>;

    pub struct List<T> {
        head: NodeLink<T>,
    }

    // for this implementattion, since we are using option, take a look at the take method in Option<T>.
    // It allows to move the value of the option into another option and replace it with None
    // let mut a = Some(5);
    // let b = a.take(); // a is now None and b is Some(5)
    impl<T> List<T> {
        // same methods as List1
        pub fn new() -> Self {
            Self { head: NodeLink::None }
        }

        // insert a new element at the beginning of the list
        pub fn push(&mut self, elem: T) {
            let old_head = self.head.take();
            self.head = Some(Box::new(Node { elem, next: old_head }));
        }

        // pop the first element of the list and return it
        fn pop(&mut self) -> Option<T> {
            match self.head.take() {
                None => None,
                Some(node) => {
                    self.head = Some(node.next?);
                    Some(node.elem)
                }
            }

            /* self.head.take().map(|node| {
                self.head = node.next;
                node.elem
            }) */
        }

        // return a referece to the first element of the list
        pub fn peek(&self) -> Option<&T> {
            match &self.head {
                None => None,
                Some(node) => {
                    Some(&node.elem)
                }
            }
        }

        // uncomment after having implemented the ListIter struct
        // return an interator over the list values
        //fn iter(&self) -> ListIter<T> {
        //    unimplemented!()
        //}
 
        // take the first n elements of the list and return a new list with them
        pub fn take(&mut self, n: usize) -> List<T> {
            let mut buf = Vec::new();

            for _ in 0..n {
                match self.pop() {
                    Some(elem) => buf.push(elem),
                    None => break,
                }
            }

            let mut new_list = List::new();
            for elem in buf.into_iter().rev() {
                new_list.push(elem);
            }

            new_list
        }
    }
}

pub mod dlist {
// *****
// double linked list suggestions:
// the node has both a next and a prev link

    use std::{cell::RefCell, rc::{Rc, Weak}};

use crate::dlist;

    type NodeLink<T> = Option<Rc<RefCell<DNode<T>>>>;
    type NodeBackLink<T> = Option<Weak<RefCell<DNode<T>>>>;
    struct DNode<T> {
        elem: T,
        prev: NodeBackLink<T>,  // which type do we use here?
        next: NodeLink<T>, // which type do we use here?
    }

    struct DList<T> {
        head: NodeLink<T>,
        tail: NodeLink<T>,
    }

    // use Rc, since we need more than one reference to the same node. 
    // You need to both strong and weak references

    // For mutating the list and changing the next and prev fields we also need to be able to mutate the node, 
    // therefore we can use RefCell too (as for the tree at lesson)

    // how to access content of Rc<RefCell<T>>:
    // es let a = Rc::new(RefCell::new(5));
    // let mut x = (*a).borrow_mut();  // with (*a) we dereference the Rc, with (*a).borrow_mut() we get a mutable reference to the content of the RefCell
    // *x = 6; // we can now change the content of the RefCell

    // hint for pop: you can return either a reference to the value or take the value out of the Rc, 
    // but usually it is not possible to take out the value from an Rc since it may be referenced elsewhere.
    // if you can guarantee it's the only reference to the value  you can use Rc::try_unwrap(a).unwrap().into_inner() to get the value
    // it first takes out the value from the Rc, then it tries to unwrap the value from the Result, and finally it takes the inner value from the Result
    // see here
    // https://stackoverflow.com/questions/70404603/how-to-return-the-contents-of-an-rc
    // otherwise you can impose the COPY trait on T 

    // other hint that may be useful: Option<T> has a default clone implementation which calls the clone of T. Therefore:
    // Some(T).clone() ->  Some(T.clone())
    // None.clone() -> None

    impl<T> DList<T> {
        pub fn new() -> Self {
            Self { head: None, tail: None }
        }

        pub fn push_front(&mut self, elem: T) {
            let new_node = Rc::new(RefCell::new(DNode {
                elem,
                prev: None,
                next: None,
            }));

            match self.head.take() {
                None => {
                    // empty list
                    self.tail = Some(Rc::clone(&new_node));
                    self.head = Some(new_node);
                },
                Some(old_head) => {
                    new_node.borrow_mut().next = Some(Rc::clone(&old_head));
                    old_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                    self.head = Some(new_node);
                }
            }
        }

        pub fn pop_front(&mut self) -> Option<T> {
            self.head.take().map(|old_head| {
                match old_head.borrow_mut().next.take() {
                    None => {
                        // era l'unico nodo, anche tail va a None
                        self.tail = None;
                    },
                    Some(next_node) => {
                        // stacca il prev dal successore (non punta più ad old_head)
                        next_node.borrow_mut().prev = None;
                        self.head = Some(next_node);
                    }
                }
                // ora old_head ha rc=1 (nessuno lo referenzia più)
                Rc::try_unwrap(old_head)
                    .ok()   // Result -> Option
                    .unwrap()   // siamo sicuri che rc = 1
                    .into_inner()   // RefCell<DNode<T>> -> DNode<T>
                    .elem
            })
        }

        pub fn push_back(&mut self, elem: T) {
            let new_node = Rc::new(RefCell::new(DNode {
                elem,
                prev: None,
                next: None,
            }));

            match self.tail.take() {
                None => {
                    // lista vuota: identico a push_front
                    self.head = Some(Rc::clone(&new_node));
                    self.tail = Some(new_node);
                },
                Some(old_tail) => {
                    // collega old_tail  a new_node
                    old_tail.borrow_mut().next = Some(Rc::clone(&new_node));
                    // collega new_node a old_tail (weak)
                    new_node.borrow_mut().prev = Some(Rc::downgrade(&old_tail));
                    // aggiorna tail
                    self.tail = Some(new_node);
                }
            }
        }

        pub fn pop_back(&mut self) -> Option<T> {
            self.tail.take().map(|old_tail| {
                match old_tail.borrow_mut().prev.take() {
                    None => {
                        // era l'unico nodo
                        self.head = None;
                    },
                    Some(prev_weak) => {
                        // upgrade: weak -> Rc
                        let prev_node = prev_weak.upgrade().expect("prev node dropped unexpectedly");
                        // stacca il next del processore
                        prev_node.borrow_mut().next = None;
                        // il predecessore diventa la nuova tail
                        self.tail = Some(prev_node);
                    }
                }
                // old_tail ora ha rc = 1
                Rc::try_unwrap(old_tail)
                    .ok()
                    .unwrap()
                    .into_inner()
                    .elem
            })
        }

        pub fn popn(&mut self, n: usize) -> Option<T> {
            let mut current = Rc::clone(self.head.as_ref()?);
            for _ in 0..n {
                let next = current.borrow().next.as_ref()?.clone();
                current = next;
            }

            let prev_weak = current.borrow_mut().prev.take();
            let next_node = current.borrow_mut().next.take();

            match (&prev_weak, &next_node) {
                (None, None) => {
                    // unico nodo
                    self.head = None;
                    self.tail = None;
                },
                (None, Some(_)) => {
                    // è la testa
                    let next = next_node.as_ref().unwrap();
                    next.borrow_mut().prev = None;
                    self.head = next_node.clone();
                },
                (Some(_), None) => {
                    // è la coda
                    let prev = prev_weak.as_ref().unwrap().upgrade().unwrap();
                    prev.borrow_mut().next = None;
                    self.tail = Some(prev);
                },
                (Some(pw), Some(nx)) => {
                    // nodo intermedio: ricollega prev a next
                    let prev = pw.upgrade().unwrap();
                    prev.borrow_mut().next = Some(Rc::clone(nx));
                    nx.borrow_mut().prev = Some(Rc::downgrade(&prev));
                }
            }

            Rc::try_unwrap(current)
                .ok()
                .unwrap()
                .into_inner()
                .elem
                .into() // Option<T>
        }
    }

}