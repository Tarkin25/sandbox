use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node<T> {
    element: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
        Self {
            element,
            next: None,
            prev: None,
        }
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Weak<RefCell<Node<T>>>>,
    len: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn push_front(&mut self, element: T) {
        let new_head = Rc::new(RefCell::new(Node::new(element)));

        match &self.head {
            Some(head) => {
                new_head.borrow_mut().next = Some(Rc::clone(head));
                head.borrow_mut().prev = Some(Rc::downgrade(&new_head));
                self.head = Some(new_head);
            },
            None => {
                self.tail = Some(Rc::downgrade(&new_head));
                self.head = Some(new_head);
            }
        }

        self.len += 1;
    }

    pub fn push_back(&mut self, element: T) {
        let new_tail = Rc::new(RefCell::new(Node::new(element)));

        match &self.tail.as_ref().and_then(|tail| tail.upgrade()) {
            Some(tail) => {
                new_tail.borrow_mut().prev = Some(Rc::downgrade(tail));
                self.tail = Some(Rc::downgrade(&new_tail));
                tail.borrow_mut().next = Some(new_tail);
            },
            None => {
                self.tail = Some(Rc::downgrade(&new_tail));
                self.head = Some(new_tail);
            }
        }

        self.len += 1;
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            current_head: self.head,
            current_tail: self.tail,
            len: self.len,
        }
    }
}

#[derive(Debug)]
pub struct IntoIter<T> {
    current_head: Option<Rc<RefCell<Node<T>>>>,
    current_tail: Option<Weak<RefCell<Node<T>>>>,
    len: usize,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current) = self.current_head.take() {
            let current = Rc::try_unwrap(current).ok().unwrap().into_inner();
            self.current_head = current.next;
            self.len -= 1;
            Some(current.element)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_front_works() {
        let mut list = LinkedList::new();

        list.push_front(1);
        assert_eq!(list.head.as_ref().unwrap().borrow().element, 1);
        assert_eq!(list.tail.as_ref().unwrap().upgrade().unwrap().borrow().element, 1);
        assert_eq!(list.len, 1);

        list.push_front(0);
        assert_eq!(list.head.as_ref().unwrap().borrow().element, 0);
        assert_eq!(list.tail.as_ref().unwrap().upgrade().unwrap().borrow().element, 1);
        assert_eq!(list.len, 2);
    }

    #[test]
    fn push_back_works() {
        let mut list = LinkedList::new();

        list.push_back(1);
        assert_eq!(list.head.as_ref().unwrap().borrow().element, 1);
        assert_eq!(list.tail.as_ref().unwrap().upgrade().unwrap().borrow().element, 1);
        assert_eq!(list.len, 1);

        list.push_back(2);
        assert_eq!(list.head.as_ref().unwrap().borrow().element, 1);
        assert_eq!(list.tail.as_ref().unwrap().upgrade().unwrap().borrow().element, 2);
        assert_eq!(list.len, 2);
    }

    #[test]
    fn into_iter_works() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);

        let mut it = list.into_iter();
        assert_eq!(it.size_hint(), (2, Some(2)));
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.size_hint(), (1, Some(1)));
        assert_eq!(it.next(), Some(2));
        assert_eq!(it.size_hint(), (0, Some(0)));
        assert_eq!(it.next(), None);
    }
}