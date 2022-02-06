type NodeRef = Option<NodeId>;

#[derive(Debug)]
struct Entry<T> {
    generation: usize,
    node: Option<Node<T>>,
}

impl<T> Entry<T> {
    fn new(generation: usize, value: T) -> Self {
        Self {
            generation,
            node: Some(Node::new(value))
        }
    }
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: NodeRef,
    prev: NodeRef,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            next: None,
            prev: None,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct NodeId {
    index: usize,
    generation: usize,
}

#[derive(Debug)]
pub struct LinkedVec<T> {
    entries: Vec<Entry<T>>,
    len: usize,
    head: NodeRef,
    tail: NodeRef,
}

impl<T> LinkedVec<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_back(&mut self, value: T) -> NodeId {
        let added = self.add_node(value);

        if let Some(tail) = self.tail {
            let tail_node = self.get_node_mut(tail).expect("Node of tail Entry to be present");
            tail_node.next = Some(added);

            self.get_node_mut(added).expect("Node of tail Entry to be present").prev = Some(tail);
        }

        self.tail = Some(added);

        if let None = self.head {
            self.head = self.tail;
        }

        added
    }

    fn add_node(&mut self, value: T) -> NodeId {
        let index = self.entries.len();
        let generation = 0;
        self.entries.push(Entry::new(generation, value));
        self.len = self.entries.len();
        NodeId { index, generation }
    }
    
    pub fn get(&self, index: NodeId) -> Option<&T> {
        self.get_node(index)
            .map(|node| &node.value)
    }

    pub fn get_mut(&mut self, index: NodeId) -> Option<&mut T> {
        self.get_node_mut(index)
            .map(|node| &mut node.value)
    }

    fn get_node(&self, index: NodeId) -> Option<&Node<T>> {
        self.entries.get(index.index)
            .filter(|entry| entry.generation == index.generation)
            .and_then(|entry| entry.node.as_ref())
    }

    fn get_node_mut(&mut self, index: NodeId) -> Option<&mut Node<T>> {
        self.entries.get_mut(index.index)
            .filter(|entry| entry.generation == index.generation)
            .and_then(|entry| entry.node.as_mut())
    }

    pub fn len(&self) -> usize {
        self.len
    }
}

impl<T> Default for LinkedVec<T> {
    fn default() -> Self {
        Self {
            entries: Vec::default(),
            len: 0,
            head: None,
            tail: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn push_back_works() {
        let mut v = LinkedVec::new();

        let first = v.push_back(5);
        dbg!(&v);
        assert_eq!(v.tail, Some(first));
        assert_eq!(v.head, Some(first));
        assert_eq!(v.get(first), Some(&5));
        assert_eq!(v.len, 1);

        let second = v.push_back(10);
        dbg!(&v);
        assert_eq!(v.tail, Some(second));
        assert_eq!(v.head, Some(first));
        assert_eq!(v.get(second), Some(&10));
        assert_eq!(v.len, 2);
    }
}
