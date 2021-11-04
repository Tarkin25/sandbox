use std::fmt::{Debug, Formatter};

#[derive(Debug, Default)]
pub struct Tree<T> {
    nodes: Vec<Node<T>>,
    root: Option<NodeId>,
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            root: None,
        }
    }

    pub fn new_node(&mut self, value: T) -> NodeId {
        let id = self.nodes.len();

        let node = Node {
            id: NodeId(id),
            value,
            children: vec![],
            parent: None,
        };

        self.nodes.insert(id, node);

        NodeId(id)
    }

    fn append_child(&mut self, node: NodeId, child: NodeId) {
        self.nodes[node.0].children.push(child);
        self.nodes[child.0].parent = Some(node);
    }

    pub fn get(&self, id: usize) -> Option<&Node<T>> {
        self.nodes.get(id)
    }
}

#[derive(Debug)]
pub struct Node<T> {
    id: NodeId,
    value: T,
    children: Vec<NodeId>,
    parent: Option<NodeId>,
}

#[derive(Clone, Copy)]
pub struct NodeId(usize);

impl NodeId {
    pub fn append<T>(self, child: NodeId, tree: &mut Tree<T>) {
        tree.append_child(self, child);
    }
}

impl Debug for NodeId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
