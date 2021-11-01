use tree::binary_search_tree::BinarySearchTree;

#[test]
fn sorted_binary_tree() {
    let mut tree = BinarySearchTree::new();

    tree
        .insert(1, 1)
        .insert(3, 3)
        .insert(2, 2);

    println!("{:#?}", &tree);

    let vec = tree.collect();

    println!("Collected: {:?}", vec);

    assert!(is_sorted(vec.iter().map(|(key, _)| key)));
}

#[test]
fn insert_and_get_returns_correct_value() {
    let mut tree = BinarySearchTree::new();

    assert_eq!(None, tree.get(1));

    tree.insert(1, 1);

    assert_eq!(Some(&1), tree.get(1));

    tree.insert(1, -1);

    assert_eq!(Some(&-1), tree.get(1));
}

#[test]
fn insert_equal_values() {
    let mut tree = BinarySearchTree::new();

    tree.insert(2, 2)
        .insert(3, 3)
        .insert(3, 3)
        .insert(3, 3)
        .insert(3, 3)
        .insert(-2, -2);

    let collected = tree.collect();

    println!("Collected: {:?}", collected);

    assert!(is_sorted(collected.iter().map(|(key, _)| key)));
}

#[test]
fn len_returns_correct_length() {
    let mut tree = BinarySearchTree::new();

    for i in 0..10 {
        tree.insert(i, i);
    }

    assert_eq!(10, tree.len());
}

fn is_sorted<I>(mut it: I) -> bool
where
    I: Iterator,
    I::Item: Ord,
{
    match it.next() {
        None => true,
        Some(first) => it
            .scan(first, |state, next| {
                let cmp = *state <= next;
                *state = next;
                Some(cmp)
            })
            .all(|b| b),
    }
}
