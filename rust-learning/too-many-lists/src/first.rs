use std::mem;

pub struct Stack {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    element: i32,
    next: Link,
}

impl Stack {
    pub fn new() -> Self {
        Stack {
            head: Link::Empty,
        }
    }

    pub fn push(&mut self, element: i32) {
        let new_node = Box::new(Node{
            element,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.element)
            }
        }
    }
}

impl Drop for Stack {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

#[cfg(test)]
mod test {
    use super::Stack;

    #[test]
    fn basics() {
        let mut stack = Stack::new();

        assert_eq!(stack.pop(), None);

        stack.push(1);
        stack.push(2);
        stack.push(3);

        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));

        stack.push(4);
        stack.push(5);

        assert_eq!(stack.pop(), Some(5));
        assert_eq!(stack.pop(), Some(4));

        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }
}