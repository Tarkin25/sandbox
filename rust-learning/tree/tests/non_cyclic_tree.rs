use std::rc::Rc;
use tree::non_cyclic_tree::{DepthFirstIter, Node};

#[test]
fn append_links_nodes_correctly() {
    let root = Node::new(0).wrap();
    let n_1 = Node::new(1).wrap();
    let n_2 = Node::new(2).wrap();

    Node::append(&root, Rc::clone(&n_1));

    assert_eq!(Some(Rc::clone(&root)), n_1.borrow().parent());
    assert_eq!(Some(Rc::clone(&n_1)), root.borrow().first_child());
    assert_eq!(Some(Rc::clone(&n_1)), root.borrow().last_child());
    assert!(root.borrow().children().contains(&n_1));

    Node::append(&root, Rc::clone(&n_2));

    assert_eq!(Some(Rc::clone(&n_2)), root.borrow().last_child());
    assert_eq!(Some(Rc::clone(&n_2)), n_1.borrow().next_sibling());
}

#[test]
fn depth_first_iterator() {
    let root = Node::new(0).wrap();
    let n_1 = Node::new(1).wrap();
    let n_2 = Node::new(2).wrap();

    Node::append(&root, Rc::clone(&n_1));
    Node::append(&root, Rc::clone(&n_2));

    let it = DepthFirstIter::new(root);

    for node in it.take(3) {
        println!("{:#?}", node.borrow());
    }
}
