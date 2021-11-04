use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::rc::Rc;

pub type TreeNode<T> = Rc<RefCell<Node<T>>>;

pub struct Node<T> {
    parent: Option<TreeNode<T>>,
    value: T,
    children: Vec<TreeNode<T>>,
}

pub struct NodeBuilder<T> {
    parent: Option<TreeNode<T>>,
    value: T,
    children: Vec<TreeNode<T>>,
}

impl<T> NodeBuilder<T> {
    fn new(value: T) -> Self {
        Self {
            parent: None,
            value,
            children: vec![],
        }
    }

    pub fn parent(mut self, parent: TreeNode<T>) -> Self {
        self.parent = Some(parent);
        self
    }

    pub fn child(mut self, child: TreeNode<T>) -> Self {
        self.children.push(child);
        self
    }

    pub fn children(mut self, children: Vec<TreeNode<T>>) -> Self {
        self.children = children;
        self
    }

    pub fn build(self) -> Node<T> {
        let NodeBuilder {
            parent,
            value,
            children,
        } = self;

        Node {
            parent,
            value,
            children,
        }
    }
}

impl<T> Node<T> {
    pub fn builder(value: T) -> NodeBuilder<T> {
        NodeBuilder::new(value)
    }

    pub fn wrap(self) -> TreeNode<T> {
        Rc::new(RefCell::new(self))
    }

    pub fn push_child(&mut self, child: TreeNode<T>) {
        self.children.push(child);
    }

    pub fn debug_children(&self) -> impl Debug + '_
        where
            T: Debug,
    {
        WithChildren(self)
    }

    pub fn debug_parent(&self) -> impl Debug + '_
        where
            T: Debug,
    {
        WithParent(self)
    }
}

impl<T> Debug for Node<T>
    where
        T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let children = &self
            .children
            .iter()
            .map(|child| WithChildren(child.borrow()))
            .collect::<Vec<_>>();
        let parent = &self
            .parent
            .as_ref()
            .map(|parent| WithParent(parent.borrow()));

        f.debug_struct("Node")
            .field("value", &self.value)
            .field("parent", parent)
            .field("children", children)
            .finish()
    }
}

struct WithParent<T, R>(R)
    where
        T: Debug,
        R: Deref<Target = Node<T>>;

impl<T, R> Debug for WithParent<T, R>
    where
        T: Debug,
        R: Deref<Target = Node<T>>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let node = self.0.deref();
        let parent = &node
            .parent
            .as_ref()
            .map(|parent| WithParent(parent.borrow()));

        f.debug_struct("Node")
            .field("value", &node.value)
            .field("parent", parent)
            .finish()
    }
}

struct WithChildren<T, R>(R)
    where
        T: Debug,
        R: Deref<Target = Node<T>>;

impl<T, R> Debug for WithChildren<T, R>
    where
        T: Debug,
        R: Deref<Target = Node<T>>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let node = self.0.deref();

        f.debug_struct("Node")
            .field("value", &node.value)
            .field(
                "children",
                &node
                    .children
                    .iter()
                    .map(|child| WithChildren(child.borrow()))
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}