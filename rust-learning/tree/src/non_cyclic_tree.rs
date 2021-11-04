use std::cell::RefCell;
use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::vec::IntoIter;

type TreeCell<T> = RefCell<Node<T>>;
pub type TreeNode<T> = Rc<TreeCell<T>>;
type WeakNode<T> = Weak<TreeCell<T>>;

pub trait NodeTrait {
    fn append(&self, child: Self);
}

impl<T> NodeTrait for Rc<RefCell<Node<T>>> {
    fn append(&self, child: Self) {
        child.borrow_mut().parent = Rc::downgrade(&self);

        let mut this = self.borrow_mut();

        if let Some(first_child) = this.first_child() {
            first_child.borrow_mut().next_sibling = Rc::downgrade(&child);
        } else {
            this.first_child = Rc::downgrade(&child);
        }

        if this.first_child().is_none() {
            this.first_child = Rc::downgrade(&child);
        }

        this.last_child = Rc::downgrade(&child);

        this.children.push(child);
    }
}

#[derive(Debug)]
pub struct Node<T> {
    data: T,
    children: Vec<TreeNode<T>>,
    parent: WeakNode<T>,
    first_child: WeakNode<T>,
    last_child: WeakNode<T>,
    next_sibling: WeakNode<T>,
}

impl<T> Node<T> {

    pub fn new(data: T) -> Self {
        Self {
            data,
            children: vec![],
            parent: Weak::new(),
            first_child: Weak::new(),
            last_child: Weak::new(),
            next_sibling: Weak::new(),
        }
    }

    pub fn wrap(self) -> TreeNode<T> {
        Rc::new(RefCell::new(self))
    }

    pub fn parent(&self) -> Option<TreeNode<T>> {
        self.parent.upgrade()
    }

    pub fn first_child(&self) -> Option<TreeNode<T>> {
        self.first_child.upgrade()
    }

    pub fn last_child(&self) -> Option<TreeNode<T>> {
        self.last_child.upgrade()
    }

    pub fn next_sibling(&self) -> Option<TreeNode<T>> {
        self.next_sibling.upgrade()
    }

    pub fn children(&self) -> &Vec<TreeNode<T>> {
        &self.children
    }

    pub fn strong_debug(&self) -> impl Debug + '_ where T: Debug {
        StrongDebug(self)
    }

}

impl<T> PartialEq for Node<T> where T: PartialEq {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }
}

struct StrongDebug<T, R>(R) where R: Deref<Target=Node<T>>, T: Debug;

impl<T, R> Debug for StrongDebug<T, R> where R: Deref<Target=Node<T>>, T: Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let node = self.0.deref();

        f.debug_struct("Node")
            .field("parent", &node.parent())
            .field("first_child", &node.first_child())
            .field("last_child", &node.last_child())
            .field("next_sibling", &node.next_sibling())
            .finish()
    }
}

pub struct DepthFirstIter<T> {
    iter: IntoIter<TreeNode<T>>,
}

impl<T> DepthFirstIter<T> {
    pub fn new(node: &TreeNode<T>) -> Self {
        let mut stack = vec![];
        Self::collect_children(node, &mut stack);
        let iter = stack.into_iter();

        Self {
            iter,
        }
    }

    fn collect_children(node: &TreeNode<T>, stack: &mut Vec<TreeNode<T>>) {
        for child in &node.borrow().children {
            Self::collect_children(child, stack);
        }

        stack.push(Rc::clone(node));
    }
}

impl<T> Iterator for DepthFirstIter<T> {
    type Item = TreeNode<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
