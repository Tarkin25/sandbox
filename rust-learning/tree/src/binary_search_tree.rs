use core::cmp::Ordering::*;

#[derive(Debug, Default)]
pub struct BinarySearchTree<K: Ord, V> {
    root: Option<Node<K, V>>,
}

impl<K, V> BinarySearchTree<K, V> where K: Ord {
    pub fn new() -> Self {
        Self {
            root: None,
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> &mut Self {
        if let Some(ref mut root) = self.root {
            root.insert(key, value);
        } else {
            self.root = Some(Node::new(key, value));
        }

        self
    }

    pub fn get(&self, key: K) -> Option<&V> {
        self.root.as_ref().and_then(|root| root.get(key))
    }

    pub fn collect(self) -> Vec<(K, V)> {
        let mut vec = vec![];

        if let Some(root) = self.root {
            root.collect(&mut vec);
        }

        vec
    }

    pub fn len(&self) -> usize {
        if let Some(ref root) = self.root {
            root.len()
        } else {
            0
        }
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}

#[derive(Debug)]
struct Node<K: Ord, V> {
    key: K,
    value: V,
    left: Option<Box<Node<K, V>>>,
    right: Option<Box<Node<K, V>>>,
}

impl<K, V> Node<K, V> where K: Ord {
    pub fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, key: K, value: V) {
        match key.cmp(&self.key) {
            Equal => {
                self.value = value;
            },
            Less => {
                Self::insert_in_child(&mut self.left, key, value);
            },
            Greater => {
                Self::insert_in_child(&mut self.right, key, value);
            }
        }
    }

    fn insert_in_child(child: &mut Option<Box<Node<K, V>>>, key: K, value: V) {
        if let Some(child) = child {
            child.insert(key, value);
        } else {
            *child = Some(Box::new(Self::new(key, value)));
        }
    }

    pub fn get(&self, key: K) -> Option<&V> {
        match key.cmp(&self.key) {
            Equal => {
                Some(&self.value)
            },
            Less => {
                Self::get_from_child(&self.left, key)
            },
            Greater => {
                Self::get_from_child(&self.right, key)
            }
        }
    }

    fn get_from_child(child: &Option<Box<Node<K, V>>>, key: K) -> Option<&V> {
        child.as_ref().and_then(|child| child.get(key))
    }

    fn collect(self, vec: &mut Vec<(K, V)>) {
        if let Some(left) = self.left {
            left.collect(vec);
        }

        vec.push((self.key, self.value));

        if let Some(right) = self.right {
            right.collect(vec);
        }
    }

    pub fn len(&self) -> usize {
        let mut len = 1_usize;

        if let Some(ref left) = self.left {
            len += left.len();
        }

        if let Some(ref right) = self.right {
            len += right.len();
        }

        len
    }

}
