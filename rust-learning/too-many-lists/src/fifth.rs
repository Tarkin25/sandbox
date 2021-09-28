use std::ptr;

pub struct Queue<T> {
    head: Link<T>,
    tail: *mut Node<T>, // DANGER DANGER
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    element: T,
    next: Link<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { head: None, tail: ptr::null_mut() }
    }

    pub fn push(&mut self, element: T) {
        let mut new_tail = Box::new(Node {
            element,
            next: None,
        });

        let raw_tail: *mut _ = &mut *new_tail;

        // .is_null checks for null, equivalent to checking for None
        if !self.tail.is_null() {
            // If the old tail existed, update it to point to the new tail
            unsafe {
                (*self.tail).next = Some(new_tail);
            }
        } else {
            // Otherwise, update the head to point to it
            self.head = Some(new_tail);
        }

        self.tail = raw_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let element = head.element;
            self.head = head.next;

            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }

            element
        })
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref()
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut()
        }
    }
}

impl<T> IntoIterator for Queue<T> {
    type Item = <IntoIter<T> as Iterator>::Item;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

pub struct IntoIter<T>(Queue<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.element
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.element
        })
    }
}

#[cfg(test)]
mod test {
    use super::Queue;

    #[test]
    fn basics() {
        let mut queue = Queue::new();

        // Check empty queue behaves right
        assert_eq!(queue.pop(), None);

        // Populate queue
        queue.push(1);
        queue.push(2);
        queue.push(3);

        // Check normal removal
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        queue.push(4);
        queue.push(5);

        // Check normal removal
        assert_eq!(queue.pop(), Some(3));
        assert_eq!(queue.pop(), Some(4));

        // Check exhaustion
        assert_eq!(queue.pop(), Some(5));
        assert_eq!(queue.pop(), None);

        // Check the exhaustion case fixed the pointer right
        queue.push(6);
        queue.push(7);

        // Check normal removal
        assert_eq!(queue.pop(), Some(6));
        assert_eq!(queue.pop(), Some(7));
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn into_iter() {
        let mut queue = Queue::new();
        queue.push(1); queue.push(2); queue.push(3);

        let mut iter = queue.into_iter();
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut queue = Queue::new();
        queue.push(1); queue.push(2); queue.push(3);

        let mut iter = queue.iter();
        assert_eq!(iter.next(), Some(&1));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut queue = Queue::new();
        queue.push(1); queue.push(2); queue.push(3);

        let mut iter = queue.iter_mut();
        assert_eq!(iter.next(), Some(&mut 1));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), None);
    }
}
