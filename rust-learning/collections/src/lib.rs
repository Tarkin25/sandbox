use std::ops::{Deref, DerefMut};

enum Node<T> {
    None,
    Tail { value: T },
    Link { value: T, next: Box<Node<T>>}
}

pub struct LinkedList<T> {
    head: Node<T>,
    length: i32,
}

impl <T: Copy> LinkedList<T> {

    pub fn new() -> Self {
        LinkedList {
            head: Node::None,
            length: 0
        }
    }

    pub fn len(&self) -> i32 {
        self.length
    }

    pub fn for_each(&self, callback: fn(&T)) {
        Self::for_each_rec(&self.head, callback);
    }

    fn for_each_rec(node: &Node<T>, callback: fn(&T)) {
        match node {
            Node::None => (),
            Node::Tail { value } => {
                callback(value);
            },
            Node::Link { value, next } => {
                callback(value);
                Self::for_each_rec(next.deref(), callback);
            }
        }
    }

    pub fn add(&mut self, value: T) -> &mut Self {
        match self.head {
            Node::None => {
                self.head = Node::Tail { value };
            },
            Node::Tail { value: tail_value } => {
                self.head = Node::Link {
                    value: tail_value,
                    next: Box::new(Node::Tail { value })
                }
            },
            Node::Link { value: _, ref mut next} => {
                Self::add_last(next.deref_mut(), value);
            }
        };

        self.length += 1;

        self
    }

    fn add_last(node: &mut Node<T>, value: T) {
        match node {
            Node::Tail { value: node_value } => {
                *node = Node::Link {
                    value: *node_value,
                    next: Box::new(Node::Tail { value })
                }
            }
            Node::Link { next, .. } => {
                Self::add_last(next.deref_mut(), value);
            },
            _ => println!("Cannot add to None")
        }
    }

}