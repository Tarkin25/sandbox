use std::rc::Rc;
use std::cell::RefCell;
use std::ops::Deref;
use std::fmt::{Debug, Formatter};
use std::borrow::Borrow;
use std::marker::PhantomData;

struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Rc<RefCell<Node<T>>>>,
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    length: usize,
}

pub struct Iter<'a, T: 'a> {
    current: Option<Rc<RefCell<Node<T>>>>,
    phantom_data: PhantomData<&'a T>,
}

impl<T: Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node {{ value: {:?}, next: {:?}}}", self.value, self.next)
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList {
            head: None,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn add(&mut self, value: T) -> &mut Self where T: Debug {
        if let Some(ref head) = self.head {
            let mut temp = head.clone();

            while let Some(next) = temp.clone().deref().borrow().next.as_ref() {
                temp = next.clone();
            }

            let next = Node { prev: Some(temp.clone()), value, next: None};

            temp.borrow_mut().next = Some(Rc::new(RefCell::new(next)));
        } else {
            self.head = Some(Rc::new(RefCell::new(Node { value, prev: None, next: None})));
        }

        self.length += 1;

        self
    }
}

impl<'a, T: 'a> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        if let Some(ref current) = self.current {
            let value = Some(&current.deref().borrow().value);

            self.current = current.clone().into_inner().next;

            value
        } else {
            None
        }
    }
}
