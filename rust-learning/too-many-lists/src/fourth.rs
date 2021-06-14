use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};

pub struct Deque<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    element: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(element: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            element,
            next: None,
            prev: None,
        }))
    }
}

impl<T> Deque<T> {
    pub fn new() -> Self {
        Deque {
            head: None,
            tail: None,
        }
    }

    pub fn push_front(&mut self, element: T) {
        // new node needs +2 links, everything else should be +0
        let new_head = Node::new(element);

        match self.head.take() {
            Some(old_head) => {
                // non-empty deque, need to connect to the old_head
                old_head.borrow_mut().prev = Some(new_head.clone()); // +1 new_head
                new_head.borrow_mut().next = Some(old_head);         // +1 old_head
                self.head = Some(new_head);             // +1 new_head, -1 old_head
                // total: +2 new_head, +0 old_head -- OK!
            },
            None => {
                // empty deque, need to set the tail
                self.tail = Some(new_head.clone());     // +1 new_head
                self.head = Some(new_head);             // +1 new_head
                // total: +2 new_head -- OK!
            }
        }
    }

    pub fn push_back(&mut self, element: T) {
        let new_tail = Node::new(element);

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
            },
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {   // -1 old
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {         // -1 new
                    new_head.borrow_mut().prev.take();          // -1 old
                    self.head = Some(new_head);
                },
                None => {
                    self.tail.take();
                }
            }

            Rc::try_unwrap(old_head).ok().unwrap().into_inner().element
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                },
                None => {
                    self.head.take();
                }
            }

            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().element
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        Self::peek(&self.head)
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        Self::peek_mut(&mut self.head)
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        Self::peek(&self.tail)
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        Self::peek_mut(&mut self.tail)
    }

    fn peek(end: &Link<T>) -> Option<Ref<T>> {
        end.as_ref().map(|node| {
            Ref::map(node.borrow(), |node| &node.element)
        })
    }

    fn peek_mut(end: &mut Link<T>) -> Option<RefMut<T>> {
        end.as_ref().map(|node| {
            RefMut::map(node.borrow_mut(), |node| &mut node.element)
        })
    }
}

impl<T> Drop for Deque<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T> IntoIterator for Deque<T> {
    type Item = <IntoIter<T> as Iterator>::Item;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

pub struct IntoIter<T>(Deque<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

#[cfg(test)]
mod test {
    use super::Deque;

    #[test]
    fn basics() {
        let mut deque = Deque::new();

        // Check empty deque behaves right
        assert_eq!(deque.pop_front(), None);

        // Populate deque
        deque.push_front(1);
        deque.push_front(2);
        deque.push_front(3);

        // Check normal removal
        assert_eq!(deque.pop_front(), Some(3));
        assert_eq!(deque.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        deque.push_front(4);
        deque.push_front(5);

        // Check normal removal
        assert_eq!(deque.pop_front(), Some(5));
        assert_eq!(deque.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), None);

        // ---- back -----

        // Check empty deque behaves right
        assert_eq!(deque.pop_back(), None);

        // Populate deque
        deque.push_back(1);
        deque.push_back(2);
        deque.push_back(3);

        // Check normal removal
        assert_eq!(deque.pop_back(), Some(3));
        assert_eq!(deque.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        deque.push_back(4);
        deque.push_back(5);

        // Check normal removal
        assert_eq!(deque.pop_back(), Some(5));
        assert_eq!(deque.pop_back(), Some(4));

        // Check exhaustion
        assert_eq!(deque.pop_back(), Some(1));
        assert_eq!(deque.pop_back(), None);
    }

    #[test]
    fn peek() {
        let mut deque = Deque::new();
        assert!(deque.peek_front().is_none());
        assert!(deque.peek_back().is_none());
        assert!(deque.peek_front_mut().is_none());
        assert!(deque.peek_back_mut().is_none());

        deque.push_front(1); deque.push_front(2); deque.push_front(3);

        assert_eq!(&*deque.peek_front().unwrap(), &3);
        assert_eq!(&mut *deque.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*deque.peek_back().unwrap(), &1);
        assert_eq!(&mut *deque.peek_back_mut().unwrap(), &mut 1);
    }

    #[test]
    fn into_iter() {
        let mut deque = Deque::new();
        deque.push_front(1);
        deque.push_front(2);
        deque.push_front(3);

        let mut iter = deque.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }

}
