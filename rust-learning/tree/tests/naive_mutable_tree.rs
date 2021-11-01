use tree::naive_mutable_tree::Node;

#[test]
fn tree_test() {
    let root = Node::builder(0).build().wrap();

    let child10 = Node::builder(10).parent(root.clone()).build().wrap();

    let child11 = Node::builder(11).parent(child10.clone()).build().wrap();

    let child12 = Node::builder(12).parent(child10.clone()).build().wrap();

    let child20 = Node::builder(20).parent(root.clone()).build().wrap();

    let child21 = Node::builder(21).parent(child20.clone()).build().wrap();

    let child22 = Node::builder(22).parent(child20.clone()).build().wrap();

    {
        let mut root_handle = root.borrow_mut();
        root_handle.push_child(child10.clone());
        root_handle.push_child(child20.clone());

        let mut child10_handle = child10.borrow_mut();
        child10_handle.push_child(child11.clone());
        child10_handle.push_child(child12.clone());

        let mut child20_handle = child20.borrow_mut();
        child20_handle.push_child(child21.clone());
        child20_handle.push_child(child22.clone());
    }

    println!("{:#?}", root.borrow());
}
