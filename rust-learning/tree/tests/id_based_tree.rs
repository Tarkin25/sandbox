use tree::id_based_tree::{Node, Tree};

#[test]
fn append_and_get_works_correctly() {
    let mut tree = Tree::new();

    let n_1 = tree.new_node("1");

    let n_1_1 = tree.new_node("1_1");

    n_1.append(n_1_1, &mut tree);

    println!("{:#?}", tree);
}